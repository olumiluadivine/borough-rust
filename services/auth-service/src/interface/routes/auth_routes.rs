use crate::config::pipeline::controller_setup::Controllers;
use crate::interface::controllers::RefreshTokenController;
use actix_web::{get, post, web};
use shared::entities::dtos::auth::auth::LoginRequest;
use shared::entities::dtos::auth::otp::{SendOtpRequest, VerifyOtpRequest};
use shared::entities::dtos::auth::password::{PasswordResetConfirmRequest, PasswordResetRequest};
use shared::entities::dtos::auth::question::{SetSecurityQuestionsRequest, VerifySecurityQuestionsRequest};
use shared::entities::dtos::auth::token::RefreshTokenRequest;
// pub fn refresh_token_routes() -> Scope {
//     web::scope("/tokens").route("/refresh", web::post().to(refresh_access_token))
// }

// Auth Controller Handlers
#[post("/login")]
pub async fn login(
    controller: web::Data<Controllers>,
    req: web::Json<LoginRequest>,
) -> actix_web::Result<actix_web::HttpResponse> {
    controller.auth.login(req).await
}

#[post("/logout")]
pub async fn logout(
    controller: web::Data<Controllers>,
) -> actix_web::Result<actix_web::HttpResponse> {
    controller.auth.logout().await
}

#[post("/refresh")]
pub async fn refresh_token(
    controller: web::Data<Controllers>,
) -> actix_web::Result<actix_web::HttpResponse> {
    controller.auth.refresh_token().await
}

// OTP Controller Handlers
#[post("/send")]
pub async fn send_otp(
    controller: web::Data<Controllers>,
    req: web::Json<SendOtpRequest>,
) -> actix_web::Result<actix_web::HttpResponse> {
    controller.otp.send_otp(req).await
}

#[post("/verify")]
pub async fn verify_otp(
    controller: web::Data<Controllers>,
    req: web::Json<VerifyOtpRequest>,
) -> actix_web::Result<actix_web::HttpResponse> {
    controller.otp.verify_otp(req).await
}

// Password Controller Handlers
#[post("/request")]
pub async fn request_password_reset(
    controller: web::Data<Controllers>,
    req: web::Json<PasswordResetRequest>,
) -> actix_web::Result<actix_web::HttpResponse> {
    controller.password.request_password_reset(req).await
}

#[post("/confirm")]
pub async fn confirm_password_reset(
    controller: web::Data<Controllers>,
    req: web::Json<PasswordResetConfirmRequest>,
) -> actix_web::Result<actix_web::HttpResponse> {
    controller.password.confirm_password_reset(req).await
}

// Security Question Controller Handlers
#[post("/create/{user_id}")]
pub async fn create_security_questions(
    controller: web::Data<Controllers>,
    path: web::Path<uuid::Uuid>,
    req: web::Json<SetSecurityQuestionsRequest>,
) -> actix_web::Result<actix_web::HttpResponse> {
    controller
        .security_question
        .create_security_questions(path, req)
        .await
}

#[get("/{user_id}")]
pub async fn get_user_questions(
    controller: web::Data<Controllers>,
    path: web::Path<uuid::Uuid>,
) -> actix_web::Result<actix_web::HttpResponse> {
    controller.security_question.get_user_questions(path).await
}

#[get("/")]
pub async fn get_questions(
    controller: web::Data<Controllers>,
) -> actix_web::Result<actix_web::HttpResponse> {
    controller.security_question.get_security_questions().await
}

#[post("/verify/{user_id}")]
pub async fn verify_security_answers(
    controller: web::Data<Controllers>,
    path: web::Path<uuid::Uuid>,
    req: web::Json<VerifySecurityQuestionsRequest>,
) -> actix_web::Result<actix_web::HttpResponse> {
    controller
        .security_question
        .verify_security_answers(path, req)
        .await
}

// Refresh Token Controller Handlers
async fn refresh_access_token(
    controller: web::Data<RefreshTokenController>,
    req: web::Json<RefreshTokenRequest>,
) -> actix_web::Result<actix_web::HttpResponse> {
    controller.refresh_access_token(req).await
}
