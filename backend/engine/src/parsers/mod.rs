pub mod carrier;
pub mod shopee;
pub mod tiktok;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardSettlementRecord {
    pub order_id: String,
    pub platform: String,
    pub payout_id: Option<String>,
    pub transaction_type: String,
    pub order_status: String,
    pub buyer_username: Option<String>,
    pub tracking_number: Option<String>,
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
    pub ordered_at: Option<NaiveDateTime>,
    pub delivered_at: Option<NaiveDateTime>,
    pub settled_at: Option<NaiveDateTime>,
    pub raw_attributes: serde_json::Value,
    pub raw_fee_breakdown: serde_json::Value,
}

// Backward-compatible alias
pub type StandardOrderRecord = StandardSettlementRecord;

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

/// Helper function to parse diverse currency string representations (VND, dot/comma separators, negative accounting brackets)
pub fn parse_currency_decimal(s: &str) -> Decimal {
    let raw = s.trim();
    if raw.is_empty() || raw == "-" || raw == "N/A" || raw == "null" {
        return Decimal::ZERO;
    }

    let is_negative_bracket = raw.starts_with('(') && raw.ends_with(')');
    let cleaned = raw
        .trim_matches(|c| c == '(' || c == ')')
        .replace("VND", "")
        .replace("vnd", "")
        .replace('đ', "")
        .replace('₫', "")
        .replace(' ', "");

    // Detect format: If string contains both '.' and ',', determine thousands vs decimal separator
    let normalized = if cleaned.contains('.') && cleaned.contains(',') {
        let dot_pos = cleaned.rfind('.').unwrap();
        let comma_pos = cleaned.rfind(',').unwrap();
        if dot_pos > comma_pos {
            // e.g., 1,250.00
            cleaned.replace(',', "")
        } else {
            // e.g., 1.250,00
            cleaned.replace('.', "").replace(',', ".")
        }
    } else if cleaned.contains('.') {
        // e.g., 150.000 (VND thousands) or 150.50
        let parts: Vec<&str> = cleaned.split('.').collect();
        if parts.len() > 2 || (parts.len() == 2 && parts[1].len() == 3 && !cleaned.contains('-')) {
            // Likely Vietnamese thousands separator 150.000
            cleaned.replace('.', "")
        } else {
            cleaned
        }
    } else if cleaned.contains(',') {
        // e.g. 150,000 or 150,50
        let parts: Vec<&str> = cleaned.split(',').collect();
        if parts.len() > 2 || (parts.len() == 2 && parts[1].len() == 3) {
            cleaned.replace(',', "")
        } else {
            cleaned.replace(',', ".")
        }
    } else {
        cleaned
    };

    let mut decimal = Decimal::from_str(&normalized).unwrap_or(Decimal::ZERO);
    if is_negative_bracket && decimal > Decimal::ZERO {
        decimal = -decimal;
    }
    decimal
}

/// Helper function to parse multiple date/time string formats
pub fn parse_flexible_datetime(s: &str) -> Option<NaiveDateTime> {
    let raw = s.trim();
    if raw.is_empty() {
        return None;
    }

    let formats = [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%d-%m-%Y %H:%M:%S",
        "%d-%m-%Y %H:%M",
        "%d/%m/%Y %H:%M:%S",
        "%d/%m/%Y %H:%M",
        "%Y/%m/%d %H:%M:%S",
        "%Y/%m/%d %H:%M",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%dT%H:%M:%SZ",
        "%Y-%m-%d",
        "%d-%m-%Y",
        "%d/%m/%Y",
    ];

    for fmt in &formats {
        if let Ok(dt) = NaiveDateTime::parse_from_str(raw, fmt) {
            return Some(dt);
        }
        if let Ok(d) = chrono::NaiveDate::parse_from_str(raw, fmt) {
            if let Some(dt) = d.and_hms_opt(0, 0, 0) {
                return Some(dt);
            }
        }
    }

    None
}
