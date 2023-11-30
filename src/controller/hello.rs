use axum::extract;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{http::StatusCode, Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
struct HelloData {
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct HelloResponse {
    pub request_id: String,
    pub name: String,
}

pub fn routes() -> Router {
    Router::new()
        // hello routes
        .route("/hello", get(hello_query).post(hello_body))
        .route("/hello/:name", get(hello_name))
}

// curl -v http://172.31.255.20:8080/hello?name=jimbo
async fn hello_query(extract::Query(params): extract::Query<HelloData>) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let response = HelloResponse {
        request_id: id.to_string(),
        name: params.name,
    };
    (StatusCode::OK, Json(response))
}

// curl -v -X POST http://172.31.255.20:8080/hello -H "Content-Type: application/json" -d '{"name":"jimbo"}'
async fn hello_body(extract::Json(params): extract::Json<HelloData>) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let response = HelloResponse {
        request_id: id.to_string(),
        name: params.name,
    };
    (StatusCode::OK, Json(response))
}

// curl -v http://172.31.255.20:8080/hello/jimbo
async fn hello_name(extract::Path(params): extract::Path<HelloData>) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let response = HelloResponse {
        request_id: id.to_string(),
        name: params.name,
    };
    (StatusCode::OK, Json(response))
}
