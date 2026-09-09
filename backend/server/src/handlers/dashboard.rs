use crate::AppState;
use axum::{extract::State, Json};
use rust_decimal::prelude::ToPrimitive;
use serde_json::json;

pub async fn get_dashboard_metrics_handler(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    if let Some(ref pool) = state.pool {
        let summary_row = sqlx::query_as::<_, (
            Option<rust_decimal::Decimal>,
            Option<rust_decimal::Decimal>,
            Option<rust_decimal::Decimal>,
            i64,
        )>(
            r#"
            SELECT 
                SUM(gross_amount) as total_gross,
                SUM(net_settlement) as total_net,
                SUM(commission_fee + service_fee + payment_fee + other_fees) as total_fees,
                COUNT(*) as total_count
            FROM unified_transactions
            "#
        )
        .fetch_optional(pool)
        .await
        .unwrap_or(None);

        if let Some((Some(gross), Some(net), Some(fees), count)) = summary_row {
            if count > 0 {
                let gross_f64 = gross.to_f64().unwrap_or(0.0);
                let net_f64 = net.to_f64().unwrap_or(0.0);
                let fees_f64 = fees.to_f64().unwrap_or(0.0);

                // Channel breakdown
                let channel_rows = sqlx::query_as::<_, (
                    String,
                    Option<rust_decimal::Decimal>,
                    Option<rust_decimal::Decimal>,
                )>(
                    r#"
                    SELECT 
                        t.platform,
                        SUM(t.gross_amount) as channel_gross,
                        SUM(t.commission_fee + t.service_fee + t.payment_fee + t.other_fees) as channel_fees
                    FROM unified_transactions t
                    GROUP BY t.platform
                    "#
                )
                .fetch_all(pool)
                .await
                .unwrap_or_default();

                let channel_breakdown: Vec<serde_json::Value> = channel_rows
                    .into_iter()
                    .map(|(platform, ch_gross, ch_fees)| {
                        let ch_gross_f64 = ch_gross.and_then(|d| d.to_f64()).unwrap_or(0.0);
                        let ch_fees_f64 = ch_fees.and_then(|d| d.to_f64()).unwrap_or(0.0);
                        let fee_rate = if ch_gross_f64 > 0.0 {
                            ((ch_fees_f64 / ch_gross_f64) * 100.0 * 10.0).round() / 10.0
                        } else {
                            0.0
                        };
                        json!({
                            "channel": platform,
                            "revenue": ch_gross_f64,
                            "fees": ch_fees_f64,
                            "fee_rate": fee_rate
                        })
                    })
                    .collect();

                return Json(json!({
                    "success": true,
                    "data": {
                        "summary": {
                            "gross_revenue": gross_f64,
                            "net_settled": net_f64,
                            "total_platform_fees": fees_f64,
                            "cod_pending": 0.0,
                            "total_discrepancy_amount": (gross_f64 - fees_f64 - net_f64).abs(),
                            "discrepancy_count": count
                        },
                        "channel_breakdown": channel_breakdown,
                        "cashflow_trend": [
                            { "date": chrono::Utc::now().format("%Y-%m-%d").to_string(), "expected": gross_f64, "actual": net_f64 }
                        ]
                    }
                }));
            }
        }
    }

    // Default / fallback metrics when database is empty
    Json(json!({
        "success": true,
        "data": {
            "summary": {
                "gross_revenue": 0.0,
                "net_settled": 0.0,
                "total_platform_fees": 0.0,
                "cod_pending": 0.0,
                "total_discrepancy_amount": 0.0,
                "discrepancy_count": 0
            },
            "channel_breakdown": [],
            "cashflow_trend": []
        }
    }))
}
