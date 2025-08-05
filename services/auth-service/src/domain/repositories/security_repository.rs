use crate::domain::entities::security_question::SecurityQuestion;
use async_trait::async_trait;
use shared::features::errors::SystemResult;
use uuid::Uuid;
use crate::domain::entities::user_security_question::UserSecurityQuestion;

#[async_trait]
pub trait SecurityQuestionRepository: Send + Sync {
    async fn get_all_questions(&self) -> SystemResult<Vec<SecurityQuestion>>;
    async fn get_active_questions(&self) -> SystemResult<Vec<SecurityQuestion>>;
    async fn find_by_id(&self, id: Uuid) -> SystemResult<Option<SecurityQuestion>>;
}

#[async_trait]
pub trait UserSecurityQuestionRepository: Send + Sync {
    async fn create(
        &self,
        user_question: &UserSecurityQuestion,
    ) -> SystemResult<UserSecurityQuestion>;
    async fn find_by_user_id(&self, user_id: Uuid) -> SystemResult<Vec<UserSecurityQuestion>>;
    async fn update(
        &self,
        user_question: &UserSecurityQuestion,
    ) -> SystemResult<UserSecurityQuestion>;
    async fn delete_by_user_id(&self, user_id: Uuid) -> SystemResult<()>;
    async fn find_by_user_and_question(
        &self,
        user_id: Uuid,
        question_id: Uuid,
    ) -> SystemResult<Option<UserSecurityQuestion>>;
}
