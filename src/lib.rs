//! Axum middleware to rewrite a request such that a version prefix is added to the path.

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
    ops::Deref,
    sync::LazyLock,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::{debug, error};

static VERSION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^v(\d{1,4})$"#).expect("version regex is valid"));

/// Axum middleware to rewrite a request such that a version prefix is added to the path. This is
/// based on a set of API versions and an optional `"x-api-version"` custom HTTP header: if no such
/// header is present, the highest version is used. Yet this only applies to requests the URIs of
/// which pass a filter; others are not rewritten.
///
/// Notice that paths of a router wrapped with an `ApiVersionLayer` must not start with a version
/// prefix, e.g. `"/v0"`, else a `200 BadRequest` is returned.
///
/// # Examples
///
/// The middleware needs to be applied to the "root" router:
///
/// ```ignore
/// let app = Router::new()
///     .route("/", get(ok_0))
///     .route("/v0/test", get(ok_0))
///     .route("/v1/test", get(ok_1))
///     .route("/foo", get(ok_foo));
///
/// const API_VERSIONS: ApiVersions<2> = ApiVersions::new([0, 1]);
///
/// let mut app = ApiVersionLayer::new(API_VERSIONS, FooFilter).layer(app);
/// ```
#[derive(Clone)]
pub struct ApiVersionLayer<const N: usize, F> {
    versions: ApiVersions<N>,
    filter: F,
}

impl<const N: usize, F> ApiVersionLayer<N, F> {
    /// Create a new API version layer.
    pub fn new(versions: ApiVersions<N>, filter: F) -> Self {
        Self { versions, filter }
    }
}

impl<const N: usize, S, F> Layer<S> for ApiVersionLayer<N, F>
where
    F: ApiVersionFilter,
{
    type Service = ApiVersionService<N, S, F>;

    fn layer(&self, inner: S) -> Self::Service {
        ApiVersionService {
            inner,
            versions: self.versions,
            filter: self.filter.clone(),
        }
    }
}

/// API versions; a validated newtype for a `u16` array.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApiVersions<const N: usize>([u16; N]);

impl<const N: usize> ApiVersions<N> {
    /// Create API versions. The given numbers must not be empty, must be strictly monotonically
    /// increasing and less than `10_000`; otherwise `new` fails to compile in const contexts or
    /// panics otherwise.
    ///
    /// # Examples
    ///
    /// Strictly monotonically versions `1` and `2` are valid:
    ///
    /// ```
    /// # use api_version::ApiVersions;
    /// const VERSIONS: ApiVersions<2> = ApiVersions::new([1, 2]);;
    /// ```
    ///
    /// Empty versions or such that are not strictly monotonically increasing are invalid and fail
    /// to compile in const contexts or panic otherwise.
    ///
    /// ```compile_fail
    /// # use api_version::ApiVersions;
    /// /// API versions must not be empty!
    /// const VERSIONS: ApiVersions<0> = ApiVersions::new([]);
    /// /// API versions must be strictly monotonically increasing!
    /// const VERSIONS: ApiVersions<0> = ApiVersions::new([2, 1]);
    /// /// API versions must be within 0u16..10_000!
    /// const VERSIONS: ApiVersions<0> = ApiVersions::new([10_000]);
    /// ```
    pub const fn new(versions: [u16; N]) -> Self {
        assert!(!versions.is_empty(), "API versions must not be empty");
        assert!(
            is_monotonically_increasing(versions),
            "API versions must be strictly monotonically increasing"
        );
        assert!(
            versions[N - 1] < 10_000,
            "API versions must be within 0u16..10_000"
        );

        Self(versions)
    }
}

impl<const N: usize> Deref for ApiVersions<N> {
    type Target = [u16; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Filter to determine which requests are rewritten.
#[trait_variant::make(Send)]
pub trait ApiVersionFilter: Clone + Send + 'static {
    type Error: std::error::Error;

    /// Requests are only rewritten, if the given URI passes, i.e. results in `true`.
    async fn filter(&self, uri: &Uri) -> Result<bool, Self::Error>;
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

/// See [ApiVersionLayer].
#[derive(Clone)]
pub struct ApiVersionService<const N: usize, S, F> {
    inner: S,
    versions: ApiVersions<N>,
    filter: F,
}

impl<const N: usize, S, F> Service<Request> for ApiVersionService<N, S, F>
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
                    error!(error = chained_sources(error), "cannot apply filter");
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
/// starting with `'v'` followed by a number within `0u16..10_000` without leading zero, e.g. `v0`.
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

fn chained_sources<E>(error: E) -> String
where
    E: StdError,
{
    let mut sources = vec![];
    sources.push(error.to_string());

    let mut source = error.source();
    while let Some(s) = source {
        sources.push(s.to_string());
        source = s.source();
    }

    sources.join(": ")
}

const fn is_monotonically_increasing<const N: usize>(versions: [u16; N]) -> bool {
    if N < 2 {
        return true;
    }

    let mut n = 1;
    while n < N {
        if versions[n - 1] >= versions[n] {
            return false;
        }
        n += 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::{VERSION, is_monotonically_increasing};
    use assert_matches::assert_matches;

    #[test]
    fn test_x_api_header() {
        let version = VERSION
            .captures("v0")
            .and_then(|c| c.get(1))
            .map(|m| m.as_str());
        assert_matches!(version, Some("0"));

        let version = VERSION
            .captures("v1")
            .and_then(|c| c.get(1))
            .map(|m| m.as_str());
        assert_matches!(version, Some("1"));

        let version = VERSION
            .captures("v99")
            .and_then(|c| c.get(1))
            .map(|m| m.as_str());
        assert_matches!(version, Some("99"));

        let version = VERSION
            .captures("v9999")
            .and_then(|c| c.get(1))
            .map(|m| m.as_str());
        assert_matches!(version, Some("9999"));

        let version = VERSION
            .captures("v10000")
            .and_then(|c| c.get(1))
            .map(|m| m.as_str());
        assert_matches!(version, None);

        let version = VERSION
            .captures("vx")
            .and_then(|c| c.get(1))
            .map(|m| m.as_str());
        assert_matches!(version, None);
    }

    #[test]
    fn test_is_monotonically_increasing() {
        assert!(is_monotonically_increasing([]));
        assert!(is_monotonically_increasing([0]));
        assert!(is_monotonically_increasing([0, 1]));

        assert!(!is_monotonically_increasing([0, 0]));
        assert!(!is_monotonically_increasing([1, 0]));
    }
}
