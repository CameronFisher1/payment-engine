use crate::domain::client::ClientId;
use serde::{Deserialize, Serialize};
use crate::domain::transaction_id::TxId;

#[derive(Debug, Deserialize)]
pub struct CsvTransactionRecord {
    #[serde(rename = "type")]
    pub tx_type: String,
    pub client: ClientId,
    pub tx: TxId,
    pub amount: Option<String>,
}

#[derive(Serialize)]
pub struct CsvAccountOutputRecord {
    pub client: ClientId,
    pub available: String,
    pub held: String,
    pub total: String,
    pub locked: bool,
}
