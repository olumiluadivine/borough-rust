use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginAttempt {
    pub id: Uuid,
    pub identifier: String, // email or phone
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub is_successful: bool,
    pub failure_reason: Option<String>,
    country: Option<String>,
    city: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl LoginAttempt {
    pub fn new(
        identifier: String,
        ip_address: String,
        user_agent: Option<String>,
        is_successful: bool,
        failure_reason: Option<String>,
        country: Option<String>,
        city: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            identifier,
            ip_address,
            user_agent,
            is_successful,
            failure_reason,
            country,
            city,
            created_at: Utc::now(),
        }
    }
}
