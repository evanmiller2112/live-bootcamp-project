use axum::{response::Html, routing::get, Router};
use tower_http::services::ServeDir;
use auth_service::Application;

#[tokio::main]
async fn main() {
    let app = Application::build("0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");

}

async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Hello, Rust Pals!</h1>")
}
