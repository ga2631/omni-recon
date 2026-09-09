use omni_engine::duckdb_runner::DuckDbRunner;
use omni_engine::parsers::shopee::ShopeeParser;
use rust_decimal::Decimal;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_user_attached_sample_file_parsing() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("mau_bang_ke_shopee.csv");

    // Exact content provided by the user in the prompt
    let csv_content = "\
Mã đơn hàng,Ngày hoàn thành,Trạng thái đơn hàng,Tổng tiền hàng,Phí vận chuyển người mua trả,Trợ giá phí vận chuyển của Shopee,Phí vận chuyển thực tế,Phí thanh toán,Phí cố định,Phí Dịch Vụ,Số tiền chuyển cho Người bán
260908NORMAL01,2026-09-05 14:00,Hoàn thành,200000,15000,0,15000,8000,8000,10000,174000
260908SHIPPING,2026-09-06 10:30,Hoàn thành,300000,20000,0,45000,12000,12000,15000,236000
260908RETURN03,2026-09-07 09:15,Trả hàng hoàn tiền,150000,25000,15000,40000,0,0,0,0
260908HIDDEN04,2026-09-08 16:45,Hoàn thành,100000,15000,0,15000,4000,4000,5000,70000
";

    {
        let mut file = File::create(&file_path).unwrap();
        file.write_all(csv_content.as_bytes()).unwrap();
    }

    let records = ShopeeParser::parse_statement(&file_path).expect("Failed to parse user sample CSV");
    assert_eq!(records.len(), 4, "Should parse all 4 rows from the sample file");

    // Row 1: Normal balanced order
    let r1 = &records[0];
    assert_eq!(r1.order_id, "260908NORMAL01");
    assert_eq!(r1.order_status, "Hoàn thành");
    assert_eq!(r1.gross_amount, Decimal::from(200000));
    assert_eq!(r1.buyer_shipping_fee, Decimal::from(15000));
    assert_eq!(r1.shipping_subsidy, Decimal::from(0));
    assert_eq!(r1.seller_shipping_fee, Decimal::from(15000));
    assert_eq!(r1.payment_fee, Decimal::from(8000));
    assert_eq!(r1.commission_fee, Decimal::from(8000));
    assert_eq!(r1.service_fee, Decimal::from(10000));
    assert_eq!(r1.net_settlement, Decimal::from(174000));
    assert!(r1.settled_at.is_some());
    // Verify raw attributes contain original headers and values
    assert_eq!(r1.raw_attributes["Mã đơn hàng"], "260908NORMAL01");
    assert_eq!(r1.raw_attributes["Tổng tiền hàng"], "200000");
    assert_eq!(r1.raw_attributes["Số tiền chuyển cho Người bán"], "174000");

    // Row 2: Shipping adjustment order
    let r2 = &records[1];
    assert_eq!(r2.order_id, "260908SHIPPING");
    assert_eq!(r2.gross_amount, Decimal::from(300000));
    assert_eq!(r2.buyer_shipping_fee, Decimal::from(20000));
    assert_eq!(r2.seller_shipping_fee, Decimal::from(45000));
    assert_eq!(r2.payment_fee, Decimal::from(12000));
    assert_eq!(r2.commission_fee, Decimal::from(12000));
    assert_eq!(r2.service_fee, Decimal::from(15000));
    assert_eq!(r2.net_settlement, Decimal::from(236000));

    // Row 3: Return & Refund order
    let r3 = &records[2];
    assert_eq!(r3.order_id, "260908RETURN03");
    assert_eq!(r3.order_status, "Trả hàng hoàn tiền");
    assert_eq!(r3.gross_amount, Decimal::from(150000));
    assert_eq!(r3.net_settlement, Decimal::from(0));

    // Row 4: Order with discrepancy (hidden deduction)
    let r4 = &records[3];
    assert_eq!(r4.order_id, "260908HIDDEN04");
    assert_eq!(r4.gross_amount, Decimal::from(100000));
    assert_eq!(r4.net_settlement, Decimal::from(70000));

    // DuckDB In-Memory Staging & Sanity verification
    let runner = DuckDbRunner::in_memory().expect("Failed to initialize DuckDB");
    runner.stage_settlement_records(&records).expect("Failed to stage settlement records");

    let sanity = runner.verify_financial_integrity().expect("Failed to verify sanity");
    assert_eq!(sanity.total_rows, 4);
    assert_eq!(sanity.total_gross, Decimal::from(750000)); // 200k + 300k + 150k + 100k
    assert_eq!(sanity.total_net, Decimal::from(480000));   // 174k + 236k + 0 + 70k
    assert_eq!(sanity.balanced_rows, 2, "Rows 1 and 2 should be balanced");
    assert_eq!(sanity.discrepant_rows, 2, "Row 3 (Return) and Row 4 (Discrepant) should be flagged");
}
