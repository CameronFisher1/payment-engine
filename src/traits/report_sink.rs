use crate::domain::account::AccountSnapshot;
use crate::error::app_error::AppError;

pub trait ReportSink {
    fn write_accounts(&mut self, accounts: &[AccountSnapshot]) -> Result<(), AppError>;
}
