use std::error::Error;

use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};

use sqlx::PgPool;

use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    Email, Password, User,
};
use color_eyre::eyre::{eyre, Context, Result};

pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    #[tracing::instrument(name = "Adding user to PostgreSQL", skip_all)]
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let password_hash = compute_password_hash(user.password.as_ref())
            .await
            .map_err(UserStoreError::UnexpectedError)?;

        sqlx::query!(r#"
            INSERT INTO users (email, password_hash, requires_2fa)
            VALUES ($1, $2, $3)"#,
            user.email.as_ref(),
            &password_hash,
            user.requires_2fa
        )
            .execute(&self.pool)
            .await
            .map_err(|e| UserStoreError::UnexpectedError(e.into()))?;

        Ok(())
    }

    #[tracing::instrument(name = "Retrieving user from PostgreSQL", skip_all)]
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        sqlx::query!(r#"
            SELECT email, password_hash, requires_2fa
            FROM users
            WHERE email = $1"#,
            email.as_ref()
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| UserStoreError::UnexpectedError(e.into()))?
            .map(|row| {
                Ok(User {
                    email: Email::parse(row.email)
                        .map_err(|e| UserStoreError::UnexpectedError(eyre!(e)))?,
                    password: Password::parse(row.password_hash)
                        .map_err(|e| UserStoreError::UnexpectedError(eyre!(e)))?,
                    requires_2fa: row.requires_2fa,
                })
            })
            .ok_or(UserStoreError::UserNotFound)?
    }

    #[tracing::instrument(name = "Validating user credentials in PostgreSQL", skip_all)]
    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;

        verify_password_hash(
            user.password.as_ref(),
            password.as_ref(),
        )
            .await
            .map_err(|_| UserStoreError::InvalidCredentials)
    }
}

#[tracing::instrument(name = "Verify password hash", skip_all)]
async fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<()> { // Changed!
    let current_span: tracing::Span = tracing::Span::current();
    let result = tokio::task::spawn_blocking(move || {
        current_span.in_scope(|| {
            let expected_password_hash: PasswordHash<'_> =
                PasswordHash::new(&expected_password_hash)?;

            Argon2::default()
                .verify_password(password_candidate.as_bytes(), &expected_password_hash)
                .wrap_err("failed to verify password hash")
        })
    })
        .await;

    result?
}

#[tracing::instrument(name = "Computing password hash", skip_all)]
async fn compute_password_hash(password: String) -> Result<String> { // Changed!
    let current_span: tracing::Span = tracing::Span::current();

    let result = tokio::task::spawn_blocking(move || {
        current_span.in_scope(|| {
            let salt: SaltString = SaltString::generate(&mut rand::thread_rng());
            let password_hash = Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(15000, 2, 1, None)?,
            )
                .hash_password(password.as_bytes(), &salt)?
                .to_string();

            // Ok(password_hash)
            Err(eyre!("oh no!")) // New!
        })
    })
        .await;

    result?
}