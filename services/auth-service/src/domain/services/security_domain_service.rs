use shared::features::errors::{SystemError, SystemResult};
use uuid::Uuid;
use shared::features::helper::security_question_helper::SecurityHelper;
use crate::domain::entities::user_security_question::UserSecurityQuestion;

pub struct SecurityDomainService;

impl SecurityDomainService {
    pub fn validate_security_answers(
        user_questions: &[UserSecurityQuestion],
        provided_answers: &[(Uuid, String)],
    ) -> SystemResult<()> {
        if user_questions.is_empty() {
            return Err(SystemError::SecurityQuestionFailed);
        }

        if provided_answers.len() != user_questions.len() {
            return Err(SystemError::SecurityQuestionFailed);
        }

        for (question_id, answer) in provided_answers {
            let user_question = user_questions
                .iter()
                .find(|uq| uq.question_id == *question_id)
                .ok_or(SystemError::SecurityQuestionFailed)?;

            let is_valid =
                SecurityHelper::verify_security_answer(answer, &user_question.answer_hash)
                    .map_err(|e| SystemError::InternalError(e.to_string()))?;

            if !is_valid {
                return Err(SystemError::SecurityQuestionFailed);
            }
        }

        Ok(())
    }

    pub fn hash_security_answer(answer: &str) -> SystemResult<String> {
        SecurityHelper::hash_security_answer(answer)
            .map_err(|e| SystemError::InternalError(e.to_string()))
    }

    pub fn validate_security_question_setup(
        questions_answers: &[(Uuid, String)],
    ) -> SystemResult<()> {
        if questions_answers.len() < 2 {
            return Err(SystemError::InternalError(
                "At least 2 security questions required".to_string(),
            ));
        }

        if questions_answers.len() > 5 {
            return Err(SystemError::InternalError(
                "Maximum 5 security questions allowed".to_string(),
            ));
        }

        // Check for duplicate questions
        let mut question_ids = std::collections::HashSet::new();
        for (question_id, _) in questions_answers {
            if !question_ids.insert(*question_id) {
                return Err(SystemError::InternalError(
                    "Duplicate security questions not allowed".to_string(),
                ));
            }
        }

        // Validate answer length
        for (_, answer) in questions_answers {
            if answer.trim().len() < 2 {
                return Err(SystemError::InternalError(
                    "Security question answers must be at least 2 characters".to_string(),
                ));
            }

            if answer.len() > 200 {
                return Err(SystemError::InternalError(
                    "Security question answers must not exceed 200 characters".to_string(),
                ));
            }
        }

        Ok(())
    }
}
