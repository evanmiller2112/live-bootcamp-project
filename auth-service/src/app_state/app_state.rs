use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::UserStore;
use crate::services::HashmapUserStore;

// Using a type alias to improve readability!
pub type UserStoreType = Arc<RwLock<dyn UserStore>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType) -> Self {
        Self { user_store }
    }
}