mod common;

use common::integration_helpers::{
    OUTPUT_HEADER, row_for_client, run_app, sorted_data_rows,
};

fn csv(rows: &str) -> String {
    format!("type,client,tx,amount\n{rows}")
}

#[test]
fn single_deposit_produces_correct_output() {
    let output = run_app(&csv("deposit,1,1,1.0\n"));

    assert_eq!(output, format!("{OUTPUT_HEADER}\n1,1,0,1,false\n"));
}

#[test]
fn multiple_deposits_for_one_client_accumulate_correctly() {
    let output = run_app(&csv("deposit,1,1,1.5\ndeposit,1,2,2.25\n"));
    let row = row_for_client(&output, 1);

    assert_eq!(row, "1,3.75,0,3.75,false");
}

#[test]
fn deposits_for_multiple_clients_produce_multiple_rows() {
    let output = run_app(&csv("deposit,1,1,1.0\ndeposit,2,2,2.0\n"));

    assert_eq!(
        sorted_data_rows(&output),
        vec!["1,1,0,1,false".to_string(), "2,2,0,2,false".to_string()]
    );
}

#[test]
fn deposit_and_withdrawal_produce_correct_final_balances() {
    let output = run_app(&csv("deposit,1,1,2.0\nwithdrawal,1,2,0.75\n"));
    let row = row_for_client(&output, 1);

    assert_eq!(row, "1,1.25,0,1.25,false");
}

#[test]
fn output_ordering_is_deterministic_if_expected() {
    let input = csv("deposit,1,1,1.0\ndeposit,1,2,0.5\nwithdrawal,1,3,0.25\n");
    let first = run_app(&input);
    let second = run_app(&input);

    assert_eq!(first, second);
}
