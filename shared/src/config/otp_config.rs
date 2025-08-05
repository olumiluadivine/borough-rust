use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtpConfig {
    pub length: usize,
    pub expiry_seconds: u64,
    pub max_attempts: u32,
    pub rate_limit_window: u64,
    pub max_requests_per_window: u32,
}

impl OtpConfig {
    pub fn from_env() -> Self {
        Self {
            length: env::var("OTP_LENGTH")
                .unwrap_or_else(|_| "6".to_string())
                .parse()
                .expect("OTP_LENGTH must be a valid number"),
            expiry_seconds: env::var("OTP_EXPIRY_SECONDS")
                .unwrap_or_else(|_| "300".to_string()) // 5 minutes
                .parse()
                .expect("OTP_EXPIRY_SECONDS must be a valid number"),
            max_attempts: env::var("OTP_MAX_ATTEMPTS")
                .unwrap_or_else(|_| "3".to_string())
                .parse()
                .expect("OTP_MAX_ATTEMPTS must be a valid number"),
            rate_limit_window: env::var("OTP_RATE_LIMIT_WINDOW")
                .unwrap_or_else(|_| "3600".to_string()) // 1 hour
                .parse()
                .expect("OTP_RATE_LIMIT_WINDOW must be a valid number"),
            max_requests_per_window: env::var("OTP_MAX_REQUESTS_PER_WINDOW")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .expect("OTP_MAX_REQUESTS_PER_WINDOW must be a valid number"),
        }
    }
}
