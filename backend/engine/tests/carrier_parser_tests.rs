use omni_engine::parsers::carrier::CarrierParser;
use rust_decimal::Decimal;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_carrier_csv_parsing() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("ghn_sample.csv");

    let csv_content = "\
Mã vận đơn,Mã đơn đối tác,Tiền thu hộ,Cước phí,Khối lượng tính cước,Trạng thái giao hàng,Ngày giao hàng
GHN992817263,240830SHOPEE88912,450.000,22.000,500g,Giao hàng thành công,31/08/2026 10:30
GHN88712300,240830SHOPEE99182,1.200.000,35.000,1200g,Giao hàng thành công,31/08/2026 11:15
";

    {
        let mut file = File::create(&file_path).unwrap();
        file.write_all(csv_content.as_bytes()).unwrap();
    }

    let records = CarrierParser::parse_carrier_statement(&file_path, "GHN")
        .expect("Failed to parse carrier CSV");

    assert_eq!(records.len(), 2);

    let r1 = &records[0];
    assert_eq!(r1.tracking_code, "GHN992817263");
    assert_eq!(r1.order_id, "240830SHOPEE88912");
    assert_eq!(r1.carrier_code, "GHN");
    assert_eq!(r1.cod_amount, Decimal::from(450000));
    assert_eq!(r1.shipping_fee, Decimal::from(22000));
    assert_eq!(r1.charged_weight_gram, 500);
    assert_eq!(r1.delivery_status, "Giao hàng thành công");
    assert!(r1.delivered_at.is_some());

    let r2 = &records[1];
    assert_eq!(r2.tracking_code, "GHN88712300");
    assert_eq!(r2.order_id, "240830SHOPEE99182");
    assert_eq!(r2.cod_amount, Decimal::from(1200000));
}
