use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermission {
    pub id: Uuid,
    pub user_id: Uuid,
    pub permission: String,
    pub granted_by: Option<Uuid>,
    pub granted_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl UserPermission {
    pub fn new(
        user_id: Uuid,
        permission: String,
        granted_by: Option<Uuid>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            permission,
            granted_by,
            granted_at: Utc::now(),
            expires_at,
        }
    }

    pub fn is_valid(&self) -> bool {
        match self.expires_at {
            Some(expiry) => Utc::now() < expiry,
            None => true, // Permanent permission
        }
    }

    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(expiry) => Utc::now() >= expiry,
            None => false,
        }
    }

    pub fn extend_expiry(&mut self, new_expires_at: Option<DateTime<Utc>>) {
        self.expires_at = new_expires_at;
    }
}

// Common permission constants
pub mod permissions {
    // Property permissions
    pub const READ_PROPERTIES: &str = "read:properties";
    pub const WRITE_PROPERTIES: &str = "write:properties";
    pub const DELETE_PROPERTIES: &str = "delete:properties";
    pub const MANAGE_PROPERTIES: &str = "manage:properties";

    // Booking permissions
    pub const READ_BOOKINGS: &str = "read:bookings";
    pub const WRITE_BOOKINGS: &str = "write:bookings";
    pub const CANCEL_BOOKINGS: &str = "cancel:bookings";
    pub const MANAGE_BOOKINGS: &str = "manage:bookings";

    // Transaction permissions
    pub const READ_TRANSACTIONS: &str = "read:transactions";
    pub const PROCESS_PAYMENTS: &str = "process:payments";
    pub const REFUND_PAYMENTS: &str = "refund:payments";
    pub const MANAGE_TRANSACTIONS: &str = "manage:transactions";

    // User management permissions
    pub const READ_USERS: &str = "read:users";
    pub const WRITE_USERS: &str = "write:users";
    pub const DELETE_USERS: &str = "delete:users";
    pub const MANAGE_USERS: &str = "manage:users";

    // Admin permissions
    pub const ADMIN_ACCESS: &str = "admin:access";
    pub const SYSTEM_ADMIN: &str = "system:admin";
    pub const AUDIT_LOGS: &str = "read:audit_logs";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrantPermissionRequest {
    pub user_id: Uuid,
    pub permission: String,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevokePermissionRequest {
    pub user_id: Uuid,
    pub permission: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPermissionResponse {
    pub permission: String,
    pub granted_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_valid: bool,
}