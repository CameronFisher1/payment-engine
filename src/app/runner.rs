use crate::domain::account::AccountSnapshot;
use crate::engine::payment_engine::PaymentEngine;
use crate::error::app_error::AppError;
use crate::io::csv_input::CsvTransactionSource;
use crate::io::csv_output::CsvReportSink;
use crate::ledger::in_memory::InMemoryLedger;
use crate::traits::ledger::Ledger;
use crate::traits::report_sink::ReportSink;
use crate::traits::transaction_source::TransactionSource;
use std::io::{Read, Write};

pub fn run<R: Read, W: Write>(input: R, output: W) -> Result<(), AppError> {
    let mut source: CsvTransactionSource<R> = CsvTransactionSource::new(input);
    let mut writer: CsvReportSink<W> = CsvReportSink::new(output);
    let mut ledger: InMemoryLedger = InMemoryLedger::new();
    let mut engine: PaymentEngine = PaymentEngine::new();

    while let Some(tx) = source.next_transaction()? {
        let _ = engine.apply(tx, &mut ledger);
    }

    let snapshot: Vec<AccountSnapshot> = ledger.snapshot_accounts();
    writer.write_accounts(snapshot.as_slice())?;

    Ok(())
}
