use crate::application::use_cases::LoginUseCase;
use actix_web::{web, HttpResponse, Result};
use std::sync::Arc;
use shared::entities::dtos::auth::auth::LoginRequest;
use shared::features::errors::map_auth_error_to_response;

pub struct AuthController {
    login_use_case: Arc<LoginUseCase>,
}

impl AuthController {
    pub fn new(login_use_case: Arc<LoginUseCase>) -> Self {
        Self { login_use_case }
    }

    pub async fn login(&self, req: web::Json<LoginRequest>) -> Result<HttpResponse> {
        match self
            .login_use_case
            .execute(req.into_inner(), "".to_string(), Some("".to_string()))
            .await
        {
            Ok(response) => Ok(HttpResponse::Ok().json(response)),
            Err(err) => Ok(map_auth_error_to_response(&err)),
        }
    }

    pub async fn logout(&self) -> Result<HttpResponse> {
        // For stateless JWT, logout is handled client-side
        // For refresh tokens; they should be revoked
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Logged out successfully"
        })))
    }

    pub async fn refresh_token(&self) -> Result<HttpResponse> {
        // Implement refresh security logic
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Token refreshed successfully"
        })))
    }
}
