use omni_common::AppConfig;
use omni_server::{create_router_with_state, AppState};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,omni_server=debug,omni_engine=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::load_from_env();
    info!("Starting OmniRecon Backend API Server on port {}", config.app_port);

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(p) => {
            info!("Successfully connected to PostgreSQL at {}", config.database_url);
            Some(p)
        }
        Err(e) => {
            error!("PostgreSQL connection failed: {}. Continuing without DB pool.", e);
            None
        }
    };

    let state = AppState {
        pool,
        config: config.clone(),
    };

    let app = create_router_with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.app_port));
    info!("🚀 Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

