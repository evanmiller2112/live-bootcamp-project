use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    app_state::AppState, 
    services::hashmap_user_store::HashmapUserStore, 
    Application,
    services::hashset_banned_token_store::HashsetBannedTokenStore,
};
use auth_service::app_state::BannedTokenStoreType;
use auth_service::services::{HashmapTwoFACodeStore, MockEmailClient};

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
    let email_client = Arc::new(MockEmailClient);
    let app_state = AppState::new(user_store, banned_token_store, two_fa_code_store, email_client);

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}