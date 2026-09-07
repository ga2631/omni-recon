pub mod carrier;
pub mod shopee;
pub mod tiktok;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardOrderRecord {
    pub order_id: String,
    pub channel_code: String,
    pub payout_id: Option<String>,
    pub gross_amount: Decimal,
    pub seller_discount: Decimal,
    pub platform_voucher: Decimal,
    pub commission_fee: Decimal,
    pub service_fee: Decimal,
    pub payment_fee: Decimal,
    pub shipping_fee: Decimal,
    pub net_settlement: Decimal,
    pub settled_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardCarrierRecord {
    pub tracking_code: String,
    pub order_id: String,
    pub carrier_code: String,
    pub cod_amount: Decimal,
    pub shipping_fee: Decimal,
    pub charged_weight_gram: i32,
    pub delivery_status: String,
    pub delivered_at: Option<NaiveDateTime>,
}
