use tokio::net::TcpListener;

use anyhow::Result;
use axum::{Router, ServiceExt};

use tower::Layer;
use tower_http::normalize_path::{NormalizePath, NormalizePathLayer};
use tracing::instrument;

use crate::controller;
// use crate::shutdown::shutdown_handler;

pub fn app() -> NormalizePath<Router> {
    let router = Router::new()
        .merge(controller::root::routes())
        .merge(controller::ruok::routes())
        .merge(controller::hello::routes())
        // fallback route
        .fallback(controller::error::not_found_404);
    // .layer(NormalizePathLayer::trim_trailing_slash());
    NormalizePathLayer::trim_trailing_slash().layer(router)
}

#[instrument]
pub async fn run(listener: TcpListener, app: NormalizePath<Router>) -> Result<()> {
    tracing::debug!(target: "pilot_axum", "listening on {}", listener.local_addr()?);

    axum::serve(listener, app.into_inner());
    Ok(())
}
