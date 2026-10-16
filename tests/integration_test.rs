use api_version::{ApiVersionLayer, ApiVersions, X_API_VERSION};
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
};
use futures::{TryStreamExt, future::ok};
use std::iter::Extend;
use tower::{Layer, Service, ServiceExt, limit::ConcurrencyLimitLayer};

const API_VERSIONS: ApiVersions<2> = ApiVersions::new([0, 1]);

#[tokio::test]
async fn test() {
    let app = Router::new()
        .route("/ready", get(ready))
        .route("/api/v0", get(ok_0))
        .route("/api/v0/test", get(ok_0))
        .route("/api/v1/test", get(ok_1))
        .route("/api-doc/openapi.json", get(api_doc));

    let mut app = ApiVersionLayer::new("/api", API_VERSIONS).layer(app);

    // Verify that the base path is working.
    let request = Request::builder()
        .uri("/ready")
        .body(Body::empty())
        .unwrap();
    let response = app.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(text(response).await, "ready");

    // No version should return the highest version.
    let request = Request::builder()
        .uri("/api/test")
        .body(Body::empty())
        .unwrap();
    let response = app.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(text(response).await, "1");

    // Existing version.
    let request = Request::builder()
        .uri("/api/test")
        .header(&X_API_VERSION, "v0")
        .body(Body::empty())
        .unwrap();
    let response = app.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(text(response).await, "0");

    // Another existing version.
    let request = Request::builder()
        .uri("/api/test")
        .header(&X_API_VERSION, "v1")
        .body(Body::empty())
        .unwrap();
    let response = app.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(text(response).await, "1");

    // Non-existing version.
    let request = Request::builder()
        .uri("/api/test")
        .header(&X_API_VERSION, "v2")
        .body(Body::empty())
        .unwrap();
    let response = app.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // Invalid version.
    for version in ["1", "vx", "v10000"] {
        let request = Request::builder()
            .uri("/api/test")
            .header(&X_API_VERSION, version)
            .body(Body::empty())
            .unwrap();
        let response = app.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    // Valid version prefix (existing version).
    let request = Request::builder()
        .uri("/api/v0/test")
        .body(Body::empty())
        .unwrap();
    let response = app.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(text(response).await, "0");

    // Valid version prefix without trailing path.
    let request = Request::builder()
        .uri("/api/v0")
        .body(Body::empty())
        .unwrap();
    let response = app.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(text(response).await, "0");

    // Invalid version prefix (nonexistent version).
    let request = Request::builder()
        .uri("/api/v2/test")
        .body(Body::empty())
        .unwrap();
    let response = app.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // Different base path with same prefix.
    let request = Request::builder()
        .uri("/api-doc/openapi.json")
        .body(Body::empty())
        .unwrap();
    let response = app.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(text(response).await, "api-doc");
}

/// The inner service made ready by `poll_ready` is the one handling the request.
#[tokio::test]
async fn test_inner_service_readiness() {
    let app = Router::new().route("/api/v1/test", get(ok_1));
    let app = ConcurrencyLimitLayer::new(1).layer(app);

    let mut app = ApiVersionLayer::new("/api", API_VERSIONS).layer(app);

    let request = Request::builder()
        .uri("/api/test")
        .body(Body::empty())
        .unwrap();
    let response = app.ready().await.unwrap().call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(text(response).await, "1");
}

#[test]
#[should_panic(expected = "base path must not be empty")]
fn test_empty_base_path() {
    ApiVersionLayer::new("", API_VERSIONS);
}

#[test]
#[should_panic(expected = "base path must not be empty")]
fn test_root_base_path() {
    ApiVersionLayer::new("/", API_VERSIONS);
}

#[test]
#[should_panic(expected = "base path must start with '/'")]
fn test_relative_base_path() {
    ApiVersionLayer::new("api", API_VERSIONS);
}

async fn ready() -> impl IntoResponse {
    "ready"
}

async fn ok_0() -> impl IntoResponse {
    "0"
}

async fn ok_1() -> impl IntoResponse {
    "1"
}

async fn api_doc() -> impl IntoResponse {
    "api-doc"
}

async fn text(response: Response) -> String {
    let text = response
        .into_body()
        .into_data_stream()
        .try_fold(vec![], |mut acc, bytes| {
            acc.extend(bytes);
            ok(acc)
        })
        .await;
    assert!(text.is_ok());
    String::from_utf8(text.unwrap()).unwrap()
}
