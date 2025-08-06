use serde_json::json;
use shared::events::{ExchangeType, RoutingKey};
use shared::features::errors::SystemResult;
use shared::utils::messaging::MessageBroker;

#[derive(Clone)]
pub struct NotificationPublisher {
    broker: MessageBroker,
}

impl NotificationPublisher {
    pub fn new(broker: MessageBroker) -> Self {
        Self { broker }
    }

    pub async fn setup(&self, queue_name: &str, routing_keys: &[RoutingKey], exchange: ExchangeType) -> SystemResult<()> {
        self.broker.setup_queue(queue_name, routing_keys, exchange).await
    }

    pub async fn send_email_otp(&self, email: &str, otp_code: &str) -> SystemResult<()> {
        let event = json!({
            "email": email,
            "otp_code": otp_code,
            "template": "otp_verification"
        });
        self.broker
            .publish(RoutingKey::EmailOtp, event, ExchangeType::Topic)
            .await
    }

    pub async fn send_sms_otp(&self, phone: &str, otp_code: &str) -> SystemResult<()> {
        let event = json!({
            "phone": phone,
            "otp_code": otp_code,
            "template": "otp_verification"
        });
        self.broker
            .publish(RoutingKey::SmsOtp, event, ExchangeType::Topic)
            .await
    }

    pub async fn send_password_reset_email(&self, email: &str, reset_token: &str) -> SystemResult<()> {
        let event = json!({
            "email": email,
            "reset_token": reset_token,
            "template": "password_reset"
        });
        self.broker
            .publish(RoutingKey::EmailPasswordReset, event, ExchangeType::Topic)
            .await
    }

    pub async fn send_password_changed_confirmation(&self, email: &str) -> SystemResult<()> {
        let event = json!({
            "email": email,
            "template": "password_changed"
        });
        self.broker
            .publish(RoutingKey::EmailPasswordChanged, event, ExchangeType::Topic)
            .await
    }

    pub async fn send_broadcast_notification(&self, message: &str) -> SystemResult<()> {
        let event = json!({
            "message": message,
            "template": "broadcast"
        });
        self.broker
            .publish(RoutingKey::Broadcast, event, ExchangeType::Fanout)
            .await
    }
}
