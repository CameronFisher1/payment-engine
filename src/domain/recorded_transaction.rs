use crate::domain::amount::Amount;
use crate::domain::client::ClientId;
use crate::domain::dispute::DisputeState;
use crate::domain::transaction::Transaction;
use crate::domain::transaction_id::TxId;

pub struct RecordedTransaction {
    pub tx_id: TxId,
    pub client_id: ClientId,
    pub amount: Amount,
    pub kind: Transaction, // Technically, this value can only be deposit or withdrawal
    pub dispute_state: DisputeState,
}

impl RecordedTransaction {
    pub fn tx_id(&self) -> TxId {
        self.tx_id
    }
}
