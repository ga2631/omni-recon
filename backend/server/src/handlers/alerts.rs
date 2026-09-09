use crate::AppState;
use axum::{extract::State, Json};
use rust_decimal::prelude::ToPrimitive;
use serde_json::json;

pub async fn list_alerts_handler(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    if let Some(ref pool) = state.pool {
        let rows = sqlx::query_as::<_, (
            uuid::Uuid,
            String,
            String,
            Option<String>,
            rust_decimal::Decimal,
            rust_decimal::Decimal,
            rust_decimal::Decimal,
            rust_decimal::Decimal,
            rust_decimal::Decimal,
            rust_decimal::Decimal,
            String,
            chrono::DateTime<chrono::Utc>,
        )>(
            r#"
            SELECT 
                t.id,
                t.platform_order_id as order_id,
                COALESCE(UPPER(s.platform), UPPER(t.platform)) as channel_code,
                o.tracking_number as tracking_code,
                t.gross_amount,
                t.net_settlement,
                t.commission_fee,
                t.service_fee,
                t.payment_fee,
                t.other_fees,
                COALESCE(o.order_status, 'COMPLETED') as order_status,
                t.created_at
            FROM unified_transactions t
            LEFT JOIN unified_orders o ON t.order_id = o.id
            LEFT JOIN shops s ON t.shop_id = s.id
            ORDER BY t.created_at DESC
            LIMIT 100
            "#
        )
        .fetch_all(pool)
        .await;

        if let Ok(records) = rows {
            if !records.is_empty() {
                let alerts: Vec<serde_json::Value> = records
                    .into_iter()
                    .filter_map(|(id, order_id, channel_code, _tracking, gross, net, comm, serv, pay, other, order_status, created_at)| {
                        let total_fee_dec = comm + serv + pay + other;
                        let gross_f64 = gross.to_f64().unwrap_or(0.0);
                        let net_f64 = net.to_f64().unwrap_or(0.0);
                        let total_fee_f64 = total_fee_dec.to_f64().unwrap_or(0.0);
                        let expected_net = gross_f64 - total_fee_f64;
                        let discrepancy = (net_f64 - expected_net).round();

                        if order_status == "RETURNED" {
                            Some(json!({
                                "id": id.to_string(),
                                "order_id": order_id,
                                "channel_code": channel_code,
                                "alert_type": "COD_MISMATCH",
                                "severity": "CRITICAL",
                                "expected_amount": gross_f64,
                                "actual_amount": net_f64,
                                "discrepancy_amount": -gross_f64,
                                "status": "OPEN",
                                "created_at": created_at.to_rfc3339(),
                                "notes": format!("Đơn hàng bị trả hàng hoàn tiền nhưng cần kiểm tra cước phí vận chuyển hoàn")
                            }))
                        } else if discrepancy.abs() >= 1.0 {
                            let severity = if discrepancy.abs() > 50000.0 { "CRITICAL" } else { "HIGH" };
                            Some(json!({
                                "id": id.to_string(),
                                "order_id": order_id,
                                "channel_code": channel_code,
                                "alert_type": "FEE_MISMATCH",
                                "severity": severity,
                                "expected_amount": expected_net,
                                "actual_amount": net_f64,
                                "discrepancy_amount": discrepancy,
                                "status": "OPEN",
                                "created_at": created_at.to_rfc3339(),
                                "notes": format!("Phí thực tế sàn trừ lệch {:+.0}đ so với tổng chi tiết biểu phí", discrepancy)
                            }))
                        } else {
                            None
                        }
                    })
                    .collect();

                return Json(json!({
                    "success": true,
                    "data": alerts
                }));
            }
        }
    }

    Json(json!({
        "success": true,
        "data": []
    }))
}
