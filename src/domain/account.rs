use crate::domain::amount::Amount;
use crate::domain::client::ClientId;

pub struct Account {
    client_id: ClientId,
    available: Amount,
    held: Amount,
    locked: bool
}