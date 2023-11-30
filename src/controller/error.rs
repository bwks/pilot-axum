use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn not_found_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
