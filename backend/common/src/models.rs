use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ==============================================================================
// 1. Merchant / Organization
// ==============================================================================
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Merchant {
    pub id: Uuid,
    pub name: String,
    pub tax_id: Option<String>,
    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type Organization = Merchant;

// ==============================================================================
// 2. User & Authentication
// ==============================================================================
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub full_name: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ==============================================================================
// 3. Multi-channel Shop / Channel
// ==============================================================================
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Shop {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub code: String,
    pub name: String,
    pub platform: String,
    pub shop_identifier: Option<String>,
    pub config_json: serde_json::Value,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type Channel = Shop;

// ==============================================================================
// 4. Bronze Layer Metadata: Upload Logs
// ==============================================================================
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UploadLog {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub shop_id: Option<Uuid>,
    pub platform: String,
    pub report_type: String,
    pub original_filename: String,
    pub file_path: String,
    pub file_hash: String,
    pub file_size_bytes: i64,
    pub total_rows: i32,
    pub successful_rows: i32,
    pub failed_rows: i32,
    pub status: String,
    pub error_summary: serde_json::Value,
    pub uploaded_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
}

pub type StatementBatch = UploadLog;

// ==============================================================================
// 5. Silver Layer: Unified Orders
// ==============================================================================
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UnifiedOrder {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub shop_id: Uuid,
    pub upload_log_id: Option<Uuid>,
    pub platform: String,
    pub platform_order_id: String,
    pub order_status: String,
    pub buyer_username: Option<String>,
    pub tracking_number: Option<String>,
    pub ordered_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub raw_attributes: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ==============================================================================
// 6. Silver Layer: Unified Transactions
// ==============================================================================
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UnifiedTransaction {
    pub id: Uuid,
    pub order_id: Option<Uuid>,
    pub merchant_id: Uuid,
    pub shop_id: Uuid,
    pub upload_log_id: Uuid,
    pub platform: String,
    pub platform_order_id: String,
    pub payout_id: Option<String>,
    pub transaction_type: String,
    pub gross_amount: Decimal,
    pub seller_discount: Decimal,
    pub platform_voucher: Decimal,
    pub buyer_shipping_fee: Decimal,
    pub seller_shipping_fee: Decimal,
    pub shipping_subsidy: Decimal,
    pub commission_fee: Decimal,
    pub service_fee: Decimal,
    pub payment_fee: Decimal,
    pub affiliate_commission_fee: Decimal,
    pub other_fees: Decimal,
    pub net_settlement: Decimal,
    pub settled_at: Option<DateTime<Utc>>,
    pub raw_fee_breakdown: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

// ==============================================================================
// 7. Reconciliation Jobs
// ==============================================================================
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ReconciliationJob {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub title: String,
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
    pub status: String,
    pub progress_percent: i32,
    pub total_matched: i32,
    pub total_discrepant: i32,
    pub total_amount_expected: Decimal,
    pub total_amount_settled: Decimal,
    pub total_fee_deducted: Decimal,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

// ==============================================================================
// 8. Discrepancy Alerts
// ==============================================================================
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DiscrepancyAlert {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub job_id: Option<Uuid>,
    pub shop_code: String,
    pub order_id: String,
    pub tracking_code: Option<String>,
    pub alert_type: String,
    pub severity: String,
    pub expected_amount: Decimal,
    pub actual_amount: Decimal,
    pub discrepancy_amount: Decimal,
    pub notes: Option<String>,
    pub status: String,
    pub resolved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

// ==============================================================================
// 9. Alert Rules
// ==============================================================================
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AlertRule {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub rule_code: String,
    pub name: String,
    pub conditions_json: serde_json::Value,
    pub severity: String,
    pub is_enabled: bool,
    pub notification_channels: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

// ==============================================================================
// 10. Cashflow Timeline
// ==============================================================================
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CashflowTimeline {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub record_date: NaiveDate,
    pub shop_id: Option<Uuid>,
    pub gross_sales: Decimal,
    pub net_settlement: Decimal,
    pub total_fees: Decimal,
    pub cod_pending: Decimal,
    pub discrepancy_sum: Decimal,
    pub created_at: DateTime<Utc>,
}
