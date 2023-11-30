use axum::response::IntoResponse;
use axum::routing::get;
use axum::{http::StatusCode, Json, Router};
use serde::Serialize;
use uuid::Uuid;

use crate::konst::ROOT_MESSAGE;

#[derive(Serialize)]
pub struct Message {
    pub request_id: String,
    pub message: String,
}

pub fn routes() -> Router {
    Router::new().route("/", get(index))
}

async fn index() -> impl IntoResponse {
    let id = Uuid::new_v4();
    let response = Message {
        request_id: id.to_string(),
        message: ROOT_MESSAGE.to_string(),
    };
    (StatusCode::OK, Json(response))
}
