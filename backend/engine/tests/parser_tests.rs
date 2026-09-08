use omni_engine::duckdb_runner::DuckDbRunner;
use omni_engine::parsers::shopee::ShopeeParser;
use omni_engine::parsers::tiktok::TikTokShopParser;
use rust_decimal::Decimal;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_shopee_csv_parsing_and_duckdb_staging() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("shopee_sample.csv");

    // Create a mock Shopee settlement CSV
    let csv_content = "\
Mã đơn hàng,Mã phiên thanh toán,Giá bán gốc,Giảm giá của Shop,Voucher của Shopee,Phí cố định,Phí dịch vụ,Phí thanh toán,Phí vận chuyển người bán trả,Trợ giá vận chuyển từ Shopee,Số tiền nhận được,Thời gian thanh toán
240901SHOPEE01,PAY_VN_001,250000,10000,15000,10000,15000,11250,0,0,203750,01-09-2026 14:30
240901SHOPEE02,PAY_VN_001,500000,50000,0,20000,30000,22500,15000,15000,377500,01-09-2026 14:35
";

    {
        let mut file = File::create(&file_path).unwrap();
        file.write_all(csv_content.as_bytes()).unwrap();
    }

    let records = ShopeeParser::parse_statement(&file_path).expect("Failed to parse Shopee CSV");
    assert_eq!(records.len(), 2);

    let r1 = &records[0];
    assert_eq!(r1.order_id, "240901SHOPEE01");
    assert_eq!(r1.payout_id.as_deref(), Some("PAY_VN_001"));
    assert_eq!(r1.gross_amount, Decimal::from(250000));
    assert_eq!(r1.seller_discount, Decimal::from(10000));
    assert_eq!(r1.commission_fee, Decimal::from(10000));
    assert_eq!(r1.service_fee, Decimal::from(15000));
    assert_eq!(r1.payment_fee, Decimal::from(11250));
    assert_eq!(r1.net_settlement, Decimal::from(203750));
    assert!(r1.settled_at.is_some());

    // DuckDB In-Memory Staging & Sanity verification
    let runner = DuckDbRunner::in_memory().expect("Failed to create in-memory DuckDB");
    runner.stage_settlement_records(&records).expect("Failed to stage records in DuckDB");

    let sanity = runner.verify_financial_integrity().expect("Failed to verify integrity");
    assert_eq!(sanity.total_rows, 2);
    assert_eq!(sanity.total_gross, Decimal::from(750000));
    assert_eq!(sanity.balanced_rows, 2);
    assert_eq!(sanity.discrepant_rows, 0);
}

#[test]
fn test_tiktok_csv_parsing_and_duckdb_staging() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("tiktok_sample.csv");

    // Create a mock TikTok settlement CSV
    let csv_content = "\
Order ID,Settlement ID,Subtotal Before Discount,Seller Discount,TikTok Shop Discount,Marketplace Commission Fee,Transaction Fee,Affiliate Commission,Shipping Fee Paid by Seller,Shipping Fee Incentive,Actual Amount Transferred,Settlement Time
5789123456789,TT_SETTLE_101,300000,20000,30000,15000,10500,15000,0,0,239500,2026-09-01 16:00:00
5789123456790,TT_SETTLE_101,150000,0,10000,7500,5250,0,0,0,137250,2026-09-01 16:05:00
";

    {
        let mut file = File::create(&file_path).unwrap();
        file.write_all(csv_content.as_bytes()).unwrap();
    }

    let records = TikTokShopParser::parse_statement(&file_path).expect("Failed to parse TikTok CSV");
    assert_eq!(records.len(), 2);

    let r1 = &records[0];
    assert_eq!(r1.order_id, "5789123456789");
    assert_eq!(r1.payout_id.as_deref(), Some("TT_SETTLE_101"));
    assert_eq!(r1.gross_amount, Decimal::from(300000));
    assert_eq!(r1.seller_discount, Decimal::from(20000));
    assert_eq!(r1.commission_fee, Decimal::from(15000));
    assert_eq!(r1.payment_fee, Decimal::from(10500));
    assert_eq!(r1.affiliate_commission_fee, Decimal::from(15000));
    assert_eq!(r1.net_settlement, Decimal::from(239500));

    // DuckDB In-Memory Staging & Sanity verification
    let runner = DuckDbRunner::in_memory().expect("Failed to create in-memory DuckDB");
    runner.stage_settlement_records(&records).expect("Failed to stage records in DuckDB");

    let sanity = runner.verify_financial_integrity().expect("Failed to verify integrity");
    assert_eq!(sanity.total_rows, 2);
    assert_eq!(sanity.total_gross, Decimal::from(450000));
    assert_eq!(sanity.balanced_rows, 2);
    assert_eq!(sanity.discrepant_rows, 0);
}
