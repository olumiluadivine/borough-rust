use crate::domain::entities::refresh_token::RefreshToken;
use crate::domain::repositories::refresh_token_repository::RefreshTokenRepository;
use async_trait::async_trait;
use shared::features::errors::SystemResult;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PostgresRefreshTokenRepository {
    pool: PgPool,
}

impl PostgresRefreshTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RefreshTokenRepository for PostgresRefreshTokenRepository {
    async fn create(&self, token: &RefreshToken) -> SystemResult<RefreshToken> {
        log::info!("create() called with security: {:?}", token);
        Ok(token.clone())
    }

    async fn find_by_token_hash(&self, token_hash: &str) -> SystemResult<Option<RefreshToken>> {
        log::info!(
            "find_by_token_hash() called with token_hash: {}",
            token_hash
        );
        Ok(None)
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> SystemResult<Vec<RefreshToken>> {
        log::info!("find_by_user_id() called with user_id: {}", user_id);
        Ok(vec![])
    }

    async fn update(&self, token: &RefreshToken) -> SystemResult<RefreshToken> {
        log::info!("update() called with security: {:?}", token);
        Ok(token.clone())
    }

    async fn revoke_all_for_user(&self, user_id: Uuid) -> SystemResult<()> {
        log::info!("revoke_all_for_user() called with user_id: {}", user_id);
        Ok(())
    }

    async fn cleanup_expired(&self) -> SystemResult<u64> {
        log::info!("cleanup_expired() called");
        Ok(0)
    }
}
