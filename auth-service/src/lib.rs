use std::error::Error;
use axum::{http::StatusCode, response::IntoResponse,
           routing::post, serve::Serve, Router};
use tower_http::services::ServeDir;
use routes::*;

pub mod routes;

// application related logic
pub struct Application {
    server: Serve<Router, Router>,
    // 'address' is public so we can use it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let router: Router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify_2fa))
            .route("/verify-token", post(verify_token));

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









