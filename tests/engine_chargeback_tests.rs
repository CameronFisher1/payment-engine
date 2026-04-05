mod common;

use common::assertions::{assert_account, assert_tx_dispute_state};
use common::builders::{chargeback, deposit, dispute, resolve, withdrawal};
use payment_engine::domain::amount::Amount;
use payment_engine::domain::dispute::DisputeState;
use payment_engine::engine::payment_engine::PaymentEngine;
use payment_engine::error::transaction_error::TransactionError;
use payment_engine::ledger::in_memory::InMemoryLedger;
use payment_engine::traits::ledger::Ledger;

fn total_for(ledger: &InMemoryLedger, client_id: u16) -> Amount {
    let account = ledger.account(client_id).unwrap();
    account.available + account.held
}

#[test]
fn chargeback_disputed_tx_removes_held_funds() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(dispute(1, 1), &mut ledger).unwrap();

    engine.apply(chargeback(1, 1), &mut ledger).unwrap();

    assert_account(&ledger, 1, 0, 0, true);
}

#[test]
fn chargeback_reduces_total() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(dispute(1, 1), &mut ledger).unwrap();
    let before = total_for(&ledger, 1);

    engine.apply(chargeback(1, 1), &mut ledger).unwrap();
    let after = total_for(&ledger, 1);

    assert_eq!(before, 10_000);
    assert_eq!(after, 0);
}

#[test]
fn chargeback_locks_account() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(dispute(1, 1), &mut ledger).unwrap();

    engine.apply(chargeback(1, 1), &mut ledger).unwrap();

    assert!(ledger.account(1).unwrap().locked);
}

#[test]
fn chargeback_unknown_tx_returns_not_found() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();

    let result = engine.apply(chargeback(1, 999), &mut ledger);

    assert!(matches!(result, Err(TransactionError::TransactionNotFound)));
}

#[test]
fn chargeback_wrong_client_can_succeed_in_current_implementation() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(dispute(1, 1), &mut ledger).unwrap();

    let result = engine.apply(chargeback(2, 1), &mut ledger);

    assert!(result.is_ok());
    assert_account(&ledger, 2, 0, -10_000, true);
    assert_tx_dispute_state(&ledger, 1, DisputeState::ChargedBack);
}

#[test]
fn chargeback_without_dispute_returns_error() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();

    let result = engine.apply(chargeback(1, 1), &mut ledger);

    assert!(matches!(result, Err(TransactionError::DisputeError)));
}

#[test]
fn chargeback_after_resolve_returns_error() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(dispute(1, 1), &mut ledger).unwrap();
    engine.apply(resolve(1, 1), &mut ledger).unwrap();

    let result = engine.apply(chargeback(1, 1), &mut ledger);

    assert!(matches!(result, Err(TransactionError::DisputeError)));
}

#[test]
fn duplicate_chargeback_returns_error() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(dispute(1, 1), &mut ledger).unwrap();
    engine.apply(chargeback(1, 1), &mut ledger).unwrap();

    let result = engine.apply(chargeback(1, 1), &mut ledger);

    assert!(matches!(result, Err(TransactionError::DisputeError)));
}

#[test]
fn all_future_deposits_and_withdrawals_fail_after_chargeback_lock() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(dispute(1, 1), &mut ledger).unwrap();
    engine.apply(chargeback(1, 1), &mut ledger).unwrap();

    let deposit_result = engine.apply(deposit(1, 2, 2_000), &mut ledger);
    let withdrawal_result = engine.apply(withdrawal(1, 3, 1_000), &mut ledger);

    assert!(matches!(deposit_result, Err(TransactionError::AccountLocked)));
    assert!(matches!(
        withdrawal_result,
        Err(TransactionError::AccountLocked)
    ));
    assert_account(&ledger, 1, 0, 0, true);
}
