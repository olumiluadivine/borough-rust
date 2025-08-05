use crate::domain::entities::refresh_token::RefreshToken;
use async_trait::async_trait;
use shared::features::errors::SystemResult;
use uuid::Uuid;

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn create(&self, token: &RefreshToken) -> SystemResult<RefreshToken>;
    async fn find_by_token_hash(&self, token_hash: &str) -> SystemResult<Option<RefreshToken>>;
    async fn find_by_user_id(&self, user_id: Uuid) -> SystemResult<Vec<RefreshToken>>;
    async fn update(&self, token: &RefreshToken) -> SystemResult<RefreshToken>;
    async fn revoke_all_for_user(&self, user_id: Uuid) -> SystemResult<()>;
    async fn cleanup_expired(&self) -> SystemResult<u64>;
}
