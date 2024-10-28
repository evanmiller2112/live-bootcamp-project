use crate::domain::UserStoreError;
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
            ) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let password = match Password::parse(request.password) {
        Ok(password) => password,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };
    let user_store = &state.user_store.write().await;

    if user_store.validate_user(&email, &password).await.is_err() {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    let auth_cookie = match generate_auth_cookie(&user.email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };
    
    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok((StatusCode::OK.into_response())))

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