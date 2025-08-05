use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityQuestion {
    pub id: Uuid,
    pub created_by: Uuid,
    pub question: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SecurityQuestion {
    pub fn new(question: String, created_by: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_by,
            question,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    pub fn update_question(&mut self, new_question: String) {
        self.question = new_question;
        self.updated_at = Utc::now();
    }
}