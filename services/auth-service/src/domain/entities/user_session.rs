use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_token: String,
    pub device_id: Option<String>,
    pub device_name: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub is_active: bool,
    pub last_activity_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl UserSession {
    pub fn new(
        user_id: Uuid,
        session_token: String,
        expires_at: DateTime<Utc>,
        device_id: Option<String>,
        device_name: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            session_token,
            device_id,
            device_name,
            ip_address,
            user_agent,
            is_active: true,
            last_activity_at: now,
            expires_at,
            created_at: now,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.is_active && Utc::now() < self.expires_at
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    pub fn update_activity(&mut self) {
        self.last_activity_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.last_activity_at = Utc::now();
    }

    pub fn extend_expiry(&mut self, new_expires_at: DateTime<Utc>) {
        self.expires_at = new_expires_at;
        self.last_activity_at = Utc::now();
    }
}