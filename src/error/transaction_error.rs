pub enum TransactionError {
    InvalidInput,
    InvalidFunds,
    LedgerError,
    AccountLocked,
    TransactionNotFound,
    DisputeError,
}
