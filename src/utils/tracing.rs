use std::error::Error;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use url::Url;
use std::process;
use crate::config::AppEnv;

pub fn setup_tracing(url: &str, env: AppEnv) -> Result<(), Box<dyn Error>> {
    let parsed_url = Url::parse(url).expect("Failed to parse loki url");

    let (layer, task) = tracing_loki::builder()
        .label("service_name", env!("CARGO_PKG_NAME"))?
        .label("environment", env.as_ref())?
        .extra_field("pid", format!("{}", process::id()))?
        .build_url(parsed_url)?;

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .parse("").unwrap();
    // We need to register our layer with `tracing`.
    tracing_subscriber::registry()
        .with(filter)
        .with(layer)
        // One could add more layers here, for example logging to stdout:
        .with(tracing_subscriber::fmt::Layer::new())
        .init();

    // The background task needs to be spawned so the logs actually get
    // delivered.
    tokio::spawn(task);

    Ok(())
}
