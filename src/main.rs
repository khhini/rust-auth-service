use actix_web::{middleware::Logger, web, App, HttpServer};
use rust_auth_service::{
    api::api_v1_cfg,
    config::get_config_from_env,
    infrastructure::postgres_database::PostgresDatabase,
    util::{logging::custom_status_info, tracing::setup_tracing},
};
use std::{error::Error, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Arc::new(get_config_from_env());
    setup_tracing(&config.loki.get_url(), config.env)?;

    let db = PostgresDatabase::new(config.database.clone()).await;

    let app_data_config = web::Data::new(Arc::clone(&config));

    let _ = HttpServer::new(move || {
        App::new()
            .wrap(
                Logger::new(
                    "%{STATUS_INFO}xo %a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
                )
                .custom_response_replace("STATUS_INFO", |res| custom_status_info(res).to_string()),
            )
            .app_data(app_data_config.clone())
            .service(web::scope("/api/v1").configure(|cfg| api_v1_cfg(cfg, db.pool.clone())))
    })
    .bind((config.host.clone(), config.port))?
    .run()
    .await;

    Ok(())
}
