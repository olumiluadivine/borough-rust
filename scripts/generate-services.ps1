# PowerShell script to generate all microservices
$services = @(
    @{name="booking-service"; port=8004; grpcPort=9004},
    @{name="transaction-service"; port=8005; grpcPort=9005},
    @{name="notification-service"; port=8006; grpcPort=9006},
    @{name="feedback-service"; port=8007; grpcPort=9007},
    @{name="search-service"; port=8008; grpcPort=9008},
    @{name="external-comm-service"; port=8009; grpcPort=9009}
)

foreach ($service in $services) {
    $serviceName = $service.name
    $port = $service.port
    $grpcPort = $service.grpcPort
    
    Write-Host "Creating $serviceName..."
    
    # Create Cargo.toml
    $cargoContent = @"
[package]
name = "$serviceName"
version = "0.1.0"
edition = "2021"

[dependencies]
shared = { path = "../../shared" }

# Web Framework
actix-web = { workspace = true }
actix-cors = { workspace = true }
tokio = { workspace = true }

# gRPC
tonic = { workspace = true }
prost = { workspace = true }

# Database
sqlx = { workspace = true }
redis = { workspace = true }

# Serialization
serde = { workspace = true }
serde_json = { workspace = true }

# Utilities
uuid = { workspace = true }
chrono = { workspace = true }
env_logger = { workspace = true }
log = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }

# Configuration
config = { workspace = true }
dotenv = { workspace = true }

# Async Runtime
futures = { workspace = true }

[build-dependencies]
tonic-build = { workspace = true }
"@

    $mainContent = @"
use actix_web::{web, App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use std::env;
use tokio::select;
use tonic::transport::Server;

mod application;
mod domain;
mod infrastructure;
mod interface;
mod config;
mod grpc;
mod cache;

use crate::interface::routes::configure_routes;
use crate::grpc::health::HealthServiceImpl;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    dotenv().ok();
    env_logger::init();
    
    // Initialize tracing
    shared::observability::init_tracing("$serviceName");
    
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "$port".to_string())
        .parse()
        .expect("PORT must be a valid number");
    
    let grpc_port: u16 = env::var("GRPC_PORT")
        .unwrap_or_else(|_| "$grpcPort".to_string())
        .parse()
        .expect("GRPC_PORT must be a valid number");

    log::info!("Starting $(($serviceName -split '-' | ForEach-Object { (Get-Culture).TextInfo.ToTitleCase($_) }) -join ' ') on {}:{} (HTTP) and {}:{} (gRPC)", host, port, host, grpc_port);

    // Start HTTP server
    let http_server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(configure_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run();

    // Start gRPC server
    let grpc_addr = format!("{}:{}", host, grpc_port).parse().unwrap();
    let health_service = HealthServiceImpl::default();
    
    let grpc_server = Server::builder()
        .add_service(shared::grpc_clients::health::health_server::HealthServer::new(health_service))
        .serve(grpc_addr);

    // Run both servers concurrently
    select! {
        result = http_server => {
            if let Err(err) = result {
                log::error!("HTTP server error: {}", err);
            }
        }
        result = grpc_server => {
            if let Err(err) = result {
                log::error!("gRPC server error: {}", err);
            }
        }
    }

    Ok(())
}
"@

    # Create directory structure and files
    New-Item -ItemType Directory -Path "services\$serviceName\src" -Force | Out-Null
    $cargoContent | Out-File -FilePath "services\$serviceName\Cargo.toml" -Encoding UTF8
    $mainContent | Out-File -FilePath "services\$serviceName\src\main.rs" -Encoding UTF8
}

Write-Host "All services created successfully!"
