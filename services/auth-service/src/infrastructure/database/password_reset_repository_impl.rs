use crate::domain::entities::password_reset_token::PasswordResetToken;
use crate::domain::repositories::password_reset_repository::PasswordResetRepository;
use async_trait::async_trait;
use shared::features::errors::SystemResult;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PostgresPasswordResetRepository {
    pool: PgPool,
}

impl PostgresPasswordResetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PasswordResetRepository for PostgresPasswordResetRepository {
    async fn create(&self, token: &PasswordResetToken) -> SystemResult<PasswordResetToken> {
        log::info!("create() called with security: {:?}", token);
        Ok(token.clone())
    }

    async fn find_by_token_hash(&self, token_hash: &str) -> SystemResult<Option<PasswordResetToken>> {
        log::info!(
            "find_by_token_hash() called with token_hash: {}",
            token_hash
        );
        Ok(None)
    }

    async fn update(&self, token: &PasswordResetToken) -> SystemResult<PasswordResetToken> {
        log::info!("update() called with security: {:?}", token);
        Ok(token.clone())
    }

    async fn cleanup_expired(&self) -> SystemResult<u64> {
        log::info!("cleanup_expired() called");
        Ok(0)
    }

    async fn revoke_all_for_user(&self, user_id: Uuid) -> SystemResult<()> {
        log::info!("revoke_all_for_user() called with user_id: {}", user_id);
        Ok(())
    }
}
