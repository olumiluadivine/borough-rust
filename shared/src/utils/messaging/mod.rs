use lapin::{options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties};
use std::sync::Arc;
use tokio::sync::broadcast;
use futures::stream::StreamExt;
use crate::config::messaging_config::MessagingConfig;
use crate::features::errors::{SystemError, SystemResult};

#[derive(Clone)]
pub struct MessageBroker {
    channel: Arc<Channel>,
    connection: Arc<Connection>,
}

impl MessageBroker {
    pub async fn new(config: MessagingConfig) -> SystemResult<Self> {
        let connection = Connection::connect(&config.rabbitmq_url, ConnectionProperties::default())
            .await
            .map_err(|e| SystemError::MessageBrokerError(e.to_string()))?;

        let channel = connection
            .create_channel()
            .await
            .map_err(|e| SystemError::MessageBrokerError(e.to_string()))?;

        Ok(Self {
            channel: Arc::new(channel),
            connection: Arc::new(connection),
        })
    }

    pub async fn setup_queue(&self, queue_name: &str, routing_keys: &[&str], exchange_type: &str) -> SystemResult<()> {
        self.channel
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

        for routing_key in routing_keys {
            self.channel
                .queue_bind(
                    queue_name,
                    exchange_type,
                    routing_key,
                    QueueBindOptions::default(),
                    FieldTable::default(),
                )
                .await
                .map_err(|e| SystemError::MessageBrokerError(e.to_string()))?;
        }
        Ok(())
    }

    pub async fn publish<T: serde::Serialize>(
        &self,
        routing_key: &str,
        payload: T,
        exchange_type: &str,
    ) -> SystemResult<()> {
        let payload = serde_json::to_vec(&payload)
            .map_err(|e| SystemError::MessageBrokerError(format!("Failed to serialize payload: {}", e)))?;
        println!("Publishing to {}: {:?}", routing_key, String::from_utf8_lossy(&payload));

        let confirm = self
            .channel
            .basic_publish(
                exchange_type,
                routing_key,
                BasicPublishOptions::default(),
                &payload,
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
            .channel
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

        self.channel
            .basic_cancel(consumer_tag, BasicCancelOptions::default())
            .await
            .map_err(|e| SystemError::MessageBrokerError(e.to_string()))?;
        println!("Consumer canceled");
        Ok(())
    }

    pub async fn close(&self) -> SystemResult<()> {
        self.channel
            .close(200, "Normal shutdown")
            .await
            .map_err(|e| SystemError::MessageBrokerError(e.to_string()))?;
        self.connection
            .close(200, "Normal shutdown")
            .await
            .map_err(|e| SystemError::MessageBrokerError(e.to_string()))?;
        println!("MessageBroker closed cleanly");
        Ok(())
    }
}