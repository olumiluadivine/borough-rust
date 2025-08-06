use tokio::sync::broadcast;
use shared::events::{ExchangeType, RoutingKey};
use shared::features::errors::SystemResult;
use shared::utils::messaging::MessageBroker;
use crate::infrastructure::config::AppConfig;
use crate::infrastructure::messaging::notification_consumer::NotificationConsumer;
use crate::infrastructure::messaging::notification_publisher::NotificationPublisher;

pub async fn setup_messaging(config: &AppConfig) -> SystemResult<(MessageBroker, NotificationPublisher, broadcast::Sender<()>)> {
    let broker = MessageBroker::new(&config.messaging).await.expect("Wahala Wahala");
    let publisher = NotificationPublisher::new(broker.clone());
    let consumer = NotificationConsumer::new(broker.clone());

    // Setup queues
    let topic_queue = "auth_notification_queue";
    let routing_keys = [
        RoutingKey::EmailOtp,
        RoutingKey::SmsOtp,
        RoutingKey::EmailPasswordReset,
        RoutingKey::EmailPasswordChanged,
        RoutingKey::Broadcast,
    ];
    publisher.setup(topic_queue, &routing_keys, ExchangeType::Topic).await?;

    // Setup consumer queue
    consumer.setup(topic_queue, &routing_keys, ExchangeType::Topic).await?;

    // Spawn consumer task
    let (shutdown_tx, shutdown_rx) = broadcast::channel(1);
    tokio::spawn(async move {
        consumer.consume(topic_queue, shutdown_rx).await.expect("Consumer failed");
    });

    Ok((broker, publisher, shutdown_tx))
}