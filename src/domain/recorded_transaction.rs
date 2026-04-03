use crate::domain::amount::Amount;
use crate::domain::client::ClientId;
use crate::domain::dispute::DisputeState;
use crate::domain::transaction::Transaction;
use crate::domain::transaction_id::TxId;

pub struct RecordedTransaction {
    tx_id: TxId,
    client_id: ClientId,
    amount: Amount,
    kind: Transaction,
    dispute_state: DisputeState
}