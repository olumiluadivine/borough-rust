use crate::domain::entities::user::User;
use async_trait::async_trait;
use shared::features::errors::SystemResult;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> SystemResult<User>;
    async fn find_by_id(&self, id: Uuid) -> SystemResult<Option<User>>;
    async fn find_by_email(&self, email: &str) -> SystemResult<Option<User>>;
    async fn find_by_phone(&self, phone: &str) -> SystemResult<Option<User>>;
    async fn update(&self, user: &User) -> SystemResult<User>;
    async fn delete(&self, id: Uuid) -> SystemResult<()>;
    async fn exists_by_email(&self, email: &str) -> SystemResult<bool>;
    async fn exists_by_phone(&self, phone: &str) -> SystemResult<bool>;
}
