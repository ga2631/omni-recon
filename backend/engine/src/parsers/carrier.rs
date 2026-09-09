use super::{parse_currency_decimal, parse_flexible_datetime, StandardCarrierRecord};
use calamine::{open_workbook_auto, Data, Range, Reader};
use omni_common::{OmniError, Result};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
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

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        if ext == "csv" {
            Self::parse_csv(path, carrier_code)
        } else {
            Self::parse_excel(path, carrier_code)
        }
    }

    fn parse_excel(path: &Path, carrier_code: &str) -> Result<Vec<StandardCarrierRecord>> {
        let mut workbook = open_workbook_auto(path)
            .map_err(|e| OmniError::Parser(format!("Failed to read carrier Excel: {}", e)))?;

        let sheet_names = workbook.sheet_names();
        let first_sheet = sheet_names
            .first()
            .ok_or_else(|| OmniError::Parser("No sheet found in carrier workbook".to_string()))?;

        let range = workbook
            .worksheet_range(first_sheet)
            .map_err(|e| OmniError::Parser(format!("Failed to open sheet: {}", e)))?;

        Self::parse_range(&range, carrier_code)
    }

    fn parse_range(range: &Range<Data>, carrier_code: &str) -> Result<Vec<StandardCarrierRecord>> {
        let rows: Vec<&[Data]> = range.rows().collect();
        if rows.is_empty() {
            return Ok(Vec::new());
        }

        let mut header_row_idx = 0;
        let mut col_indices: HashMap<String, usize> = HashMap::new();

        for (r_idx, row) in rows.iter().enumerate().take(10) {
            let candidate_cols: HashMap<String, usize> = row
                .iter()
                .enumerate()
                .map(|(c_idx, cell)| (cell.to_string().trim().to_lowercase(), c_idx))
                .collect();

            if candidate_cols.keys().any(|k| {
                k.contains("mã vận đơn") || k.contains("tracking") || k.contains("mã đơn")
            }) {
                header_row_idx = r_idx;
                col_indices = candidate_cols;
                break;
            }
        }

        let get_idx = |names: &[&str]| -> Option<usize> {
            for name in names {
                for (key, &idx) in &col_indices {
                    if key.contains(name) {
                        return Some(idx);
                    }
                }
            }
            None
        };

        let idx_tracking = get_idx(&["mã vận đơn", "tracking code", "tracking number", "mã đvvc"]);
        let idx_order_id = get_idx(&["mã đơn hàng", "mã đơn đối tác", "order id", "order_sn"]);
        let idx_cod = get_idx(&["tiền thu hộ", "tiền cod", "cod amount", "tiền thu"]);
        let idx_ship_fee = get_idx(&["cước phí", "phí vận chuyển", "shipping fee", "tổng cước"]);
        let idx_weight = get_idx(&["trọng lượng", "khối lượng tính cước", "cân nặng", "weight"]);
        let idx_status = get_idx(&["trạng thái giao hàng", "trạng thái", "status"]);
        let idx_delivered = get_idx(&["ngày giao hàng", "thời gian giao", "delivered date", "delivered at"]);

        let mut records = Vec::new();

        for row in rows.iter().skip(header_row_idx + 1) {
            let tracking_code = idx_tracking
                .and_then(|i| row.get(i))
                .map(|c| c.to_string().trim().to_string())
                .unwrap_or_default();

            if tracking_code.is_empty() || tracking_code == "-" {
                continue;
            }

            let order_id = idx_order_id
                .and_then(|i| row.get(i))
                .map(|c| c.to_string().trim().to_string())
                .unwrap_or_default();

            let cod_amount = idx_cod
                .and_then(|i| row.get(i))
                .map(|c| parse_currency_decimal(&c.to_string()))
                .unwrap_or(Decimal::ZERO);

            let shipping_fee = idx_ship_fee
                .and_then(|i| row.get(i))
                .map(|c| parse_currency_decimal(&c.to_string()))
                .unwrap_or(Decimal::ZERO);

            let charged_weight_gram = idx_weight
                .and_then(|i| row.get(i))
                .and_then(|c| c.to_string().replace("g", "").replace("kg", "").trim().parse::<i32>().ok())
                .unwrap_or(0);

            let delivery_status = idx_status
                .and_then(|i| row.get(i))
                .map(|c| c.to_string().trim().to_string())
                .unwrap_or_else(|| "DELIVERED".to_string());

            let delivered_at = idx_delivered
                .and_then(|i| row.get(i))
                .and_then(|c| parse_flexible_datetime(&c.to_string()));

            records.push(StandardCarrierRecord {
                tracking_code,
                order_id,
                carrier_code: carrier_code.to_string(),
                cod_amount,
                shipping_fee,
                charged_weight_gram,
                delivery_status,
                delivered_at,
            });
        }

        info!("Successfully parsed {} carrier shipment records", records.len());
        Ok(records)
    }

    fn parse_csv(path: &Path, carrier_code: &str) -> Result<Vec<StandardCarrierRecord>> {
        let file = File::open(path)
            .map_err(|e| OmniError::Parser(format!("Failed to open CSV file: {}", e)))?;
        let reader = BufReader::new(file);
        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .trim(csv::Trim::All)
            .from_reader(reader);

        let headers = rdr
            .headers()
            .map_err(|e| OmniError::Parser(format!("Failed to read CSV headers: {}", e)))?
            .clone();

        let col_indices: HashMap<String, usize> = headers
            .iter()
            .enumerate()
            .map(|(idx, h)| (h.trim().to_lowercase(), idx))
            .collect();

        let get_idx = |names: &[&str]| -> Option<usize> {
            for name in names {
                for (key, &idx) in &col_indices {
                    if key.contains(name) {
                        return Some(idx);
                    }
                }
            }
            None
        };

        let idx_tracking = get_idx(&["mã vận đơn", "tracking code", "tracking number", "mã đvvc"]);
        let idx_order_id = get_idx(&["mã đơn hàng", "mã đơn đối tác", "order id", "order_sn"]);
        let idx_cod = get_idx(&["tiền thu hộ", "tiền cod", "cod amount", "tiền thu"]);
        let idx_ship_fee = get_idx(&["cước phí", "phí vận chuyển", "shipping fee"]);
        let idx_weight = get_idx(&["trọng lượng", "khối lượng tính cước", "weight"]);
        let idx_status = get_idx(&["trạng thái giao hàng", "trạng thái", "status"]);
        let idx_delivered = get_idx(&["ngày giao hàng", "thời gian giao", "delivered date"]);

        let mut records = Vec::new();

        for result in rdr.records() {
            let record = result.map_err(|e| OmniError::Parser(format!("CSV row read error: {}", e)))?;

            let tracking_code = idx_tracking
                .and_then(|i| record.get(i))
                .map(|c| c.trim().to_string())
                .unwrap_or_default();

            if tracking_code.is_empty() || tracking_code == "-" {
                continue;
            }

            let order_id = idx_order_id
                .and_then(|i| record.get(i))
                .map(|c| c.trim().to_string())
                .unwrap_or_default();

            let cod_amount = idx_cod
                .and_then(|i| record.get(i))
                .map(parse_currency_decimal)
                .unwrap_or(Decimal::ZERO);

            let shipping_fee = idx_ship_fee
                .and_then(|i| record.get(i))
                .map(parse_currency_decimal)
                .unwrap_or(Decimal::ZERO);

            let charged_weight_gram = idx_weight
                .and_then(|i| record.get(i))
                .and_then(|c| c.replace("g", "").replace("kg", "").trim().parse::<i32>().ok())
                .unwrap_or(0);

            let delivery_status = idx_status
                .and_then(|i| record.get(i))
                .map(|c| c.trim().to_string())
                .unwrap_or_else(|| "DELIVERED".to_string());

            let delivered_at = idx_delivered
                .and_then(|i| record.get(i))
                .and_then(parse_flexible_datetime);

            records.push(StandardCarrierRecord {
                tracking_code,
                order_id,
                carrier_code: carrier_code.to_string(),
                cod_amount,
                shipping_fee,
                charged_weight_gram,
                delivery_status,
                delivered_at,
            });
        }

        info!("Successfully parsed {} carrier CSV records", records.len());
        Ok(records)
    }
}
