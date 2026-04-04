mod common;

use common::integration_helpers::{csv_fields, data_rows, row_for_client, run_app};

fn csv(rows: &str) -> String {
    format!("type,client,tx,amount\n{rows}")
}

#[test]
fn deposit_then_dispute_then_chargeback_removes_funds_and_locks_account() {
    let output = run_app(&csv("deposit,1,1,2.0\ndispute,1,1,\nchargeback,1,1,\n"));
    let row = row_for_client(&output, 1);

    assert_eq!(row, "1,0,0,0,true");
}

#[test]
fn output_after_chargeback_shows_locked_true() {
    let output = run_app(&csv("deposit,1,1,2.0\ndispute,1,1,\nchargeback,1,1,\n"));
    let row = row_for_client(&output, 1);

    assert_eq!(csv_fields(&row)[4], "true");
}

#[test]
fn future_deposit_ignored_after_chargeback() {
    let output = run_app(&csv(
        "deposit,1,1,2.0\ndispute,1,1,\nchargeback,1,1,\ndeposit,1,2,5.0\n",
    ));

    assert_eq!(data_rows(&output), vec!["1,0,0,0,true".to_string()]);
}

#[test]
fn future_withdrawal_ignored_after_chargeback() {
    let output = run_app(&csv(
        "deposit,1,1,2.0\ndispute,1,1,\nchargeback,1,1,\nwithdrawal,1,2,1.0\n",
    ));

    assert_eq!(data_rows(&output), vec!["1,0,0,0,true".to_string()]);
}

#[test]
fn future_dispute_resolve_and_chargeback_ignored_after_chargeback() {
    let output = run_app(&csv(
        "deposit,1,1,2.0\ndispute,1,1,\nchargeback,1,1,\ndispute,1,1,\nresolve,1,1,\nchargeback,1,1,\n",
    ));

    assert_eq!(data_rows(&output), vec!["1,0,0,0,true".to_string()]);
}
