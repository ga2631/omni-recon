use axum::{
    body::Body,
    http::{header, Method, Request, StatusCode},
};
use http_body_util::BodyExt;
use omni_server::create_router;
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn test_healthcheck_endpoint() {
    let app = create_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"OK");
}

#[tokio::test]
async fn test_auth_login_success() {
    let app = create_router();

    let payload = serde_json::json!({
        "email": "admin@omnirecon.local",
        "password": "Admin@123456"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/auth/login")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["success"], true);
    assert!(json["data"]["token"].is_string());
    assert_eq!(json["data"]["user"]["email"], "admin@omnirecon.local");
    assert_eq!(json["data"]["user"]["role"], "ADMIN");
}

#[tokio::test]
async fn test_auth_login_invalid_credentials() {
    let app = create_router();

    let payload = serde_json::json!({
        "email": "admin@omnirecon.local",
        "password": "WrongPassword"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/auth/login")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["success"], false);
    assert_eq!(json["message"], "Invalid email or password");
}

#[tokio::test]
async fn test_channels_list_endpoint() {
    let app = create_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/channels")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["success"], true);
    let channels = json["data"].as_array().unwrap();
    assert!(channels.len() >= 3);

    let shopee = channels.iter().find(|c| c["platform_type"] == "SHOPEE").unwrap();
    assert_eq!(shopee["code"], "shopee_official");
    assert_eq!(shopee["is_active"], true);
}

#[tokio::test]
async fn test_dashboard_metrics_endpoint() {
    let app = create_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/dashboard/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["success"], true);
    assert!(json["data"]["summary"]["gross_revenue"].as_f64().unwrap() > 0.0);
    assert!(json["data"]["summary"]["net_settled"].as_f64().unwrap() > 0.0);
    assert!(json["data"]["channel_breakdown"].as_array().unwrap().len() >= 3);
    assert!(json["data"]["cashflow_trend"].as_array().unwrap().len() >= 7);
}

#[tokio::test]
async fn test_alerts_list_endpoint() {
    let app = create_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/alerts")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["success"], true);
    let alerts = json["data"].as_array().unwrap();
    assert!(!alerts.is_empty());
}

#[tokio::test]
async fn test_reconciliation_items_endpoint() {
    let app = create_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/reconciliation/items")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["success"], true);
    let items = json["data"].as_array().unwrap();
    assert!(items.len() >= 3);
}
