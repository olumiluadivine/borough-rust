use shared::features::errors::{SystemError, SystemResult};
use shared::utils::caching::CacheService;

#[derive(Clone)]
pub struct OtpCacheService {
    cache_service: CacheService,
    rate_limit_window_minutes: i64,
    max_otp_requests_per_window: i32,
}

impl OtpCacheService {
    pub fn new(
        cache_service: CacheService,
        rate_limit_window_minutes: i64,
        max_otp_requests_per_window: i32,
    ) -> Self {
        Self {
            cache_service,
            rate_limit_window_minutes,
            max_otp_requests_per_window,
        }
    }

    pub async fn store_otp(
        &self,
        identifier: &str,
        otp_code: &str,
        expiry_minutes: i64,
    ) -> SystemResult<()> {
        let otp_key = format!("otp:{}", identifier);
        let expiry_seconds = expiry_minutes * 60;

        self.cache_service
            .set(&otp_key, otp_code, Some(expiry_seconds as u64))
            .await?;

        Ok(())
    }

    pub async fn get_otp(&self, identifier: &str) -> SystemResult<Option<String>> {
        let otp_key = format!("otp:{}", identifier);
        self.cache_service.get::<String>(&otp_key).await
    }

    pub async fn invalidate_otp(&self, identifier: &str) -> SystemResult<()> {
        let otp_key = format!("otp:{}", identifier);
        self.cache_service.delete(&otp_key).await
    }

    pub async fn check_otp_rate_limit(&self, identifier: &str) -> SystemResult<()> {
        let rate_limit_key = format!("otp_rate_limit:{}", identifier);
        let window_seconds = self.rate_limit_window_minutes * 60;

        let current_count = self
            .cache_service
            .get::<i32>(&rate_limit_key)
            .await?;

        match current_count {
            Some(count) if count >= self.max_otp_requests_per_window => {
                Err(SystemError::OtpRateLimitExceeded)
            }
            Some(_) => {
                self.cache_service.increment(&rate_limit_key).await?;
                Ok(())
            }
            None => {
                self.cache_service
                    .set(&rate_limit_key, 1, Some(window_seconds as u64))
                    .await
            }
        }
    }
}