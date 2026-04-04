mod common;

use common::builders::account_snapshot;
use payment_engine::io::csv_output::CsvReportSink;
use payment_engine::traits::report_sink::ReportSink;

fn write_csv(accounts: &[payment_engine::domain::account::AccountSnapshot]) -> String {
    let mut output = Vec::new();
    {
        let mut sink = CsvReportSink::new(&mut output);
        sink.write_accounts(accounts).unwrap();
    }
    String::from_utf8(output).unwrap()
}

#[test]
fn writes_expected_header_row() {
    let csv_text = write_csv(&[account_snapshot(1, 0, 0, 0, false)]);
    let header = csv_text.lines().next().unwrap();

    assert_eq!(header, "client,available,held,total,locked");
}

#[test]
fn writes_one_row_per_account() {
    let csv_text = write_csv(&[
        account_snapshot(1, 10_000, 0, 10_000, false),
        account_snapshot(2, 20_000, 0, 20_000, false),
    ]);

    let row_count = csv_text.lines().skip(1).count();
    assert_eq!(row_count, 2);
}

#[test]
fn columns_are_in_correct_order() {
    let csv_text = write_csv(&[account_snapshot(1, 10_000, 500, 10_500, true)]);
    let row = csv_text.lines().nth(1).unwrap();
    let fields: Vec<&str> = row.split(',').collect();

    assert_eq!(fields[0], "1");
    assert_eq!(fields[1], "1");
    assert_eq!(fields[2], "0.05");
    assert_eq!(fields[3], "1.05");
    assert_eq!(fields[4], "true");
}

#[test]
fn amounts_formatted_with_current_implementation() {
    let csv_text = write_csv(&[account_snapshot(3, 15_000, 5_000, 20_000, false)]);
    let row = csv_text.lines().nth(1).unwrap();
    let fields: Vec<&str> = row.split(',').collect();

    assert_eq!(fields[1], "1.5");
    assert_eq!(fields[2], "0.5");
    assert_eq!(fields[3], "2");
}

#[test]
fn locked_written_correctly() {
    let unlocked = write_csv(&[account_snapshot(1, 0, 0, 0, false)]);
    let locked = write_csv(&[account_snapshot(1, 0, 0, 0, true)]);

    assert_eq!(unlocked.lines().nth(1).unwrap().split(',').nth(4).unwrap(), "false");
    assert_eq!(locked.lines().nth(1).unwrap().split(',').nth(4).unwrap(), "true");
}

#[test]
fn multiple_rows_serialize_correctly() {
    let csv_text = write_csv(&[
        account_snapshot(1, 10_000, 0, 10_000, false),
        account_snapshot(2, -20_000, 0, -20_000, true),
        account_snapshot(3, 12_345, 55, 12_400, false),
    ]);
    let rows: Vec<&str> = csv_text.lines().collect();

    assert_eq!(rows[0], "client,available,held,total,locked");
    assert_eq!(rows[1], "1,1,0,1,false");
    assert_eq!(rows[2], "2,-2,0,-2,true");
    assert_eq!(rows[3], "3,1.2345,0.0055,1.24,false");
}
