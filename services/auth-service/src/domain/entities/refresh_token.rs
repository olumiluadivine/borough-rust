use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    device_id: Option<String>,
    device_name: Option<String>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

impl RefreshToken {
    pub fn new(
        user_id: Uuid,
        token_hash: String,
        device_id: Option<String>,
        device_name: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
        expires_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            token_hash,
            device_id,
            device_name,
            ip_address,
            user_agent,
            expires_at,
            is_revoked: false,
            created_at: Utc::now(),
            revoked_at: None,
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.is_revoked && Utc::now() < self.expires_at
    }

    pub fn revoke(&mut self) {
        self.is_revoked = true;
    }
}
