use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagingConfig {
    pub rabbitmq_url: String,
    pub exchange_name: String,
    pub queue_name: String,
    pub routing_key: String,
    pub connection_timeout: u64,
    pub heartbeat: u16,
}

impl MessagingConfig {
    pub fn from_env() -> Self {
        Self {
            rabbitmq_url: env::var("RABBITMQ_URL")
                .unwrap_or_else(|_| "amqp://localhost:5672".to_string()),
            exchange_name: env::var("MESSAGING_EXCHANGE_NAME")
                .unwrap_or_else(|_| "auth_events".to_string()),
            queue_name: env::var("MESSAGING_QUEUE_NAME")
                .unwrap_or_else(|_| "auth_notifications".to_string()),
            routing_key: env::var("MESSAGING_ROUTING_KEY")
                .unwrap_or_else(|_| "auth.notifications".to_string()),
            connection_timeout: env::var("MESSAGING_CONNECTION_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .expect("MESSAGING_CONNECTION_TIMEOUT must be a valid number"),
            heartbeat: env::var("MESSAGING_HEARTBEAT")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .expect("MESSAGING_HEARTBEAT must be a valid number"),
        }
    }
}
