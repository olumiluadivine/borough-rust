use std::sync::Arc;

mod application;
mod cache;
mod config;
mod domain;
mod infrastructure;
mod interface;
use crate::config::pipeline::controller_setup::build_controllers;
use crate::config::pipeline::{database_setup::create_database_pool, env_setup::load_env};
use crate::config::pipeline::redis_setup::create_redis_client;
use crate::config::pipeline::service_setup::build_use_cases;
use crate::config::pipeline::start_http_server;
use crate::infrastructure::messaging::notification_publisher::NotificationPublisher;
use infrastructure::config::AppConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_env();

    env_logger::init();

    let config = AppConfig::from_env();

    let db_pool = create_database_pool(&config)
        .await
        .expect("Failed to create database connection pool");

    let redis_client = create_redis_client(&config).expect("Failed to create Redis client");

    let notification_publisher =
        Arc::new(NotificationPublisher::new(&config.messaging.rabbitmq_url).await);

    let use_cases = build_use_cases(&config, &db_pool, &redis_client, &notification_publisher);

    let controllers = build_controllers(use_cases);

    start_http_server(config, controllers).await
}
