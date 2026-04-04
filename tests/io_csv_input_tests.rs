mod common;

use common::builders::{csv_record, csv_source};
use payment_engine::domain::transaction::Transaction;
use payment_engine::error::app_error::AppError;
use payment_engine::traits::transaction_source::TransactionSource;

fn next_tx(csv: &str) -> Transaction {
    let mut source = csv_source(csv);
    source.next_transaction().unwrap().unwrap()
}

#[test]
fn parse_deposit_row() {
    let tx = next_tx("type,client,tx,amount\ndeposit,1,1,1.5\n");
    assert!(matches!(
        tx,
        Transaction::Deposit {
            client_id: 1,
            tx_id: 1,
            amount: 15_000
        }
    ));
}

#[test]
fn parse_withdrawal_row() {
    let tx = next_tx("type,client,tx,amount\nwithdrawal,1,2,2.5\n");
    assert!(matches!(
        tx,
        Transaction::Withdrawal {
            client_id: 1,
            tx_id: 2,
            amount: 25_000
        }
    ));
}

#[test]
fn parse_dispute_row() {
    let tx = next_tx("type,client,tx,amount\ndispute,1,3,\n");
    assert!(matches!(
        tx,
        Transaction::Dispute {
            client_id: 1,
            tx_id: 3
        }
    ));
}

#[test]
fn parse_resolve_row() {
    let tx = next_tx("type,client,tx,amount\nresolve,1,3,\n");
    assert!(matches!(
        tx,
        Transaction::Resolve {
            client_id: 1,
            tx_id: 3
        }
    ));
}

#[test]
fn parse_chargeback_row() {
    let tx = next_tx("type,client,tx,amount\nchargeback,1,3,\n");
    assert!(matches!(
        tx,
        Transaction::Chargeback {
            client_id: 1,
            tx_id: 3
        }
    ));
}

#[test]
fn parse_csv_with_header_row() {
    let mut source = csv_source("type,client,tx,amount\ndeposit,2,1,1.0\n");
    let first = source.next_transaction().unwrap();
    let second = source.next_transaction().unwrap();

    assert!(first.is_some());
    assert!(second.is_none());
}

#[test]
fn parse_csv_with_spaces_after_commas() {
    let tx = next_tx("type, client, tx, amount\ndeposit, 7, 9, 1.5\n");
    assert!(matches!(
        tx,
        Transaction::Deposit {
            client_id: 7,
            tx_id: 9,
            amount: 15_000
        }
    ));
}

#[test]
fn deposit_missing_amount_defaults_to_zero_in_current_implementation() {
    let tx = next_tx("type,client,tx,amount\ndeposit,1,1,\n");
    assert!(matches!(
        tx,
        Transaction::Deposit {
            client_id: 1,
            tx_id: 1,
            amount: 0
        }
    ));
}

#[test]
fn withdrawal_missing_amount_defaults_to_zero_in_current_implementation() {
    let tx = next_tx("type,client,tx,amount\nwithdrawal,1,2,\n");
    assert!(matches!(
        tx,
        Transaction::Withdrawal {
            client_id: 1,
            tx_id: 2,
            amount: 0
        }
    ));
}

#[test]
fn dispute_resolve_and_chargeback_allow_missing_amount() {
    let dispute_tx = next_tx("type,client,tx,amount\ndispute,1,3,\n");
    let resolve_tx = next_tx("type,client,tx,amount\nresolve,1,3,\n");
    let chargeback_tx = next_tx("type,client,tx,amount\nchargeback,1,3,\n");

    assert!(matches!(dispute_tx, Transaction::Dispute { .. }));
    assert!(matches!(resolve_tx, Transaction::Resolve { .. }));
    assert!(matches!(chargeback_tx, Transaction::Chargeback { .. }));
}

#[test]
fn invalid_transaction_type_rejected() {
    let mut source = csv_source("type,client,tx,amount\nrefund,1,1,1.0\n");
    let result = source.next_transaction();

    assert!(matches!(result, Err(AppError::Error)));
}

#[test]
fn malformed_amount_becomes_zero_in_current_implementation() {
    let tx = next_tx("type,client,tx,amount\ndeposit,1,1,not-a-number\n");
    assert!(matches!(
        tx,
        Transaction::Deposit {
            client_id: 1,
            tx_id: 1,
            amount: 0
        }
    ));
}

#[test]
fn mapping_from_raw_csv_record_to_domain_transaction_is_correct() {
    let tx = Transaction::try_from(csv_record("withdrawal", 8, 99, Some("12.3456"))).unwrap();

    assert!(matches!(
        tx,
        Transaction::Withdrawal {
            client_id: 8,
            tx_id: 99,
            amount: 123_456
        }
    ));
}
