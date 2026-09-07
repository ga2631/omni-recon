use axum::Json;
use serde_json::json;

pub async fn get_dashboard_metrics_handler() -> Json<serde_json::Value> {
    Json(json!({
        "success": true,
        "data": {
            "summary": {
                "gross_revenue": 1425800000.0,
                "net_settled": 1218900000.0,
                "total_platform_fees": 206900000.0,
                "cod_pending": 85400000.0,
                "total_discrepancy_amount": 14750000.0,
                "discrepancy_count": 48
            },
            "channel_breakdown": [
                { "channel": "Shopee", "revenue": 820000000.0, "fees": 123000000.0, "fee_rate": 15.0 },
                { "channel": "TikTok Shop", "revenue": 480000000.0, "fees": 67200000.0, "fee_rate": 14.0 },
                { "channel": "Lazada", "revenue": 125800000.0, "fees": 16700000.0, "fee_rate": 13.2 }
            ],
            "cashflow_trend": [
                { "date": "2026-08-25", "expected": 48000000.0, "actual": 47500000.0 },
                { "date": "2026-08-26", "expected": 52000000.0, "actual": 51200000.0 },
                { "date": "2026-08-27", "expected": 61000000.0, "actual": 58900000.0 },
                { "date": "2026-08-28", "expected": 74000000.0, "actual": 71800000.0 },
                { "date": "2026-08-29", "expected": 68000000.0, "actual": 67200000.0 },
                { "date": "2026-08-30", "expected": 82000000.0, "actual": 79500000.0 },
                { "date": "2026-08-31", "expected": 95000000.0, "actual": 91300000.0 }
            ]
        }
    }))
}
