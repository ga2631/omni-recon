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

pub struct ShopeeParser;

impl ShopeeParser {
    /// Parse Shopee Income Statement (Auto-detects Excel .xlsx/.xls or CSV)
    pub fn parse_statement<P: AsRef<Path>>(file_path: P) -> Result<Vec<StandardSettlementRecord>> {
        let path = file_path.as_ref();
        info!("Parsing Shopee settlement report: {:?}", path);

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

    /// Parse Shopee Excel workbook (.xlsx, .xls)
    fn parse_excel(path: &Path) -> Result<Vec<StandardSettlementRecord>> {
        let mut workbook = open_workbook_auto(path)
            .map_err(|e| OmniError::Parser(format!("Failed to read Shopee Excel: {}", e)))?;

        let sheet_names = workbook.sheet_names();
        let first_sheet = sheet_names
            .first()
            .ok_or_else(|| OmniError::Parser("No worksheet found in Shopee workbook".to_string()))?;

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

        // Find header row by scanning first 15 rows for key Shopee identifiers
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
                k.contains("mã đơn hàng") || k.contains("order id") || k.contains("mã đơn")
            }) {
                header_row_idx = r_idx;
                col_indices = candidate_cols;
                found_header = true;
                break;
            }
        }

        if !found_header && !rows.is_empty() {
            // Fallback to row 0
            col_indices = rows[0]
                .iter()
                .enumerate()
                .map(|(c_idx, cell)| (cell.to_string().trim().to_lowercase(), c_idx))
                .collect();
        }

        let raw_headers: Vec<String> = rows[header_row_idx]
            .iter()
            .map(|c| c.to_string().trim().to_string())
            .collect();

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

        let idx_order_id = get_idx(&["mã đơn hàng", "order id", "mã đơn", "order_id"]);
        let idx_payout_id = get_idx(&["mã phiên thanh toán", "mã đợt thanh toán", "payout id", "mã giao dịch"]);
        let idx_order_status = get_idx(&["trạng thái đơn hàng", "trạng thái", "status"]);
        let idx_buyer = get_idx(&["tên người mua", "người mua", "buyer username", "buyer"]);
        let idx_tracking = get_idx(&["mã vận đơn", "mã đvvc", "tracking number", "tracking"]);
        let idx_gross = get_idx(&["tổng tiền hàng", "tổng tiền", "giá bán gốc", "tổng giá bán", "doanh thu", "order amount", "giá niêm yết"]);
        let idx_seller_discount = get_idx(&["giảm giá của shop", "khuyến mãi của shop", "seller discount"]);
        let idx_voucher = get_idx(&["voucher của shopee", "trợ giá của shopee", "shopee voucher", "mã giảm giá shopee"]);
        let idx_comm = get_idx(&["phí cố định", "commission fee", "phí hoa hồng"]);
        let idx_service = get_idx(&["phí dịch vụ", "service fee", "phí freeship xtra", "phí hoàn xu", "phí freeship"]);
        let idx_payment = get_idx(&["phí thanh toán", "payment fee", "phí xử lý giao dịch"]);
        let idx_buyer_ship = get_idx(&["phí vận chuyển người mua trả", "buyer shipping", "phí ship người mua"]);
        let idx_seller_ship = get_idx(&["phí vận chuyển thực tế", "phí vận chuyển người bán trả", "phí vận chuyển thực", "phí vận chuyển", "shipping fee"]);
        let idx_subsidy_ship = get_idx(&["trợ giá phí vận chuyển của shopee", "trợ giá vận chuyển từ shopee", "trợ giá vận chuyển", "trợ giá phí vận chuyển", "shipping rebate", "trợ giá shopee"]);
        let idx_net = get_idx(&["số tiền chuyển cho người bán", "tiền chuyển người bán", "số tiền nhận được", "tiền thực nhận", "net amount", "tổng tiền thanh toán", "actual amount"]);
        let idx_settled = get_idx(&["ngày hoàn thành", "thời gian hoàn thành", "thời gian thanh toán", "ngày thanh toán", "settlement date", "thời gian chuyển tiền", "completion date"]);
        let idx_ordered = get_idx(&["thời gian đặt hàng", "ngày đặt hàng", "order creation date"]);

        for row in rows.iter().skip(header_row_idx + 1) {
            let order_id = idx_order_id
                .and_then(|i| row.get(i))
                .map(|c| c.to_string().trim().to_string())
                .unwrap_or_default();

            if order_id.is_empty() || order_id == "-" || order_id.to_lowercase().contains("tổng cộng") {
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
            let service_fee = cell_to_decimal(idx_service);
            let payment_fee = cell_to_decimal(idx_payment);
            let buyer_shipping_fee = cell_to_decimal(idx_buyer_ship);
            let seller_shipping_fee = cell_to_decimal(idx_seller_ship);
            let shipping_subsidy = cell_to_decimal(idx_subsidy_ship);
            let net_settlement = cell_to_decimal(idx_net);

            // Raw fee breakdown dictionary for JSONB storage
            let raw_fee_breakdown = json!({
                "commission_fee": commission_fee,
                "fixed_fee": commission_fee,
                "service_fee": service_fee,
                "payment_fee": payment_fee,
                "buyer_shipping_fee": buyer_shipping_fee,
                "seller_shipping_fee": seller_shipping_fee,
                "actual_shipping_fee": seller_shipping_fee,
                "shipping_subsidy": shipping_subsidy,
                "shopee_shipping_subsidy": shipping_subsidy
            });

            // Capture ALL raw columns into raw_attributes to preserve complete lineage
            let mut raw_attrs = Map::new();
            for (col_i, header_name) in raw_headers.iter().enumerate() {
                if !header_name.is_empty() {
                    if let Some(cell_val) = row.get(col_i) {
                        let s = cell_val.to_string().trim().to_string();
                        if !s.is_empty() && s != "-" {
                            raw_attrs.insert(header_name.clone(), Value::String(s));
                        }
                    }
                }
            }

            if let Some(status) = cell_to_str(idx_order_status) {
                raw_attrs.insert("platform_status".to_string(), Value::String(status));
            }
            if let Some(buyer) = cell_to_str(idx_buyer) {
                raw_attrs.insert("buyer".to_string(), Value::String(buyer));
            }

            let order_status_str = cell_to_str(idx_order_status).unwrap_or_else(|| "Hoàn thành".to_string());

            records.push(StandardSettlementRecord {
                order_id,
                platform: "SHOPEE".to_string(),
                payout_id: cell_to_str(idx_payout_id),
                transaction_type: "ORDER_SETTLEMENT".to_string(),
                order_status: order_status_str,
                buyer_username: cell_to_str(idx_buyer),
                tracking_number: cell_to_str(idx_tracking),
                gross_amount,
                seller_discount,
                platform_voucher,
                buyer_shipping_fee,
                seller_shipping_fee,
                shipping_subsidy,
                commission_fee,
                service_fee,
                payment_fee,
                affiliate_commission_fee: Decimal::ZERO,
                other_fees: Decimal::ZERO,
                net_settlement,
                ordered_at: cell_to_datetime(idx_ordered),
                delivered_at: None,
                settled_at: cell_to_datetime(idx_settled),
                raw_attributes: Value::Object(raw_attrs),
                raw_fee_breakdown,
            });
        }

        info!("Successfully parsed {} Shopee settlement records", records.len());
        Ok(records)
    }

    /// Parse Shopee CSV format
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

        let raw_headers: Vec<String> = headers
            .iter()
            .map(|h| h.trim().to_string())
            .collect();

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

        let idx_order_id = get_idx(&["mã đơn hàng", "order id", "mã đơn", "order_id"]);
        let idx_payout_id = get_idx(&["mã phiên thanh toán", "mã đợt thanh toán", "payout id", "mã giao dịch"]);
        let idx_order_status = get_idx(&["trạng thái đơn hàng", "trạng thái", "status"]);
        let idx_buyer = get_idx(&["tên người mua", "người mua", "buyer username", "buyer"]);
        let idx_tracking = get_idx(&["mã vận đơn", "mã đvvc", "tracking number", "tracking"]);
        let idx_gross = get_idx(&["tổng tiền hàng", "tổng tiền", "giá bán gốc", "tổng giá bán", "doanh thu", "order amount", "giá niêm yết"]);
        let idx_seller_discount = get_idx(&["giảm giá của shop", "khuyến mãi của shop", "seller discount"]);
        let idx_voucher = get_idx(&["voucher của shopee", "trợ giá của shopee", "shopee voucher", "mã giảm giá shopee"]);
        let idx_comm = get_idx(&["phí cố định", "commission fee", "phí hoa hồng"]);
        let idx_service = get_idx(&["phí dịch vụ", "service fee", "phí freeship xtra", "phí hoàn xu", "phí freeship"]);
        let idx_payment = get_idx(&["phí thanh toán", "payment fee", "phí xử lý giao dịch"]);
        let idx_buyer_ship = get_idx(&["phí vận chuyển người mua trả", "buyer shipping", "phí ship người mua"]);
        let idx_seller_ship = get_idx(&["phí vận chuyển thực tế", "phí vận chuyển người bán trả", "phí vận chuyển thực", "phí vận chuyển", "shipping fee"]);
        let idx_subsidy_ship = get_idx(&["trợ giá phí vận chuyển của shopee", "trợ giá vận chuyển từ shopee", "trợ giá vận chuyển", "trợ giá phí vận chuyển", "shipping rebate", "trợ giá shopee"]);
        let idx_net = get_idx(&["số tiền chuyển cho người bán", "tiền chuyển người bán", "số tiền nhận được", "tiền thực nhận", "net amount", "tổng tiền thanh toán", "actual amount"]);
        let idx_settled = get_idx(&["ngày hoàn thành", "thời gian hoàn thành", "thời gian thanh toán", "ngày thanh toán", "settlement date", "thời gian chuyển tiền", "completion date"]);
        let idx_ordered = get_idx(&["thời gian đặt hàng", "ngày đặt hàng", "order creation date"]);

        let mut records = Vec::new();

        for result in rdr.records() {
            let record = result.map_err(|e| OmniError::Parser(format!("CSV row read error: {}", e)))?;

            let order_id = idx_order_id
                .and_then(|i| record.get(i))
                .map(|c| c.trim().to_string())
                .unwrap_or_default();

            if order_id.is_empty() || order_id == "-" || order_id.to_lowercase().contains("tổng cộng") {
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

            let gross_amount = cell_to_decimal(idx_gross);
            let seller_discount = cell_to_decimal(idx_seller_discount);
            let platform_voucher = cell_to_decimal(idx_voucher);
            let commission_fee = cell_to_decimal(idx_comm);
            let service_fee = cell_to_decimal(idx_service);
            let payment_fee = cell_to_decimal(idx_payment);
            let buyer_shipping_fee = cell_to_decimal(idx_buyer_ship);
            let seller_shipping_fee = cell_to_decimal(idx_seller_ship);
            let shipping_subsidy = cell_to_decimal(idx_subsidy_ship);
            let net_settlement = cell_to_decimal(idx_net);

            let raw_fee_breakdown = json!({
                "commission_fee": commission_fee,
                "fixed_fee": commission_fee,
                "service_fee": service_fee,
                "payment_fee": payment_fee,
                "buyer_shipping_fee": buyer_shipping_fee,
                "seller_shipping_fee": seller_shipping_fee,
                "actual_shipping_fee": seller_shipping_fee,
                "shipping_subsidy": shipping_subsidy,
                "shopee_shipping_subsidy": shipping_subsidy
            });

            // Capture ALL raw columns into raw_attributes to preserve complete lineage
            let mut raw_attrs = Map::new();
            for (col_i, header_name) in raw_headers.iter().enumerate() {
                if !header_name.is_empty() {
                    if let Some(cell_val) = record.get(col_i) {
                        let s = cell_val.trim().to_string();
                        if !s.is_empty() && s != "-" {
                            raw_attrs.insert(header_name.clone(), Value::String(s));
                        }
                    }
                }
            }

            if let Some(status) = cell_to_str(idx_order_status) {
                raw_attrs.insert("platform_status".to_string(), Value::String(status));
            }
            if let Some(buyer) = cell_to_str(idx_buyer) {
                raw_attrs.insert("buyer".to_string(), Value::String(buyer));
            }

            let order_status_str = cell_to_str(idx_order_status).unwrap_or_else(|| "Hoàn thành".to_string());

            records.push(StandardSettlementRecord {
                order_id,
                platform: "SHOPEE".to_string(),
                payout_id: cell_to_str(idx_payout_id),
                transaction_type: "ORDER_SETTLEMENT".to_string(),
                order_status: order_status_str,
                buyer_username: cell_to_str(idx_buyer),
                tracking_number: cell_to_str(idx_tracking),
                gross_amount,
                seller_discount,
                platform_voucher,
                buyer_shipping_fee,
                seller_shipping_fee,
                shipping_subsidy,
                commission_fee,
                service_fee,
                payment_fee,
                affiliate_commission_fee: Decimal::ZERO,
                other_fees: Decimal::ZERO,
                net_settlement,
                ordered_at: cell_to_datetime(idx_ordered),
                delivered_at: None,
                settled_at: cell_to_datetime(idx_settled),
                raw_attributes: Value::Object(raw_attrs),
                raw_fee_breakdown,
            });
        }

        info!("Successfully parsed {} Shopee CSV records", records.len());
        Ok(records)
    }
}
