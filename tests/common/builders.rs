#![allow(dead_code)]

use payment_engine::domain::account::AccountSnapshot;
use payment_engine::domain::amount::Amount;
use payment_engine::domain::client::ClientId;
use payment_engine::domain::dispute::DisputeState;
use payment_engine::domain::recorded_transaction::RecordedTransaction;
use payment_engine::domain::transaction::Transaction;
use payment_engine::domain::transaction_id::TxId;
use payment_engine::io::csv_input::CsvTransactionSource;
use payment_engine::io::record::CsvTransactionRecord;
use std::io::Cursor;

pub fn amount(value: i32) -> Amount {
    value
}

pub fn account_snapshot(
    client_id: ClientId,
    available: Amount,
    held: Amount,
    total: Amount,
    locked: bool,
) -> AccountSnapshot {
    AccountSnapshot {
        client_id,
        available,
        held,
        total,
        locked,
    }
}

pub fn deposit(client_id: ClientId, tx_id: TxId, amount: Amount) -> Transaction {
    Transaction::Deposit {
        client_id,
        tx_id,
        amount,
    }
}

pub fn withdrawal(client_id: ClientId, tx_id: TxId, amount: Amount) -> Transaction {
    Transaction::Withdrawal {
        client_id,
        tx_id,
        amount,
    }
}

pub fn dispute(client_id: ClientId, tx_id: TxId) -> Transaction {
    Transaction::Dispute { client_id, tx_id }
}

pub fn resolve(client_id: ClientId, tx_id: TxId) -> Transaction {
    Transaction::Resolve { client_id, tx_id }
}

pub fn chargeback(client_id: ClientId, tx_id: TxId) -> Transaction {
    Transaction::Chargeback { client_id, tx_id }
}

pub fn recorded_deposit(
    client_id: ClientId,
    tx_id: TxId,
    amount: Amount,
    dispute_state: DisputeState,
) -> RecordedTransaction {
    RecordedTransaction {
        tx_id,
        client_id,
        amount,
        kind: deposit(client_id, tx_id, amount),
        dispute_state,
    }
}

pub fn recorded_withdrawal(
    client_id: ClientId,
    tx_id: TxId,
    amount: Amount,
    dispute_state: DisputeState,
) -> RecordedTransaction {
    RecordedTransaction {
        tx_id,
        client_id,
        amount,
        kind: withdrawal(client_id, tx_id, amount),
        dispute_state,
    }
}

pub fn csv_record(
    tx_type: &str,
    client: ClientId,
    tx: TxId,
    amount: Option<&str>,
) -> CsvTransactionRecord {
    CsvTransactionRecord {
        tx_type: tx_type.to_string(),
        client,
        tx,
        amount: amount.map(str::to_string),
    }
}

pub fn csv_source(input: &str) -> CsvTransactionSource<Cursor<Vec<u8>>> {
    CsvTransactionSource::new(Cursor::new(input.as_bytes().to_vec()))
}
