use crate::cache::auth_cache::AuthCacheService;
use crate::domain::entities::refresh_token::RefreshToken;
use crate::domain::repositories::refresh_token_repository::RefreshTokenRepository;
use crate::domain::repositories::user_repository::UserRepository;
use shared::features::errors::{SystemError, SystemResult};
use std::sync::Arc;
use uuid::Uuid;
use shared::entities::dtos::auth::auth::LoginResponse;
use shared::entities::dtos::auth::token::RefreshTokenRequest;
use shared::features::helper::jwt_helper::JwtHelper;
use shared::features::helper::password_helper::PasswordHelper;

pub struct RefreshTokenUseCase {
    user_repo: Arc<dyn UserRepository>,
    refresh_token_repo: Arc<dyn RefreshTokenRepository>,
    cache_service: AuthCacheService,
    jwt_secret: String,
}

impl RefreshTokenUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        refresh_token_repo: Arc<dyn RefreshTokenRepository>,
        cache_service: AuthCacheService,
        jwt_secret: String,
    ) -> Self {
        Self {
            user_repo,
            refresh_token_repo,
            cache_service,
            jwt_secret,
        }
    }

    pub async fn execute(&self, request: RefreshTokenRequest) -> SystemResult<LoginResponse> {
        // Hash the provided refresh security
        let token_hash = PasswordHelper::hash_string(&request.refresh_token)
            .map_err(|e| SystemError::InternalError(e.to_string()))?;

        // Find the refresh security
        let mut refresh_token = self
            .refresh_token_repo
            .find_by_token_hash(&token_hash)
            .await?
            .ok_or(SystemError::InvalidRefreshToken)?;

        // Validate the refresh security
        if !refresh_token.is_valid() {
            return Err(SystemError::InvalidRefreshToken);
        }

        // Get the user
        let user = self
            .user_repo
            .find_by_id(refresh_token.user_id)
            .await?
            .ok_or(SystemError::UserNotFound)?;

        // Check if user can still log in
        user.can_login()?;

        // Revoke the old refresh security
        refresh_token.revoke();
        self.refresh_token_repo.update(&refresh_token).await?;

        // Generate new tokens
        let new_access_token =
            JwtHelper::generate_access_token(
                user.id,
                user.email.clone(),
                user.role.clone(),
                vec![],
                &self.jwt_secret,
                1,
                "",
                "",
                Uuid::new_v4(),
            )
                .map_err(|e| SystemError::InternalError(e.to_string()))?;

        let new_refresh_token_value = JwtHelper::generate_secure_token();
        let new_refresh_token_hash = PasswordHelper::hash_string(&new_refresh_token_value)
            .map_err(|e| SystemError::InternalError(e.to_string()))?;

        let new_refresh_token = RefreshToken::new(
            user.id,
            new_refresh_token_hash,
            None,
            None,
            None,
            None,
            chrono::Utc::now() + chrono::Duration::days(30)
        );

        self.refresh_token_repo.create(&new_refresh_token).await?;

        // Cache new session
        self.cache_service
            .cache_user_session(user.id, &new_access_token)
            .await?;

        Ok(LoginResponse {
            access_token: new_access_token,
            refresh_token: new_refresh_token_value,
            expires_in: 3600,
            // user_info: UserInfo {
            //     id: user.id,
            //     email: user.email,
            //     role: user.role,
            //     is_verified: user.is_verified,
            //     created_at: user.created_at,
            //     phone_number: None,
            // },
        })
    }
}
