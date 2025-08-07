use crate::config::pipeline::controller_setup::Controllers;
use crate::config::routing;
use crate::infrastructure::config::AppConfig;
use crate::interface::middleware::rate_limiter::RateLimiter;
use crate::interface::middleware::request_logger::RequestLogger;
use actix_web::{middleware, web, App, HttpServer};
use std::time::Duration;

pub mod controller_setup;
pub mod database_setup;
pub mod env_setup;
pub mod redis_setup;
pub mod service_setup;
pub mod queue_setup;

pub async fn start_http_server(config: AppConfig, controllers: Controllers) -> std::io::Result<()> {
    let server_config = config.server.clone();
    let bind_address = format!("{}:{}", server_config.host, server_config.port);

    log::info!("Starting auth service on {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(RequestLogger)
            .wrap(RateLimiter::new(100, Duration::from_secs(60)))
            .app_data(web::Data::new(controllers.clone()))
            .configure(routing::configure_services)
    })
    .workers(server_config.workers)
    .keep_alive(Duration::from_secs(server_config.keep_alive))
    .client_request_timeout(Duration::new(server_config.client_timeout, 0))
    .client_disconnect_timeout(Duration::new(server_config.client_shutdown, 0))
    .bind(&bind_address)?
    .run()
    .await
}
