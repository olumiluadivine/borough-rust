use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use async_trait::async_trait;
use shared::features::errors::SystemResult;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &User) -> SystemResult<User> {
        log::info!("create() called with user: {:?}", user);
        Ok(user.clone())
    }

    async fn find_by_id(&self, id: Uuid) -> SystemResult<Option<User>> {
        log::info!("find_by_id() called with id: {}", id);
        Ok(None)
    }

    async fn find_by_email(&self, email: &str) -> SystemResult<Option<User>> {
        log::info!("find_by_email() called with email: {}", email);
        Ok(None)
    }

    async fn find_by_phone(&self, phone: &str) -> SystemResult<Option<User>> {
        log::info!("find_by_phone() called with phone: {}", phone);
        Ok(None)
    }

    async fn update(&self, user: &User) -> SystemResult<User> {
        log::info!("update() called with user: {:?}", user);
        Ok(user.clone())
    }

    async fn delete(&self, id: Uuid) -> SystemResult<()> {
        log::info!("delete() called with id: {}", id);
        Ok(())
    }

    async fn exists_by_email(&self, email: &str) -> SystemResult<bool> {
        log::info!("exists_by_email() called with email: {}", email);
        Ok(false)
    }

    async fn exists_by_phone(&self, phone: &str) -> SystemResult<bool> {
        log::info!("exists_by_phone() called with phone: {}", phone);
        Ok(false)
    }
}
