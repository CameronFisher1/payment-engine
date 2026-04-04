mod common;

use common::assertions::assert_account;
use common::builders::{chargeback, deposit, dispute, resolve};
use payment_engine::domain::dispute::DisputeState;
use payment_engine::engine::payment_engine::PaymentEngine;
use payment_engine::error::transaction_error::TransactionError;
use payment_engine::ledger::in_memory::InMemoryLedger;
use payment_engine::traits::ledger::Ledger;

#[test]
fn invalid_operations_do_not_panic() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = engine.apply(dispute(1, 999), &mut ledger);
        let _ = engine.apply(resolve(1, 999), &mut ledger);
        let _ = engine.apply(chargeback(1, 999), &mut ledger);
    }));

    assert!(result.is_ok());
}

#[test]
fn invalid_operations_do_not_mutate_unrelated_accounts() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(deposit(2, 2, 20_000), &mut ledger).unwrap();

    let _ = engine.apply(resolve(1, 1), &mut ledger);
    let _ = engine.apply(chargeback(1, 1), &mut ledger);

    assert_account(&ledger, 2, 20_000, 0, false);
}

#[test]
fn dispute_resolve_chargeback_on_missing_tx_return_not_found() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();

    let dispute_result = engine.apply(dispute(1, 9), &mut ledger);
    let resolve_result = engine.apply(resolve(1, 9), &mut ledger);
    let chargeback_result = engine.apply(chargeback(1, 9), &mut ledger);

    assert!(matches!(
        dispute_result,
        Err(TransactionError::TransactionNotFound)
    ));
    assert!(matches!(
        resolve_result,
        Err(TransactionError::TransactionNotFound)
    ));
    assert!(matches!(
        chargeback_result,
        Err(TransactionError::TransactionNotFound)
    ));
}

#[test]
fn duplicate_dispute_returns_error() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(dispute(1, 1), &mut ledger).unwrap();

    let result = engine.apply(dispute(1, 1), &mut ledger);

    assert!(matches!(result, Err(TransactionError::DisputeError)));
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
fn chargeback_without_dispute_returns_error() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();

    let result = engine.apply(chargeback(1, 1), &mut ledger);

    assert!(matches!(result, Err(TransactionError::DisputeError)));
}

#[test]
fn repeated_resolve_and_chargeback_return_error() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(dispute(1, 1), &mut ledger).unwrap();
    engine.apply(resolve(1, 1), &mut ledger).unwrap();

    let resolve_again = engine.apply(resolve(1, 1), &mut ledger);
    let chargeback_after_resolve = engine.apply(chargeback(1, 1), &mut ledger);

    assert!(matches!(resolve_again, Err(TransactionError::DisputeError)));
    assert!(matches!(
        chargeback_after_resolve,
        Err(TransactionError::DisputeError)
    ));
}

#[test]
fn cross_client_dispute_can_succeed_with_sufficient_funds_in_current_implementation() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 5_000), &mut ledger).unwrap();
    engine.apply(deposit(2, 2, 20_000), &mut ledger).unwrap();

    let result = engine.apply(dispute(2, 1), &mut ledger);

    assert!(result.is_ok());
    assert_account(&ledger, 2, 15_000, 5_000, false);
    assert_eq!(ledger.recorded_tx(1).unwrap().dispute_state, DisputeState::Disputed);
}

#[test]
fn cross_client_resolve_can_mutate_wrong_account_in_current_implementation() {
    let mut engine = PaymentEngine::new();
    let mut ledger = InMemoryLedger::new();
    engine.apply(deposit(1, 1, 10_000), &mut ledger).unwrap();
    engine.apply(dispute(1, 1), &mut ledger).unwrap();

    let result = engine.apply(resolve(2, 1), &mut ledger);

    assert!(result.is_ok());
    assert_account(&ledger, 2, 10_000, -10_000, false);
}
