use std::sync::{Arc, RwLock};
use axum_extra::extract::CookieJar;
use auth_service::{
    app_state::{AppState, BannedTokenStoreType},
    services::{
        hashmap_user_store::HashmapUserStore, hashset_banned_token_store::HashsetBannedTokenStore,
    },
    Application,
};
use reqwest::cookie::Jar;
use auth_service::domain::BannedTokenStoreError;
use auth_service::utils::constants::{prod, test};

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub banned_token_store: BannedTokenStoreType,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = Arc::new(tokio::sync::RwLock::new(HashmapUserStore::default()));
        let banned_token_store = Arc::new(tokio::sync::RwLock::new(HashsetBannedTokenStore::default()));
        let app_state = AppState::new(user_store, banned_token_store.clone());
        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::new(); // Create a Reqwest http client instance

        let cookie_jar = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        Self {
            address,
            cookie_jar,
            banned_token_store,
            http_client,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
    pub async fn login(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn verify_2fa(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn verify_token(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute login request.")
    }
    pub async fn post_logout(&self) -> reqwest::Response
    {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute logout. ")
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", uuid::Uuid::new_v4())
}