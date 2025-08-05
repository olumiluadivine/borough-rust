use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use std::env;
use tokio::select;

mod application;
mod cache;
mod config;
mod domain;
mod grpc;
mod infrastructure;
mod interface;

use crate::interface::routes::configure_routes;
// use crate::grpc::health::HealthServiceImpl;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    dotenv().ok();
    env_logger::init();
    
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8004".to_string())
        .parse()
        .expect("PORT must be a valid number");

    let grpc_port: u16 = env::var("GRPC_PORT")
        .unwrap_or_else(|_| "9004".to_string())
        .parse()
        .expect("GRPC_PORT must be a valid number");

    log::info!(
        "Starting Booking Service on {}:{} (HTTP) and {}:{} (gRPC)",
        host,
        port,
        host,
        grpc_port
    );

    // Start HTTP server
    let http_server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(configure_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run();

    // Start gRPC server
    // let grpc_addr = format!("{}:{}", host, grpc_port).parse().unwrap();
    // let health_service = HealthServiceImpl::default();

    // let grpc_server = Server::builder()
    //     .add_service(shared::grpc_clients::health::health_server::HealthServer::new(health_service))
    //     .serve(grpc_addr);

    // Run both servers concurrently
    select! {
        result = http_server => {
            if let Err(err) = result {
                log::error!("HTTP server error: {}", err);
            }
        }
        // result = grpc_server => {
        //     if let Err(err) = result {
        //         log::error!("gRPC server error: {}", err);
        //     }
        // }
    }

    Ok(())
}
