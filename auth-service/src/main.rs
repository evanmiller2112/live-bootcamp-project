use auth_service::app_state::AppState;
use auth_service::services::hashmap_user_store::HashmapUserStore as HashmapUserStore;
use axum::{response::Html, routing::get, Router};
use tower_http::services::ServeDir;
use auth_service::Application;

#[tokio::main]
async fn main() {
    //let user_store = HashmapUserStore(());
    //let app_state = AppState::new(user_store);
    let user_store = todo!();
    let app_state = todo!();

    let app = Application::build(app_state, "0.0.0.0:0")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");

}
