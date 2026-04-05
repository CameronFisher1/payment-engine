use crate::domain::recorded_transaction::RecordedTransaction;
use crate::domain::transaction_id::TxId;
use crate::error::transaction_error::TransactionError;
use crate::ledger::in_memory::InMemoryLedger;
use crate::traits::ledger::Ledger;

pub fn check_transaction_exists(tx_id: TxId, ledger: &InMemoryLedger) -> Option<&RecordedTransaction> {
    ledger.recorded_tx(tx_id)
}

pub fn ensure_transaction_is_new(tx_id: TxId, ledger: &InMemoryLedger) -> Result<(), TransactionError> {
    if check_transaction_exists(tx_id, ledger).is_some() {
        return Err(TransactionError::DuplicateTransaction);
    }

    Ok(())
}

pub fn get_transaction_or_not_found(
    tx_id: TxId,
    ledger: &InMemoryLedger,
) -> Result<&RecordedTransaction, TransactionError> {
    check_transaction_exists(tx_id, ledger).ok_or(TransactionError::TransactionNotFound)
}
