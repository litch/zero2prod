use actix_web::{HttpServer, HttpResponse};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}