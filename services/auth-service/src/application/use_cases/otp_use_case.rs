use crate::cache::otp_cache::OtpCacheService;
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::messaging::notification_publisher::NotificationPublisher;
use shared::features::errors::{SystemError, SystemResult};
use std::sync::Arc;
use log::{info, debug, warn, error};
use shared::entities::dtos::auth::otp::{SendOtpRequest, VerifyOtpRequest};
use shared::entities::enums::IdentifierType;
use shared::features::helper::otp_helper::OtpHelper;

pub struct OtpUseCase {
    user_repo: Arc<dyn UserRepository>,
    otp_cache: OtpCacheService,
    notification_publisher: Arc<NotificationPublisher>, // Temporarily disabled
    otp_expiry_minutes: i64,
}

impl OtpUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        otp_cache: OtpCacheService,
        notification_publisher: Arc<NotificationPublisher>,
        otp_config: shared::config::otp_config::OtpConfig,
    ) -> Self {
        Self {
            user_repo,
            otp_cache,
            notification_publisher, // Temporarily disabled
            otp_expiry_minutes: otp_config.expiry_seconds as i64 / 60,
        }
    }

    pub async fn send_otp(&self, request: SendOtpRequest) -> SystemResult<()> {
        info!("Received OTP request for identifier: {} [{:?}]", request.identifier, request.identifier_type);

        // Check rate limiting
        debug!("Checking OTP rate limit for identifier: {}", request.identifier);
        if let Err(e) = self.otp_cache.check_otp_rate_limit(request.identifier.as_ref()).await {
            warn!("Rate limit exceeded for identifier: {}. Error: {:?}", request.identifier, e);
            return Err(e);
        }
        debug!("Rate limit check passed for identifier: {}", request.identifier);

        // Validate identifier format
        match request.identifier_type {
            IdentifierType::Email => {
                debug!("Validating email format for identifier: {}", request.identifier);
                if let Err(e) = self.validate_email_format(request.identifier.as_ref()) {
                    error!("Invalid email format for {}: {:?}", request.identifier, e);
                    return Err(e);
                }
            }
            IdentifierType::Phone => {
                debug!("Validating phone format for identifier: {}", request.identifier);
                if let Err(e) = self.validate_phone_format(request.identifier.as_ref()) {
                    error!("Invalid phone format for {}: {:?}", request.identifier, e);
                    return Err(e);
                }
            }
        }

        // Generate OTP
        let otp_code = OtpHelper::generate_otp(6);
        info!("Generated OTP for {}: {}", request.identifier, otp_code);

        // Store OTP in cache
        debug!("Storing OTP in cache for identifier: {}", request.identifier);
        if let Err(e) = self.otp_cache.store_otp(request.identifier.as_ref(), otp_code.as_ref(), self.otp_expiry_minutes).await {
            error!("Failed to store OTP in cache for {}: {:?}", request.identifier, e);
            return Err(e);
        }
        info!("OTP cached successfully for identifier: {}", request.identifier);

        // Send OTP via notification service
        info!("Sending OTP notification to identifier: {}", request.identifier);
        if let Err(e) = self.send_otp_notification(&request, otp_code.as_ref()).await {
            error!("Failed to send OTP notification to {}: {:?}", request.identifier, e);
            return Err(e);
        }
        info!("OTP notification sent successfully to identifier: {}", request.identifier);

        Ok(())
    }

    pub async fn verify_otp(&self, request: VerifyOtpRequest) -> SystemResult<()> {
        // Retrieve and validate OTP
        let stored_otp = self
            .otp_cache
            .get_otp(request.identifier.as_ref())
            .await?
            .ok_or(SystemError::OtpNotFound)?;

        if stored_otp != request.otp_code {
            return Err(SystemError::InvalidOtp);
        }

        // Mark OTP as used
        self.otp_cache.invalidate_otp(request.identifier.as_ref()).await?;

        // If this is for an existing user, mark as verified
        if let Ok(Some(mut user)) = self.user_repo.find_by_email(&request.identifier).await {
            if !user.is_verified {
                user.verify_account();
                self.user_repo.update(&user).await?;
            }
        } else if let Ok(Some(mut user)) = self.user_repo.find_by_phone(&request.identifier).await {
            if !user.is_verified {
                user.verify_account();
                self.user_repo.update(&user).await?;
            }
        }

        Ok(())
    }

    fn validate_email_format(&self, email: &str) -> SystemResult<()> {
        if !email.contains('@') || !email.contains('.') {
            return Err(SystemError::InternalError("Invalid email format".to_string()));
        }
        Ok(())
    }

    fn validate_phone_format(&self, phone: &str) -> SystemResult<()> {
        if phone.len() < 10 || !phone.chars().all(|c| c.is_ascii_digit() || c == '+') {
            return Err(SystemError::InternalError("Invalid phone format".to_string()));
        }
        Ok(())
    }

    async fn send_otp_notification(
        &self,
        request: &SendOtpRequest,
        otp_code: &str,
    ) -> SystemResult<()> {
        match request.identifier_type {
            IdentifierType::Email => Ok(self
                .notification_publisher
                .as_ref()
                .send_email_otp(&request.identifier.as_ref(), otp_code)
                .await?),
            IdentifierType::Phone => Ok(self
                .notification_publisher
                .as_ref()
                .send_sms_otp(request.identifier.as_ref(), otp_code)
                .await?),
        }
    }
}
