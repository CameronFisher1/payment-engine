#[derive(Debug, PartialEq, Eq)]
pub enum TransactionError {
    InvalidInput,
    InvalidFunds,
    LedgerError,
    AccountLocked,
    TransactionNotFound,
    DisputeError,
    DuplicateTransaction,
}
