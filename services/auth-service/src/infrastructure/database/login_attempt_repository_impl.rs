use crate::domain::entities::login_attempt::LoginAttempt;
use crate::domain::repositories::login_attempt_repository::LoginAttemptRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use shared::features::errors::SystemResult;
use sqlx::PgPool;

pub struct PostgresLoginAttemptRepository {
    pool: PgPool,
}

impl PostgresLoginAttemptRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LoginAttemptRepository for PostgresLoginAttemptRepository {
    async fn create(&self, attempt: &LoginAttempt) -> SystemResult<LoginAttempt> {
        log::info!("create() called with attempt: {:?}", attempt);
        Ok(attempt.clone())
    }

    async fn get_recent_attempts(
        &self,
        identifier: &str,
        since: DateTime<Utc>,
    ) -> SystemResult<Vec<LoginAttempt>> {
        log::info!(
            "get_recent_attempts() called with identifier: {}, since: {}",
            identifier,
            since
        );
        Ok(vec![])
    }

    async fn get_failed_attempts_count(
        &self,
        identifier: &str,
        since: DateTime<Utc>,
    ) -> SystemResult<i64> {
        log::info!(
            "get_failed_attempts_count() called with identifier: {}, since: {}",
            identifier,
            since
        );
        Ok(5)
    }

    async fn get_attempts_by_ip(
        &self,
        ip_address: &str,
        since: DateTime<Utc>,
    ) -> SystemResult<Vec<LoginAttempt>> {
        log::info!(
            "get_attempts_by_ip() called with ip_address: {}, since: {}",
            ip_address,
            since
        );
        Ok(vec![])
    }

    async fn count_failed_attempts_by_ip(&self, ip: &str, since: DateTime<Utc>) -> SystemResult<i64> {
        log::info!(
            "count_failed_attempts_by_ip() called with ip: {}, since: {}",
            ip,
            since
        );
        Ok(3)
    }

    async fn cleanup_old_attempts(&self, before: DateTime<Utc>) -> SystemResult<u64> {
        log::info!("cleanup_old_attempts() called with before: {}", before);
        Ok(10)
    }
}
