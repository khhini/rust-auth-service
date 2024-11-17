use crate::config::AppEnv;
use std::error::Error;
use std::process;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use url::Url;

pub fn setup_tracing(url: &str, env: AppEnv) -> Result<(), Box<dyn Error>> {
    let parsed_url = Url::parse(url).expect("Failed to parse loki url");

    let (layer, task) = tracing_loki::builder()
        .label("service_name", env!("CARGO_PKG_NAME"))?
        .label("environment", env.as_ref())?
        .extra_field("pid", format!("{}", process::id()))?
        .build_url(parsed_url)?;

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .parse("")
        .unwrap();

    tracing_subscriber::registry()
        .with(filter)
        .with(layer)
        .with(tracing_subscriber::fmt::Layer::new())
        .init();

    tokio::spawn(task);

    Ok(())
}
