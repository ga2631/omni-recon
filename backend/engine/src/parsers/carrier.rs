use super::StandardCarrierRecord;
use calamine::{open_workbook_auto, Reader};
use omni_common::{OmniError, Result};
use rust_decimal::Decimal;
use std::path::Path;
use tracing::info;

pub struct CarrierParser;

impl CarrierParser {
    pub fn parse_carrier_statement<P: AsRef<Path>>(
        file_path: P,
        carrier_code: &str,
    ) -> Result<Vec<StandardCarrierRecord>> {
        let path = file_path.as_ref();
        info!("Parsing carrier statement for {}: {:?}", carrier_code, path);

        let mut workbook = open_workbook_auto(path)
            .map_err(|e| OmniError::Parser(format!("Failed to read carrier Excel: {}", e)))?;

        let sheet_names = workbook.sheet_names();
        let first_sheet = sheet_names.first()
            .ok_or_else(|| OmniError::Parser("No sheet found in carrier workbook".to_string()))?;

        let range = workbook.worksheet_range(first_sheet)
            .map_err(|e| OmniError::Parser(format!("Failed to open sheet: {}", e)))?;

        let mut records = Vec::new();
        for row in range.rows().skip(1) {
            if let Some(tracking_cell) = row.get(0) {
                let tracking_code = tracking_cell.to_string().trim().to_string();
                if tracking_code.is_empty() {
                    continue;
                }

                let order_id = row.get(1).map(|c| c.to_string().trim().to_string()).unwrap_or_default();

                records.push(StandardCarrierRecord {
                    tracking_code,
                    order_id,
                    carrier_code: carrier_code.to_string(),
                    cod_amount: Decimal::ZERO,
                    shipping_fee: Decimal::ZERO,
                    charged_weight_gram: 0,
                    delivery_status: "DELIVERED".to_string(),
                    delivered_at: None,
                });
            }
        }

        info!("Successfully parsed {} carrier shipment records", records.len());
        Ok(records)
    }
}
