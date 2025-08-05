use crate::domain::repositories::security_repository::{
    SecurityQuestionRepository, UserSecurityQuestionRepository,
};
use crate::domain::services::security_domain_service::SecurityDomainService;
use shared::features::errors::{SystemError, SystemResult};
use std::sync::Arc;
use uuid::Uuid;
use shared::entities::dtos::auth::question::{SecurityQuestion, SecurityQuestionAnswer, SetSecurityQuestionsRequest, VerifySecurityQuestionsRequest};
use crate::domain::entities::user_security_question::UserSecurityQuestion;

pub struct SecurityQuestionUseCase {
    security_question_repo: Arc<dyn SecurityQuestionRepository>,
    user_security_question_repo: Arc<dyn UserSecurityQuestionRepository>,
}

impl SecurityQuestionUseCase {
    pub fn new(
        security_question_repo: Arc<dyn SecurityQuestionRepository>,
        user_security_question_repo: Arc<dyn UserSecurityQuestionRepository>,
    ) -> Self {
        Self {
            security_question_repo,
            user_security_question_repo,
        }
    }

    pub async fn get_available_questions(&self) -> SystemResult<Vec<SecurityQuestion>> {
        let questions = self.security_question_repo.get_active_questions().await?;

        Ok(questions
            .into_iter()
            .map(|q| SecurityQuestion {
                id: q.id,
                question: q.question,
            })
            .collect())
    }

    pub async fn set_security_questions(
        &self,
        user_id: Uuid,
        request: SetSecurityQuestionsRequest,
    ) -> SystemResult<()> {
        // Validate the request
        let questions_answers: Vec<(Uuid, String)> = request
            .questions
            .iter()
            .map(|qa| (qa.question_id, qa.answer.clone()))
            .collect();

        SecurityDomainService::validate_security_question_setup(&questions_answers)?;

        // Verify all question IDs exist
        for (question_id, _) in &questions_answers {
            self.security_question_repo
                .find_by_id(*question_id)
                .await?
                .ok_or_else(|| SystemError::InternalError("Invalid question ID".to_string()))?;
        }

        // Delete existing security questions for user
        self.user_security_question_repo
            .delete_by_user_id(user_id)
            .await?;

        // Create new security questions
        for SecurityQuestionAnswer {
            question_id,
            answer,
        } in request.questions
        {
            let answer_hash = SecurityDomainService::hash_security_answer(&answer)?;

            let user_security_question =
                UserSecurityQuestion::new(user_id, question_id, answer_hash);

            self.user_security_question_repo
                .create(&user_security_question)
                .await?;
        }

        Ok(())
    }

    pub async fn verify_security_questions(
        &self,
        user_id: Uuid,
        request: VerifySecurityQuestionsRequest,
    ) -> SystemResult<()> {
        // Get user's security questions
        let user_questions = self
            .user_security_question_repo
            .find_by_user_id(user_id)
            .await?;

        if user_questions.is_empty() {
            return Err(SystemError::SecurityQuestionFailed);
        }

        // Convert request to the format expected by domain service
        let provided_answers: Vec<(Uuid, String)> = request
            .answers
            .iter()
            .map(|qa| (qa.question_id, qa.answer.clone()))
            .collect();

        // Validate answers
        SecurityDomainService::validate_security_answers(&user_questions, &provided_answers)?;

        Ok(())
    }

    pub async fn get_user_security_questions(
        &self,
        user_id: Uuid,
    ) -> SystemResult<Vec<SecurityQuestion>> {
        let user_questions = self
            .user_security_question_repo
            .find_by_user_id(user_id)
            .await?;

        let mut result = Vec::new();

        for user_question in user_questions {
            if let Some(question) = self
                .security_question_repo
                .find_by_id(user_question.question_id)
                .await?
            {
                result.push(SecurityQuestion {
                    id: question.id,
                    question: question.question,
                });
            }
        }

        Ok(result)
    }
}
