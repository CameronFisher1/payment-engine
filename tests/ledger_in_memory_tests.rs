mod common;

use common::builders::{recorded_deposit, recorded_withdrawal};
use payment_engine::domain::dispute::DisputeState;
use payment_engine::error::ledger_error::LedgerError;
use payment_engine::ledger::in_memory::InMemoryLedger;
use payment_engine::traits::ledger::Ledger;

#[test]
fn create_or_retrieve_missing_account() {
    let mut ledger = InMemoryLedger::new();
    assert!(ledger.account(42).is_none());

    let account = ledger.account_mut_or_create(42);

    assert_eq!(account.client_id, 42);
    assert_eq!(account.available, 0);
    assert_eq!(account.held, 0);
    assert!(!account.locked);
}

#[test]
fn retrieve_existing_account() {
    let mut ledger = InMemoryLedger::new();
    ledger.account_mut_or_create(1).available = 10_000;

    let account = ledger.account(1).unwrap();
    assert_eq!(account.available, 10_000);
}

#[test]
fn account_mutations_persist() {
    let mut ledger = InMemoryLedger::new();
    {
        let account = ledger.account_mut_or_create(1);
        account.available = 20_000;
        account.held = 5_000;
        account.locked = true;
    }

    let account = ledger.account(1).unwrap();
    assert_eq!(account.available, 20_000);
    assert_eq!(account.held, 5_000);
    assert!(account.locked);
}

#[test]
fn insert_recorded_transaction() {
    let mut ledger = InMemoryLedger::new();

    let result = ledger.insert_recorded_tx(recorded_deposit(1, 7, 12_345, DisputeState::NotDisputed));

    assert!(result.is_ok());
    assert!(ledger.recorded_tx(7).is_some());
}

#[test]
fn duplicate_transaction_id_rejected() {
    let mut ledger = InMemoryLedger::new();
    ledger
        .insert_recorded_tx(recorded_deposit(1, 7, 12_345, DisputeState::NotDisputed))
        .unwrap();

    let result =
        ledger.insert_recorded_tx(recorded_withdrawal(1, 7, 1_000, DisputeState::NotDisputed));

    assert!(matches!(result, Err(LedgerError::DuplicateTransaction)));
}

#[test]
fn retrieve_existing_transaction() {
    let mut ledger = InMemoryLedger::new();
    ledger
        .insert_recorded_tx(recorded_deposit(1, 10, 5_000, DisputeState::NotDisputed))
        .unwrap();

    let tx = ledger.recorded_tx(10).unwrap();
    assert_eq!(tx.tx_id(), 10);
    assert_eq!(tx.client_id, 1);
    assert_eq!(tx.amount, 5_000);
}

#[test]
fn unknown_transaction_returns_none() {
    let ledger = InMemoryLedger::new();
    assert!(ledger.recorded_tx(999).is_none());
}

#[test]
fn snapshot_contains_all_accounts() {
    let mut ledger = InMemoryLedger::new();
    ledger.account_mut_or_create(1).available = 5_000;
    ledger.account_mut_or_create(2).available = 7_500;

    let snapshots = ledger.snapshot_accounts();

    assert_eq!(snapshots.len(), 2);
    assert!(snapshots.iter().any(|s| s.client_id == 1 && s.available == 5_000));
    assert!(snapshots.iter().any(|s| s.client_id == 2 && s.available == 7_500));
}
