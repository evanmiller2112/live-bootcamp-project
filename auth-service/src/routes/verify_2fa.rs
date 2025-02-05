use serde::{Serialize, Deserialize};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use crate::{
    app_state::AppState,
    domain::{LoginAttemptId, TwoFACode, Email, AuthAPIError}
};

pub async fn verify_2fa(
    State(state): State<AppState>,
    Json(request): Json<Verify2FARequest>
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };
    
    let login_attempt_id = match LoginAttemptId::parse(request.login_attempt_id) {
        Ok(login_attempt_id) => login_attempt_id,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };
    
    let two_fa_code = match TwoFACode::parse(request.two_factor_code) {
        Ok(two_factor_code) => two_factor_code,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };
    
    let mut two_fa_code_store = state.two_fa_code_store.write().await;
    
    let code_tuple = match two_fa_code_store.get_code(&email).await {
        Ok(code_tuple) => code_tuple,
        Err(_) => return Err(AuthAPIError::IncorrectCredentials),
    };
    
    if !code_tuple.0.eq(&login_attempt_id) || !code_tuple.1.eq(&two_fa_code) {
        return Err(AuthAPIError::InvalidCredentials);
    }
    
    if two_fa_code_store.remove_code(&email).await.is_err() {
        return Err(AuthAPIError::UnexpectedError);
    }
    
    Ok(StatusCode::OK.into_response())
    
}


// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct Verify2FARequest {
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    pub two_factor_code: String,
}