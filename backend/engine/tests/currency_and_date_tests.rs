use chrono::{Datelike, Timelike};
use omni_engine::parsers::{parse_currency_decimal, parse_flexible_datetime};
use rust_decimal::Decimal;

#[test]
fn test_parse_currency_decimal_standard_vietnamese() {
    assert_eq!(parse_currency_decimal("150000"), Decimal::from(150000));
    assert_eq!(parse_currency_decimal("150.000"), Decimal::from(150000));
    assert_eq!(parse_currency_decimal("1.500.000"), Decimal::from(1500000));
    assert_eq!(parse_currency_decimal("12.345.678"), Decimal::from(12345678));
}

#[test]
fn test_parse_currency_decimal_with_symbols_and_whitespace() {
    assert_eq!(parse_currency_decimal(" 250.000 đ "), Decimal::from(250000));
    assert_eq!(parse_currency_decimal("1.500.000 VND"), Decimal::from(1500000));
    assert_eq!(parse_currency_decimal("450.000 ₫"), Decimal::from(450000));
    assert_eq!(parse_currency_decimal(" 99.000vnd "), Decimal::from(99000));
}

#[test]
fn test_parse_currency_decimal_accounting_negative_brackets() {
    assert_eq!(parse_currency_decimal("(15.000)"), Decimal::from(-15000));
    assert_eq!(parse_currency_decimal("(250.000)"), Decimal::from(-250000));
    assert_eq!(parse_currency_decimal("(1.250.000 đ)"), Decimal::from(-1250000));
    assert_eq!(parse_currency_decimal("-15.000"), Decimal::from(-15000));
}

#[test]
fn test_parse_currency_decimal_comma_dot_variants() {
    // US Format: 1,250.50
    assert_eq!(
        parse_currency_decimal("1,250.50"),
        Decimal::new(125050, 2)
    );
    // EU Format: 1.250,50
    assert_eq!(
        parse_currency_decimal("1.250,50"),
        Decimal::new(125050, 2)
    );
}

#[test]
fn test_parse_currency_decimal_edge_cases() {
    assert_eq!(parse_currency_decimal(""), Decimal::ZERO);
    assert_eq!(parse_currency_decimal("   "), Decimal::ZERO);
    assert_eq!(parse_currency_decimal("-"), Decimal::ZERO);
    assert_eq!(parse_currency_decimal("N/A"), Decimal::ZERO);
    assert_eq!(parse_currency_decimal("null"), Decimal::ZERO);
    assert_eq!(parse_currency_decimal("0"), Decimal::ZERO);
    assert_eq!(parse_currency_decimal("0.00"), Decimal::ZERO);
}

#[test]
fn test_parse_flexible_datetime_formats() {
    // Format: YYYY-MM-DD HH:MM:SS
    let dt1 = parse_flexible_datetime("2026-09-01 14:30:00").unwrap();
    assert_eq!(dt1.year(), 2026);
    assert_eq!(dt1.month(), 9);
    assert_eq!(dt1.day(), 1);
    assert_eq!(dt1.hour(), 14);
    assert_eq!(dt1.minute(), 30);

    // Format: DD-MM-YYYY HH:MM
    let dt2 = parse_flexible_datetime("01-09-2026 14:30").unwrap();
    assert_eq!(dt2.year(), 2026);
    assert_eq!(dt2.month(), 9);
    assert_eq!(dt2.day(), 1);
    assert_eq!(dt2.hour(), 14);

    // Format: DD/MM/YYYY HH:MM:SS
    let dt3 = parse_flexible_datetime("15/08/2026 09:15:45").unwrap();
    assert_eq!(dt3.year(), 2026);
    assert_eq!(dt3.month(), 8);
    assert_eq!(dt3.day(), 15);
    assert_eq!(dt3.hour(), 9);
    assert_eq!(dt3.minute(), 15);

    // Format: ISO 8601 YYYY-MM-DDTHH:MM:SSZ
    let dt4 = parse_flexible_datetime("2026-09-01T14:30:00Z").unwrap();
    assert_eq!(dt4.year(), 2026);
    assert_eq!(dt4.month(), 9);
    assert_eq!(dt4.day(), 1);

    // Format: Date only YYYY-MM-DD
    let dt5 = parse_flexible_datetime("2026-09-01").unwrap();
    assert_eq!(dt5.year(), 2026);
    assert_eq!(dt5.month(), 9);
    assert_eq!(dt5.day(), 1);
    assert_eq!(dt5.hour(), 0);

    // Invalid string
    assert!(parse_flexible_datetime("").is_none());
    assert!(parse_flexible_datetime("invalid-date-string").is_none());
}
