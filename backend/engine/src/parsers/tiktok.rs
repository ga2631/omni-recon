use super::StandardOrderRecord;
use calamine::{open_workbook_auto, Reader};
use omni_common::{OmniError, Result};
use rust_decimal::Decimal;
use std::path::Path;
use tracing::info;

pub struct TikTokShopParser;

impl TikTokShopParser {
    pub fn parse_settlement<P: AsRef<Path>>(file_path: P) -> Result<Vec<StandardOrderRecord>> {
        let path = file_path.as_ref();
        info!("Parsing TikTok Shop settlement statement: {:?}", path);

        let mut workbook = open_workbook_auto(path)
            .map_err(|e| OmniError::Parser(format!("Failed to read TikTok Excel: {}", e)))?;

        let sheet_names = workbook.sheet_names();
        let first_sheet = sheet_names.first()
            .ok_or_else(|| OmniError::Parser("No sheet found in TikTok workbook".to_string()))?;

        let range = workbook.worksheet_range(first_sheet)
            .map_err(|e| OmniError::Parser(format!("Failed to open sheet: {}", e)))?;

        let mut records = Vec::new();
        for row in range.rows().skip(1) {
            if let Some(order_id_cell) = row.get(0) {
                let order_id = order_id_cell.to_string().trim().to_string();
                if order_id.is_empty() {
                    continue;
                }

                records.push(StandardOrderRecord {
                    order_id,
                    channel_code: "TIKTOK".to_string(),
                    payout_id: row.get(1).map(|c| c.to_string()),
                    gross_amount: Decimal::ZERO,
                    seller_discount: Decimal::ZERO,
                    platform_voucher: Decimal::ZERO,
                    commission_fee: Decimal::ZERO,
                    service_fee: Decimal::ZERO,
                    payment_fee: Decimal::ZERO,
                    shipping_fee: Decimal::ZERO,
                    net_settlement: Decimal::ZERO,
                    settled_at: None,
                });
            }
        }

        info!("Successfully parsed {} TikTok Shop records", records.len());
        Ok(records)
    }
}
