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
use tower::{Layer, Service};

const API_VERSIONS: ApiVersions<2> = ApiVersions::new([0, 1]);

#[tokio::test]
async fn test() {
    let app = Router::new()
        .route("/ready", get(ready))
        .route("/api/v0/test", get(ok_0))
        .route("/api/v1/test", get(ok_1));

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

    // Valid version prefix (existing version).
    let request = Request::builder()
        .uri("/api/v0/test")
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
