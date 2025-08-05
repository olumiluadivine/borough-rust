use serde::{Deserialize, Serialize};
use std::env;
use std::thread;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub keep_alive: u64,
    pub client_timeout: u64,
    pub client_shutdown: u64,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        Self {
            host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8001".to_string())
                .parse()
                .expect("SERVER_PORT must be a valid number"),
            workers: env::var("SERVER_WORKERS")
                .unwrap_or_else(|_| thread::available_parallelism().unwrap().get().to_string())
                .parse()
                .expect("SERVER_WORKERS must be a valid number"),
            keep_alive: env::var("SERVER_KEEP_ALIVE")
                .unwrap_or_else(|_| "75".to_string())
                .parse()
                .expect("SERVER_KEEP_ALIVE must be a valid number"),
            client_timeout: env::var("SERVER_CLIENT_TIMEOUT")
                .unwrap_or_else(|_| "5000".to_string())
                .parse()
                .expect("SERVER_CLIENT_TIMEOUT must be a valid number"),
            client_shutdown: env::var("SERVER_CLIENT_SHUTDOWN")
                .unwrap_or_else(|_| "5000".to_string())
                .parse()
                .expect("SERVER_CLIENT_SHUTDOWN must be a valid number"),
        }
    }
}
