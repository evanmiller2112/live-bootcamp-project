use axum::{http::StatusCode, response::IntoResponse};

pub async fn get_logout() -> impl IntoResponse {
    StatusCode::OK.into_response()
}