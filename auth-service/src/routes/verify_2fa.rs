use axum::response::IntoResponse;
use axum::http::StatusCode;

pub async fn verify_2fa() -> impl IntoResponse {
    StatusCode::OK.into_response()
}