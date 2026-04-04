# Test Plan

## Unit Tests

Unit tests focus on testing each part of the system in isolation.

### Amount
- parse valid amounts (e.g. "1", "1.0", "1.2345")
- format to 4 decimal places
- reject invalid formats
- addition and subtraction work correctly

### Account
- new account starts empty and unlocked
- deposit increases available + total
- withdrawal works with enough funds
- withdrawal fails with insufficient funds
- dispute moves available → held
- resolve moves held → available
- chargeback removes funds and locks account
- locked account ignores further updates

### Ledger
- accounts are created when needed
- account updates persist
- transactions can be stored and retrieved
- duplicate transaction ids are rejected

### CSV Input
- parse all transaction types correctly
- handle spaces in CSV
- amount required for deposit/withdrawal
- amount optional for dispute/resolve/chargeback
- invalid rows are rejected

### CSV Output
- correct headers are written
- values formatted to 4 decimals
- one row per account
- output is consistent

### Engine

#### Deposits / Withdrawals
- deposits create/update accounts
- withdrawals work or fail correctly
- duplicate transactions are ignored
- accounts stay isolated per client

#### Disputes / Resolves
- dispute moves funds to held
- resolve moves funds back
- invalid disputes/resolves are ignored

#### Chargebacks
- chargeback removes funds
- account becomes locked
- locked account ignores all future transactions

#### Invalid Operations
- invalid actions are ignored
- no crashes or unexpected mutations
- cross-client operations are blocked

---

## Integration Tests

Integration tests verify the full flow:

CSV → parsing → engine → output

### Happy Path
- deposits and withdrawals produce correct balances
- multiple clients handled correctly

### Dispute Flow
- dispute and resolve update balances correctly
- invalid dispute/resolve has no effect

### Chargeback Flow
- chargeback removes funds and locks account
- locked accounts ignore future transactions

### Invalid Cases
- insufficient withdrawals are ignored
- duplicate transactions handled correctly
- invalid references are ignored

### Output Format
- correct headers
- correct values
- 4 decimal formatting

---

## Manual Testing

Manual testing was done using a sample transactions file:

- [transactions.csv](../transactions.csv)

### How to run

```bash
cargo run -- transactions.csv > accounts.csv