use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityQuestion {
    pub id: Uuid,
    pub question: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetSecurityQuestionsRequest {
    pub questions: Vec<SecurityQuestionAnswer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityQuestionAnswer {
    pub question_id: Uuid,
    pub answer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifySecurityQuestionsRequest {
    pub answers: Vec<SecurityQuestionAnswer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityQuestionRequest {
    pub user_id: Uuid,
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityQuestionResponse {
    pub id: Uuid,
    pub question: String,
    pub created_at: DateTime<Utc>,
}
