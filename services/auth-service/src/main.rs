mod application;
mod cache;
mod config;
mod domain;
mod infrastructure;
mod interface;

use std::sync::Arc;
use crate::config::pipeline::controller_setup::build_controllers;
use crate::config::pipeline::{database_setup::create_database_pool, env_setup::load_env};
use crate::config::pipeline::redis_setup::create_redis_client;
use crate::config::pipeline::service_setup::build_use_cases;
use crate::config::pipeline::start_http_server;
use infrastructure::config::AppConfig;
use crate::config::pipeline::queue_setup::setup_messaging;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_env();

    env_logger::init();

    let config = AppConfig::from_env();

    let db_pool = create_database_pool(&config)
        .await
        .expect("Failed to create database connection pool");

    let redis_client = create_redis_client(&config).expect("Failed to create Redis client");

    let (broker, publisher, shutdown_tx) = setup_messaging(&config).await.expect("Failed to setup messaging");

    let use_cases = build_use_cases(&config, &db_pool, redis_client.clone(), &Arc::new(publisher.clone()));

    let controllers = build_controllers(use_cases);

    // Start HTTP server and handle shutdown
    tokio::select! {
        result = start_http_server(config, controllers) => {
            result?;
        }
        _ = tokio::signal::ctrl_c() => {
            log::info!("Received shutdown signal");
        }
    }

    // Send shutdown signal and close broker
    shutdown_tx.send(()).expect("Failed to send shutdown signal");
    broker.close().await.expect("Failed to close MessageBroker");
    Ok(())
}
