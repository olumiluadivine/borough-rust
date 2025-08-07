use crate::application::use_cases::SecurityQuestionUseCase;
use actix_web::{web, HttpResponse, Result};
use std::sync::Arc;
use uuid::Uuid;
use shared::entities::dtos::auth::question::{SetSecurityQuestionsRequest, VerifySecurityQuestionsRequest};
use shared::features::errors::{map_auth_error_to_response, map_success_to_response};

pub struct SecurityQuestionController {
    security_question_use_case: Arc<SecurityQuestionUseCase>,
}

impl SecurityQuestionController {
    pub fn new(security_question_use_case: Arc<SecurityQuestionUseCase>) -> Self {
        Self {
            security_question_use_case,
        }
    }

    pub async fn create_security_questions(
        &self,
        path: web::Path<Uuid>,
        req: web::Json<SetSecurityQuestionsRequest>,
    ) -> Result<HttpResponse> {
        match self
            .security_question_use_case
            .as_ref()
            .set_security_questions(path.into_inner(), req.into_inner())
            .await
        {
            Ok(questions) => Ok(map_success_to_response::<()>(questions, None, None)),
            Err(err) => Ok(map_auth_error_to_response(&err)),
        }
    }

    pub async fn get_user_questions(&self, path: web::Path<Uuid>) -> Result<HttpResponse> {
        let user_id = path.into_inner();

        match self
            .security_question_use_case
            .as_ref()
            .get_user_security_questions(user_id)
            .await
        {
            Ok(questions) => Ok(map_success_to_response(questions.1, Some(questions.0), None)),
            Err(err) => Ok(map_auth_error_to_response(&err)),
        }
    }

    pub async fn get_security_questions(&self) -> Result<HttpResponse> {
        match self
            .security_question_use_case
            .as_ref()
            .get_available_questions()
            .await
        {
            Ok(questions) => Ok(map_success_to_response(questions.1, Some(questions.0), None)),
            Err(err) => Ok(map_auth_error_to_response(&err)),
        }
    }

    pub async fn verify_security_answers(
        &self,
        path: web::Path<Uuid>,
        req: web::Json<VerifySecurityQuestionsRequest>,
    ) -> Result<HttpResponse> {
        let user_id = path.into_inner();
        let request = req.into_inner();

        match self
            .security_question_use_case
            .as_ref()
            .verify_security_questions(user_id, request)
            .await
        {
            Ok(response) => Ok(map_success_to_response::<()>(response, None, None)),
            Err(err) => Ok(map_auth_error_to_response(&err)),
        }
    }
}
