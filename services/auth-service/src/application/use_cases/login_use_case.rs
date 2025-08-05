use crate::cache::auth_cache::AuthCacheService;
use crate::domain::repositories::login_attempt_repository::LoginAttemptRepository;
use crate::domain::repositories::refresh_token_repository::RefreshTokenRepository;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::{
    entities::{login_attempt::LoginAttempt, refresh_token::RefreshToken, user::User},
    services::auth_domain_service,
};
use std::sync::Arc;
use uuid::Uuid;
use shared::entities::dtos::auth::auth::{LoginRequest, LoginResponse};
use shared::features::errors::{SystemError, SystemResult};
use shared::features::helper::jwt_helper::JwtHelper;
use shared::features::helper::password_helper::PasswordHelper;

pub struct LoginUseCase {
    user_repo: Arc<dyn UserRepository>,
    refresh_token_repo: Arc<dyn RefreshTokenRepository>,
    login_attempt_repo: Arc<dyn LoginAttemptRepository>,
    cache_service: AuthCacheService,
    jwt_secret: String,
    max_login_attempts: i32,
    lockout_duration_minutes: i64,
}

impl LoginUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        refresh_token_repo: Arc<dyn RefreshTokenRepository>,
        login_attempt_repo: Arc<dyn LoginAttemptRepository>,
        cache_service: AuthCacheService,
        jwt_secret: String,
        max_login_attempts: i32,
        lockout_duration_minutes: i64,
    ) -> Self {
        Self {
            user_repo,
            refresh_token_repo,
            login_attempt_repo,
            cache_service,
            jwt_secret,
            max_login_attempts,
            lockout_duration_minutes,
        }
    }

    pub async fn execute(
        &self,
        request: LoginRequest,
        ip_address: String,
        user_agent: Option<String>,
    ) -> SystemResult<LoginResponse> {
        // Check rate limiting
        self.check_rate_limiting(&request.identifier, &ip_address)
            .await?;

        // Find user
        let mut user = self
            .user_repo
            .find_by_email(&request.identifier)
            .await?
            .ok_or(SystemError::InvalidCredentials)?;

        // Validate credentials
        let login_result = auth_domain_service::AuthDomainService::validate_login_credentials(
            &user,
            &request.password,
        );

        // Record login attempt
        let is_successful: bool = login_result.is_ok();
        let failure_reason: Option<String> = if let Err(ref e) = login_result {
            Some("".to_string())
        } else {
            None
        };

        let login_attempt = LoginAttempt::new(
            request.identifier.clone(),
            ip_address.clone(),
            user_agent,
            is_successful,
            failure_reason,
            None,
            None
        );

        self.login_attempt_repo.create(&login_attempt).await?;

        // Handle failed login
        if let Err(e) = login_result {
            user.increment_failed_attempts(self.max_login_attempts, self.lockout_duration_minutes);
            self.user_repo.update(&user).await?;
            return Err(e);
        }

        // Successful login - reset failed attempts
        user.reset_failed_attempts();
        let updated_user = self.user_repo.update(&user).await?;

        // Generate tokens
        let (access_token, refresh_token) = self.generate_tokens(&updated_user, ip_address).await?;

        // Cache user session
        self.cache_service
            .cache_user_session(user.id, &access_token)
            .await?;

        Ok(LoginResponse {
            access_token,
            refresh_token,
            expires_in: 3600, // 1 hour
            // user_info: UserInfo {
            //     id: updated_user.id,
            //     email: updated_user.email,
            //     role: updated_user.role,
            //     is_verified: updated_user.is_verified,
            //     created_at: updated_user.created_at,
            //     phone_number: None,
            // },
        })
    }

    async fn check_rate_limiting(&self, email: &str, ip_address: &str) -> SystemResult<()> {
        let since = chrono::Utc::now() - chrono::Duration::minutes(15);

        let email_attempts = self
            .login_attempt_repo
            .get_failed_attempts_count(email, since)
            .await?;

        let ip_attempts = self
            .login_attempt_repo
            .count_failed_attempts_by_ip(ip_address, since)
            .await?;

        if email_attempts >= 5 || ip_attempts >= 10 {
            return Err(SystemError::OtpRateLimitExceeded);
        }

        Ok(())
    }

    async fn generate_tokens(
        &self,
        user: &User,
        ip_address: String,
    ) -> SystemResult<(String, String)> {
        // Generate access security
        let access_token =
            JwtHelper::generate_access_token(
                user.id,
                user.email.clone(),
                user.role.clone(),
                vec![ip_address.clone()],
                &self.jwt_secret,
                1,
                "",
                "",
                Uuid::new_v4(),
                )
                .map_err(|e| SystemError::InternalError(e.to_string()))?;

        // Generate refresh security
        let refresh_token_value = JwtHelper::generate_secure_token();
        let refresh_token_hash = PasswordHelper::hash_string(&refresh_token_value)
            .map_err(|e| SystemError::InternalError(e.to_string()))?;

        let refresh_token_entity = RefreshToken::new(
            user.id,
            refresh_token_hash,
            None,
            Some(ip_address),
            None,
            None,
            chrono::Utc::now() + chrono::Duration::days(30),
        );

        self.refresh_token_repo
            .create(&refresh_token_entity)
            .await?;

        Ok((access_token, refresh_token_value))
    }
}
