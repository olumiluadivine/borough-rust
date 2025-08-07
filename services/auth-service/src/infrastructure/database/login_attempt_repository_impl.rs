use crate::domain::entities::login_attempt::LoginAttempt;
use crate::domain::repositories::login_attempt_repository::LoginAttemptRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use shared::features::errors::{SystemError, SystemResult};
use sqlx::{Pool, Postgres, Row};

pub struct PostgresLoginAttemptRepository {
    pool: Pool<Postgres>,
}

impl PostgresLoginAttemptRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LoginAttemptRepository for PostgresLoginAttemptRepository {
    async fn create(&self, attempt: &LoginAttempt) -> SystemResult<LoginAttempt> {
        log::info!("create() called with attempt: {:?}", attempt);

        let row = sqlx::query(
            r#"
            INSERT INTO login_attempts (
            id,
            identifier,
            ip_address,
            user_agent,
            is_successful,
            failure_reason,
            country,
            city,
            created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, identifier, ip_address, user_agent, is_successful, failure_reason, country, city, created_at
            "#
        )
        .bind(&attempt.id)
        .bind(&attempt.identifier)
        .bind(&attempt.ip_address)
        .bind(&attempt.user_agent)
        .bind(attempt.is_successful)
        .bind(&attempt.failure_reason)
        .bind(&attempt.country)
        .bind(&attempt.city)
        .bind(&attempt.created_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| SystemError::from(e))?;

        let row = LoginAttempt {
            id: row.get("id"),
            identifier: row.get("identifier"),
            ip_address: row.get("ip_address"),
            user_agent: row.get("user_agent"),
            is_successful: row.get("is_successful"),
            failure_reason: row.get("failure_reason"),
            country: row.get("country"),
            city: row.get("city"),
            created_at: row.get("created_at"),
        };

        Ok(row)
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

        let rows = sqlx::query(
            r#"
            SELECT id, identifier, ip_address, user_agent, is_successful, failure_reason, country, city, created_at
            FROM login_attempts
            WHERE identifier = $1 AND created_at >= $2
            ORDER BY created_at DESC
            "#
        )
            .bind(identifier)
            .bind(since)
            .fetch_all(&self.pool)
            .await
            .map_err(SystemError::from)?;

        let attempts = rows
            .into_iter()
            .map(|row| LoginAttempt {
                id: row.get("id"),
                identifier: row.get("identifier"),
                ip_address: row.get("ip_address"),
                user_agent: row.get("user_agent"),
                is_successful: row.get("is_successful"),
                failure_reason: row.get("failure_reason"),
                country: row.get("country"),
                city: row.get("city"),
                created_at: row.get("created_at"),
            })
            .collect();

        Ok(attempts)
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

        let row = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM login_attempts
            WHERE identifier = $1
              AND created_at >= $2
              AND is_successful = FALSE
            "#
        )
            .bind(identifier)
            .bind(since)
            .fetch_one(&self.pool)
            .await
            .map_err(SystemError::from)?;

        let count: i64 = row.get("count");
        Ok(count)
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

        let rows = sqlx::query(
            r#"
            SELECT id, identifier, ip_address, user_agent, is_successful, failure_reason, country, city, created_at
            FROM login_attempts
            WHERE ip_address = $1 AND created_at >= $2
            ORDER BY created_at DESC
            "#
        )
            .bind(ip_address)
            .bind(since)
            .fetch_all(&self.pool)
            .await
            .map_err(SystemError::from)?;

        let attempts = rows
            .into_iter()
            .map(|row| LoginAttempt {
                id: row.get("id"),
                identifier: row.get("identifier"),
                ip_address: row.get("ip_address"),
                user_agent: row.get("user_agent"),
                is_successful: row.get("is_successful"),
                failure_reason: row.get("failure_reason"),
                country: row.get("country"),
                city: row.get("city"),
                created_at: row.get("created_at"),
            })
            .collect();

        Ok(attempts)
    }

    async fn count_failed_attempts_by_ip(&self, ip: &str, since: DateTime<Utc>) -> SystemResult<i64> {
        log::info!(
            "count_failed_attempts_by_ip() called with ip: {}, since: {}",
            ip,
            since
        );

        let row = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM login_attempts
            WHERE ip_address = $1::inet
              AND created_at >= $2
              AND is_successful = FALSE
            "#
        )
            .bind(ip)
            .bind(since)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                log::error!(
                    "Database error in count_failed_attempts_by_ip: {:?}",
                    e
                );
                SystemError::from(e)
            })?;

        let count: i64 = row.get("count");
        Ok(count)
    }

    async fn cleanup_old_attempts(&self, before: DateTime<Utc>) -> SystemResult<u64> {
        log::info!("cleanup_old_attempts() called with before: {}", before);
        
        let result = sqlx::query(
            r#"
            DELETE FROM login_attempts
            WHERE created_at < $1
            "#
        )
            .bind(before)
            .execute(&self.pool)
            .await
            .map_err(SystemError::from)?;

        Ok(result.rows_affected())
    }
}
