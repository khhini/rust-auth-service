use actix_web::{http::header::ContentType, web, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct HealthCheckMessage {
    status: String,
}

pub fn health_check_cfg(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(health_check_handler)));
}

async fn health_check_handler() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(HealthCheckMessage {
            status: String::from("healthy"),
        })
}
