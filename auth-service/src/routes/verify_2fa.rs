use serde::{Serialize, Deserialize};
use axum::{http::StatusCode, response::IntoResponse, Json};
use crate::domain::{LoginAttemptId, TwoFACode, Email, AuthAPIError};

pub async fn verify_2fa(Json(request): Json<Verify2FARequest>) -> impl IntoResponse {
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