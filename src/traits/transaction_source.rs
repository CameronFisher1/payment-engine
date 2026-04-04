use crate::domain::transaction::Transaction;
use crate::error::app_error::AppError;

pub trait TransactionSource {
    fn next_transaction(&mut self) -> Result<Option<Transaction>, AppError>;
}
