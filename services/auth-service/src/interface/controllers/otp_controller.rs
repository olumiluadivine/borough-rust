use crate::application::use_cases::OtpUseCase;
use actix_web::{web, HttpResponse, Result};
use std::sync::Arc;
use shared::entities::dtos::auth::otp::{SendOtpRequest, VerifyOtpRequest};
use shared::features::errors::{map_auth_error_to_response, map_success_to_response};

pub struct OtpController {
    otp_use_case: Arc<OtpUseCase>,
}

impl OtpController {
    pub fn new(otp_use_case: Arc<OtpUseCase>) -> Self {
        Self { otp_use_case }
    }

    pub async fn send_otp(&self, req: web::Json<SendOtpRequest>) -> Result<HttpResponse> {
        match self.otp_use_case.as_ref().send_otp(req.into_inner()).await {
            Ok(response) => Ok(map_success_to_response::<()>(response, None, None)),
            Err(err) => Ok(map_auth_error_to_response(&err)),
        }
    }

    pub async fn verify_otp(&self, req: web::Json<VerifyOtpRequest>) -> Result<HttpResponse> {
        match self.otp_use_case.as_ref().verify_otp(req.into_inner()).await {
            Ok(response) => Ok(map_success_to_response::<()>(response, None, None)),
            Err(err) => Ok(map_auth_error_to_response(&err)),
        }
    }
}
