use crate::domain::entities::password_reset_token::PasswordResetToken;
use async_trait::async_trait;
use shared::features::errors::SystemResult;
use uuid::Uuid;

#[async_trait]
pub trait PasswordResetRepository: Send + Sync {
    async fn create(&self, token: &PasswordResetToken) -> SystemResult<PasswordResetToken>;
    async fn find_by_token_hash(&self, token_hash: &str) -> SystemResult<Option<PasswordResetToken>>;
    async fn update(&self, token: &PasswordResetToken) -> SystemResult<PasswordResetToken>;
    async fn cleanup_expired(&self) -> SystemResult<u64>;
    async fn revoke_all_for_user(&self, user_id: Uuid) -> SystemResult<()>;
}
