use crate::domain::dispute::DisputeState;
use crate::domain::recorded_transaction::RecordedTransaction;
use crate::domain::transaction::Transaction;
use crate::engine::helpers::{ensure_transaction_is_new, get_transaction_or_not_found};
use crate::error::transaction_error::TransactionError;
use crate::ledger::in_memory::InMemoryLedger;
use crate::traits::ledger::Ledger;

pub fn handle_deposit(
    tx: Transaction,
    ledger: &mut InMemoryLedger,
) -> Result<(), TransactionError> {
    // Check if transaction already exists
    ensure_transaction_is_new(tx.tx_id(), ledger)?;

    // Get amount and verify it's not negative
    let amount = tx.amount().ok_or(TransactionError::InvalidInput)?;
    if amount < 0 {
        return Err(TransactionError::InvalidInput);
    }

    // Get account
    let account = ledger.account_mut_or_create(tx.client_id());

    // Check if account is locked
    if account.locked {
        return Err(TransactionError::AccountLocked);
    }

    // Deposit money into account
    account.available = account
        .available
        .checked_add(amount)
        .ok_or(TransactionError::InvalidInput)?;

    // Add transaction to RecordTransaction map
    ledger
        .insert_recorded_tx(RecordedTransaction {
            tx_id: tx.tx_id(),
            client_id: tx.client_id(),
            amount,
            kind: tx,
            dispute_state: DisputeState::NotDisputed,
        })
        .map_err(|_| TransactionError::LedgerError)?;

    Ok(())
}

pub fn handle_withdraw(
    tx: Transaction,
    ledger: &mut InMemoryLedger,
) -> Result<(), TransactionError> {
    // Check if transaction already exists
    ensure_transaction_is_new(tx.tx_id(), ledger)?;

    // Get amount and verify it's not negative
    let amount = tx.amount().ok_or(TransactionError::InvalidInput)?;
    if amount < 0 {
        return Err(TransactionError::InvalidInput);
    }

    // Get account
    let account = ledger.account_mut_or_create(tx.client_id());

    // Check if account is locked
    if account.locked {
        return Err(TransactionError::AccountLocked);
    }

    // Check account balance
    if account.available < amount {
        return Err(TransactionError::InvalidFunds);
    }

    // Subtract amount from account
    account.available = account
        .available
        .checked_sub(amount)
        .ok_or(TransactionError::InvalidInput)?;

    // Add transaction to RecordTransaction map
    ledger
        .insert_recorded_tx(RecordedTransaction {
            tx_id: tx.tx_id(),
            client_id: tx.client_id(),
            amount,
            kind: tx,
            dispute_state: DisputeState::NotDisputed,
        })
        .map_err(|_| TransactionError::LedgerError)?;

    Ok(())
}

pub fn handle_dispute(
    tx: Transaction,
    ledger: &mut InMemoryLedger,
) -> Result<(), TransactionError> {
    // Read and validate referenced transaction without holding a mutable borrow.
    let disputed_amount = {
        let transaction = get_transaction_or_not_found(tx.tx_id(), ledger)?;
        if transaction.dispute_state != DisputeState::NotDisputed {
            return Err(TransactionError::DisputeError);
        }
        transaction.amount
    };

    // Get users account
    let account = ledger.account_mut_or_create(tx.client_id()); // account should already exist

    // Move funds from available to held.
    account.available = account
        .available
        .checked_sub(disputed_amount)
        .ok_or(TransactionError::InvalidInput)?;
    account.held = account
        .held
        .checked_add(disputed_amount)
        .ok_or(TransactionError::InvalidInput)?;

    // Mark transaction as disputed.
    let transaction = ledger
        .recorded_tx_mut(tx.tx_id())
        .ok_or(TransactionError::TransactionNotFound)?;
    transaction.dispute_state = DisputeState::Disputed;

    Ok(())
}

pub fn handle_resolve(
    tx: Transaction,
    ledger: &mut InMemoryLedger,
) -> Result<(), TransactionError> {
    // Read and validate referenced transaction without holding a mutable borrow.
    let disputed_amount = {
        let transaction = get_transaction_or_not_found(tx.tx_id(), ledger)?;
        if transaction.dispute_state != DisputeState::Disputed {
            return Err(TransactionError::DisputeError);
        }
        transaction.amount
    };

    // Get users account
    let account = ledger.account_mut_or_create(tx.client_id()); // account should already exist

    // Move funds from available to held.
    account.available = account
        .available
        .checked_add(disputed_amount)
        .ok_or(TransactionError::InvalidInput)?;
    account.held = account
        .held
        .checked_sub(disputed_amount)
        .ok_or(TransactionError::InvalidInput)?;

    // Mark transaction as not disputed
    let transaction = ledger
        .recorded_tx_mut(tx.tx_id())
        .ok_or(TransactionError::TransactionNotFound)?;
    transaction.dispute_state = DisputeState::NotDisputed;

    Ok(())
}

pub fn handle_chargeback(
    tx: Transaction,
    ledger: &mut InMemoryLedger,
) -> Result<(), TransactionError> {
    // Read and validate referenced transaction without holding a mutable borrow.
    let disputed_amount = {
        let transaction = get_transaction_or_not_found(tx.tx_id(), ledger)?;
        if transaction.dispute_state != DisputeState::Disputed {
            return Err(TransactionError::DisputeError);
        }
        transaction.amount
    };

    // Get users account
    let account = ledger.account_mut_or_create(tx.client_id()); // account should already exist

    // Remove funds from accounts held balance and lock account
    account.held = account
        .held
        .checked_sub(disputed_amount)
        .ok_or(TransactionError::InvalidInput)?;
    account.locked = true;

    // Mark transaction as not disputed
    let transaction = ledger
        .recorded_tx_mut(tx.tx_id())
        .ok_or(TransactionError::TransactionNotFound)?;
    transaction.dispute_state = DisputeState::ChargedBack;

    Ok(())
}
