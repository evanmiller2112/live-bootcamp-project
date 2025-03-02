use auth_service::{get_postgres_pool, get_redis_client};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    app_state::AppState,
    utils::constants::{DATABASE_URL, REDIS_HOST_NAME},
    Application,
};
use auth_service::services::MockEmailClient;
use auth_service::services::data_stores::{postgres_user_store::PostgresUserStore, 
                                          redis_banned_token_store::RedisBannedTokenStore, 
                                          redis_two_fa_code_store::RedisTwoFACodeStore};
use auth_service::utils::tracing::init_tracing;

#[tokio::main]
async fn main() {
    color_eyre::install().expect("Failed to install color_eyre"); // New!
    init_tracing();
    let pg_pool = configure_postgresql().await;
    let redis_connection = Arc::new(RwLock::new(configure_redis()));

    let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
    let banned_token_store = Arc::new(RwLock::new(RedisBannedTokenStore::new(
        redis_connection.clone(),
    )));
    let two_fa_code_store = Arc::new(RwLock::new(RedisTwoFACodeStore::new(redis_connection)));
    let email_client = Arc::new(MockEmailClient);
    let app_state = AppState::new(user_store, banned_token_store, two_fa_code_store, email_client);

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

async fn configure_postgresql() -> PgPool {
    // Create a new database connection pool
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    // Run database migrations against our test database! 
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}

fn configure_redis() -> redis::Connection {
    get_redis_client(REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Redis client")
        .get_connection()
        .expect("Failed to get Redis connection")
}