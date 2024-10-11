use axum::response::IntoResponse;
use axum::http::StatusCode;

pub async fn logout() -> impl IntoResponse {
    StatusCode::OK.into_response()
}