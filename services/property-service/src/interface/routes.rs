use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use shared::features::HealthResponse;

pub async fn health_check() -> Result<HttpResponse> {
    let response = HealthResponse {
        status: "OK".to_string(),
        timestamp: Utc::now(),
        service: "property-service".to_string(),
    };
    Ok(HttpResponse::Ok().json(response))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}
