mod common;

use common::assertions::{assert_account, assert_account_absent, assert_tx_dispute_state};
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
fn dispute_existing_deposit_moves_available_to_held() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();

    engine.apply(dispute(1, 1), &mut ledger).unwrap();

    assert_account(&ledger, 1, 0, 10_000, false);
}

#[test]
fn dispute_preserves_total() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 12_000), &mut ledger).unwrap();
    let before = total_for(&ledger, 1);

    engine.apply(dispute(1, 1), &mut ledger).unwrap();
    let after = total_for(&ledger, 1);

    assert_eq!(before, after);
}

#[test]
fn dispute_marks_transaction_disputed() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();

    engine.apply(dispute(1, 1), &mut ledger).unwrap();

    assert_tx_dispute_state(&ledger, 1, DisputeState::Disputed);
}

#[test]
fn dispute_unknown_tx_returns_not_found_and_does_not_create_account() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();

    let result = engine.apply(dispute(1, 999), &mut ledger);

    assert!(matches!(result, Err(TransactionError::TransactionNotFound)));
    assert_account_absent(&ledger, 1);
}

#[test]
fn dispute_wrong_client_can_succeed_in_current_implementation() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();

    let result = engine.apply(dispute(2, 1), &mut ledger);

    assert!(result.is_ok());
    assert_account(&ledger, 2, -10_000, 10_000, false);
    assert_tx_dispute_state(&ledger, 1, DisputeState::Disputed);
}

#[test]
fn duplicate_dispute_returns_error() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(dispute(1, 1), &mut ledger).unwrap();

    let result = engine.apply(dispute(1, 1), &mut ledger);

    assert!(matches!(result, Err(TransactionError::DisputeError)));
    assert_tx_dispute_state(&ledger, 1, DisputeState::Disputed);
}

#[test]
fn dispute_withdrawal_is_allowed_in_current_implementation() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 20_000), &mut ledger).unwrap();
    engine.apply(withdrawal(1, 2, 5_000), &mut ledger).unwrap();

    let result = engine.apply(dispute(1, 2), &mut ledger);

    assert!(result.is_ok());
    assert_account(&ledger, 1, 10_000, 5_000, false);
    assert_tx_dispute_state(&ledger, 2, DisputeState::Disputed);
}

#[test]
fn resolve_disputed_tx_moves_held_to_available() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(dispute(1, 1), &mut ledger).unwrap();

    engine.apply(resolve(1, 1), &mut ledger).unwrap();

    assert_account(&ledger, 1, 10_000, 0, false);
}

#[test]
fn resolve_preserves_total() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(dispute(1, 1), &mut ledger).unwrap();
    let before = total_for(&ledger, 1);

    engine.apply(resolve(1, 1), &mut ledger).unwrap();
    let after = total_for(&ledger, 1);

    assert_eq!(before, after);
}

#[test]
fn resolve_unknown_tx_returns_not_found() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();

    let result = engine.apply(resolve(1, 999), &mut ledger);

    assert!(matches!(result, Err(TransactionError::TransactionNotFound)));
}

#[test]
fn resolve_wrong_client_can_succeed_on_wrong_account_in_current_implementation() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(dispute(1, 1), &mut ledger).unwrap();

    let result = engine.apply(resolve(2, 1), &mut ledger);

    assert!(result.is_ok());
    assert_account(&ledger, 2, 10_000, -10_000, false);
    assert_tx_dispute_state(&ledger, 1, DisputeState::NotDisputed);
}

#[test]
fn resolve_without_dispute_returns_error() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();

    let result = engine.apply(resolve(1, 1), &mut ledger);

    assert!(matches!(result, Err(TransactionError::DisputeError)));
}

#[test]
fn resolve_after_chargeback_returns_error() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(dispute(1, 1), &mut ledger).unwrap();
    engine.apply(chargeback(1, 1), &mut ledger).unwrap();

    let result = engine.apply(resolve(1, 1), &mut ledger);

    assert!(matches!(result, Err(TransactionError::DisputeError)));
}
