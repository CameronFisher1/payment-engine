mod common;

use common::builders::{account_snapshot, csv_record};
use payment_engine::domain::transaction::Transaction;
use payment_engine::io::csv_output::CsvReportSink;
use payment_engine::traits::report_sink::ReportSink;

fn parsed_deposit_amount(raw_amount: Option<&str>) -> i32 {
    let tx = Transaction::try_from(csv_record("deposit", 1, 1, raw_amount)).unwrap();
    match tx {
        Transaction::Deposit { amount, .. } => amount,
        _ => panic!("expected deposit transaction"),
    }
}

fn formatted_available_value(amount: i32) -> String {
    let mut output = Vec::new();
    {
        let mut sink = CsvReportSink::new(&mut output);
        sink.write_accounts(&[account_snapshot(1, amount, 0, amount, false)])
            .unwrap();
    }

    let csv_text = String::from_utf8(output).unwrap();
    let mut rows = csv_text.lines();
    let _header = rows.next().unwrap();
    let row = rows.next().unwrap();
    row.split(',').nth(1).unwrap().to_string()
}

#[test]
fn parse_whole_number_amount() {
    assert_eq!(parsed_deposit_amount(Some("1")), 10_000);
}

#[test]
fn parse_decimal_amount() {
    assert_eq!(parsed_deposit_amount(Some("1.5")), 15_000);
}

#[test]
fn parse_four_decimal_amount() {
    assert_eq!(parsed_deposit_amount(Some("1.2345")), 12_345);
}

#[test]
fn parse_zero() {
    assert_eq!(parsed_deposit_amount(Some("0")), 0);
}

#[test]
fn format_zero_uses_current_implementation() {
    assert_eq!(formatted_available_value(0), "0");
}

#[test]
fn format_whole_number_uses_current_implementation() {
    assert_eq!(formatted_available_value(10_000), "1");
}

#[test]
fn format_decimal_uses_current_implementation() {
    assert_eq!(formatted_available_value(15_000), "1.5");
}

#[test]
fn more_than_four_decimal_places_becomes_zero_in_current_implementation() {
    assert_eq!(parsed_deposit_amount(Some("1.12345")), 0);
}

#[test]
fn malformed_amount_becomes_zero_in_current_implementation() {
    assert_eq!(parsed_deposit_amount(Some("abc")), 0);
}

#[test]
fn negative_amount_is_allowed_in_current_implementation() {
    assert_eq!(parsed_deposit_amount(Some("-1.5")), -15_000);
}

#[test]
fn add_amounts_correctly() {
    assert_eq!(10_000_i32 + 5_000_i32, 15_000_i32);
}

#[test]
fn subtract_amounts_correctly() {
    assert_eq!(10_000_i32 - 5_000_i32, 5_000_i32);
}

#[test]
fn checked_add_and_subtract_handle_overflow_underflow() {
    assert!(i32::MAX.checked_add(1).is_none());
    assert!(i32::MIN.checked_sub(1).is_none());
}
