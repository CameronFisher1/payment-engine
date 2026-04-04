use crate::domain::transaction::Transaction;
use crate::engine::payment_engine::PaymentEngine;
use crate::ledger::in_memory::InMemoryLedger;
use crate::traits::ledger::Ledger;
use std::io::{Read, Write};

pub fn run<R: Read, W: Write>(input: R, output: W) {
    let mut ledger: InMemoryLedger = InMemoryLedger::new();
    let mut engine: PaymentEngine = PaymentEngine::new();

    let sample = Transaction::Deposit {
        client_id: 2,
        tx_id: 1,
        amount: 3,
    };

    engine.apply(sample, &mut ledger);

    println!("{}", ledger.account(2).unwrap().available);
    let sample = Transaction::Withdrawal {
        client_id: 2,
        tx_id: 2,
        amount: 2,
    };
    engine.apply(sample, &mut ledger);

    println!("{}", ledger.account(2).unwrap().available);

    let sample = Transaction::Dispute {
        client_id: 2,
        tx_id: 1,
    };
    engine.apply(sample, &mut ledger);

    println!("{}", ledger.account(2).unwrap().available);

    let sample = Transaction::Chargeback {
        client_id: 2,
        tx_id: 1,
    };
    engine.apply(sample, &mut ledger);

    println!("{}", ledger.account(2).unwrap().available);
}
