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
fn duplicate_deposit_tx_id_does_not_mutate_balances() {
    let output = run_app(&csv("deposit,1,1,1.0\ndeposit,1,1,2.0\n"));

    assert_eq!(data_rows(&output), vec!["1,1,0,1,false".to_string()]);
}

#[test]
fn duplicate_withdrawal_tx_id_does_not_mutate_balances() {
    let output = run_app(&csv(
        "deposit,1,1,5.0\nwithdrawal,1,2,2.0\nwithdrawal,1,2,1.0\n",
    ));

    assert_eq!(data_rows(&output), vec!["1,3,0,3,false".to_string()]);
}

#[test]
fn negative_deposit_is_ignored() {
    let output = run_app(&csv("deposit,1,1,1.0\ndeposit,1,2,-1.0\n"));

    assert_eq!(data_rows(&output), vec!["1,1,0,1,false".to_string()]);
}

#[test]
fn negative_withdrawal_is_ignored() {
    let output = run_app(&csv("deposit,1,1,1.0\nwithdrawal,1,2,-0.5\n"));

    assert_eq!(data_rows(&output), vec!["1,1,0,1,false".to_string()]);
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
fn cross_client_dispute_can_mutate_wrong_account_in_current_implementation() {
    let output = run_app(&csv("deposit,1,1,5.0\ndeposit,2,2,10.0\ndispute,2,1,\n"));

    assert_eq!(
        sorted_data_rows(&output),
        vec![
            "1,5,0,5,false".to_string(),
            "2,5,5,10,false".to_string()
        ]
    );
}

#[test]
fn cross_client_resolve_can_mutate_wrong_account_in_current_implementation() {
    let output = run_app(&csv(
        "deposit,1,1,5.0\ndeposit,2,2,10.0\ndispute,1,1,\ndispute,2,2,\nresolve,2,1,\n",
    ));

    assert_eq!(
        sorted_data_rows(&output),
        vec![
            "1,0,5,5,false".to_string(),
            "2,5,5,10,false".to_string()
        ]
    );
}

#[test]
fn cross_client_chargeback_can_mutate_wrong_account_in_current_implementation() {
    let output = run_app(&csv(
        "deposit,1,1,5.0\ndeposit,2,2,10.0\ndispute,1,1,\ndispute,2,2,\nchargeback,2,1,\n",
    ));

    assert_eq!(
        sorted_data_rows(&output),
        vec![
            "1,0,5,5,false".to_string(),
            "2,0,5,5,true".to_string()
        ]
    );
}
