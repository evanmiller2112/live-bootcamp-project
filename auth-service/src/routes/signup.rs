use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use email_address::*;

use crate::{app_state::AppState, domain::User};
use crate::domain::AuthAPIError;

pub async fn signup(
    state: State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // Create a new `User` instance using data in the `request`
    let user = User::new(request.email, request.password, request.requires_2fa);
    let mut user_store = state.user_store.write().await;

    // check for email not valid
    if !EmailAddress::is_valid(&user.email) {
        return Err(AuthAPIError::InvalidCredentials);
    }

    // Check for password too short.
    if user.password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials);
    }

    // If user exists, throw that error.
    if let Ok(User) = user_store.get_user(&user.email).await {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    // If there's an error, throw an error
    if let Err(e) = user_store.add_user(user).await
    {
        return Err(AuthAPIError::UnexpectedError);
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}


#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub message: String,
}