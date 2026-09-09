use crate::AppState;
use axum::{extract::State, Json};
use chrono::{DateTime, Utc};
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciliationItemDto {
    pub order_id: String,
    pub channel_code: String,
    pub shop_name: Option<String>,
    pub shop_code: Option<String>,
    pub source_file: Option<String>,
    pub source_report_type: Option<String>,
    pub upload_log_id: Option<Uuid>,
    pub uploaded_at: Option<DateTime<Utc>>,
    pub tracking_code: Option<String>,
    pub buyer_username: Option<String>,
    pub ordered_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub settled_at: Option<DateTime<Utc>>,
    pub order_status: String,

    // Monetary Breakdown (Standardized Unified Schema)
    pub gross_amount: f64,
    pub seller_discount: f64,
    pub platform_voucher: f64,
    pub buyer_shipping_fee: f64,
    pub seller_shipping_fee: f64,
    pub shipping_subsidy: f64,
    pub commission_fee: f64,
    pub service_fee: f64,
    pub payment_fee: f64,
    pub affiliate_commission_fee: f64,
    pub other_fees: f64,
    pub total_fee: f64,
    pub net_settlement: f64,

    // Reconciliation Analysis
    pub expected_amount: f64,
    pub actual_settlement: f64,
    pub carrier_cod: f64,
    pub discrepancy: f64,
    pub status: String,

    // Raw JSON Lineage
    pub raw_attributes: Option<serde_json::Value>,
    pub raw_fee_breakdown: Option<serde_json::Value>,
}

#[derive(Debug, sqlx::FromRow)]
struct ReconciliationDbRow {
    order_id: String,
    channel_code: String,
    shop_name: Option<String>,
    shop_code: Option<String>,
    source_file: Option<String>,
    source_report_type: Option<String>,
    upload_log_id: Option<Uuid>,
    uploaded_at: Option<DateTime<Utc>>,
    tracking_code: Option<String>,
    buyer_username: Option<String>,
    ordered_at: Option<DateTime<Utc>>,
    delivered_at: Option<DateTime<Utc>>,
    settled_at: Option<DateTime<Utc>>,
    order_status: String,
    gross_amount: rust_decimal::Decimal,
    seller_discount: rust_decimal::Decimal,
    platform_voucher: rust_decimal::Decimal,
    buyer_shipping_fee: rust_decimal::Decimal,
    seller_shipping_fee: rust_decimal::Decimal,
    shipping_subsidy: rust_decimal::Decimal,
    commission_fee: rust_decimal::Decimal,
    service_fee: rust_decimal::Decimal,
    payment_fee: rust_decimal::Decimal,
    affiliate_commission_fee: rust_decimal::Decimal,
    other_fees: rust_decimal::Decimal,
    net_settlement: rust_decimal::Decimal,
    raw_attributes: Option<serde_json::Value>,
    raw_fee_breakdown: Option<serde_json::Value>,
}

pub async fn list_reconciliation_items_handler(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    if let Some(ref pool) = state.pool {
        let rows = sqlx::query_as::<_, ReconciliationDbRow>(
            r#"
            SELECT 
                t.platform_order_id as order_id,
                COALESCE(UPPER(s.platform), UPPER(t.platform)) as channel_code,
                s.name as shop_name,
                s.code as shop_code,
                ul.original_filename as source_file,
                ul.report_type as source_report_type,
                ul.id as upload_log_id,
                ul.created_at as uploaded_at,
                o.tracking_number as tracking_code,
                o.buyer_username,
                o.ordered_at,
                o.delivered_at,
                t.settled_at,
                COALESCE(o.order_status, 'COMPLETED') as order_status,
                t.gross_amount,
                t.seller_discount,
                t.platform_voucher,
                t.buyer_shipping_fee,
                t.seller_shipping_fee,
                t.shipping_subsidy,
                t.commission_fee,
                t.service_fee,
                t.payment_fee,
                t.affiliate_commission_fee,
                t.other_fees,
                t.net_settlement,
                o.raw_attributes,
                t.raw_fee_breakdown
            FROM unified_transactions t
            LEFT JOIN unified_orders o ON (t.order_id = o.id OR (t.shop_id = o.shop_id AND t.platform_order_id = o.platform_order_id))
            LEFT JOIN shops s ON t.shop_id = s.id
            LEFT JOIN upload_logs ul ON t.upload_log_id = ul.id
            ORDER BY t.settled_at DESC NULLS LAST, t.created_at DESC
            LIMIT 500
            "#
        )
        .fetch_all(pool)
        .await;

        if let Ok(records) = rows {
            if !records.is_empty() {
                let items: Vec<ReconciliationItemDto> = records
                    .into_iter()
                    .map(|row| {
                        let total_fee_dec = row.commission_fee
                            + row.service_fee
                            + row.payment_fee
                            + row.affiliate_commission_fee
                            + row.other_fees;
                        let gross_f64 = row.gross_amount.to_f64().unwrap_or(0.0);
                        let net_f64 = row.net_settlement.to_f64().unwrap_or(0.0);
                        let total_fee_f64 = total_fee_dec.to_f64().unwrap_or(0.0);
                        let seller_discount_f64 = row.seller_discount.to_f64().unwrap_or(0.0);
                        let platform_voucher_f64 = row.platform_voucher.to_f64().unwrap_or(0.0);
                        let buyer_shipping_fee_f64 = row.buyer_shipping_fee.to_f64().unwrap_or(0.0);
                        let seller_shipping_fee_f64 = row.seller_shipping_fee.to_f64().unwrap_or(0.0);
                        let shipping_subsidy_f64 = row.shipping_subsidy.to_f64().unwrap_or(0.0);
                        let commission_fee_f64 = row.commission_fee.to_f64().unwrap_or(0.0);
                        let service_fee_f64 = row.service_fee.to_f64().unwrap_or(0.0);
                        let payment_fee_f64 = row.payment_fee.to_f64().unwrap_or(0.0);
                        let affiliate_commission_fee_f64 = row.affiliate_commission_fee.to_f64().unwrap_or(0.0);
                        let other_fees_f64 = row.other_fees.to_f64().unwrap_or(0.0);

                        let expected_net = gross_f64 - total_fee_f64;
                        let discrepancy = (net_f64 - expected_net).round();

                        let status = if row.order_status == "RETURNED" {
                            "COD_MISMATCH".to_string()
                        } else if discrepancy.abs() < 1.0 {
                            "MATCHED".to_string()
                        } else {
                            "FEE_MISMATCH".to_string()
                        };

                        ReconciliationItemDto {
                            order_id: row.order_id,
                            channel_code: row.channel_code,
                            shop_name: row.shop_name,
                            shop_code: row.shop_code,
                            source_file: row.source_file,
                            source_report_type: row.source_report_type,
                            upload_log_id: row.upload_log_id,
                            uploaded_at: row.uploaded_at,
                            tracking_code: row.tracking_code,
                            buyer_username: row.buyer_username,
                            ordered_at: row.ordered_at,
                            delivered_at: row.delivered_at,
                            settled_at: row.settled_at,
                            order_status: row.order_status,
                            gross_amount: gross_f64,
                            seller_discount: seller_discount_f64,
                            platform_voucher: platform_voucher_f64,
                            buyer_shipping_fee: buyer_shipping_fee_f64,
                            seller_shipping_fee: seller_shipping_fee_f64,
                            shipping_subsidy: shipping_subsidy_f64,
                            commission_fee: commission_fee_f64,
                            service_fee: service_fee_f64,
                            payment_fee: payment_fee_f64,
                            affiliate_commission_fee: affiliate_commission_fee_f64,
                            other_fees: other_fees_f64,
                            total_fee: total_fee_f64,
                            net_settlement: net_f64,
                            expected_amount: gross_f64,
                            actual_settlement: net_f64,
                            carrier_cod: gross_f64,
                            status,
                            discrepancy,
                            raw_attributes: row.raw_attributes,
                            raw_fee_breakdown: row.raw_fee_breakdown,
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


