use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub is_used: bool,
    pub created_at: DateTime<Utc>,
    used_at: Option<String>
}

impl PasswordResetToken {
    pub fn new(user_id: Uuid, token_hash: String, expires_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            token_hash,
            expires_at,
            is_used: false,
            created_at: Utc::now(),
            used_at: None
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.is_used && Utc::now() < self.expires_at
    }

    pub fn mark_as_used(&mut self) {
        self.is_used = true;
    }
}
