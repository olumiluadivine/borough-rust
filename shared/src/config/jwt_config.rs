use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub access_token_expiry: u64,
    pub refresh_token_expiry: u64,
    pub issuer: String,
    pub audience: String,
}

impl JwtConfig {
    pub fn from_env() -> Self {
        Self {
            secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            access_token_expiry: env::var("JWT_ACCESS_TOKEN_EXPIRY")
                .unwrap_or_else(|_| "900".to_string()) // 15 minutes
                .parse()
                .expect("JWT_ACCESS_TOKEN_EXPIRY must be a valid number"),
            refresh_token_expiry: env::var("JWT_REFRESH_TOKEN_EXPIRY")
                .unwrap_or_else(|_| "604800".to_string()) // 7 days
                .parse()
                .expect("JWT_REFRESH_TOKEN_EXPIRY must be a valid number"),
            issuer: env::var("JWT_ISSUER").unwrap_or_else(|_| "auth-service".to_string()),
            audience: env::var("JWT_AUDIENCE").unwrap_or_else(|_| "borough-platform".to_string()),
        }
    }
}
