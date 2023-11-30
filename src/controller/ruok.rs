use axum::response::IntoResponse;
use axum::routing::get;
use axum::{http::StatusCode, Router};

pub fn routes() -> Router {
    Router::new()
        // hello routes
        .route("/ruok", get(alive_check))
}

// curl -v http://172.31.255.20:8080/ruok
async fn alive_check() -> impl IntoResponse {
    StatusCode::OK
}
