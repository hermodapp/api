use actix_web::HttpResponse;

/// Get(/health_check) returns a 200 to indicate the application is running
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
