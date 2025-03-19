use api_version::{ApiVersionFilter, ApiVersionLayer, ApiVersions, X_API_VERSION};
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
};
use futures::{TryStreamExt, future::ok};
use std::{convert::Infallible, iter::Extend};
use tower::{Layer, Service};

#[tokio::test]
async fn test() {
    let app = Router::new()
        .route("/", get(ready))
        .route("/v0/test", get(ok_0))
        .route("/v1/test", get(ok_1));

    const API_VERSIONS: ApiVersions<2> = ApiVersions::new([0, 1]);

    let mut app = ApiVersionLayer::new(API_VERSIONS, ReadyFilter).layer(app);

    // Verify that the filter is working.
    let request = Request::builder().uri("/").body(Body::empty()).unwrap();
    let response = app.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(text(response).await, "ready");

    // No version should return the highest version.
    let request = Request::builder().uri("/test").body(Body::empty()).unwrap();
    let response = app.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(text(response).await, "1");

    // Existing version.
    let request = Request::builder()
        .uri("/test")
        .header(&X_API_VERSION, "v0")
        .body(Body::empty())
        .unwrap();
    let response = app.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(text(response).await, "0");

    // Another existing version.
    let request = Request::builder()
        .uri("/test")
        .header(&X_API_VERSION, "v1")
        .body(Body::empty())
        .unwrap();
    let response = app.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(text(response).await, "1");

    // Non-existing version.
    let request = Request::builder()
        .uri("/test")
        .header(&X_API_VERSION, "v2")
        .body(Body::empty())
        .unwrap();
    let response = app.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // Valid version prefix (existing version).
    let request = Request::builder()
        .uri("/v0/test")
        .body(Body::empty())
        .unwrap();
    let response = app.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(text(response).await, "0");

    // Invalid version prefix (nonexistent version).
    let request = Request::builder()
        .uri("/v2/test")
        .body(Body::empty())
        .unwrap();
    let response = app.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[derive(Clone)]
struct ReadyFilter;

impl ApiVersionFilter for ReadyFilter {
    type Error = Infallible;

    async fn should_rewrite(&self, uri: &Uri) -> Result<bool, Self::Error> {
        Ok(uri.path() != "/")
    }
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
