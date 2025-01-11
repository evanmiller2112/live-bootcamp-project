// This struct encapsulates our application-related logic.
use axum::routing::post;
use axum::routing::get;
use axum::Router;
use axum::serve::Serve;
use std::error::Error;
use axum::response::IntoResponse;
use tower_http::services::ServeDir;
use routes::{login, signup, logout, verify_2fa, verify_token};

mod routes;

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/login", post(login))
            .route("/signup", post(signup))
            .route("/logout", post(logout::get_logout))
            .route("/verify-2fa", post(verify_2fa::get_verify2fa))
            .route("/verify-token", post(verify_token::verify_token));


        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}