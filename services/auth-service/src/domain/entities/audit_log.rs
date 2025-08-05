use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub old_values: Option<JsonValue>,
    pub new_values: Option<JsonValue>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
}

impl AuditLog {
    pub fn new(
        user_id: Option<Uuid>,
        action: String,
        resource_type: Option<String>,
        resource_id: Option<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            action,
            resource_type,
            resource_id,
            old_values: None,
            new_values: None,
            ip_address: None,
            user_agent: None,
            metadata: None,
            created_at: Utc::now(),
        }
    }

    pub fn with_changes(
        user_id: Option<Uuid>,
        action: String,
        resource_type: Option<String>,
        resource_id: Option<Uuid>,
        old_values: Option<JsonValue>,
        new_values: Option<JsonValue>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            action,
            resource_type,
            resource_id,
            old_values,
            new_values,
            ip_address: None,
            user_agent: None,
            metadata: None,
            created_at: Utc::now(),
        }
    }

    pub fn with_context(mut self, ip_address: Option<String>, user_agent: Option<String>) -> Self {
        self.ip_address = ip_address;
        self.user_agent = user_agent;
        self
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, JsonValue>) -> Self {
        self.metadata = Some(serde_json::to_value(metadata).unwrap_or(JsonValue::Null));
        self
    }

    pub fn add_metadata_field(&mut self, key: &str, value: JsonValue) {
        let mut metadata_map = if let Some(metadata) = &self.metadata {
            metadata.as_object().cloned().unwrap_or_default()
        } else {
            serde_json::Map::new()
        };

        metadata_map.insert(key.to_string(), value);
        self.metadata = Some(JsonValue::Object(metadata_map));
    }
}

// Common audit actions
pub mod audit_actions {
    // Authentication actions
    pub const LOGIN_SUCCESS: &str = "login_success";
    pub const LOGIN_FAILED: &str = "login_failed";
    pub const LOGOUT: &str = "logout";
    pub const PASSWORD_CHANGE: &str = "password_change";
    pub const PASSWORD_RESET_REQUEST: &str = "password_reset_request";
    pub const PASSWORD_RESET_COMPLETE: &str = "password_reset_complete";

    // Account management
    pub const ACCOUNT_CREATED: &str = "account_created";
    pub const ACCOUNT_UPDATED: &str = "account_updated";
    pub const ACCOUNT_DEACTIVATED: &str = "account_deactivated";
    pub const ACCOUNT_REACTIVATED: &str = "account_reactivated";
    pub const ACCOUNT_LOCKED: &str = "account_locked";
    pub const ACCOUNT_UNLOCKED: &str = "account_unlocked";

    // Permission changes
    pub const PERMISSION_GRANTED: &str = "permission_granted";
    pub const PERMISSION_REVOKED: &str = "permission_revoked";
    pub const ROLE_CHANGED: &str = "role_changed";

    // Security events
    pub const SECURITY_QUESTION_SET: &str = "security_question_set";
    pub const SECURITY_QUESTION_VERIFIED: &str = "security_question_verified";
    pub const SECURITY_BREACH_DETECTED: &str = "security_breach_detected";
    pub const SUSPICIOUS_ACTIVITY: &str = "suspicious_activity";

    // Token management
    pub const TOKEN_BLACKLISTED: &str = "token_blacklisted";
    pub const REFRESH_TOKEN_USED: &str = "refresh_token_used";
    pub const SESSION_CREATED: &str = "session_created";
    pub const SESSION_ENDED: &str = "session_ended";
}

// Common resource types
pub mod resource_types {
    pub const USER: &str = "user";
    pub const SESSION: &str = "session";
    pub const TOKEN: &str = "security";
    pub const PERMISSION: &str = "permission";
    pub const SECURITY_QUESTION: &str = "security_question";
    pub const AUDIT_LOG: &str = "audit_log";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAuditLogRequest {
    pub action: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub old_values: Option<JsonValue>,
    pub new_values: Option<JsonValue>,
    pub metadata: Option<HashMap<String, JsonValue>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogResponse {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogFilter {
    pub user_id: Option<Uuid>,
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}