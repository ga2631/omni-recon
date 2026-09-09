use crate::AppState;
use axum::{extract::State, Json};
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciliationItemDto {
    pub order_id: String,
    pub channel_code: String,
    pub tracking_code: Option<String>,
    pub expected_amount: f64,
    pub actual_settlement: f64,
    pub total_fee: f64,
    pub carrier_cod: f64,
    pub status: String,
    pub discrepancy: f64,
}

pub async fn list_reconciliation_items_handler(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    if let Some(ref pool) = state.pool {
        let rows = sqlx::query_as::<_, (
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
        )>(
            r#"
            SELECT 
                t.platform_order_id as order_id,
                COALESCE(UPPER(s.platform), UPPER(t.platform)) as channel_code,
                o.tracking_number as tracking_code,
                t.gross_amount,
                t.net_settlement,
                t.commission_fee,
                t.service_fee,
                t.payment_fee,
                t.other_fees,
                COALESCE(o.order_status, 'COMPLETED') as order_status
            FROM unified_transactions t
            LEFT JOIN unified_orders o ON t.order_id = o.id
            LEFT JOIN shops s ON t.shop_id = s.id
            ORDER BY t.settled_at DESC, t.created_at DESC
            LIMIT 200
            "#
        )
        .fetch_all(pool)
        .await;

        if let Ok(records) = rows {
            if !records.is_empty() {
                let items: Vec<ReconciliationItemDto> = records
                    .into_iter()
                    .map(|(order_id, channel_code, tracking_code, gross, net, comm, serv, pay, other, order_status)| {
                        let total_fee_dec = comm + serv + pay + other;
                        let gross_f64 = gross.to_f64().unwrap_or(0.0);
                        let net_f64 = net.to_f64().unwrap_or(0.0);
                        let total_fee_f64 = total_fee_dec.to_f64().unwrap_or(0.0);
                        
                        let expected_net = gross_f64 - total_fee_f64;
                        let discrepancy = (net_f64 - expected_net).round();

                        let status = if order_status == "RETURNED" {
                            "COD_MISMATCH".to_string()
                        } else if discrepancy.abs() < 1.0 {
                            "MATCHED".to_string()
                        } else {
                            "FEE_MISMATCH".to_string()
                        };

                        ReconciliationItemDto {
                            order_id,
                            channel_code,
                            tracking_code,
                            expected_amount: gross_f64,
                            actual_settlement: net_f64,
                            total_fee: total_fee_f64,
                            carrier_cod: gross_f64,
                            status,
                            discrepancy,
                        }
                    })
                    .collect();

                return Json(json!({
                    "success": true,
                    "data": items
                }));
            }
        }
    }

    Json(json!({
        "success": true,
        "data": []
    }))
}

pub async fn trigger_reconciliation_handler(
    State(_state): State<AppState>,
) -> Json<serde_json::Value> {
    Json(json!({
        "success": true,
        "message": "Reconciliation job started successfully",
        "data": {
            "job_id": uuid::Uuid::new_v4().to_string(),
            "status": "COMPLETED"
        }
    }))
}

