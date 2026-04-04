use crate::domain::account::{Account, AccountSnapshot};
use crate::domain::client::ClientId;
use crate::domain::recorded_transaction::RecordedTransaction;
use crate::domain::transaction_id::TxId;
use crate::error::ledger_error::LedgerError;

pub trait Ledger {
    fn account(&self, client_id: ClientId) -> Option<&Account>;
    fn account_mut_or_create(&mut self, client_id: ClientId) -> &mut Account;
    fn recorded_tx(&self, tx_id: TxId) -> Option<&RecordedTransaction>;
    fn recorded_tx_mut(&mut self, tx_id: TxId) -> Option<&mut RecordedTransaction>;
    fn insert_recorded_tx(&mut self, tx: RecordedTransaction) -> Result<(), LedgerError>;
    fn snapshot_accounts(&self) -> Vec<AccountSnapshot>;
}
