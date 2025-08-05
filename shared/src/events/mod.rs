use chrono::{DateTime, Utc};
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