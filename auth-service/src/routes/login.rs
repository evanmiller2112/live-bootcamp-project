use crate::domain::data_stores::UserStore;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::app_state::AppState;
use crate::domain::{Email, Password};
use crate::domain::error::AuthAPIError;
use crate::domain::user::User;

pub async fn login(State(state): State<AppState>, Json(request): Json<LoginRequest>) -> Result<impl IntoResponse, AuthAPIError> {
    let email =
        Email::parse(request.email.clone()).map_err(|_| AuthAPIError::UnprocessableEntity)?;
    let password =
        Password::parse(request.password.clone()).map_err(|_| AuthAPIError::UnprocessableEntity)?;
    let mut user_store = state.user_store.write().await;

    let validate_user = user_store.validate_user(&email, &password).await;
    if validate_user.is_err() {
        return Err(AuthAPIError::IncorrectCredentials);
    }
    let response = Json(LoginResponse {
        message: "User logged in successfully!".to_string(),
    });
    Ok((StatusCode::OK, response))

}


#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
}