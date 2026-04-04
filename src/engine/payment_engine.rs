use crate::domain::transaction::Transaction;
use crate::engine::transaction_handlers::{
    handle_chargeback, handle_deposit, handle_dispute, handle_resolve, handle_withdraw,
};
use crate::error::transaction_error::TransactionError;
use crate::ledger::in_memory::InMemoryLedger;

pub struct PaymentEngine;

impl PaymentEngine {
    pub fn new() -> PaymentEngine {
        PaymentEngine
    }

    pub fn apply(
        &mut self,
        tx: Transaction,
        ledger: &mut InMemoryLedger,
    ) -> Result<(), TransactionError> {
        match tx {
            Transaction::Deposit { .. } => handle_deposit(tx, ledger),
            Transaction::Withdrawal { .. } => handle_withdraw(tx, ledger),
            Transaction::Dispute { .. } => handle_dispute(tx, ledger),
            Transaction::Resolve { .. } => handle_resolve(tx, ledger),
            Transaction::Chargeback { .. } => handle_chargeback(tx, ledger),
        }
    }
}
