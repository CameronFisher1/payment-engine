#![allow(dead_code)]

use payment_engine::domain::amount::Amount;
use payment_engine::domain::client::ClientId;
use payment_engine::domain::dispute::DisputeState;
use payment_engine::domain::transaction_id::TxId;
use payment_engine::ledger::in_memory::InMemoryLedger;
use payment_engine::traits::ledger::Ledger;

pub fn assert_account(
    ledger: &InMemoryLedger,
    client_id: ClientId,
    available: Amount,
    held: Amount,
    locked: bool,
) {
    let account = ledger
        .account(client_id)
        .unwrap_or_else(|| panic!("expected account for client {}", client_id));

    assert_eq!(account.client_id, client_id);
    assert_eq!(account.available, available);
    assert_eq!(account.held, held);
    assert_eq!(account.locked, locked);
}

pub fn assert_account_absent(ledger: &InMemoryLedger, client_id: ClientId) {
    assert!(
        ledger.account(client_id).is_none(),
        "expected no account for client {}",
        client_id
    );
}

pub fn assert_tx_dispute_state(ledger: &InMemoryLedger, tx_id: TxId, state: DisputeState) {
    let tx = ledger
        .recorded_tx(tx_id)
        .unwrap_or_else(|| panic!("expected recorded transaction {}", tx_id));

    assert_eq!(tx.dispute_state, state);
}
