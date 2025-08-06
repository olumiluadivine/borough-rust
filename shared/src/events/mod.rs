use std::fmt::Display;
use chrono::{DateTime, Utc};
use lapin::ExchangeKind;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod auth_event;
pub mod user_event;

// Event types for inter-service communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub source_service: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExchangeType {
    Topic,
    Fanout,
    Direct,
    Headers,
}

impl Display for ExchangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ExchangeType::Topic => "notification.topic".to_string(),
            ExchangeType::Fanout => "notification.fanout".to_string(),
            ExchangeType::Direct => "notification.direct".to_string(),
            ExchangeType::Headers => "notification.headers".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<ExchangeType> for ExchangeKind {
    fn from(exchange_type: ExchangeType) -> Self {
        match exchange_type {
            ExchangeType::Topic => ExchangeKind::Topic,
            ExchangeType::Fanout => ExchangeKind::Fanout,
            ExchangeType::Direct => ExchangeKind::Direct,
            ExchangeType::Headers => ExchangeKind::Headers,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingKey {
    EmailOtp,
    SmsOtp,
    EmailPasswordReset,
    EmailPasswordChanged,
    Broadcast,
}

impl Display for RoutingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RoutingKey::EmailOtp => "notification.email.otp".to_string(),
            RoutingKey::SmsOtp => "notification.sms.otp".to_string(),
            RoutingKey::EmailPasswordReset => "notification.email.password_reset".to_string(),
            RoutingKey::EmailPasswordChanged => "notification.email.password_changed".to_string(),
            RoutingKey::Broadcast => "notification.broadcast".to_string(),
        };
        write!(f, "{}", str)
    }
}