use axum::Json;
use serde_json::json;

pub async fn list_reconciliation_items_handler() -> Json<serde_json::Value> {
    Json(json!({
        "success": true,
        "data": [
            {
                "order_id": "240830SHOPEE88912",
                "channel_code": "SHOPEE",
                "tracking_code": "GHN992817263",
                "expected_amount": 450000.0,
                "actual_settlement": 405000.0,
                "total_fee": 45000.0,
                "carrier_cod": 450000.0,
                "status": "MATCHED",
                "discrepancy": 0.0
            },
            {
                "order_id": "240830TIKTOK77123",
                "channel_code": "TIKTOK",
                "tracking_code": "GHTK11200921",
                "expected_amount": 620000.0,
                "actual_settlement": 540000.0,
                "total_fee": 80000.0,
                "carrier_cod": 600000.0,
                "status": "COD_MISMATCH",
                "discrepancy": -20000.0
            },
            {
                "order_id": "240830SHOPEE99182",
                "channel_code": "SHOPEE",
                "tracking_code": "GHN88712300",
                "expected_amount": 1200000.0,
                "actual_settlement": 980000.0,
                "total_fee": 220000.0,
                "carrier_cod": 1200000.0,
                "status": "FEE_MISMATCH",
                "discrepancy": -55000.0
            }
        ]
    }))
}

pub async fn trigger_reconciliation_handler() -> Json<serde_json::Value> {
    Json(json!({
        "success": true,
        "message": "Reconciliation job started successfully",
        "data": {
            "job_id": "00000000-0000-0000-0000-000000000050",
            "status": "RUNNING"
        }
    }))
}
