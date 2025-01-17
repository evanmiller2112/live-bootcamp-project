use std::sync::Arc;

use auth_service::{services::HashmapUserStore, Application, app_state::AppState};
use tokio::sync::RwLock;
use auth_service::domain::UserStore;

#[tokio::main]
async fn main() {

    let user_store = HashmapUserStore::new();
    let app_state = AppState::new(Arc::new(RwLock::new(user_store)));
    let app = Application::build(app_state, "0.0.0.0:0")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");

}
