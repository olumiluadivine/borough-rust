use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSecurityQuestion {
    pub id: Uuid,
    pub user_id: Uuid,
    pub question_id: Uuid,
    pub answer_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserSecurityQuestion {
    pub fn new(user_id: Uuid, question_id: Uuid, answer_hash: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            question_id,
            answer_hash,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn verify_answer(&self, provided_answer_hash: &str) -> bool {
        self.answer_hash == provided_answer_hash
    }
}