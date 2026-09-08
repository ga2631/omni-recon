use super::{parse_currency_decimal, parse_flexible_datetime, StandardSettlementRecord};
use calamine::{open_workbook_auto, Data, Range, Reader};
use omni_common::{OmniError, Result};
use rust_decimal::Decimal;
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tracing::info;

pub struct TikTokShopParser;

impl TikTokShopParser {
    /// Parse TikTok Shop Settlement Report (Excel .xlsx or CSV)
    pub fn parse_statement<P: AsRef<Path>>(file_path: P) -> Result<Vec<StandardSettlementRecord>> {
        let path = file_path.as_ref();
        info!("Parsing TikTok Shop settlement report: {:?}", path);

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        if ext == "csv" {
            Self::parse_csv(path)
        } else {
            Self::parse_excel(path)
        }
    }

    /// Parse TikTok Excel workbook (.xlsx, .xls)
    fn parse_excel(path: &Path) -> Result<Vec<StandardSettlementRecord>> {
        let mut workbook = open_workbook_auto(path)
            .map_err(|e| OmniError::Parser(format!("Failed to read TikTok Excel: {}", e)))?;

        let sheet_names = workbook.sheet_names();
        let first_sheet = sheet_names
            .first()
            .ok_or_else(|| OmniError::Parser("No worksheet found in TikTok workbook".to_string()))?;

        let range = workbook
            .worksheet_range(first_sheet)
            .map_err(|e| OmniError::Parser(format!("Failed to open sheet '{}': {}", first_sheet, e)))?;

        Self::parse_range(&range)
    }

    /// Parse Calamine 2D cell range
    fn parse_range(range: &Range<Data>) -> Result<Vec<StandardSettlementRecord>> {
        let rows: Vec<&[Data]> = range.rows().collect();
        if rows.is_empty() {
            return Ok(Vec::new());
        }

        // Find header row by scanning first 15 rows for key TikTok identifiers
        let mut header_row_idx = 0;
        let mut col_indices: HashMap<String, usize> = HashMap::new();
        let mut found_header = false;

        for (r_idx, row) in rows.iter().enumerate().take(15) {
            let candidate_cols: HashMap<String, usize> = row
                .iter()
                .enumerate()
                .map(|(c_idx, cell)| (cell.to_string().trim().to_lowercase(), c_idx))
                .collect();

            if candidate_cols.keys().any(|k| {
                k.contains("order id") || k.contains("settlement id") || k.contains("mã đơn hàng")
            }) {
                header_row_idx = r_idx;
                col_indices = candidate_cols;
                found_header = true;
                break;
            }
        }

        if !found_header && !rows.is_empty() {
            col_indices = rows[0]
                .iter()
                .enumerate()
                .map(|(c_idx, cell)| (cell.to_string().trim().to_lowercase(), c_idx))
                .collect();
        }

        let mut records = Vec::new();

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

        let idx_order_id = get_idx(&["order id", "order id / adjustment id", "mã đơn hàng", "order_id"]);
        let idx_payout_id = get_idx(&["settlement id", "payout id", "mã quyết toán", "mã đợt thanh toán"]);
        let idx_order_status = get_idx(&["order status", "trạng thái đơn hàng", "status"]);
        let idx_tracking = get_idx(&["tracking id", "tracking number", "mã vận đơn"]);
        let idx_gross = get_idx(&["subtotal before discount", "order subtotal", "subtotal", "giá trước giảm"]);
        let idx_seller_discount = get_idx(&["seller discount", "chiết khấu người bán", "giảm giá của shop"]);
        let idx_voucher = get_idx(&["tiktok shop discount", "platform discount", "voucher tiktok"]);
        let idx_comm = get_idx(&["marketplace commission fee", "platform commission fee", "commission fee", "phí hoa hồng sàn"]);
        let idx_payment = get_idx(&["transaction fee", "payment fee", "phí giao dịch"]);
        let idx_affiliate = get_idx(&["affiliate commission", "affiliate partner fee", "phí tiếp thị liên kết"]);
        let idx_buyer_ship = get_idx(&["customer shipping fee", "buyer shipping fee", "phí ship người mua"]);
        let idx_seller_ship = get_idx(&["shipping fee paid by seller", "actual shipping fee", "phí ship người bán"]);
        let idx_subsidy_ship = get_idx(&["shipping fee incentive", "shipping subsidy", "trợ giá ship"]);
        let idx_net = get_idx(&["actual amount transferred", "net settlement amount", "settlement amount", "tiền thực nhận"]);
        let idx_settled = get_idx(&["settlement time", "settlement date", "thời gian quyết toán"]);
        let idx_ordered = get_idx(&["order created time", "order time", "thời gian tạo đơn"]);

        for row in rows.iter().skip(header_row_idx + 1) {
            let order_id = idx_order_id
                .and_then(|i| row.get(i))
                .map(|c| c.to_string().trim().to_string())
                .unwrap_or_default();

            if order_id.is_empty() || order_id == "-" || order_id.to_lowercase().contains("total") {
                continue;
            }

            let cell_to_str = |opt_idx: Option<usize>| -> Option<String> {
                opt_idx.and_then(|i| row.get(i)).and_then(|c| {
                    let s = c.to_string().trim().to_string();
                    if s.is_empty() || s == "-" {
                        None
                    } else {
                        Some(s)
                    }
                })
            };

            let cell_to_decimal = |opt_idx: Option<usize>| -> Decimal {
                opt_idx
                    .and_then(|i| row.get(i))
                    .map(|c| parse_currency_decimal(&c.to_string()))
                    .unwrap_or(Decimal::ZERO)
            };

            let cell_to_datetime = |opt_idx: Option<usize>| -> Option<chrono::NaiveDateTime> {
                opt_idx
                    .and_then(|i| row.get(i))
                    .and_then(|c| parse_flexible_datetime(&c.to_string()))
            };

            let gross_amount = cell_to_decimal(idx_gross);
            let seller_discount = cell_to_decimal(idx_seller_discount);
            let platform_voucher = cell_to_decimal(idx_voucher);
            let commission_fee = cell_to_decimal(idx_comm);
            let payment_fee = cell_to_decimal(idx_payment);
            let affiliate_commission_fee = cell_to_decimal(idx_affiliate);
            let buyer_shipping_fee = cell_to_decimal(idx_buyer_ship);
            let seller_shipping_fee = cell_to_decimal(idx_seller_ship);
            let shipping_subsidy = cell_to_decimal(idx_subsidy_ship);
            let net_settlement = cell_to_decimal(idx_net);

            let raw_fee_breakdown = json!({
                "commission_fee": commission_fee,
                "payment_fee": payment_fee,
                "affiliate_commission_fee": affiliate_commission_fee,
                "seller_shipping_fee": seller_shipping_fee,
                "shipping_subsidy": shipping_subsidy
            });

            let mut raw_attrs = Map::new();
            if let Some(status) = cell_to_str(idx_order_status) {
                raw_attrs.insert("platform_status".to_string(), Value::String(status));
            }

            records.push(StandardSettlementRecord {
                order_id,
                platform: "TIKTOK".to_string(),
                payout_id: cell_to_str(idx_payout_id),
                transaction_type: "ORDER_SETTLEMENT".to_string(),
                order_status: cell_to_str(idx_order_status).unwrap_or_else(|| "COMPLETED".to_string()),
                buyer_username: None,
                tracking_number: cell_to_str(idx_tracking),
                gross_amount,
                seller_discount,
                platform_voucher,
                buyer_shipping_fee,
                seller_shipping_fee,
                shipping_subsidy,
                commission_fee,
                service_fee: Decimal::ZERO,
                payment_fee,
                affiliate_commission_fee,
                other_fees: Decimal::ZERO,
                net_settlement,
                ordered_at: cell_to_datetime(idx_ordered),
                delivered_at: None,
                settled_at: cell_to_datetime(idx_settled),
                raw_attributes: Value::Object(raw_attrs),
                raw_fee_breakdown,
            });
        }

        info!("Successfully parsed {} TikTok Shop settlement records", records.len());
        Ok(records)
    }

    /// Parse TikTok CSV format
    fn parse_csv(path: &Path) -> Result<Vec<StandardSettlementRecord>> {
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

        let idx_order_id = get_idx(&["order id", "order id / adjustment id", "mã đơn hàng"]);
        let idx_payout_id = get_idx(&["settlement id", "payout id", "mã quyết toán"]);
        let idx_order_status = get_idx(&["order status", "status"]);
        let idx_tracking = get_idx(&["tracking id", "tracking number"]);
        let idx_gross = get_idx(&["subtotal before discount", "order subtotal", "subtotal"]);
        let idx_seller_discount = get_idx(&["seller discount"]);
        let idx_voucher = get_idx(&["tiktok shop discount", "platform discount"]);
        let idx_comm = get_idx(&["marketplace commission fee", "commission fee"]);
        let idx_payment = get_idx(&["transaction fee", "payment fee"]);
        let idx_affiliate = get_idx(&["affiliate commission"]);
        let idx_buyer_ship = get_idx(&["customer shipping fee"]);
        let idx_seller_ship = get_idx(&["shipping fee paid by seller"]);
        let idx_subsidy_ship = get_idx(&["shipping fee incentive", "shipping subsidy"]);
        let idx_net = get_idx(&["actual amount transferred", "net settlement amount"]);
        let idx_settled = get_idx(&["settlement time", "settlement date"]);

        let mut records = Vec::new();

        for result in rdr.records() {
            let record = result.map_err(|e| OmniError::Parser(format!("CSV row read error: {}", e)))?;

            let order_id = idx_order_id
                .and_then(|i| record.get(i))
                .map(|c| c.trim().to_string())
                .unwrap_or_default();

            if order_id.is_empty() || order_id == "-" {
                continue;
            }

            let cell_to_str = |opt_idx: Option<usize>| -> Option<String> {
                opt_idx.and_then(|i| record.get(i)).and_then(|c| {
                    let s = c.trim().to_string();
                    if s.is_empty() || s == "-" {
                        None
                    } else {
                        Some(s)
                    }
                })
            };

            let cell_to_decimal = |opt_idx: Option<usize>| -> Decimal {
                opt_idx
                    .and_then(|i| record.get(i))
                    .map(parse_currency_decimal)
                    .unwrap_or(Decimal::ZERO)
            };

            let cell_to_datetime = |opt_idx: Option<usize>| -> Option<chrono::NaiveDateTime> {
                opt_idx
                    .and_then(|i| record.get(i))
                    .and_then(parse_flexible_datetime)
            };

            let commission_fee = cell_to_decimal(idx_comm);
            let payment_fee = cell_to_decimal(idx_payment);
            let affiliate_commission_fee = cell_to_decimal(idx_affiliate);
            let seller_shipping_fee = cell_to_decimal(idx_seller_ship);
            let shipping_subsidy = cell_to_decimal(idx_subsidy_ship);

            let raw_fee_breakdown = json!({
                "commission_fee": commission_fee,
                "payment_fee": payment_fee,
                "affiliate_commission_fee": affiliate_commission_fee,
                "seller_shipping_fee": seller_shipping_fee,
                "shipping_subsidy": shipping_subsidy
            });

            records.push(StandardSettlementRecord {
                order_id,
                platform: "TIKTOK".to_string(),
                payout_id: cell_to_str(idx_payout_id),
                transaction_type: "ORDER_SETTLEMENT".to_string(),
                order_status: cell_to_str(idx_order_status).unwrap_or_else(|| "COMPLETED".to_string()),
                buyer_username: None,
                tracking_number: cell_to_str(idx_tracking),
                gross_amount: cell_to_decimal(idx_gross),
                seller_discount: cell_to_decimal(idx_seller_discount),
                platform_voucher: cell_to_decimal(idx_voucher),
                buyer_shipping_fee: cell_to_decimal(idx_buyer_ship),
                seller_shipping_fee,
                shipping_subsidy,
                commission_fee,
                service_fee: Decimal::ZERO,
                payment_fee,
                affiliate_commission_fee,
                other_fees: Decimal::ZERO,
                net_settlement: cell_to_decimal(idx_net),
                ordered_at: None,
                delivered_at: None,
                settled_at: cell_to_datetime(idx_settled),
                raw_attributes: json!({}),
                raw_fee_breakdown,
            });
        }

        info!("Successfully parsed {} TikTok Shop CSV records", records.len());
        Ok(records)
    }
}
