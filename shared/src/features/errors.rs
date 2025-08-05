use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use anyhow::Result;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SystemError {
    #[error("An error occurred with broker service: {0}")]
    MessageBrokerError(String),

    #[error("Phone provided is invalid")]
    InvalidPhone,

    #[error("Email provided is invalid")]
    InvalidEmail,

    #[error("Too many requests")]
    RateLimitExceeded,

    #[error("Invalid credentials provided")]
    InvalidCredentials,

    #[error("Account is locked due to too many failed attempts")]
    AccountLocked,

    #[error("Account is not verified")]
    AccountNotVerified,

    #[error("Account is inactive")]
    AccountInactive,

    #[error("Email not verified")]
    EmailNotVerified,

    #[error("OTP has expired")]
    OtpExpired,

    #[error("Token has expired")]
    TokenExpired,

    #[error("Invalid OTP provided")]
    InvalidOtp,

    #[error("OTP not found")]
    OtpNotFound,

    #[error("Too many OTP requests. Please wait before requesting again")]
    OtpRateLimitExceeded,

    #[error("Invalid or expired security")]
    InvalidToken,

    #[error("Token has been blacklisted")]
    TokenBlacklisted,

    #[error("Refresh security is invalid or expired")]
    InvalidRefreshToken,

    #[error("Password does not meet security requirements")]
    WeakPassword,

    #[error("Reset security is invalid or expired")]
    InvalidResetToken,

    #[error("Security question answer is incorrect")]
    SecurityQuestionFailed,

    #[error("User not found")]
    UserNotFound,

    #[error("Email already exists")]
    EmailExists,

    #[error("Phone number already exists")]
    PhoneExists,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Redis error: {0}")]
    RedisError(String),

    #[error("External service error: {0}")]
    ExternalServiceError(String),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Hashing error: {0}")]
    HashingError(String),

    #[error("Token error: {0}")]
    TokenError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl From<sqlx::Error> for SystemError {
    fn from(err: sqlx::Error) -> Self {
        SystemError::DatabaseError(err.to_string())
    }
}

impl From<redis::RedisError> for SystemError {
    fn from(err: redis::RedisError) -> Self {
        SystemError::RedisError(err.to_string())
    }
}

pub type SystemResult<T> = Result<T, SystemError>;

pub fn map_auth_error_to_response(err: &SystemError) -> HttpResponse {
    let (status, message) = match err {
        SystemError::RateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS, err.to_string()),
        SystemError::InvalidCredentials => (StatusCode::UNAUTHORIZED, err.to_string()),
        SystemError::AccountLocked => (StatusCode::FORBIDDEN, err.to_string()),
        SystemError::AccountNotVerified => (StatusCode::FORBIDDEN, err.to_string()),
        SystemError::AccountInactive => (StatusCode::FORBIDDEN, err.to_string()),
        SystemError::EmailNotVerified => (StatusCode::FORBIDDEN, err.to_string()),
        SystemError::OtpExpired => (StatusCode::BAD_REQUEST, err.to_string()),
        SystemError::TokenExpired => (StatusCode::UNAUTHORIZED, err.to_string()),
        SystemError::InvalidOtp => (StatusCode::BAD_REQUEST, err.to_string()),
        SystemError::OtpNotFound => (StatusCode::NOT_FOUND, err.to_string()),
        SystemError::OtpRateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS, err.to_string()),
        SystemError::InvalidToken => (StatusCode::UNAUTHORIZED, err.to_string()),
        SystemError::TokenBlacklisted => (StatusCode::UNAUTHORIZED, err.to_string()),
        SystemError::InvalidRefreshToken => (StatusCode::UNAUTHORIZED, err.to_string()),
        SystemError::WeakPassword => (StatusCode::BAD_REQUEST, err.to_string()),
        SystemError::InvalidResetToken => (StatusCode::BAD_REQUEST, err.to_string()),
        SystemError::SecurityQuestionFailed => (StatusCode::BAD_REQUEST, err.to_string()),
        SystemError::UserNotFound => (StatusCode::NOT_FOUND, err.to_string()),
        SystemError::EmailExists => (StatusCode::CONFLICT, err.to_string()),
        SystemError::PhoneExists => (StatusCode::CONFLICT, err.to_string()),
        SystemError::DatabaseError(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error".to_string(),
        ),
        SystemError::RedisError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Cache error".to_string()),
        SystemError::ExternalServiceError(_) => (
            StatusCode::BAD_GATEWAY,
            "External service error".to_string(),
        ),
        SystemError::InternalError(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        ),
        SystemError::HashingError(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Hashing error".to_string(),
        ),
        SystemError::TokenError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Token error".to_string()),
        SystemError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        SystemError::MessageBrokerError(msg) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            msg.clone(),
        ),
        SystemError::InvalidEmail => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Hashing error".to_string(),
        ),
        SystemError::InvalidPhone => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Hashing error".to_string(),
        ),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
    };

    HttpResponse::build(status).json(json!({
        "error": message
    }))
}