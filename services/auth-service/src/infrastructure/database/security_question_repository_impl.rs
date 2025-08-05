use crate::domain::entities::security_question::SecurityQuestion;
use crate::domain::repositories::security_repository::{
    SecurityQuestionRepository, UserSecurityQuestionRepository,
};
use async_trait::async_trait;
use shared::features::errors::SystemResult;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::entities::user_security_question::UserSecurityQuestion;

pub struct PostgresSecurityQuestionRepository {
    pool: PgPool,
}

impl PostgresSecurityQuestionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SecurityQuestionRepository for PostgresSecurityQuestionRepository {
    async fn get_all_questions(&self) -> SystemResult<Vec<SecurityQuestion>> {
        log::info!("get_all_questions() called");
        Ok(vec![]) // Fake empty list
    }

    async fn get_active_questions(&self) -> SystemResult<Vec<SecurityQuestion>> {
        log::info!("get_active_questions() called");
        Ok(vec![]) // Fake empty list
    }

    async fn find_by_id(&self, id: Uuid) -> SystemResult<Option<SecurityQuestion>> {
        log::info!("find_by_id() called with id: {}", id);
        Ok(None) // Fake none
    }
}

pub struct PostgresUserSecurityQuestionRepository {
    pool: PgPool,
}

impl PostgresUserSecurityQuestionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserSecurityQuestionRepository for PostgresUserSecurityQuestionRepository {
    async fn create(
        &self,
        user_question: &UserSecurityQuestion,
    ) -> SystemResult<UserSecurityQuestion> {
        log::info!("create() called with user_question: {:?}", user_question);
        Ok(user_question.clone())
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> SystemResult<Vec<UserSecurityQuestion>> {
        log::info!("find_by_user_id() called with user_id: {}", user_id);
        Ok(vec![])
    }

    async fn update(
        &self,
        user_question: &UserSecurityQuestion,
    ) -> SystemResult<UserSecurityQuestion> {
        log::info!("update() called with user_question: {:?}", user_question);
        Ok(user_question.clone())
    }

    async fn delete_by_user_id(&self, user_id: Uuid) -> SystemResult<()> {
        log::info!("delete_by_user_id() called with user_id: {}", user_id);
        Ok(())
    }

    async fn find_by_user_and_question(
        &self,
        user_id: Uuid,
        question_id: Uuid,
    ) -> SystemResult<Option<UserSecurityQuestion>> {
        log::info!(
            "find_by_user_and_question() called with user_id: {}, question_id: {}",
            user_id,
            question_id
        );
        Ok(None)
    }
}
