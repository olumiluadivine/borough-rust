use crate::application::use_cases::LoginUseCase;
use actix_web::{web, HttpResponse, Result};
use std::sync::Arc;
use shared::entities::dtos::auth::auth::LoginRequest;
use shared::features::errors::{map_auth_error_to_response, map_success_to_response};

pub struct AuthController {
    login_use_case: Arc<LoginUseCase>,
}

impl AuthController {
    pub fn new(login_use_case: Arc<LoginUseCase>) -> Self {
        Self { login_use_case }
    }

    pub async fn login(
        &self,
        req: web::Json<LoginRequest>,
        http_req: actix_web::HttpRequest,
    ) -> Result<HttpResponse> {
        // Get IP address
        let ip_address = http_req
            .connection_info()
            .realip_remote_addr()
            .map(|s| s.to_string())
            .unwrap_or_default();

        // Get User-Agent header
        let user_agent = http_req
            .headers()
            .get(actix_web::http::header::USER_AGENT)
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        match self
            .login_use_case
            .as_ref()
            .execute(req.into_inner(), ip_address, user_agent)
            .await
        {
            Ok((response, success)) => Ok(map_success_to_response(success, Some(response), None)),
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
