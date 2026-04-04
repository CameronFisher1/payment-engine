mod common;

use common::assertions::assert_account;
use common::builders::{chargeback, deposit, dispute, resolve, withdrawal};
use payment_engine::domain::account::AccountSnapshot;
use payment_engine::engine::payment_engine::PaymentEngine;
use payment_engine::error::transaction_error::TransactionError;
use payment_engine::ledger::in_memory::InMemoryLedger;
use payment_engine::traits::ledger::Ledger;

fn snapshot_for(ledger: &InMemoryLedger, client_id: u16) -> AccountSnapshot {
    ledger
        .snapshot_accounts()
        .into_iter()
        .find(|snapshot| snapshot.client_id == client_id)
        .unwrap_or_else(|| panic!("missing snapshot for client {}", client_id))
}

#[test]
fn new_account_starts_zeroed_and_unlocked() {
    let mut ledger = InMemoryLedger::new();

    let account = ledger.account_mut_or_create(7);

    assert_eq!(account.client_id, 7);
    assert_eq!(account.available, 0);
    assert_eq!(account.held, 0);
    assert!(!account.locked);
}

#[test]
fn total_equals_available_plus_held() {
    let mut ledger = InMemoryLedger::new();
    let account = ledger.account_mut_or_create(1);
    account.available = 25_000;
    account.held = 5_000;

    let snapshot = snapshot_for(&ledger, 1);
    assert_eq!(snapshot.total, snapshot.available + snapshot.held);
    assert_eq!(snapshot.total, 30_000);
}

#[test]
fn deposit_increases_available_and_total() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();

    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();

    let snapshot = snapshot_for(&ledger, 1);
    assert_eq!(snapshot.available, 10_000);
    assert_eq!(snapshot.total, 10_000);
}

#[test]
fn multiple_deposits_accumulate() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();

    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(deposit(1, 2, 5_000), &mut ledger).unwrap();

    assert_account(&ledger, 1, 15_000, 0, false);
}

#[test]
fn withdrawal_succeeds_with_sufficient_funds() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();

    let result = engine.apply(withdrawal(1, 2, 6_000), &mut ledger);

    assert!(result.is_ok());
    assert_account(&ledger, 1, 4_000, 0, false);
}

#[test]
fn withdrawal_fails_with_insufficient_funds() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 5_000), &mut ledger).unwrap();

    let result = engine.apply(withdrawal(1, 2, 7_000), &mut ledger);

    assert!(matches!(result, Err(TransactionError::InvalidFunds)));
}

#[test]
fn failed_withdrawal_does_not_mutate_balances() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 5_000), &mut ledger).unwrap();

    let _ = engine.apply(withdrawal(1, 2, 7_000), &mut ledger);

    assert_account(&ledger, 1, 5_000, 0, false);
}

#[test]
fn hold_moves_available_to_held() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();

    engine.apply(dispute(1, 1), &mut ledger).unwrap();

    assert_account(&ledger, 1, 0, 10_000, false);
}

#[test]
fn release_hold_moves_held_to_available() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(dispute(1, 1), &mut ledger).unwrap();

    engine.apply(resolve(1, 1), &mut ledger).unwrap();

    assert_account(&ledger, 1, 10_000, 0, false);
}

#[test]
fn chargeback_removes_held_funds_and_locks_account() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(dispute(1, 1), &mut ledger).unwrap();

    engine.apply(chargeback(1, 1), &mut ledger).unwrap();

    assert_account(&ledger, 1, 0, 0, true);
}

#[test]
fn locked_account_behavior_remains_correct() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(dispute(1, 1), &mut ledger).unwrap();
    engine.apply(chargeback(1, 1), &mut ledger).unwrap();

    let deposit_result = engine.apply(deposit(1, 2, 3_000), &mut ledger);
    let withdrawal_result = engine.apply(withdrawal(1, 3, 1_000), &mut ledger);

    assert!(matches!(deposit_result, Err(TransactionError::AccountLocked)));
    assert!(matches!(
        withdrawal_result,
        Err(TransactionError::AccountLocked)
    ));
    assert_account(&ledger, 1, 0, 0, true);
}
