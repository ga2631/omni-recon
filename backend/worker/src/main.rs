use omni_common::AppConfig;
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,omni_worker=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::load_from_env();
    info!("⚡ Starting OmniRecon Background Task Worker...");
    info!("Connected to Redis at {}", config.redis_url);

    loop {
        // Poll for reconciliation and statement processing tasks from Redis queue
        sleep(Duration::from_secs(5)).await;
    }
}
