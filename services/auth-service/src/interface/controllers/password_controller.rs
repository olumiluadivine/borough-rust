use crate::application::use_cases::PasswordResetUseCase;
use actix_web::{web, HttpResponse, Result};
use std::sync::Arc;
use shared::entities::dtos::auth::password::{PasswordResetConfirmRequest, PasswordResetRequest};
use shared::features::errors::map_auth_error_to_response;

pub struct PasswordController {
    password_reset_use_case: Arc<PasswordResetUseCase>,
}

impl PasswordController {
    pub fn new(password_reset_use_case: Arc<PasswordResetUseCase>) -> Self {
        Self {
            password_reset_use_case,
        }
    }

    pub async fn request_password_reset(
        &self,
        req: web::Json<PasswordResetRequest>,
    ) -> Result<HttpResponse> {
        match self
            .password_reset_use_case
            .request_password_reset(req.into_inner())
            .await
        {
            Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
                "message": "Password reset instructions sent"
            }))),
            Err(err) => Ok(map_auth_error_to_response(&err)),
        }
    }

    pub async fn confirm_password_reset(
        &self,
        req: web::Json<PasswordResetConfirmRequest>,
    ) -> Result<HttpResponse> {
        match self
            .password_reset_use_case
            .confirm_password_reset(req.into_inner())
            .await
        {
            Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
                "message": "Password reset successfully"
            }))),
            Err(err) => Ok(map_auth_error_to_response(&err)),
        }
    }
}
