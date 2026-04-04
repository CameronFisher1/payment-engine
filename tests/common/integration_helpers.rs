#![allow(dead_code)]

use payment_engine::app::runner;
use std::io::Cursor;

pub const OUTPUT_HEADER: &str = "client,available,held,total,locked";

pub fn run_app(input: &str) -> String {
    let reader = Cursor::new(input.as_bytes());
    let mut output = Vec::<u8>::new();

    runner::run(reader, &mut output).expect("runner should complete for integration test input");

    String::from_utf8(output).expect("runner output should be valid UTF-8")
}

pub fn data_rows(output: &str) -> Vec<String> {
    output
        .lines()
        .skip(1)
        .filter(|line| !line.trim().is_empty())
        .map(str::to_string)
        .collect()
}

pub fn sorted_data_rows(output: &str) -> Vec<String> {
    let mut rows = data_rows(output);
    rows.sort();
    rows
}

pub fn row_for_client(output: &str, client_id: u16) -> String {
    let client_text = client_id.to_string();

    data_rows(output)
        .into_iter()
        .find(|row| row.split(',').next() == Some(client_text.as_str()))
        .unwrap_or_else(|| panic!("expected row for client {}", client_id))
}

pub fn csv_fields(row: &str) -> Vec<&str> {
    row.split(',').collect()
}

pub fn parse_output_amount(raw: &str) -> i32 {
    let raw = raw.trim();
    let (sign, body) = if let Some(rest) = raw.strip_prefix('-') {
        (-1_i64, rest)
    } else if let Some(rest) = raw.strip_prefix('+') {
        (1_i64, rest)
    } else {
        (1_i64, raw)
    };

    let (whole_text, frac_text) = body.split_once('.').unwrap_or((body, ""));
    let whole = whole_text
        .parse::<i64>()
        .unwrap_or_else(|_| panic!("invalid amount whole part: {}", raw));

    let mut frac = frac_text.to_string();
    assert!(
        frac.len() <= 4,
        "output amount has more than 4 decimal places: {}",
        raw
    );
    while frac.len() < 4 {
        frac.push('0');
    }

    let fractional = frac
        .parse::<i64>()
        .unwrap_or_else(|_| panic!("invalid amount fractional part: {}", raw));

    i32::try_from(sign * (whole * 10_000 + fractional))
        .unwrap_or_else(|_| panic!("amount out of i32 range: {}", raw))
}
