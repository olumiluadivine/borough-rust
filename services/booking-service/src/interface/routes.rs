use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use shared::entities::models::HealthResponse;

pub async fn health_check() -> Result<HttpResponse> {
    let response = HealthResponse {
        status: "OK".to_string(),
        timestamp: Utc::now(),
        service: "booking-service".to_string(),
    };
    Ok(HttpResponse::Ok().json(response))
}

pub async fn create_booking() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Create booking endpoint - implementation pending"
    })))
}

pub async fn get_bookings() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Get bookings endpoint - implementation pending"
    })))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check))
        .route("/bookings", web::post().to(create_booking))
        .route("/bookings", web::get().to(get_bookings));
}
