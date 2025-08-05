use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub max_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
    pub pool_timeout: u64,
}

impl RedisConfig {
    pub fn from_env() -> Self {
        Self {
            url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            max_connections: env::var("REDIS_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .expect("REDIS_MAX_CONNECTIONS must be a valid number"),
            connect_timeout: env::var("REDIS_CONNECT_TIMEOUT")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .expect("REDIS_CONNECT_TIMEOUT must be a valid number"),
            idle_timeout: env::var("REDIS_IDLE_TIMEOUT")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .expect("REDIS_IDLE_TIMEOUT must be a valid number"),
            pool_timeout: env::var("REDIS_POOL_TIMEOUT")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .expect("REDIS_POOL_TIMEOUT must be a valid number")
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisFigureConfig {
    pub default_ttl_seconds: u64,
    pub rate_limit_window_minutes: i64,
    pub max_requests_per_window: i32,
}

impl RedisFigureConfig {
    pub fn from_env() -> Self {
        Self {
            default_ttl_seconds: env::var("DEFAULT_TTL_SECONDS")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .expect("DEFAULT_TTL_SECONDS must be a valid number"),
            rate_limit_window_minutes: env::var("RATE_LIMIT_WINDOW_MINUTES")
                .unwrap_or_else(|_| "15".to_string())
                .parse()
                .expect("RATE_LIMIT_WINDOW_MINUTES must be a valid number"),
            max_requests_per_window: env::var("MAX_REQUESTS_PER_WINDOW")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .expect("MAX_REQUESTS_PER_WINDOW must be a valid number"),
        }
    }
}