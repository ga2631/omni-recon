use crate::handlers::{
    alerts::list_alerts_handler,
    auth::{login_handler, me_handler},
    channels::list_channels_handler,
    dashboard::get_dashboard_metrics_handler,
    reconciliation::{list_reconciliation_items_handler, trigger_reconciliation_handler},
    statements::{list_batches_handler, upload_statement_handler},
};
use crate::ws::ws_handler;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub fn create_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_routes = Router::new()
        // Auth
        .route("/auth/login", post(login_handler))
        .route("/auth/me", get(me_handler))
        // Channels
        .route("/channels", get(list_channels_handler))
        // Statements
        .route("/statements/upload", post(upload_statement_handler))
        .route("/statements/batches", get(list_batches_handler))
        // Reconciliation
        .route("/reconciliation/items", get(list_reconciliation_items_handler))
        .route("/reconciliation/run", post(trigger_reconciliation_handler))
        // Alerts
        .route("/alerts", get(list_alerts_handler))
        // Dashboard Metrics
        .route("/dashboard/metrics", get(get_dashboard_metrics_handler));

    Router::new()
        .nest("/api/v1", api_routes)
        .route("/ws", get(ws_handler))
        .route("/healthz", get(|| async { "OK" }))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}
