mod common;

use common::integration_helpers::{data_rows, run_app, sorted_data_rows};

fn csv(rows: &str) -> String {
    format!("type,client,tx,amount\n{rows}")
}

#[test]
fn withdrawal_with_insufficient_funds_leaves_output_unchanged() {
    let output = run_app(&csv("deposit,1,1,1.0\nwithdrawal,1,2,2.0\n"));

    assert_eq!(data_rows(&output), vec!["1,1,0,1,false".to_string()]);
}

#[test]
fn duplicate_transaction_id_handled_correctly() {
    let output = run_app(&csv("deposit,1,1,1.0\ndeposit,1,1,2.0\n"));

    assert_eq!(data_rows(&output), vec!["1,3,0,3,false".to_string()]);
}

#[test]
fn dispute_unknown_tx_ignored() {
    let output = run_app(&csv("deposit,1,1,1.0\ndispute,1,999,\n"));

    assert_eq!(data_rows(&output), vec!["1,1,0,1,false".to_string()]);
}

#[test]
fn resolve_unknown_tx_ignored() {
    let output = run_app(&csv("deposit,1,1,1.0\nresolve,1,999,\n"));

    assert_eq!(data_rows(&output), vec!["1,1,0,1,false".to_string()]);
}

#[test]
fn chargeback_unknown_tx_ignored() {
    let output = run_app(&csv("deposit,1,1,1.0\nchargeback,1,999,\n"));

    assert_eq!(data_rows(&output), vec!["1,1,0,1,false".to_string()]);
}

#[test]
fn cross_client_dispute_blocked() {
    let baseline = run_app(&csv("deposit,1,1,-214748.3648\ndeposit,2,2,0\n"));
    let after_cross_client_dispute =
        run_app(&csv("deposit,1,1,-214748.3648\ndeposit,2,2,0\ndispute,2,1,\n"));

    assert_eq!(
        sorted_data_rows(&after_cross_client_dispute),
        sorted_data_rows(&baseline)
    );
}

#[test]
fn cross_client_resolve_blocked() {
    let baseline = run_app(&csv(
        "deposit,1,1,-214748.3648\ndispute,1,1,\ndeposit,2,2,-214748.3648\n",
    ));
    let after_cross_client_resolve = run_app(&csv(
        "deposit,1,1,-214748.3648\ndispute,1,1,\ndeposit,2,2,-214748.3648\nresolve,2,1,\n",
    ));

    assert_eq!(
        sorted_data_rows(&after_cross_client_resolve),
        sorted_data_rows(&baseline)
    );
}

#[test]
fn cross_client_chargeback_blocked() {
    let baseline = run_app(&csv("deposit,1,1,-214748.3648\ndispute,1,1,\ndeposit,2,2,0\n"));
    let after_cross_client_chargeback =
        run_app(&csv("deposit,1,1,-214748.3648\ndispute,1,1,\ndeposit,2,2,0\nchargeback,2,1,\n"));

    assert_eq!(
        sorted_data_rows(&after_cross_client_chargeback),
        sorted_data_rows(&baseline)
    );
}
