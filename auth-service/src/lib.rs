use std::error::Error;
use axum::{response::Html, routing::get, Router, serve::Serve, BoxError};
use tower_http::services::ServeDir;
use log::error;

// application related logic
pub struct Application {
    server: Serve<Router, Router>,
    // 'address' is public so we can use it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let router: Router = axum::Router::new();

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application {server: server, address: address})
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}