use crate::domain::amount::Amount;
use crate::domain::client::ClientId;

pub struct Account {
    pub client_id: ClientId,
    pub available: Amount,
    pub held: Amount,
    pub locked: bool,
}

pub struct AccountSnapshot {
    pub client_id: ClientId,
    pub available: Amount,
    pub held: Amount,
    pub total: Amount,
    pub locked: bool,
}

impl From<&Account> for AccountSnapshot {
    fn from(account: &Account) -> Self {
        Self {
            client_id: account.client_id,
            available: account.available,
            held: account.held,
            total: account.available + account.held,
            locked: account.locked,
        }
    }
}
