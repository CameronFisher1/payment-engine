mod common;

use common::integration_helpers::{
    csv_fields, data_rows, parse_output_amount, row_for_client, run_app,
};

fn csv(rows: &str) -> String {
    format!("type,client,tx,amount\n{rows}")
}

#[test]
fn deposit_then_dispute_moves_funds_from_available_to_held() {
    let output = run_app(&csv("deposit,1,1,1.5\ndispute,1,1,\n"));
    let row = row_for_client(&output, 1);

    assert_eq!(row, "1,0,1.5,1.5,false");
}

#[test]
fn deposit_then_dispute_then_resolve_returns_funds_to_available() {
    let output = run_app(&csv("deposit,1,1,1.5\ndispute,1,1,\nresolve,1,1,\n"));
    let row = row_for_client(&output, 1);

    assert_eq!(row, "1,1.5,0,1.5,false");
}

#[test]
fn total_remains_constant_during_dispute_and_resolve() {
    let deposited = run_app(&csv("deposit,1,1,1.5\n"));
    let disputed = run_app(&csv("deposit,1,1,1.5\ndispute,1,1,\n"));
    let resolved = run_app(&csv("deposit,1,1,1.5\ndispute,1,1,\nresolve,1,1,\n"));

    let deposited_total = parse_output_amount(csv_fields(&row_for_client(&deposited, 1))[3]);
    let disputed_total = parse_output_amount(csv_fields(&row_for_client(&disputed, 1))[3]);
    let resolved_total = parse_output_amount(csv_fields(&row_for_client(&resolved, 1))[3]);

    assert_eq!(deposited_total, disputed_total);
    assert_eq!(deposited_total, resolved_total);
}

#[test]
fn dispute_unknown_tx_leaves_output_unchanged() {
    let output = run_app(&csv("dispute,1,999,\n"));

    assert_eq!(output, "");
}

#[test]
fn dispute_wrong_client_leaves_output_unchanged() {
    let output = run_app(&csv("deposit,1,1,1.0\ndispute,1,1,\ndispute,2,1,\n"));

    assert_eq!(data_rows(&output), vec!["1,0,1,1,false".to_string()]);
}

#[test]
fn resolve_without_dispute_leaves_output_unchanged() {
    let output = run_app(&csv("deposit,1,1,1.5\nresolve,1,1,\n"));

    assert_eq!(data_rows(&output), vec!["1,1.5,0,1.5,false".to_string()]);
}
