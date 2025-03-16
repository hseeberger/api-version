pub use array_macro;

use axum::{
    RequestExt,
    extract::Request,
    http::{HeaderName, HeaderValue, StatusCode, Uri, uri::PathAndQuery},
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{self, Header},
};
use futures::future::BoxFuture;
use regex::Regex;
use std::{
    convert::Infallible,
    error::Error as StdError,
    fmt::Debug,
    future::Future,
    sync::LazyLock,
    task::{Context, Poll},
};
use thiserror::Error;
use tower::{Layer, Service};
use tracing::{debug, error};

/// Create an [ApiVersionLayer] correctly initialized with non-empty and strictly monotonically
/// increasing versions in the given inclusive range.
#[macro_export]
macro_rules! api_version {
    ($from:literal..=$to:literal) => {
        {
            $crate::api_version!($from..=$to, $crate::All)
        }
    };

    ($from:literal..=$to:literal, $filter:expr) => {
        {
            let versions = $crate::array_macro::array![n => n as u16 + $from; $to - $from + 1];
            $crate::ApiVersionLayer::new(versions, $filter).expect("versions are valid")
        }
    };
}

static VERSION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^v(0|[1-9][0-9]?)$"#).expect("version regex is valid"));

/// Axum middleware to rewrite a request such that a version prefix is added to the path. This is
/// based on a set of versions and an optional `"x-api-version"` custom HTTP header: if no such
/// header is present, the highest version is used. Yet this only applies to requests the URIs of
/// which pass a filter; others are not rewritten.
///
/// Paths must not start with a version prefix, e.g. `"/v0"`.
#[derive(Clone)]
pub struct ApiVersionLayer<const N: usize, F> {
    versions: [u16; N],
    filter: F,
}

impl<const N: usize, F> ApiVersionLayer<N, F> {
    /// Create a new [ApiVersionLayer].
    ///
    /// The given versions must not be empty and must be strictly monotonically increasing, e.g.
    /// `[0, 1, 2]`.
    pub fn new(versions: [u16; N], filter: F) -> Result<Self, NewApiVersionLayerError> {
        if versions.is_empty() {
            return Err(NewApiVersionLayerError::Empty);
        }

        if versions.as_slice().windows(2).any(|w| w[0] >= w[1]) {
            return Err(NewApiVersionLayerError::NotIncreasing);
        }

        Ok(Self { versions, filter })
    }
}

impl<const N: usize, S, F> Layer<S> for ApiVersionLayer<N, F>
where
    F: ApiVersionFilter,
{
    type Service = ApiVersion<N, S, F>;

    fn layer(&self, inner: S) -> Self::Service {
        ApiVersion {
            inner,
            versions: self.versions,
            filter: self.filter.clone(),
        }
    }
}

/// Determine which requests are rewritten.
pub trait ApiVersionFilter: Clone + Send + 'static {
    type Error: std::error::Error;

    /// Requests are only rewritten, if the given URI passes, i.e. results in `true`.
    fn filter(&self, uri: &Uri) -> impl Future<Output = Result<bool, Self::Error>> + Send;
}

/// [ApiVersionFilter] making all requests be rewritten.
#[derive(Clone, Copy)]
pub struct All;

impl ApiVersionFilter for All {
    type Error = Infallible;

    async fn filter(&self, _uri: &Uri) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

/// Error creating an [ApiVersionLayer].
#[derive(Debug, Error)]
pub enum NewApiVersionLayerError {
    #[error("versions must not be empty")]
    Empty,

    #[error("versions must be strictly monotonically increasing")]
    NotIncreasing,
}

/// See [ApiVersionLayer].
#[derive(Clone)]
pub struct ApiVersion<const N: usize, S, F> {
    inner: S,
    versions: [u16; N],
    filter: F,
}

impl<const N: usize, S, F> Service<Request> for ApiVersion<N, S, F>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
    F: ApiVersionFilter,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request) -> Self::Future {
        let mut inner = self.inner.clone();
        let versions = self.versions;
        let filter = self.filter.clone();

        Box::pin(async move {
            // Do not allow the path to start with one of the valid version prefixes.
            if versions
                .iter()
                .any(|version| request.uri().path().starts_with(&format!("/v{version}")))
            {
                let response = (
                    StatusCode::BAD_REQUEST,
                    "path must not start with version prefix like '/v0'",
                );
                return Ok(response.into_response());
            }

            let pass_through = match filter.filter(request.uri()).await {
                Ok(pass_through) => pass_through,

                Err(error) => {
                    error!(error = error.as_chain(), "cannot apply filter");
                    return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
                }
            };

            if !pass_through {
                debug!(uri = %request.uri(), "not rewriting the path");
                return inner.call(request).await;
            }

            // Determine API version.
            let version = request.extract_parts::<TypedHeader<XApiVersion>>().await;
            let version = version
                .as_ref()
                .map(|TypedHeader(XApiVersion(v))| v)
                .unwrap_or_else(|_| versions.last().expect("versions is not empty"));
            if !versions.contains(version) {
                let response = (
                    StatusCode::NOT_FOUND,
                    format!("unknown version '{version}'"),
                );
                return Ok(response.into_response());
            }
            debug!(?version, "using API version");

            // Prepend the suitable prefix to the request URI.
            let mut parts = request.uri().to_owned().into_parts();
            let paq = parts.path_and_query.expect("uri has 'path and query'");
            let mut paq_parts = paq.as_str().split('?');
            let path = paq_parts.next().expect("uri has path");
            let paq = match paq_parts.next() {
                Some(query) => format!("/v{version}{path}?{query}"),
                None if path != "/" => format!("/v{version}{path}"),
                None => format!("/v{version}"),
            };
            let paq = PathAndQuery::from_maybe_shared(paq).expect("new 'path and query' is valid");
            parts.path_and_query = Some(paq);
            let uri = Uri::from_parts(parts).expect("parts are valid");

            // Rewrite the request URI and run the downstream services.
            debug!(original_uri = %request.uri(), %uri, "rewrote the path");
            request.uri_mut().clone_from(&uri);
            inner.call(request).await
        })
    }
}

/// Header name for the [XApiVersion] custom HTTP header.
pub static X_API_VERSION: HeaderName = HeaderName::from_static("x-api-version");

/// Custom HTTP header conveying the API version, which is expected to be a version designator
/// starting with `'v'` followed by a number from 0..+99 without leading zero, e.g. `v0`.
#[derive(Debug)]
pub struct XApiVersion(u16);

impl Header for XApiVersion {
    fn name() -> &'static HeaderName {
        &X_API_VERSION
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .next()
            .and_then(|v| v.to_str().ok())
            .and_then(|s| VERSION.captures(s).and_then(|c| c.get(1)))
            .and_then(|m| m.as_str().parse().ok())
            .map(XApiVersion)
            .ok_or_else(headers::Error::invalid)
    }

    fn encode<E: Extend<HeaderValue>>(&self, _values: &mut E) {
        // We do not yet need to encode this header.
        unimplemented!("not yet needed");
    }
}

trait StdErrorExt
where
    Self: StdError,
{
    fn as_chain(&self) -> String {
        let mut sources = vec![];
        sources.push(self.to_string());

        let mut source = self.source();
        while let Some(s) = source {
            sources.push(s.to_string());
            source = s.source();
        }

        sources.join(": ")
    }
}

impl<T> StdErrorExt for T where T: StdError {}
