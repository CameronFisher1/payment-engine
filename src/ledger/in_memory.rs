use crate::domain::account::{Account, AccountSnapshot};
use crate::domain::client::ClientId;
use crate::domain::recorded_transaction::RecordedTransaction;
use crate::domain::transaction_id::TxId;
use crate::error::ledger_error::LedgerError;
use crate::traits::ledger::Ledger;
use std::collections::HashMap;

pub struct InMemoryLedger {
    accounts: HashMap<ClientId, Account>,
    transactions: HashMap<TxId, RecordedTransaction>,
}

impl InMemoryLedger {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
        }
    }
}

impl Ledger for InMemoryLedger {
    fn account(&self, client_id: ClientId) -> Option<&Account> {
        self.accounts.get(&client_id)
    }

    fn account_mut_or_create(&mut self, client_id: ClientId) -> &mut Account {
        self.accounts.entry(client_id).or_insert(Account {
            client_id,
            available: 0,
            held: 0,
            locked: false,
        })
    }

    fn recorded_tx(&self, tx_id: TxId) -> Option<&RecordedTransaction> {
        self.transactions.get(&tx_id)
    }

    fn recorded_tx_mut(&mut self, tx_id: TxId) -> Option<&mut RecordedTransaction> {
        self.transactions.get_mut(&tx_id)
    }

    fn insert_recorded_tx(&mut self, tx: RecordedTransaction) -> Result<(), LedgerError> {
        if self.transactions.get(&tx.tx_id()).is_some() {
            return Err(LedgerError::DuplicateTransaction);
        }
        self.transactions.insert(tx.tx_id(), tx);
        Ok(())
    }

    fn snapshot_accounts(&self) -> Vec<AccountSnapshot> {
        self.accounts.values().map(AccountSnapshot::from).collect()
    }
}
