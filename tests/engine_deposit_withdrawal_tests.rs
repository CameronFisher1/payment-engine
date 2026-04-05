mod common;

use common::assertions::{assert_account, assert_tx_dispute_state};
use common::builders::{chargeback, deposit, dispute, withdrawal};
use payment_engine::domain::dispute::DisputeState;
use payment_engine::engine::payment_engine::PaymentEngine;
use payment_engine::error::transaction_error::TransactionError;
use payment_engine::ledger::in_memory::InMemoryLedger;
use payment_engine::traits::ledger::Ledger;

#[test]
fn deposit_creates_account() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    assert!(ledger.account(1).is_none());

    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();

    assert!(ledger.account(1).is_some());
}

#[test]
fn deposit_updates_balances_correctly() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();

    engine.apply(deposit(1, 1, 12_500), &mut ledger).unwrap();

    assert_account(&ledger, 1, 12_500, 0, false);
}

#[test]
fn deposit_records_transaction() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();

    engine.apply(deposit(1, 10, 7_000), &mut ledger).unwrap();

    let recorded = ledger.recorded_tx(10).unwrap();
    assert_eq!(recorded.client_id, 1);
    assert_eq!(recorded.amount, 7_000);
    assert_tx_dispute_state(&ledger, 10, DisputeState::NotDisputed);
}

#[test]
fn multiple_deposits_accumulate() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();

    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(deposit(1, 2, 20_000), &mut ledger).unwrap();
    engine.apply(deposit(1, 3, 30_000), &mut ledger).unwrap();

    assert_account(&ledger, 1, 60_000, 0, false);
}

#[test]
fn multiple_clients_isolated() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();

    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(deposit(2, 2, 20_000), &mut ledger).unwrap();
    engine.apply(withdrawal(1, 3, 4_000), &mut ledger).unwrap();

    assert_account(&ledger, 1, 6_000, 0, false);
    assert_account(&ledger, 2, 20_000, 0, false);
}

#[test]
fn withdrawal_succeeds_with_sufficient_funds() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 15_000), &mut ledger).unwrap();

    let result = engine.apply(withdrawal(1, 2, 10_000), &mut ledger);

    assert!(result.is_ok());
}

#[test]
fn withdrawal_reduces_balances_correctly() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 15_000), &mut ledger).unwrap();

    engine.apply(withdrawal(1, 2, 10_000), &mut ledger).unwrap();

    assert_account(&ledger, 1, 5_000, 0, false);
}

#[test]
fn insufficient_withdrawal_returns_error_in_current_implementation() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 5_000), &mut ledger).unwrap();

    let result = engine.apply(withdrawal(1, 2, 9_000), &mut ledger);

    assert!(matches!(result, Err(TransactionError::InvalidFunds)));
}

#[test]
fn failed_withdrawal_leaves_state_unchanged() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 5_000), &mut ledger).unwrap();

    let _ = engine.apply(withdrawal(1, 2, 9_000), &mut ledger);

    assert_account(&ledger, 1, 5_000, 0, false);
    assert!(ledger.recorded_tx(2).is_none());
}

#[test]
fn duplicate_deposit_tx_id_returns_duplicate_transaction_error() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();

    let result = engine.apply(deposit(1, 1, 2_000), &mut ledger);

    assert!(matches!(
        result,
        Err(TransactionError::DuplicateTransaction)
    ));
}

#[test]
fn duplicate_deposit_tx_id_does_not_mutate_balance() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();

    let _ = engine.apply(deposit(1, 1, 2_000), &mut ledger);

    assert_account(&ledger, 1, 10_000, 0, false);
}

#[test]
fn duplicate_withdrawal_tx_id_returns_error_and_does_not_mutate_balance() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(withdrawal(1, 2, 4_000), &mut ledger).unwrap();

    let result = engine.apply(withdrawal(1, 2, 1_000), &mut ledger);

    assert!(matches!(
        result,
        Err(TransactionError::DuplicateTransaction)
    ));
    assert_account(&ledger, 1, 6_000, 0, false);
}

#[test]
fn locked_account_ignores_deposit_and_withdrawal() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(dispute(1, 1), &mut ledger).unwrap();
    engine.apply(chargeback(1, 1), &mut ledger).unwrap();

    let deposit_result = engine.apply(deposit(1, 2, 1_000), &mut ledger);
    let withdrawal_result = engine.apply(withdrawal(1, 3, 1_000), &mut ledger);

    assert!(matches!(deposit_result, Err(TransactionError::AccountLocked)));
    assert!(matches!(
        withdrawal_result,
        Err(TransactionError::AccountLocked)
    ));
    assert_account(&ledger, 1, 0, 0, true);
}

#[test]
fn negative_deposit_returns_error_and_does_not_mutate_balance() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();

    let result = engine.apply(deposit(1, 2, -1_000), &mut ledger);

    assert!(matches!(result, Err(TransactionError::InvalidInput)));
    assert_account(&ledger, 1, 10_000, 0, false);
    assert!(ledger.recorded_tx(2).is_none());
}

#[test]
fn negative_withdrawal_returns_error_and_does_not_mutate_balance() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();

    let result = engine.apply(withdrawal(1, 2, -1_000), &mut ledger);

    assert!(matches!(result, Err(TransactionError::InvalidInput)));
    assert_account(&ledger, 1, 10_000, 0, false);
    assert!(ledger.recorded_tx(2).is_none());
}
