use lapin::{options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties};
use std::sync::Arc;
use tokio::sync::broadcast;
use futures::stream::StreamExt;
use crate::config::messaging_config::MessagingConfig;
use crate::events::{ExchangeType, RoutingKey};
use crate::features::errors::{SystemError, SystemResult};

#[derive(Clone)]
pub struct MessageBroker {
    channel: Arc<Channel>,
    connection: Arc<Connection>,
}

impl MessageBroker {
    fn get_channel(&self) -> &Channel {
        self.channel.as_ref()
    }

    fn get_connection(&self) -> &Connection {
        self.connection.as_ref()
    }

    pub async fn new(config: &MessagingConfig) -> SystemResult<Self> {
        let connection = Connection::connect((&config.rabbitmq_url).as_ref(), ConnectionProperties::default())
            .await
            .map_err(|e| SystemError::MessageBrokerError(e.to_string()))?;

        let channel = connection
            .create_channel()
            .await
            .map_err(|e| SystemError::MessageBrokerError(e.to_string()))?;

        // Declare custom exchanges
        for exchange_type in [ExchangeType::Topic, ExchangeType::Fanout, ExchangeType::Direct, ExchangeType::Headers] {
            channel
                .exchange_declare(
                    &exchange_type.to_string(),
                    exchange_type.into(),
                    ExchangeDeclareOptions {
                        durable: true,
                        ..Default::default()
                    },
                    FieldTable::default(),
                )
                .await
                .map_err(|e| SystemError::MessageBrokerError(e.to_string()))?;
        }

        Ok(Self {
            channel: Arc::new(channel),
            connection: Arc::new(connection),
        })
    }

    pub async fn setup_queue(&self, queue_name: &str, routing_keys: &[RoutingKey], exchange_type: ExchangeType) -> SystemResult<()> {
        self.get_channel()
            .queue_declare(
                queue_name,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .map_err(|e| SystemError::MessageBrokerError(e.to_string()))?;

        // Skip binding for fanout exchange if no routing keys are provided
        if exchange_type != ExchangeType::Fanout || !routing_keys.is_empty() {
            for routing_key in routing_keys {
                self.get_channel()
                    .queue_bind(
                        queue_name,
                        &exchange_type.to_string(),
                        &routing_key.to_string(),
                        QueueBindOptions::default(),
                        FieldTable::default(),
                    )
                    .await
                    .map_err(|e| SystemError::MessageBrokerError(e.to_string()))?;
            }
        }
        Ok(())
    }

    pub async fn publish<T: serde::Serialize>(
        &self,
        routing_key: RoutingKey,
        payload: T,
        exchange_type: ExchangeType,
    ) -> SystemResult<()> {
        let payload = serde_json::to_vec(&payload)
            .map_err(|e| SystemError::MessageBrokerError(format!("Failed to serialize payload: {}", e)))?;
        println!("Publishing to {}: {:?}", routing_key.to_string(), String::from_utf8_lossy(payload.as_ref()));

        let confirm = self
            .get_channel()
            .basic_publish(
                exchange_type.to_string().as_ref(),
                routing_key.to_string().as_ref(),
                BasicPublishOptions::default(),
                payload.as_ref(),
                BasicProperties::default().with_delivery_mode(2),
            )
            .await
            .map_err(|e| SystemError::MessageBrokerError(e.to_string()))?;

        confirm
            .await
            .map_err(|e| SystemError::MessageBrokerError(e.to_string()))?;
        Ok(())
    }

    pub async fn consume<F, Fut>(
        &self,
        queue_name: &str,
        consumer_tag: &str,
        mut shutdown_rx: broadcast::Receiver<()>,
        handler: F,
    ) -> SystemResult<()>
    where
        F: Fn(lapin::message::Delivery) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = SystemResult<()>> + Send,
    {
        let mut consumer = self
            .get_channel()
            .basic_consume(
                queue_name,
                consumer_tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| SystemError::MessageBrokerError(e.to_string()))?;

        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    println!("Shutdown signal received, stopping consumer");
                    break;
                }
                Some(delivery_result) = consumer.next() => {
                    match delivery_result {
                        Ok(delivery) => {
                            handler(delivery).await?;
                        }
                        Err(e) => {
                            eprintln!("Error receiving message: {}", e);
                        }
                    }
                }
            }
        }

        self.get_channel()
            .basic_cancel(consumer_tag, BasicCancelOptions::default())
            .await
            .map_err(|e| SystemError::MessageBrokerError(e.to_string()))?;
        println!("Consumer canceled");
        Ok(())
    }

    pub async fn close(&self) -> SystemResult<()> {
        self.get_channel()
            .close(200, "Normal shutdown")
            .await
            .map_err(|e| SystemError::MessageBrokerError(e.to_string()))?;
        self.get_connection()
            .close(200, "Normal shutdown")
            .await
            .map_err(|e| SystemError::MessageBrokerError(e.to_string()))?;
        println!("MessageBroker closed cleanly");
        Ok(())
    }
}