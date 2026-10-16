//! Axum middleware to rewrite a request such that a version prefix, e.g. `"/v0"`, is added to the
//! path.

#![warn(missing_docs)]

use axum::{
    extract::Request,
    http::{HeaderName, HeaderValue, StatusCode, Uri, uri::PathAndQuery},
    response::{IntoResponse, Response},
};
use axum_extra::headers::{self, Header, HeaderMapExt};
use futures::future::BoxFuture;
use std::{
    iter, mem,
    ops::Deref,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::debug;

/// Header name for the [`XApiVersion`] custom HTTP header.
pub static X_API_VERSION: HeaderName = HeaderName::from_static("x-api-version");

/// Axum middleware to rewrite a request such that a version prefix is added to the path. This is
/// based on a set of API versions and an optional `"x-api-version"` custom HTTP header: if no such
/// header is present, the highest version is used; if it is invalid, the request is rejected with
/// `400 Bad Request`. Yet this only applies to requests the URIs of which start with the given
/// base path, e.g. "/api"; others are not rewritten. Also, paths starting with a valid/existing
/// version prefix, e.g. `"/api/v0"`, are not rewritten.
///
/// # Examples
///
/// The middleware needs to be applied to the "root" router:
///
/// ```ignore
/// let app = Router::new()
///     .route("/ready", get(ok_0))
///     .route("/api/v0/test", get(ok_0))
///     .route("/api/v1/test", get(ok_1));
///
/// const API_VERSIONS: ApiVersions<2> = ApiVersions::new([0, 1]);
///
/// let mut app = ApiVersionLayer::new("/api", API_VERSIONS).layer(app);
/// ```
#[derive(Clone)]
pub struct ApiVersionLayer<const N: usize> {
    base_path: String,
    versions: ApiVersions<N>,
}

impl<const N: usize> ApiVersionLayer<N> {
    /// Create a new API version layer with the given base path and api versions. Trailing slashes
    /// are removed from the base path.
    ///
    /// # Panics
    ///
    /// Panics if the base path without trailing slashes is empty, e.g. for "/", or does not start
    /// with "/".
    pub fn new(base_path: impl AsRef<str>, versions: ApiVersions<N>) -> Self {
        let base_path = base_path.as_ref().trim_end_matches('/').to_string();
        assert!(!base_path.is_empty(), "base path must not be empty");
        assert!(base_path.starts_with('/'), "base path must start with '/'");

        Self {
            base_path,
            versions,
        }
    }
}

impl<const N: usize, S> Layer<S> for ApiVersionLayer<N> {
    type Service = ApiVersionService<N, S>;

    fn layer(&self, inner: S) -> Self::Service {
        ApiVersionService {
            inner,
            base_path: self.base_path.clone(),
            versions: self.versions,
        }
    }
}

/// API versions; a validated newtype for a `u16` array.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApiVersions<const N: usize>([u16; N]);

impl<const N: usize> ApiVersions<N> {
    /// Create API versions. The given numbers must not be empty, must be strictly monotonically
    /// increasing and less than `10_000`; otherwise `new` fails to compile in const contexts or
    /// panics at runtime.
    ///
    /// # Examples
    ///
    /// Strictly monotonically increasing versions `1` and `2` are valid:
    ///
    /// ```
    /// # use api_version::ApiVersions;
    /// const VERSIONS: ApiVersions<2> = ApiVersions::new([1, 2]);
    /// ```
    ///
    /// # Panics
    ///
    /// Invalid versions fail to compile in const contexts or panic otherwise. Versions must not be
    /// empty:
    ///
    /// ```compile_fail,E0080
    /// # use api_version::ApiVersions;
    /// const VERSIONS: ApiVersions<0> = ApiVersions::new([]);
    /// ```
    ///
    /// Versions must be strictly monotonically increasing:
    ///
    /// ```compile_fail,E0080
    /// # use api_version::ApiVersions;
    /// const VERSIONS: ApiVersions<2> = ApiVersions::new([2, 1]);
    /// ```
    ///
    /// Versions must be less than `10_000`:
    ///
    /// ```compile_fail,E0080
    /// # use api_version::ApiVersions;
    /// const VERSIONS: ApiVersions<1> = ApiVersions::new([10_000]);
    /// ```
    #[must_use]
    pub const fn new(versions: [u16; N]) -> Self {
        assert!(!versions.is_empty(), "API versions must not be empty");
        assert!(
            is_strictly_increasing(versions),
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

/// See [`ApiVersionLayer`].
#[derive(Clone)]
pub struct ApiVersionService<const N: usize, S> {
    inner: S,
    base_path: String,
    versions: ApiVersions<N>,
}

impl<const N: usize, S> Service<Request> for ApiVersionService<N, S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request) -> Self::Future {
        let inner = self.inner.clone();
        let mut inner = mem::replace(&mut self.inner, inner);
        let base_path = self.base_path.clone();
        let versions = self.versions;

        Box::pin(async move {
            // Strip base path prefix or return without rewriting.
            let path = if let Some(path) = request.uri().path().strip_prefix(&base_path)
                && path.starts_with('/')
            {
                path.to_owned()
            } else {
                debug!(
                    uri = %request.uri(),
                    "not rewriting the path, because it does not start with base path"
                );
                return inner.call(request).await;
            };

            // Return without rewriting if stripped path starts with valid version prefix.
            let has_version_prefix =
                path_version(&path).is_some_and(|version| versions.contains(&version));
            if has_version_prefix {
                debug!(
                    uri = %request.uri(),
                    "not rewriting the path, because it starts with valid version prefix"
                );
                return inner.call(request).await;
            }

            // Determine version.
            let version = match request.headers().typed_try_get::<XApiVersion>() {
                Ok(Some(XApiVersion(version))) => version,

                Ok(None) => versions[N - 1],

                Err(_) => {
                    let response = (
                        StatusCode::BAD_REQUEST,
                        format!("invalid header '{X_API_VERSION}'"),
                    );
                    return Ok(response.into_response());
                }
            };
            if !versions.contains(&version) {
                let response = (
                    StatusCode::NOT_FOUND,
                    format!("unknown version '{version}'"),
                );
                return Ok(response.into_response());
            }
            debug!(?version, "using API version");

            // Insert version prefix into request URI.
            let mut parts = request.uri().to_owned().into_parts();
            let paq = parts.path_and_query.expect("uri has 'path and query'");
            let mut paq_parts = paq.as_str().split('?').skip(1);
            let paq = match paq_parts.next() {
                Some(query) => format!("{base_path}/v{version}{path}?{query}"),
                None => format!("{base_path}/v{version}{path}"),
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

/// Custom HTTP header conveying the API version: `'v'` followed by a number from `0` to `9999`
/// without leading zeros, e.g. `v0` or `v42`, but not `v01`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XApiVersion(u16);

impl XApiVersion {
    /// The API version number, e.g. `42` for `v42`.
    #[must_use]
    pub fn version(self) -> u16 {
        self.0
    }
}

impl Header for XApiVersion {
    fn name() -> &'static HeaderName {
        &X_API_VERSION
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .next()
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix('v'))
            .and_then(parse_version)
            .map(XApiVersion)
            .ok_or_else(headers::Error::invalid)
    }

    fn encode<E>(&self, values: &mut E)
    where
        E: Extend<HeaderValue>,
    {
        let value = HeaderValue::try_from(format!("v{}", self.0))
            .expect("version designator is a valid header value");
        values.extend(iter::once(value));
    }
}

const fn is_strictly_increasing<const N: usize>(versions: [u16; N]) -> bool {
    let mut n = 1;
    while n < N {
        if versions[n - 1] >= versions[n] {
            return false;
        }
        n += 1;
    }

    true
}

fn path_version(path: &str) -> Option<u16> {
    let digits = path.strip_prefix("/v")?;
    let end = digits.find('/').unwrap_or(digits.len());
    parse_version(&digits[..end])
}

fn parse_version(digits: &str) -> Option<u16> {
    let digits = digits.as_bytes();
    if digits.is_empty()
        || digits.len() > 4
        || !digits.iter().all(u8::is_ascii_digit)
        || (digits[0] == b'0' && digits.len() > 1)
    {
        return None;
    }

    Some(
        digits
            .iter()
            .fold(0, |version, digit| version * 10 + u16::from(digit - b'0')),
    )
}

#[cfg(test)]
mod tests {
    use crate::{is_strictly_increasing, parse_version, path_version};

    #[test]
    fn test_parse_version() {
        assert_eq!(parse_version("0"), Some(0));
        assert_eq!(parse_version("1"), Some(1));
        assert_eq!(parse_version("99"), Some(99));
        assert_eq!(parse_version("9999"), Some(9999));

        assert_eq!(parse_version(""), None);
        assert_eq!(parse_version("x"), None);
        assert_eq!(parse_version("1x"), None);
        assert_eq!(parse_version("-1"), None);
        assert_eq!(parse_version("+1"), None);
        assert_eq!(parse_version("00"), None);
        assert_eq!(parse_version("01"), None);
        assert_eq!(parse_version("10000"), None);
        assert_eq!(parse_version("\u{663}"), None);
    }

    #[test]
    fn test_path_version() {
        assert_eq!(path_version("/v0"), Some(0));
        assert_eq!(path_version("/v0/test"), Some(0));
        assert_eq!(path_version("/v10/test"), Some(10));

        assert_eq!(path_version("/"), None);
        assert_eq!(path_version("/test"), None);
        assert_eq!(path_version("/v"), None);
        assert_eq!(path_version("/v/test"), None);
        assert_eq!(path_version("/v1x/test"), None);
        assert_eq!(path_version("/v01/test"), None);
        assert_eq!(path_version("/v10000/test"), None);
    }

    #[test]
    fn test_is_strictly_increasing() {
        assert!(is_strictly_increasing([]));
        assert!(is_strictly_increasing([0]));
        assert!(is_strictly_increasing([0, 1]));

        assert!(!is_strictly_increasing([0, 0]));
        assert!(!is_strictly_increasing([1, 0]));
    }
}
