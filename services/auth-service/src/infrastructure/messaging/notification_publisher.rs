use lapin::{options::*, BasicProperties, Channel, Connection, ConnectionProperties};
use std::sync::Arc;
use lapin::types::FieldTable;

pub struct NotificationPublisher {
    // This will be implemented to publish events to RabbitMQ,
    // which will be consumed by the notification service
    channel: Arc<Channel>, // Keep a shared channel for publishing
}

impl NotificationPublisher {
    pub async fn new(amqp_url: &str) -> Self {
        let connection = Connection::connect(amqp_url, ConnectionProperties::default())
            .await
            .expect("Failed to connect to RabbitMQ");

        let channel = connection
            .create_channel()
            .await
            .expect("Failed to create channel");

        // Optionally declare an exchange or queue here if needed
        let queue_name = "notification_queue";
        channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .expect("Failed to declare queue");

        // Bind queue to multiple routing keys
        let routing_keys = [
            "notification.email.otp",
            "notification.sms.otp",
            "notification.email.password_reset",
            "notification.email.password_changed",
        ];
        for routing_key in routing_keys.iter() {
            channel
                .queue_bind(
                    queue_name,
                    "amq.topic",
                    routing_key,
                    QueueBindOptions::default(),
                    FieldTable::default(),
                )
                .await.expect("Failed to bind queue");
        }

        Self {
            channel: Arc::new(channel),
        }
    }

    async fn publish_event<T: serde::Serialize>(
        &self,
        routing_key: &str,
        payload: T,
    ) -> Result<(), lapin::Error> {
        let payload = serde_json::to_vec(&payload).expect("Failed to serialize payload");
        println!("Publishing to {}: {:?}", routing_key, String::from_utf8_lossy(&payload));

        self.channel
            .basic_publish(
                "amq.topic", // exchange
                routing_key, // routing key
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default().with_delivery_mode(2),
            )
            .await?
            .await?; // confirm publication
        Ok(())
    }

    pub async fn send_email_otp(&self, email: &str, otp_code: &str) {
        let event = serde_json::json!({
            "email": email,
            "otp_code": otp_code,
            "template": "otp_verification"
        });
        self.publish_event("notification.email.otp", event)
            .await
            .expect("Failed to publish email OTP event");
    }

    pub async fn send_sms_otp(&self, phone: &str, otp_code: &str) {
        let event = serde_json::json!({
            "phone": phone,
            "otp_code": otp_code,
            "template": "otp_verification"
        });
        self.publish_event("notification.sms.otp", event)
            .await
            .expect("Failed to publish email OTP event");
    }

    pub async fn send_password_reset_email(&self, email: &str, reset_token: &str) {
        let event = serde_json::json!({
            "phone": email,
            "otp_code": reset_token,
            "template": "otp_verification"
        });
        self.publish_event("notification.email.password_reset", event)
            .await
            .expect("Failed to publish email OTP event");
    }

    pub async fn send_password_changed_confirmation(&self, email: &str) {
        let event = serde_json::json!({
            "phone": email,
            "template": "otp_verification"
        });
        self.publish_event("notification.email.password_changed", event)
            .await
            .expect("Failed to publish email OTP event");
    }
}
