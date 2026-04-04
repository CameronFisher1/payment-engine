use crate::domain::amount::Amount;
use crate::domain::client::ClientId;
use crate::domain::transaction_id::TxId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CsvTransactionRecord {
    #[serde(rename = "type")]
    pub tx_type: String,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<String>,
}

#[derive(Serialize)]
pub struct CsvAccountOutputRecord {
    pub client: ClientId,
    pub available: Amount,
    pub held: Amount,
    pub total: Amount,
    pub locked: bool,
}
