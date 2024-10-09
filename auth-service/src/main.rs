use auth_service::Application;
use axum::{response::Html, routing::get, Router};
use tower_http::services::ServeDir;



#[tokio::main]
async fn main() {
    let app = Application::build("0.0.0.0:3000")
        .await
        .expect("Failed to build app.");

    app.run().await.expect("Failed to run app");
}

async fn hello_handler() -> Html<&'static str> {
    // TODO: Update this to a custom message!
    Html("<h1>Hello, World!</h1>")
}
