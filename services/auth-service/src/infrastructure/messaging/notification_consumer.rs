use lapin::options::BasicAckOptions;
use tokio::sync::broadcast;
use shared::events::{ExchangeType, RoutingKey};
use shared::features::errors::{SystemError, SystemResult};
use shared::utils::messaging::MessageBroker;

pub struct NotificationConsumer {
    broker: MessageBroker,
}

impl NotificationConsumer {
    pub fn new(broker: MessageBroker) -> Self {
        Self { broker }
    }

    pub async fn setup(&self, queue_name: &str, routing_keys: &[RoutingKey], exchange: ExchangeType) -> SystemResult<()> {
        self.broker.setup_queue(queue_name, routing_keys, exchange).await
    }

    pub async fn consume(&self, queue_name: &str, shutdown_rx: broadcast::Receiver<()>) -> SystemResult<()> {
        self.broker
            .consume(queue_name, "notification_consumer", shutdown_rx, |delivery| async move {
                let payload: serde_json::Value = serde_json::from_slice(&delivery.data)
                    .map_err(|e| SystemError::MessageBrokerError(e.to_string()))?;
                let routing_key = delivery.routing_key.as_str();
                let template = payload.get("template").and_then(|t| t.as_str()).unwrap_or("");

                match (routing_key, template) {
                    ("notification.email.otp", "otp_verification") => {
                        if let (Some(email), Some(otp_code)) = (
                            payload.get("email").and_then(|e| e.as_str()),
                            payload.get("otp_code").and_then(|o| o.as_str()),
                        ) {
                            println!("Processing email OTP for {}: {}", email, otp_code);
                            // Add email sending logic here
                        } else {
                            eprintln!("Invalid email OTP payload: {:?}", payload);
                        }
                    }
                    ("notification.sms.otp", "otp_verification") => {
                        if let (Some(phone), Some(otp_code)) = (
                            payload.get("phone").and_then(|p| p.as_str()),
                            payload.get("otp_code").and_then(|o| o.as_str()),
                        ) {
                            println!("Processing SMS OTP for {}: {}", phone, otp_code);
                            // Add SMS sending logic here
                        } else {
                            eprintln!("Invalid SMS OTP payload: {:?}", payload);
                        }
                    }
                    ("notification.email.password_reset", "password_reset") => {
                        if let (Some(email), Some(reset_token)) = (
                            payload.get("email").and_then(|e| e.as_str()),
                            payload.get("reset_token").and_then(|r| r.as_str()),
                        ) {
                            println!("Processing password reset for {}: {}", email, reset_token);
                            // Add email sending logic here
                        } else {
                            eprintln!("Invalid password reset payload: {:?}", payload);
                        }
                    }
                    ("notification.email.password_changed", "password_changed") => {
                        if let Some(email) = payload.get("email").and_then(|e| e.as_str()) {
                            println!("Processing password changed confirmation for {}", email);
                            // Add email sending logic here
                        } else {
                            eprintln!("Invalid password changed payload: {:?}", payload);
                        }
                    }
                    ("notification.broadcast", "broadcast") => {
                        if let Some(message) = payload.get("message").and_then(|m| m.as_str()) {
                            println!("Processing broadcast message: {}", message);
                            // Add broadcast handling logic here
                        } else {
                            eprintln!("Invalid broadcast payload: {:?}", payload);
                        }
                    }
                    _ => {
                        eprintln!("Unknown routing key or template: {} / {}", routing_key, template);
                    }
                }

                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .map_err(|e| SystemError::MessageBrokerError(e.to_string()))?;
                Ok(())
            })
            .await
    }
}