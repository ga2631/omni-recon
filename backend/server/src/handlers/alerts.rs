use axum::Json;
use serde_json::json;

pub async fn list_alerts_handler() -> Json<serde_json::Value> {
    Json(json!({
        "success": true,
        "data": [
            {
                "id": "00000000-0000-0000-0000-000000000071",
                "order_id": "240830TIKTOK77123",
                "channel_code": "TIKTOK",
                "alert_type": "COD_MISMATCH",
                "severity": "CRITICAL",
                "expected_amount": 620000.0,
                "actual_amount": 600000.0,
                "discrepancy_amount": -20000.0,
                "status": "OPEN",
                "created_at": "2026-08-31T11:00:00Z",
                "notes": "ĐVVC GHTK thu hộ 600k nhưng giá trị đơn là 620k (lệch 20.000đ)"
            },
            {
                "id": "00000000-0000-0000-0000-000000000072",
                "order_id": "240830SHOPEE99182",
                "channel_code": "SHOPEE",
                "alert_type": "FEE_MISMATCH",
                "severity": "HIGH",
                "expected_amount": 165000.0,
                "actual_amount": 220000.0,
                "discrepancy_amount": -55000.0,
                "status": "OPEN",
                "created_at": "2026-08-31T11:05:00Z",
                "notes": "Phí sàn trừ 18.3% doanh thu (vượt mức cam kết gói 13.75%)"
            }
        ]
    }))
}
