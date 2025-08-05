use crate::application::use_cases::RefreshTokenUseCase;
use actix_web::{web, HttpResponse, Result};
use std::sync::Arc;
use shared::entities::dtos::auth::token::RefreshTokenRequest;
use shared::features::errors::map_auth_error_to_response;

pub struct RefreshTokenController {
    refresh_token_use_case: Arc<RefreshTokenUseCase>,
}

impl RefreshTokenController {
    pub fn new(refresh_token_use_case: Arc<RefreshTokenUseCase>) -> Self {
        Self {
            refresh_token_use_case,
        }
    }

    pub async fn refresh_access_token(
        &self,
        req: web::Json<RefreshTokenRequest>,
    ) -> Result<HttpResponse> {
        match self.refresh_token_use_case.execute(req.into_inner()).await {
            Ok(response) => Ok(HttpResponse::Ok().json(response)),
            Err(err) => Ok(map_auth_error_to_response(&err)),
        }
    }

    // pub async fn revoke_refresh_token(&self, req: web::Json<RefreshTokenRequest>) -> Result<HttpResponse> {
    //     match self.refresh_token_use_case.revoke_refresh_token(req.refresh_token.clone()).await {
    //         Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
    //             "message": "Refresh security revoked successfully"
    //         }))),
    //         Err(err) => Ok(map_auth_error_to_response(&err))
    //     }
    // }
    //
    // pub async fn revoke_all_user_tokens(&self, path: web::Path<Uuid>) -> Result<HttpResponse> {
    //     let user_id = path.into_inner();
    //
    //     match self.refresh_token_use_case.revoke_all_user_tokens(user_id).await {
    //         Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
    //             "message": "All refresh tokens revoked successfully"
    //         }))),
    //         Err(err) => Ok(map_auth_error_to_response(&err))
    //     }
    // }
    //
    // pub async fn get_user_active_tokens(&self, path: web::Path<Uuid>) -> Result<HttpResponse> {
    //     let user_id = path.into_inner();
    //
    //     match self.refresh_token_use_case.get_user_active_tokens(user_id).await {
    //         Ok(tokens) => Ok(HttpResponse::Ok().json(tokens)),
    //         Err(err) => Ok(map_auth_error_to_response(&err))
    //     }
    // }
}
