use crate::domain::entities::login_attempt::LoginAttempt;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use shared::features::errors::SystemResult;

#[async_trait]
pub trait LoginAttemptRepository: Send + Sync {
    async fn create(&self, attempt: &LoginAttempt) -> SystemResult<LoginAttempt>;
    async fn get_recent_attempts(
        &self,
        identifier: &str,
        since: DateTime<Utc>,
    ) -> SystemResult<Vec<LoginAttempt>>;
    async fn get_failed_attempts_count(
        &self,
        identifier: &str,
        since: DateTime<Utc>,
    ) -> SystemResult<i64>;
    async fn get_attempts_by_ip(
        &self,
        ip_address: &str,
        since: DateTime<Utc>,
    ) -> SystemResult<Vec<LoginAttempt>>;
    async fn count_failed_attempts_by_ip(&self, ip: &str, since: DateTime<Utc>) -> SystemResult<i64>;
    async fn cleanup_old_attempts(&self, before: DateTime<Utc>) -> SystemResult<u64>;
}
