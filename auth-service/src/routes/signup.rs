use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::app_state::AppState;
use crate::domain::user::User;
use crate::services::hashmap_user_store::HashmapUserStore;

pub async fn signup(State(state): State<AppState>, 
                    Json(request): Json<SignupRequest>,
                    ) -> impl IntoResponse {
    // Create a new `User` instance using data in the `request`
    let email = request.email.clone();
    let password = request.password.clone();
    let requires_2fa = request.requires_2fa;
    let user = User::new(email, password, requires_2fa);
    let mut user_store = state.user_store.write().await;

    let add_user = user_store.add_user(user);

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    (StatusCode::CREATED, response)
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}