use crate::domain::entities::security_question::SecurityQuestion;
use crate::domain::repositories::security_repository::{
    SecurityQuestionRepository, UserSecurityQuestionRepository,
};
use async_trait::async_trait;
use shared::features::errors::SystemResult;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::entities::user_security_question::UserSecurityQuestion;

pub struct PostgresSecurityQuestionRepository {
    pool: Pool<Postgres>,
}

impl PostgresSecurityQuestionRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SecurityQuestionRepository for PostgresSecurityQuestionRepository {
    async fn get_all_questions(&self) -> SystemResult<Vec<SecurityQuestion>> {
        log::info!("get_all_questions() called");

        let rows = sqlx::query_as::<_, SecurityQuestion>(
            r#"
            SELECT id, created_by, question, is_active, created_at, updated_at
            FROM security_questions
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let questions = rows
            .into_iter()
            .map(|row| SecurityQuestion {
            id: row.id,
            created_by: row.created_by,
            question: row.question,
            is_active: row.is_active,
            created_at: row.created_at,
            updated_at: row.updated_at,
            })
            .collect();

        Ok(questions)
    }

    async fn get_active_questions(&self) -> SystemResult<Vec<SecurityQuestion>> {
        log::info!("get_active_questions() called");

        let rows = sqlx::query_as::<_, SecurityQuestion>(
            r#"
            SELECT id, created_by, question, is_active, created_at, updated_at
            FROM security_questions
            WHERE is_active = true
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let questions = rows
            .into_iter()
            .map(|row| SecurityQuestion {
                id: row.id,
                created_by: row.created_by,
                question: row.question,
                is_active: row.is_active,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .collect();

        Ok(questions)
    }

    async fn find_by_id(&self, id: Uuid) -> SystemResult<Option<SecurityQuestion>> {
        log::info!("find_by_id() called with id: {}", id);

        let row = sqlx::query_as::<_, SecurityQuestion>(
            r#"
            SELECT id, created_by, question, is_active, created_at, updated_at
            FROM security_questions
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| SecurityQuestion {
            id: row.id,
            created_by: row.created_by,
            question: row.question,
            is_active: row.is_active,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }))
    }
}

pub struct PostgresUserSecurityQuestionRepository {
    pool: Pool<Postgres>,
}

impl PostgresUserSecurityQuestionRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
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

        let row = sqlx::query_as::<_, UserSecurityQuestion>(
            r#"
            INSERT INTO user_security_questions (id, user_id, question_id, answer_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, user_id, question_id, answer_hash, created_at, updated_at
            "#
        )
        .bind(user_question.id)
        .bind(user_question.user_id)
        .bind(user_question.question_id)
        .bind(&user_question.answer_hash)
        .bind(user_question.created_at)
        .bind(user_question.updated_at)
        .fetch_one(&self.pool)
        .await?;

        let created = UserSecurityQuestion {
            id: row.id,
            user_id: row.user_id,
            question_id: row.question_id,
            answer_hash: row.answer_hash,
            created_at: row.created_at,
            updated_at: row.updated_at,
        };

        Ok(created)
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> SystemResult<Vec<UserSecurityQuestion>> {
        log::info!("find_by_user_id() called with user_id: {}", user_id);

        let rows = sqlx::query_as::<_, UserSecurityQuestion>(
            r#"
            SELECT id, user_id, question_id, answer_hash, created_at, updated_at
            FROM user_security_questions
            WHERE user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        let user_questions = rows
            .into_iter()
            .map(|row| UserSecurityQuestion {
                id: row.id,
                user_id: row.user_id,
                question_id: row.question_id,
                answer_hash: row.answer_hash,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .collect();

        Ok(user_questions)
    }

    async fn update(
        &self,
        user_question: &UserSecurityQuestion,
    ) -> SystemResult<UserSecurityQuestion> {
        log::info!("update() called with user_question: {:?}", user_question);
        
        let row = sqlx::query_as::<_, UserSecurityQuestion>(
            r#"
            UPDATE user_security_questions
            SET answer_hash = $1, updated_at = $2
            WHERE id = $3
            RETURNING id, user_id, question_id, answer_hash, created_at, updated_at
            "#
        )
        .bind(&user_question.answer_hash)
        .bind(user_question.updated_at)
        .bind(user_question.id)
        .fetch_one(&self.pool)
        .await?;

        let updated = UserSecurityQuestion {
            id: row.id,
            user_id: row.user_id,
            question_id: row.question_id,
            answer_hash: row.answer_hash,
            created_at: row.created_at,
            updated_at: row.updated_at,
        };

        Ok(updated)
    }

    async fn delete_by_user_id(&self, user_id: Uuid) -> SystemResult<()> {
        log::info!("delete_by_user_id() called with user_id: {}", user_id);
        
        sqlx::query(
            r#"
            DELETE FROM user_security_questions
            WHERE user_id = $1
            "#
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

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

        let row = sqlx::query_as::<_, UserSecurityQuestion>(
            r#"
            SELECT id, user_id, question_id, answer_hash, created_at, updated_at
            FROM user_security_questions
            WHERE user_id = $1 AND question_id = $2
            "#
        )
        .bind(user_id)
        .bind(question_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| UserSecurityQuestion {
            id: row.id,
            user_id: row.user_id,
            question_id: row.question_id,
            answer_hash: row.answer_hash,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }))
    }
}
