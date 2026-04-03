use std::collections::HashMap;
use crate::domain::account::Account;
use crate::domain::client::ClientId;
use crate::domain::recorded_transaction::RecordedTransaction;
use crate::domain::transaction_id::TxId;

pub struct InMemoryLedger {
    accounts: HashMap<ClientId, Account>,
    transactions: HashMap<TxId, RecordedTransaction>,
}