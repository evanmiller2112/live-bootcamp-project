use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, User},
    utils::auth::generate_auth_cookie,
};
use crate::routes::SignupResponse;

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = Email::parse(
        request.email.clone())
            .map_err(|_| AuthAPIError::InvalidCredentials);
    let password = Password::parse(
        request.password.clone())
            .map_err(|_| AuthAPIError::InvalidCredentials);

    let user_store = state.user_store.read().await;

    let auth_result = user_store.validate_user(&email, &password)
        .await.map_err(|_| AuthAPIError::IncorrectCredentials);

    // let response = Json(LoginResponse {
    //     message: "User authenticated successfully!".to_string(),
    // });
    // Ok((StatusCode::OK, response))
    let auth_cookie = todo!();
    let updated_jar = jar.add(auth_cookie);
    (updated_jar, Ok(StatusCode::OK.into_response()))
}


#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
}