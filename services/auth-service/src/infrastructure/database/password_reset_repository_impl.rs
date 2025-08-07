use crate::domain::entities::password_reset_token::PasswordResetToken;
use crate::domain::repositories::password_reset_repository::PasswordResetRepository;
use async_trait::async_trait;
use shared::features::errors::SystemResult;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct PostgresPasswordResetRepository {
    pool: Pool<Postgres>,
}

impl PostgresPasswordResetRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PasswordResetRepository for PostgresPasswordResetRepository {
    async fn create(&self, token: &PasswordResetToken) -> SystemResult<PasswordResetToken> {
        log::info!("create() called with security: {:?}", token);

        let row = sqlx::query_as::<_, PasswordResetToken>(
            r#"
            INSERT INTO password_reset_tokens (
            id,
            user_id,
            token_hash,
            expires_at,
            is_used,
            created_at,
            used_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, user_id, token_hash, expires_at, is_used, created_at, used_at
            "#
        )
        .bind(token.id)
        .bind(token.user_id)
        .bind(&token.token_hash)
        .bind(token.expires_at)
        .bind(token.is_used)
        .bind(token.created_at)
        .bind(&token.used_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    async fn find_by_token_hash(&self, token_hash: &str) -> SystemResult<Option<PasswordResetToken>> {
        log::info!(
            "find_by_token_hash() called with token_hash: {}",
            token_hash
        );

        let row = sqlx::query_as::<_, PasswordResetToken>(
            r#"
            SELECT id, user_id, token_hash, expires_at, is_used, created_at, used_at
            FROM password_reset_tokens
            WHERE token_hash = $1
            LIMIT 1
            "#
        )
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    async fn update(&self, token: &PasswordResetToken) -> SystemResult<PasswordResetToken> {
        log::info!("update() called with security: {:?}", token);

        sqlx::query(
            r#"
            UPDATE password_reset_tokens
            SET
            is_used = $1,
            used_at = $2
            WHERE id = $3
            "#
        )
        .bind(token.is_used)
        .bind(&token.used_at)
        .bind(token.id)
        .execute(&self.pool)
        .await?;

        let updated_token = sqlx::query_as::<_, PasswordResetToken>(
            r#"
            SELECT id, user_id, token_hash, expires_at, is_used, created_at, used_at
            FROM password_reset_tokens
            WHERE id = $1
            "#
        )
        .bind(token.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_token)
    }

    async fn cleanup_expired(&self) -> SystemResult<u64> {
        log::info!("cleanup_expired() called");

        let result = sqlx::query(
            r#"
            DELETE FROM password_reset_tokens
            WHERE expires_at < NOW()
            "#
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    async fn revoke_all_for_user(&self, user_id: Uuid) -> SystemResult<()> {
        log::info!("revoke_all_for_user() called with user_id: {}", user_id);

        sqlx::query(
            r#"
            UPDATE password_reset_tokens
            SET is_used = TRUE, used_at = NOW()
            WHERE user_id = $1 AND is_used = FALSE
            "#
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
