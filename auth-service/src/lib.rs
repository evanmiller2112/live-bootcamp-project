use axum::body::Body;
use axum::{
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Json, Router,

};
use std::error::Error;
use serde::{Deserialize, Serialize};
use tower_http::{cors::CorsLayer, services::ServeDir};
use routes::*;
use app_state::AppState;
use crate::domain::error::AuthAPIError;

pub mod routes;
pub mod domain;
pub mod services;
pub mod app_state;
pub mod utils;

// application related logic
pub struct Application {
    server: Serve<Router, Router>,
    // 'address' is public so we can use it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        // Allow the app service(running on our local machine and in production) to call the auth service
        let allowed_origins = [
            "http://localhost:8000".parse()?,
        ];

        // Set up CORS
        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router: Router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify_2fa))
            .route("/verify-token", post(verify_token))
            .with_state(app_state)
            .layer(cors);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Self {server: server, address: address})
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response<Body> {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists =>
                    (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials =>
                    (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::UnexpectedError => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
                }
            AuthAPIError::IncorrectCredentials =>
                    (StatusCode::UNAUTHORIZED, "Incorrect credentials"),
            AuthAPIError::UnprocessableEntity =>
                    (StatusCode::UNPROCESSABLE_ENTITY, "Input Unprocessable or Malformed"),
            AuthAPIError::MissingToken =>
                    (StatusCode::BAD_REQUEST, "Missing auth token"),
            AuthAPIError::InvalidToken =>
                    (StatusCode::UNAUTHORIZED, "Invalid auth token"),
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}