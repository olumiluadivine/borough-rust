use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::entities::enums::IdentifierType;

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetRequest {
    pub identifier: String, // email or phone
    pub identifier_type: IdentifierType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetConfirmRequest {
    pub token: String,
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub user_id: Uuid,
    pub current_password: String,
    pub new_password: String,
}