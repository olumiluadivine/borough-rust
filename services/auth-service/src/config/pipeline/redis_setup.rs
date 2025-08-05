use crate::infrastructure::config::AppConfig;
use std::sync::Arc;
use deadpool_redis::{Config, Pool, Runtime};

pub fn create_redis_client(config: &AppConfig) -> Result<Arc<Pool>, deadpool_redis::CreatePoolError> {
    let cfg = Config::from_url(&config.redis.url);
    let pool = cfg
        .create_pool(Some(Runtime::Tokio1))
        .map(Arc::new)?;
    Ok(pool)
}