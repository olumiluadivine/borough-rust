use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::features::errors::SystemError;
use shared::features::errors::SystemResult;
use uuid::Uuid;
use shared::entities::enums::UserRole;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub phone_number: Option<String>,
    pub password_hash: String,
    pub role: UserRole,
    pub is_verified: bool,
    pub is_active: bool,
    pub failed_login_attempts: i32,
    pub locked_until: Option<DateTime<Utc>>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: String, password_hash: String, role: UserRole) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            phone_number: None,
            password_hash,
            role,
            is_verified: false,
            is_active: true,
            failed_login_attempts: 0,
            locked_until: None,
            last_login_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_locked(&self) -> bool {
        if let Some(locked_until) = self.locked_until {
            Utc::now() < locked_until
        } else {
            false
        }
    }

    pub fn can_login(&self) -> SystemResult<()> {
        if !self.is_active {
            return Err(SystemError::AccountInactive);
        }

        if self.is_locked() {
            return Err(SystemError::AccountLocked);
        }

        if !self.is_verified {
            return Err(SystemError::AccountNotVerified);
        }

        Ok(())
    }

    pub fn increment_failed_attempts(&mut self, max_attempts: i32, lockout_duration_minutes: i64) {
        self.failed_login_attempts += 1;

        if self.failed_login_attempts >= max_attempts {
            self.locked_until =
                Some(Utc::now() + chrono::Duration::minutes(lockout_duration_minutes));
        }

        self.updated_at = Utc::now();
    }

    pub fn reset_failed_attempts(&mut self) {
        self.failed_login_attempts = 0;
        self.locked_until = None;
        self.last_login_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn verify_account(&mut self) {
        self.is_verified = true;
        self.updated_at = Utc::now();
    }
}
