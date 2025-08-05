use actix_web::{web, HttpResponse, Result};

pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "auth-service",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

pub async fn ready_check() -> Result<HttpResponse> {
    // TODO: Add actual readiness checks (database, cache, etc.)
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ready",
        "service": "auth-service",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

pub fn health_routes() -> actix_web::Scope {
    web::scope("/health")
        .route("", web::get().to(health_check))
        .route("/ready", web::get().to(ready_check))
}
