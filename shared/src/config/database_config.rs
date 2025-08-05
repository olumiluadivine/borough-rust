use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        Self {
            url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set. Check your .env file contains: DATABASE_URL=postgresql://auth_user:auth_password123@localhost:5432/auth_db"),
            max_connections: env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .expect("DB_MAX_CONNECTIONS must be a valid number"),
            min_connections: env::var("DB_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "2".to_string())
                .parse()
                .expect("DB_MIN_CONNECTIONS must be a valid number"),
            connect_timeout: env::var("DB_CONNECT_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .expect("DB_CONNECT_TIMEOUT must be a valid number"),
            idle_timeout: env::var("DB_IDLE_TIMEOUT")
                .unwrap_or_else(|_| "600".to_string())
                .parse()
                .expect("DB_IDLE_TIMEOUT must be a valid number"),
        }
    }
}
