use actix_web::{middleware::Logger, web, App, HttpServer};
use rust_auth_service::{
    api::api_v1_cfg,
    config::AppConfig,
    utils::{logging::custom_status_info, tracing::setup_tracing},
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = AppConfig::from_env();

    if let Some(loki_config) = config.loki {
        setup_tracing(&loki_config.get_url(), config.env)?;
    } else {
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    }

    let _ = HttpServer::new(|| {
        App::new()
            .wrap(
                Logger::new(
                    "%{STATUS_INFO}xo %a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
                )
                .custom_response_replace("STATUS_INFO", |res| custom_status_info(res).to_string()),
            )
            .service(web::scope("/api/v1").configure(api_v1_cfg))
    })
    .bind((config.host, config.port))?
    .run()
    .await;

    Ok(())
}
