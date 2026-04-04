mod common;

use common::integration_helpers::{OUTPUT_HEADER, csv_fields, data_rows, row_for_client, run_app};

fn csv(rows: &str) -> String {
    format!("type,client,tx,amount\n{rows}")
}

fn has_exactly_4_decimal_places(value: &str) -> bool {
    let unsigned = value.trim_start_matches('-').trim_start_matches('+');
    let mut parts = unsigned.split('.');
    let whole = parts.next().unwrap_or_default();
    let fractional = parts.next();

    whole.chars().all(|c| c.is_ascii_digit())
        && parts.next().is_none()
        && fractional.is_some_and(|frac| frac.len() == 4 && frac.chars().all(|c| c.is_ascii_digit()))
}

#[test]
fn output_contains_correct_header() {
    let output = run_app(&csv("deposit,1,1,1.0\n"));

    assert_eq!(output.lines().next().unwrap(), OUTPUT_HEADER);
}

#[test]
fn output_amounts_have_exactly_4_decimal_places() {
    let output = run_app(&csv("deposit,1,1,1.2345\ndeposit,1,2,0.0001\ndispute,1,2,\n"));
    let row = row_for_client(&output, 1);
    let fields = csv_fields(&row);

    assert!(has_exactly_4_decimal_places(fields[1]));
    assert!(has_exactly_4_decimal_places(fields[2]));
    assert!(has_exactly_4_decimal_places(fields[3]));
}

#[test]
fn output_row_values_are_exact() {
    let output = run_app(&csv("deposit,1,1,2.25\nwithdrawal,1,2,1.0\n"));

    assert_eq!(data_rows(&output), vec!["1,1.25,0,1.25,false".to_string()]);
}

#[test]
fn locked_column_is_correct() {
    let unlocked_output = run_app(&csv("deposit,1,1,1.0\n"));
    let locked_output = run_app(&csv("deposit,1,1,1.0\ndispute,1,1,\nchargeback,1,1,\n"));

    assert_eq!(csv_fields(&row_for_client(&unlocked_output, 1))[4], "false");
    assert_eq!(csv_fields(&row_for_client(&locked_output, 1))[4], "true");
}

#[test]
fn output_formatting_and_newline_structure_is_correct() {
    let output = run_app(&csv("deposit,1,1,1.0\n"));

    assert!(output.starts_with(&format!("{OUTPUT_HEADER}\n")));
    assert!(output.ends_with('\n'));
    assert_eq!(output.lines().count(), 2);
}
