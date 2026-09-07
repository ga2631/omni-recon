use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub org_id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub full_name: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Channel {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub platform_type: String,
    pub config_json: serde_json::Value,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct StatementBatch {
    pub id: Uuid,
    pub org_id: Uuid,
    pub channel_id: Option<Uuid>,
    pub filename: String,
    pub file_path: String,
    pub file_size_bytes: i64,
    pub total_rows: i32,
    pub status: String,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ReconciliationJob {
    pub id: Uuid,
    pub org_id: Uuid,
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

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DiscrepancyAlert {
    pub id: Uuid,
    pub org_id: Uuid,
    pub job_id: Option<Uuid>,
    pub channel_code: String,
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

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AlertRule {
    pub id: Uuid,
    pub org_id: Uuid,
    pub rule_code: String,
    pub name: String,
    pub conditions_json: serde_json::Value,
    pub severity: String,
    pub is_enabled: bool,
    pub notification_channels: serde_json::Value,
    pub created_at: DateTime<Utc>,
}
