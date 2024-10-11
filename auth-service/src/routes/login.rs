use axum::response::IntoResponse;
use axum::http::StatusCode;

pub async fn login() -> impl IntoResponse {
    StatusCode::OK.into_response()
}