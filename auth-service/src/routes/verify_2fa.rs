use axum::{http::StatusCode, response::IntoResponse};

pub async fn get_verify2fa() -> impl IntoResponse {
    StatusCode::OK.into_response()
}