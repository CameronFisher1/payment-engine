use crate::domain::account::AccountSnapshot;
use crate::domain::amount::Amount;
use crate::error::app_error::AppError;
use crate::io::record::CsvAccountOutputRecord;
use crate::traits::report_sink::ReportSink;
use csv::WriterBuilder;
use std::io::Write;

pub struct CsvReportSink<W: Write> {
    writer: csv::Writer<W>,
}

impl<W: Write> CsvReportSink<W> {
    pub fn new(output: W) -> Self {
        let writer = WriterBuilder::new().has_headers(true).from_writer(output);

        Self { writer }
    }
}

impl<W: Write> ReportSink for CsvReportSink<W> {
    fn write_accounts(&mut self, accounts: &[AccountSnapshot]) -> Result<(), AppError> {
        for account in accounts {
            let record = CsvAccountOutputRecord {
                client: account.client_id,
                available: convert_amount_4dp(account.available),
                held: convert_amount_4dp(account.held),
                total: convert_amount_4dp(account.total),
                locked: account.locked,
            };

            let _ = self.writer.serialize(record);
        }

        let _ = self.writer.flush();
        Ok(())
    }
}

// This entire function was created using AI
fn convert_amount_4dp(amount: Amount) -> String {
    let sign = if amount < 0 { "-" } else { "" };
    let absolute = i64::from(amount).abs();
    let integer = absolute / 10_000;
    let fractional = absolute % 10_000;

    if fractional == 0 {
        return format!("{sign}{integer}");
    }

    let mut fractional_text = format!("{fractional:04}");
    while fractional_text.ends_with('0') {
        let _ = fractional_text.pop();
    }

    format!("{sign}{integer}.{fractional_text}")
}
