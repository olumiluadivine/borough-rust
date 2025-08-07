use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use anyhow::Result;
use thiserror::Error;
use crate::entities::models::ApiResponse;

#[derive(Debug, Error)]
pub enum SystemError {
    #[error("An error occurred with broker service: {0}")]
    MessageBrokerError(String),

    #[error("Phone provided is invalid: {0}")]
    InvalidPhone(String),

    #[error("Email provided is invalid: {0}")]
    InvalidEmail(String),

    #[error("Too many requests: {0}")]
    RateLimitExceeded(String),

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

    #[error("Invalid OTP provided: {0}")]
    InvalidOtp(String),

    #[error("OTP not found")]
    OtpNotFound,

    #[error("Too many OTP requests. Please wait before requesting again")]
    OtpRateLimitExceeded,

    #[error("Invalid or expired security token")]
    InvalidToken,

    #[error("Token has been blacklisted")]
    TokenBlacklisted,

    #[error("Refresh security is invalid or expired")]
    InvalidRefreshToken,

    #[error("Password does not meet security requirements: {0}")]
    WeakPassword(String),

    #[error("Reset security is invalid or expired")]
    InvalidResetToken,

    #[error("Security question answer is incorrect")]
    SecurityQuestionFailed,

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Email already exists: {0}")]
    EmailExists(String),

    #[error("Phone number already exists: {0}")]
    PhoneExists(String),

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

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("File upload error: {0}")]
    FileUploadError(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Unknown error: {0}")]
    UnknownError(String),
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

#[derive(Debug)]
pub enum SuccessResponse {
    Created,
    Updated,
    Deleted,
    Fetched,
    Accepted,
    NoContent,
    Ok,
}

impl SuccessResponse {
    pub fn status_code(&self) -> StatusCode {
        match self {
            SuccessResponse::Created => StatusCode::CREATED,
            SuccessResponse::Updated => StatusCode::OK,
            SuccessResponse::Deleted => StatusCode::NO_CONTENT,
            SuccessResponse::Fetched => StatusCode::OK,
            SuccessResponse::Accepted => StatusCode::ACCEPTED,
            SuccessResponse::NoContent => StatusCode::NO_CONTENT,
            SuccessResponse::Ok => StatusCode::OK,
        }
    }

    pub fn default_message(&self) -> &'static str {
        match self {
            SuccessResponse::Created => "Resource created successfully",
            SuccessResponse::Updated => "Resource updated successfully",
            SuccessResponse::Deleted => "Resource deleted successfully",
            SuccessResponse::Fetched => "Resource fetched successfully",
            SuccessResponse::Accepted => "Request accepted",
            SuccessResponse::NoContent => "No content",
            SuccessResponse::Ok => "Request successful",
        }
    }
}

pub fn map_success_to_response<T: serde::Serialize>(
    success: SuccessResponse,
    data: Option<T>,
    message: Option<String>,
) -> HttpResponse {
    let final_message = match message {
        Some(msg) => msg,
        None => success.default_message().to_string(),
    };

    let response = ApiResponse::<T> {
        success: true,
        data,
        message: Some(final_message),
        error: None,
    };

    HttpResponse::build(success.status_code()).json(response)
}

pub fn map_auth_error_to_response(err: &SystemError) -> HttpResponse {

    let (status, error_message) = match err {
        SystemError::RateLimitExceeded(msg) => (StatusCode::TOO_MANY_REQUESTS, msg.clone()),
        SystemError::InvalidCredentials => (StatusCode::UNAUTHORIZED, err.to_string()),
        SystemError::AccountLocked => (StatusCode::FORBIDDEN, err.to_string()),
        SystemError::AccountNotVerified => (StatusCode::FORBIDDEN, err.to_string()),
        SystemError::AccountInactive => (StatusCode::FORBIDDEN, err.to_string()),
        SystemError::EmailNotVerified => (StatusCode::FORBIDDEN, err.to_string()),
        SystemError::OtpExpired => (StatusCode::BAD_REQUEST, err.to_string()),
        SystemError::TokenExpired => (StatusCode::UNAUTHORIZED, err.to_string()),
        SystemError::InvalidOtp(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        SystemError::OtpNotFound => (StatusCode::NOT_FOUND, err.to_string()),
        SystemError::OtpRateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS, err.to_string()),
        SystemError::InvalidToken => (StatusCode::UNAUTHORIZED, err.to_string()),
        SystemError::TokenBlacklisted => (StatusCode::UNAUTHORIZED, err.to_string()),
        SystemError::InvalidRefreshToken => (StatusCode::UNAUTHORIZED, err.to_string()),
        SystemError::WeakPassword(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        SystemError::InvalidResetToken => (StatusCode::BAD_REQUEST, err.to_string()),
        SystemError::SecurityQuestionFailed => (StatusCode::BAD_REQUEST, err.to_string()),
        SystemError::UserNotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
        SystemError::EmailExists(msg) => (StatusCode::CONFLICT, msg.clone()),
        SystemError::PhoneExists(msg) => (StatusCode::CONFLICT, msg.clone()),
        SystemError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        SystemError::RedisError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        SystemError::ExternalServiceError(msg) => (StatusCode::BAD_GATEWAY, msg.clone()),
        SystemError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        SystemError::HashingError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        SystemError::TokenError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        SystemError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        SystemError::MessageBrokerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        SystemError::InvalidEmail(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        SystemError::InvalidPhone(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        SystemError::FileNotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
        SystemError::FileUploadError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        SystemError::PermissionDenied(msg) => (StatusCode::FORBIDDEN, msg.clone()),
        SystemError::ConfigurationError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        SystemError::NetworkError(msg) => (StatusCode::BAD_GATEWAY, msg.clone()),
        SystemError::TimeoutError(msg) => (StatusCode::REQUEST_TIMEOUT, msg.clone()),
        SystemError::ParseError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        SystemError::SerializationError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        SystemError::DeserializationError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        SystemError::UnknownError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
    };

    let response = ApiResponse::<()> {
        success: false,
        data: None,
        message: None,
        error: Some(error_message),
    };

    HttpResponse::build(status).json(response)
}

pub type SystemResult<T> = Result<T, SystemError>;
