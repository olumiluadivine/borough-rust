use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::entities::enums::UserRole;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: Uuid, // user_id
    pub email: String,
    pub role: UserRole,
    pub permissions: Vec<String>,
    pub iat: usize,  // issued at
    pub exp: usize,  // expiry
    pub iss: String, // issuer
    pub aud: String, // audience
    pub jti: Uuid, // JWT ID for blacklisting
}

impl JwtClaims {
    pub fn new(
        user_id: Uuid,
        email: String,
        role: UserRole,
        permissions: Vec<String>,
        issuer: String,
        audience: String,
        jti: Uuid,
        exp: usize,
    ) -> Self {
        let now = Utc::now().timestamp() as usize;

        Self {
            sub: user_id,
            email,
            role,
            permissions,
            iat: now,
            exp: now + exp,
            iss: issuer,
            aud: audience,
            jti
        }
    }
}