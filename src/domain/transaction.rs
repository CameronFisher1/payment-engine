use crate::domain::amount::Amount;
use crate::domain::client::ClientId;
use crate::domain::transaction_id::TxId;

pub enum Transaction {
    Deposit {
        client_id: ClientId,
        tx_id: TxId,
        amount: Amount,
    },
    Withdrawal {
        client_id: ClientId,
        tx_id: TxId,
        amount: Amount,
    },
    Dispute {
        client_id: ClientId,
        tx_id: TxId,
    },
    Resolve {
        client_id: ClientId,
        tx_id: TxId,
    },
    Chargeback {
        client_id: ClientId,
        tx_id: TxId,
    },
}

impl Transaction {
    pub fn client_id(&self) -> ClientId {
        match self {
            Transaction::Deposit { client_id, .. }
            | Transaction::Withdrawal { client_id, .. }
            | Transaction::Dispute { client_id, .. }
            | Transaction::Resolve { client_id, .. }
            | Transaction::Chargeback { client_id, .. } => *client_id,
        }
    }

    pub fn tx_id(&self) -> TxId {
        match self {
            Transaction::Deposit { tx_id, .. }
            | Transaction::Withdrawal { tx_id, .. }
            | Transaction::Dispute { tx_id, .. }
            | Transaction::Resolve { tx_id, .. }
            | Transaction::Chargeback { tx_id, .. } => *tx_id,
        }
    }

    pub fn amount(&self) -> Option<Amount> {
        match self {
            Transaction::Deposit { amount, .. } | Transaction::Withdrawal { amount, .. } => {
                Some(*amount)
            }
            _ => None,
        }
    }
}
