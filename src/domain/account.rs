use crate::domain::amount::Amount;
use crate::domain::client::ClientId;

pub struct Account {
    pub client_id: ClientId,
    pub available: Amount,
    pub held: Amount,
    pub locked: bool,
}
