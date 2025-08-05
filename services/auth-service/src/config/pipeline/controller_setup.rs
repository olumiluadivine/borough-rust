use crate::config::pipeline::service_setup::UseCases;
use crate::interface::controllers::{
    AuthController, OtpController, PasswordController, RefreshTokenController,
    SecurityQuestionController,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct Controllers {
    pub auth: Arc<AuthController>,
    pub otp: Arc<OtpController>,
    pub password: Arc<PasswordController>,
    pub security_question: Arc<SecurityQuestionController>,
    pub refresh_token: Arc<RefreshTokenController>,
}

pub fn build_controllers(use_cases: UseCases) -> Controllers {
    Controllers {
        auth: Arc::new(AuthController::new(use_cases.login)),
        otp: Arc::new(OtpController::new(use_cases.otp)),
        password: Arc::new(PasswordController::new(use_cases.password_reset)),
        security_question: Arc::new(SecurityQuestionController::new(use_cases.security_question)),
        refresh_token: Arc::new(RefreshTokenController::new(use_cases.refresh_token)),
    }
}
