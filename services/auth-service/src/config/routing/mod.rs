use crate::interface::routes::auth_routes;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn configure_services(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/auth")
            .service(auth_routes::login)
            .service(auth_routes::logout)
            .service(auth_routes::refresh_token)
            .service(
                web::scope("/password-reset")
                    .service(auth_routes::request_password_reset)
                    .service(auth_routes::confirm_password_reset)
            )
            .service(
                web::scope("/otp")
                    .service(auth_routes::send_otp)
                    .service(auth_routes::verify_otp)
            )
            .service(
                web::scope("/security-question")
                    .service(auth_routes::create_security_questions)
                    .service(auth_routes::get_user_questions)
                    .service(auth_routes::get_questions)
                    .service(auth_routes::verify_security_answers)
            )
    );
}
