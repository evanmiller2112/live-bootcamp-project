use crate::domain::data_stores::UserStore;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use axum::Json;
use axum_extra::extract::cookie::Cookie;
use serde::{Deserialize, Serialize};
use crate::domain::user::User;
use crate::{
    app_state::AppState,
    domain::{error::AuthAPIError, Email, Password},
    utils::auth::generate_auth_cookie,
};
pub async fn login(
        State(state): State<AppState>,
        jar: CookieJar,
        Json(request): Json<LoginRequest>
            ) -> Result<impl IntoResponse, AuthAPIError> {
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
    

    let auth_cookie = 
        generate_auth_cookie(&email).map_err(|_| AuthAPIError::UnexpectedError)?;

    let updated_jar = jar.add(auth_cookie);

    Ok((updated_jar, (StatusCode::OK.into_response())))

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