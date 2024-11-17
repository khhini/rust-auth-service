pub mod health_check;

use actix_web::web;
use crate::api::health_check::health_check_cfg;

pub fn api_v1_cfg(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/healthz").configure(health_check_cfg)
    );
}
