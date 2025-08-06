use deadpool_redis::{redis::AsyncCommands, Pool};
use std::sync::Arc;
use crate::config::redis_config::RedisFigureConfig;
use crate::features::errors::{SystemError, SystemResult};

#[derive(Clone)]
pub struct CacheService {
    redis_client: Arc<Pool>,
    config: RedisFigureConfig,
}

impl CacheService {
    fn get_client(&self) -> &Pool {
        self.redis_client.as_ref()
    }
    pub fn new(redis_client: Arc<Pool>, config: RedisFigureConfig) -> Self {
        Self { redis_client, config }
    }

    async fn get_connection(&self) -> SystemResult<deadpool_redis::Connection> {
        self.get_client()
            .get()
            .await
            .map_err(|e| SystemError::RedisError(e.to_string()))
    }

    // String operations
    pub async fn set<T: Send + Sync + deadpool_redis::redis::ToRedisArgs>(
        &self,
        key: &str,
        value: T,
        ttl_seconds: Option<u64>,
    ) -> SystemResult<()> {
        let mut conn = self.get_connection().await?;
        let ttl = ttl_seconds.unwrap_or(self.config.default_ttl_seconds);
        conn.set_ex(key, value, ttl)
            .await
            .map_err(|e| SystemError::RedisError(e.to_string()))
    }


    pub async fn get<T: deadpool_redis::redis::FromRedisValue + Send + Sync>(
        &self,
        key: &str,
    ) -> SystemResult<Option<T>> {
        let mut conn = self.get_connection().await?;
        conn.get(key)
            .await
            .map_err(|e| SystemError::RedisError(e.to_string()))
    }

    pub async fn delete(&self, key: &str) -> SystemResult<()> {
        let mut conn = self.get_connection().await?;
        conn.del(key)
            .await
            .map_err(|e| SystemError::RedisError(e.to_string()))
    }

    pub async fn expire(&self, key: &str, ttl_seconds: i64) -> SystemResult<()> {
        let mut conn = self.get_connection().await?;
        conn.expire(key, ttl_seconds)
            .await
            .map_err(|e| SystemError::RedisError(e.to_string()))
    }

    pub async fn exists(&self, key: &str) -> SystemResult<bool> {
        let mut conn = self.get_connection().await?;
        conn.exists(key)
            .await
            .map_err(|e| SystemError::RedisError(e.to_string()))
    }

    pub async fn increment(&self, key: &str) -> SystemResult<i32> {
        let mut conn = self.get_connection().await?;
        conn.incr(key, 1)
            .await
            .map_err(|e| SystemError::RedisError(e.to_string()))
    }

    // Hash operations
    pub async fn set_hash_field<T: deadpool_redis::redis::ToRedisArgs + Send + Sync>(
        &self,
        key: &str,
        field: &str,
        value: T,
        ttl_seconds: Option<u64>,
    ) -> SystemResult<()> {
        let mut conn = self.get_connection().await?;
        conn.hset::<_, _, _, ()>(key, field, value)
            .await
            .map_err(|e| SystemError::RedisError(e.to_string()))?;

        if let Some(ttl) = ttl_seconds {
            conn.expire::<_, ()>(key, ttl as i64)
                .await
                .map_err(|e| SystemError::RedisError(e.to_string()))?;
        }
        Ok(())
    }

    pub async fn get_hash_field<T: deadpool_redis::redis::FromRedisValue + Send + Sync>(
        &self,
        key: &str,
        field: &str,
    ) -> SystemResult<Option<T>> {
        let mut conn = self.get_connection().await?;
        conn.hget(key, field)
            .await
            .map_err(|e| SystemError::RedisError(e.to_string()))
    }

    pub async fn get_all_hash_fields(
        &self,
        key: &str,
    ) -> SystemResult<Vec<(String, String)>> {
        let mut conn = self.get_connection().await?;
        conn.hgetall(key)
            .await
            .map_err(|e| SystemError::RedisError(e.to_string()))
    }

    pub async fn delete_hash_field(
        &self,
        key: &str,
        field: &str,
    ) -> SystemResult<()> {
        let mut conn = self.get_connection().await?;
        conn.hdel(key, field)
            .await
            .map_err(|e| SystemError::RedisError(e.to_string()))
    }

    pub async fn delete_hash(&self, key: &str) -> SystemResult<()> {
        self.delete(key).await
    }

    pub async fn check_rate_limit(&self, key: &str) -> SystemResult<()> {
        let window_seconds = self.config.rate_limit_window_minutes * 60;
        match self.get::<i32>(key).await? {
            Some(count) if count >= self.config.max_requests_per_window => {
                Err(SystemError::OtpRateLimitExceeded)
            }
            Some(_) => {
                self.increment(key).await?;
                Ok(())
            }
            None => {
                self.set(key, 1, Some(window_seconds as u64)).await
            }
        }
    }
}