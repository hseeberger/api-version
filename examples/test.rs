use anyhow::Context;
use api_version::{ApiVersionFilter, ApiVersionLayer, ApiVersions};
use axum::{Router, ServiceExt, http::Uri, response::IntoResponse, routing::get};
use std::convert::Infallible;
use tokio::net::TcpListener;
use tower::Layer;

const API_VERSIONS: ApiVersions<2> = ApiVersions::new([0, 1]);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(ready))
        .route("/v0/test", get(ok_0))
        .route("/v1/test", get(ok_1));
    let app = ApiVersionLayer::new(API_VERSIONS, ReadyFilter).layer(app);

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

#[derive(Clone)]
struct ReadyFilter;

impl ApiVersionFilter for ReadyFilter {
    type Error = Infallible;

    async fn should_rewrite(&self, uri: &Uri) -> Result<bool, Self::Error> {
        Ok(uri.path() != "/")
    }
}
