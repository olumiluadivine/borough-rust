use serde::{Deserialize, Serialize};
use crate::entities::enums::IdentifierType;

#[derive(Debug, Serialize, Deserialize)]
pub struct SendOtpRequest {
    pub identifier: String,
    pub identifier_type: IdentifierType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyOtpRequest {
    pub identifier: String,
    pub otp_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtpResponse {
    pub verified: bool,
    pub access_token: Option<String>,
    // pub user_info: Option<UserInfo>,
}