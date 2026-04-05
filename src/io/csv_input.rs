use crate::domain::amount::Amount;
use crate::domain::transaction::Transaction;
use crate::error::app_error::AppError;
use crate::io::record::CsvTransactionRecord;
use crate::traits::transaction_source::TransactionSource;
use csv::ReaderBuilder;
use std::io::Read;

pub struct CsvTransactionSource<R: Read> {
    records: csv::DeserializeRecordsIntoIter<R, CsvTransactionRecord>,
}

impl<R: Read> CsvTransactionSource<R> {
    pub fn new(input: R) -> Self {
        let reader = ReaderBuilder::new().trim(csv::Trim::All).from_reader(input);

        let records = reader.into_deserialize();

        Self { records }
    }
}

impl<R: Read> TransactionSource for CsvTransactionSource<R> {
    fn next_transaction(&mut self) -> Result<Option<Transaction>, AppError> {
        match self.records.next() {
            Some(result) => result
                .map_err(|_| AppError::Error)
                .and_then(Transaction::try_from)
                .map(Some),
            None => Ok(None),
        }
    }
}

impl TryFrom<CsvTransactionRecord> for Transaction {
    type Error = AppError;

    fn try_from(raw: CsvTransactionRecord) -> Result<Self, Self::Error> {
        let client_id = raw.client;
        let tx_id = raw.tx;
        let amount = convert_amount(raw.amount);

        match raw.tx_type.as_str() {
            "deposit" => Ok(Transaction::Deposit {
                client_id,
                tx_id,
                amount,
            }),
            "withdrawal" => Ok(Transaction::Withdrawal {
                client_id,
                tx_id,
                amount,
            }),
            "dispute" => Ok(Transaction::Dispute { client_id, tx_id }),
            "resolve" => Ok(Transaction::Resolve { client_id, tx_id }),
            "chargeback" => Ok(Transaction::Chargeback { client_id, tx_id }),
            _ => Err(AppError::Error),
        }
    }
}

fn convert_amount(amount: Option<String>) -> Amount {
    match amount {
        None => 0,
        Some(a) => {
            parse_amount_4dp(&a).ok().unwrap_or(0) // todo come back and add error handling here
        }
    }
}

// This entire function was created using AI
fn parse_amount_4dp(s: &str) -> Result<Amount, String> {
    let s = s.trim();
    let (sign, body) = if let Some(rest) = s.strip_prefix('-') {
        (-1i64, rest)
    } else if let Some(rest) = s.strip_prefix('+') {
        (1i64, rest)
    } else {
        (1i64, s)
    };

    let (whole_str, frac_str) = body.split_once('.').unwrap_or((body, ""));
    if frac_str.len() > 4 {
        return Err("too many decimal places (max 4)".into());
    }

    let whole: i64 = whole_str.parse().map_err(|_| "invalid whole part")?;
    let mut frac = frac_str.to_string();
    while frac.len() < 4 {
        frac.push('0');
    }
    let frac: i64 = if frac.is_empty() {
        0
    } else {
        frac.parse().map_err(|_| "invalid fractional part")?
    };

    let scaled = sign
        .checked_mul(whole.checked_mul(10_000).ok_or("overflow")? + frac)
        .ok_or("overflow")?;

    i64::try_from(scaled).map_err(|_| "out of i32 range".into())
}
