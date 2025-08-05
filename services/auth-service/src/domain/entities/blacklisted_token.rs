use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlacklistedToken {
    pub id: Uuid,
    pub token_hash: String,
    pub token_type: TokenType,
    pub user_id: Option<Uuid>,
    pub expires_at: DateTime<Utc>,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    Access,
    Refresh,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Access => write!(f, "access"),
            TokenType::Refresh => write!(f, "refresh"),
        }
    }
}

impl std::str::FromStr for TokenType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "access" => Ok(TokenType::Access),
            "refresh" => Ok(TokenType::Refresh),
            _ => Err(format!("Invalid security type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlacklistReason {
    Logout,
    PasswordChange,
    SecurityBreach,
    AdminAction,
    TokenExpired,
    UserDeactivated,
}

impl std::fmt::Display for BlacklistReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlacklistReason::Logout => write!(f, "logout"),
            BlacklistReason::PasswordChange => write!(f, "password_change"),
            BlacklistReason::SecurityBreach => write!(f, "security_breach"),
            BlacklistReason::AdminAction => write!(f, "admin_action"),
            BlacklistReason::TokenExpired => write!(f, "token_expired"),
            BlacklistReason::UserDeactivated => write!(f, "user_deactivated"),
        }
    }
}

impl BlacklistedToken {
    pub fn new(
        token_hash: String,
        token_type: TokenType,
        expires_at: DateTime<Utc>,
        user_id: Option<Uuid>,
        reason: Option<BlacklistReason>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            token_hash,
            token_type,
            user_id,
            expires_at,
            reason: reason.map(|r| r.to_string()),
            created_at: Utc::now(),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    pub fn get_reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
}