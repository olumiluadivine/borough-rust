use crate::domain::entities::password_reset_token::PasswordResetToken;
use crate::domain::repositories::password_reset_repository::PasswordResetRepository;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::services::auth_domain_service::AuthDomainService;
use crate::infrastructure::messaging::notification_publisher::NotificationPublisher;
use shared::features::errors::{SuccessResponse, SystemError, SystemResult};
use std::sync::Arc;
use uuid::Uuid;
use shared::entities::dtos::auth::password::{PasswordResetConfirmRequest, PasswordResetRequest};
use shared::features::helper::otp_helper::OtpHelper;
use shared::features::helper::password_helper::PasswordHelper;

pub struct PasswordResetUseCase {
    user_repo: Arc<dyn UserRepository>,
    password_reset_repo: Arc<dyn PasswordResetRepository>, // Temporarily disabled
    notification_publisher: Arc<NotificationPublisher>,    // Temporarily disabled
    jwt_secret: String,
}

impl PasswordResetUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        password_reset_repo: Arc<dyn PasswordResetRepository>,
        notification_publisher: Arc<NotificationPublisher>,
        jwt_secret: String,
    ) -> Self {
        Self {
            user_repo,
            password_reset_repo,
            notification_publisher,
            jwt_secret,
        }
    }

    pub async fn request_password_reset(&self, request: PasswordResetRequest) -> SystemResult<SuccessResponse> {
        // Find user by email
        let user = match self
            .user_repo
            .as_ref()
            .find_by_email(request.identifier.as_ref())
            .await
        {
            Ok(user) => user.unwrap(),
            Err(_) => return Err(SystemError::UserNotFound(request.identifier.clone())),
        };
        
        // Generate reset security
        let reset_token = OtpHelper::generate_reset_token();
        let token_hash = PasswordHelper::hash_string(reset_token.as_ref())
            .map_err(|e| SystemError::InternalError(e.to_string()))?;

        // Store reset security
        let password_reset_token = PasswordResetToken::new(
            user.id,
            token_hash,
            chrono::Utc::now() + chrono::Duration::hours(1), // change later,
        );

        self.password_reset_repo
            .as_ref()
            .create(&password_reset_token)
            .await?;

        // Send reset email
        self.notification_publisher
            .as_ref()
            .send_password_reset_email(&user.email, &reset_token)
            .await.expect("TODO: panic message");

        Ok(SuccessResponse::Ok)
    }

    pub async fn confirm_password_reset(
        &self,
        request: PasswordResetConfirmRequest,
    ) -> SystemResult<SuccessResponse>{
        // Validate new password
        AuthDomainService::validate_new_password(request.new_password.as_ref())?;

        // Hash the provided security
        let token_hash = PasswordHelper::hash_string(request.token.as_ref())
            .map_err(|e| SystemError::InternalError(e.to_string()))?;

        // Find and validate reset security
        let mut reset_token = self
            .password_reset_repo
            .find_by_token_hash(&token_hash)
            .await?
            .ok_or(SystemError::InvalidResetToken)?;

        if !reset_token.is_valid() {
            return Err(SystemError::InvalidResetToken);
        }

        // Get user
        let mut user = match self
            .user_repo
            .as_ref()
            .find_by_id(&reset_token.user_id)
            .await
            {
                Ok(user) => user.unwrap(),
                Err(e) => return Err(SystemError::UserNotFound(e.to_string())),
            };

        // Hash new password
        let new_password_hash = AuthDomainService::hash_password(&request.new_password)?;

        // Update user password
        user.password_hash = new_password_hash;
        user.failed_login_attempts = 0;
        user.locked_until = None;
        user.updated_at = chrono::Utc::now();

        self.user_repo.update(&user).await?;

        // Mark reset security as used
        reset_token.mark_as_used();
        self.password_reset_repo.update(&reset_token).await?;

        // Revoke all refresh tokens for this user
        self.revoke_all_user_sessions(user.id).await?;

        // Send confirmation email
        let _ = self.notification_publisher
            .as_ref()
            .send_password_changed_confirmation(&user.email)
            .await;

        Ok(SuccessResponse::Ok)
    }

    async fn revoke_all_user_sessions(&self, user_id: Uuid) -> SystemResult<()> {
        // This would typically revoke all refresh tokens
        // Implementation depends on having refresh security repository
        Ok(())
    }
}
