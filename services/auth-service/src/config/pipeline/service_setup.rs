use crate::cache::{AuthCacheService, OtpCacheService};
use crate::infrastructure::config::AppConfig;
use crate::infrastructure::database::login_attempt_repository_impl::PostgresLoginAttemptRepository;
use crate::infrastructure::database::password_reset_repository_impl::PostgresPasswordResetRepository;
use crate::infrastructure::database::refresh_token_repository_impl::PostgresRefreshTokenRepository;
use crate::infrastructure::database::security_question_repository_impl::{
    PostgresSecurityQuestionRepository, PostgresUserSecurityQuestionRepository,
};
use crate::infrastructure::database::user_repository_impl::PostgresUserRepository;
use crate::infrastructure::messaging::notification_publisher::NotificationPublisher;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use shared::utils::caching::CacheService;
use crate::application::use_cases::{LoginUseCase, OtpUseCase, PasswordResetUseCase, RefreshTokenUseCase, SecurityQuestionUseCase};

pub struct UseCases {
    pub login: Arc<LoginUseCase>,
    pub otp: Arc<OtpUseCase>,
    pub password_reset: Arc<PasswordResetUseCase>,
    pub security_question: Arc<SecurityQuestionUseCase>,
    pub refresh_token: Arc<RefreshTokenUseCase>,
}

pub fn build_use_cases(
    config: &AppConfig,
    db_pool: &Pool<Postgres>,
    redis_client: Arc<deadpool_redis::Pool>,
    notification_publisher: &Arc<NotificationPublisher>,
) -> UseCases {
    let user_repo = Arc::new(PostgresUserRepository::new(db_pool.clone()));
    let refresh_token_repo = Arc::new(PostgresRefreshTokenRepository::new(db_pool.clone()));
    let security_question_repo = Arc::new(PostgresSecurityQuestionRepository::new(db_pool.clone()));
    let user_security_question_repo =
        Arc::new(PostgresUserSecurityQuestionRepository::new(db_pool.clone()));
    let login_attempt_repo = Arc::new(PostgresLoginAttemptRepository::new(db_pool.clone()));
    let password_reset_repo = Arc::new(PostgresPasswordResetRepository::new(db_pool.clone()));

    let cache_service = CacheService::new(redis_client.clone(), config.redis_figure_config.clone());
    
    let auth_cache_service = AuthCacheService::new(cache_service.clone());
    let otp_cache_service = OtpCacheService::new(
        cache_service.clone(),
        (config.otp.rate_limit_window / 60) as i64,
        config.otp.max_requests_per_window as i32,
    );

    UseCases {
        login: Arc::new(LoginUseCase::new(
            user_repo.clone(),
            refresh_token_repo.clone(),
            login_attempt_repo.clone(),
            auth_cache_service.clone(),
            config.jwt.secret.clone(),
            5,
            30,
        )),
        otp: Arc::new(OtpUseCase::new(
            user_repo.clone(),
            otp_cache_service.clone(),
            notification_publisher.clone(),
            config.otp.clone(),
        )),
        password_reset: Arc::new(PasswordResetUseCase::new(
            user_repo.clone(),
            password_reset_repo.clone(),
            notification_publisher.clone(),
            config.jwt.secret.clone(),
        )),
        security_question: Arc::new(SecurityQuestionUseCase::new(
            security_question_repo.clone(),
            user_security_question_repo.clone(),
        )),
        refresh_token: Arc::new(RefreshTokenUseCase::new(
            user_repo.clone(),
            refresh_token_repo.clone(),
            auth_cache_service.clone(),
            config.jwt.secret.clone(),
        )),
    }
}
