use chrono::NaiveDate;
use omni_engine::duckdb_runner::DuckDbRunner;
use omni_engine::parsers::StandardSettlementRecord;
use rust_decimal::Decimal;
use serde_json::json;

fn create_sample_settlement_record(
    order_id: &str,
    gross: i64,
    seller_discount: i64,
    comm_fee: i64,
    service_fee: i64,
    payment_fee: i64,
    shipping_paid_by_seller: i64,
    net: i64,
) -> StandardSettlementRecord {
    StandardSettlementRecord {
        order_id: order_id.to_string(),
        platform: "SHOPEE".to_string(),
        payout_id: Some("PAY_001".to_string()),
        transaction_type: "ORDER_SETTLEMENT".to_string(),
        order_status: "COMPLETED".to_string(),
        buyer_username: Some("buyer_01".to_string()),
        tracking_number: Some("TRK001".to_string()),
        gross_amount: Decimal::from(gross),
        seller_discount: Decimal::from(seller_discount),
        platform_voucher: Decimal::ZERO,
        buyer_shipping_fee: Decimal::ZERO,
        seller_shipping_fee: Decimal::from(shipping_paid_by_seller),
        shipping_subsidy: Decimal::ZERO,
        commission_fee: Decimal::from(comm_fee),
        service_fee: Decimal::from(service_fee),
        payment_fee: Decimal::from(payment_fee),
        affiliate_commission_fee: Decimal::ZERO,
        other_fees: Decimal::ZERO,
        net_settlement: Decimal::from(net),
        ordered_at: NaiveDate::from_ymd_opt(2026, 9, 1).and_then(|d| d.and_hms_opt(10, 0, 0)),
        delivered_at: None,
        settled_at: NaiveDate::from_ymd_opt(2026, 9, 2).and_then(|d| d.and_hms_opt(15, 30, 0)),
        raw_attributes: json!({ "shop": "shopee_store" }),
        raw_fee_breakdown: json!({ "comm": comm_fee }),
    }
}

#[test]
fn test_duckdb_staging_and_financial_integrity_balanced() {
    let runner = DuckDbRunner::in_memory().expect("Failed to create in-memory DuckDB");

    // Balanced: Gross (300k) - Discount (20k) - Comm (12k) - Service (18k) - Payment (13.5k) - Ship (0) = 236.5k
    let r1 = create_sample_settlement_record("ORD_BALANCED_01", 300000, 20000, 12000, 18000, 13500, 0, 236500);
    // Balanced: Gross (100k) - Discount (0) - Comm (4k) - Service (6k) - Payment (4.5k) - Ship (0) = 85.5k
    let r2 = create_sample_settlement_record("ORD_BALANCED_02", 100000, 0, 4000, 6000, 4500, 0, 85500);

    let records = vec![r1, r2];

    runner.stage_settlement_records(&records).expect("Failed to stage records in DuckDB");

    let sanity = runner.verify_financial_integrity().expect("Failed to run sanity check");
    assert_eq!(sanity.total_rows, 2);
    assert_eq!(sanity.total_gross, Decimal::from(400000));
    assert_eq!(sanity.total_net, Decimal::from(322000));
    assert_eq!(sanity.balanced_rows, 2);
    assert_eq!(sanity.discrepant_rows, 0);
}

#[test]
fn test_duckdb_staging_and_financial_integrity_with_discrepancy() {
    let runner = DuckDbRunner::in_memory().expect("Failed to create in-memory DuckDB");

    // Balanced row: 200k - 10k comm = 190k
    let r1 = create_sample_settlement_record("ORD_OK_01", 200000, 0, 10000, 0, 0, 0, 190000);
    
    // Discrepant row: Gross 500k, Comm 25k -> Expected Net is 475k, but actual Net is 400k (75k discrepancy!)
    let r2 = create_sample_settlement_record("ORD_DISCREPANT_02", 500000, 0, 25000, 0, 0, 0, 400000);

    let records = vec![r1, r2];

    runner.stage_settlement_records(&records).expect("Failed to stage records in DuckDB");

    let sanity = runner.verify_financial_integrity().expect("Failed to run sanity check");
    assert_eq!(sanity.total_rows, 2);
    assert_eq!(sanity.total_gross, Decimal::from(700000));
    assert_eq!(sanity.balanced_rows, 1);
    assert_eq!(sanity.discrepant_rows, 1);
}
