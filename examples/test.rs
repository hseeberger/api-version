use anyhow::Context;
use api_version::{ApiVersionLayer, ApiVersions};
use axum::{Router, ServiceExt, response::IntoResponse, routing::get};
use tokio::net::TcpListener;
use tower::Layer;

const API_VERSIONS: ApiVersions<2> = ApiVersions::new([0, 1]);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/ready", get(ready))
        .route("/api/v0/test", get(ok_0))
        .route("/api/v1/test", get(ok_1));
    let app = ApiVersionLayer::new("/", API_VERSIONS).layer(app);

    let listener = TcpListener::bind(("0.0.0.0", 8080))
        .await
        .context("bind TcpListener")?;
    axum::serve(listener, app.into_make_service())
        .await
        .context("run server")
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
