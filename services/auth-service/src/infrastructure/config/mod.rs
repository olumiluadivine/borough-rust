use serde::{Deserialize, Serialize};
use shared::config::{database_config, jwt_config, messaging_config, otp_config, redis_config, server_config};
use shared::config::redis_config::RedisFigureConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: database_config::DatabaseConfig,
    pub redis: redis_config::RedisConfig,
    pub jwt: jwt_config::JwtConfig,
    pub otp: otp_config::OtpConfig,
    pub server: server_config::ServerConfig,
    pub messaging: messaging_config::MessagingConfig,
    pub redis_figure_config: RedisFigureConfig
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database: database_config::DatabaseConfig::from_env(),
            redis: redis_config::RedisConfig::from_env(),
            jwt: jwt_config::JwtConfig::from_env(),
            otp: otp_config::OtpConfig::from_env(),
            server: server_config::ServerConfig::from_env(),
            messaging: messaging_config::MessagingConfig::from_env(),
            redis_figure_config: RedisFigureConfig::from_env()
        }
    }
}
