# Payment Engine

A payment processing engine built in Rust.

This project reads a stream of financial transactions from a CSV file, processes them, and outputs the resulting account balances.

---

## Author

Created by Cameron Fisher

---

## How to Run

```bash
cargo run -- transactions.csv > accounts.csv
```

### Input
- A CSV file containing transactions
- Example: `transactions.csv`

### Output
- A CSV file written to stdout (redirected to `accounts.csv`)
- Contains:
  - client
  - available
  - held
  - total
  - locked

---

## Supported Transactions

- deposit
- withdrawal
- dispute
- resolve
- chargeback

---

## Behavior Overview

- Transactions are processed in order
- Deposits increase available funds
- Withdrawals decrease available funds (if sufficient)
- Disputes move funds from available → held
- Resolves move funds from held → available
- Chargebacks remove funds and lock the account
- Locked accounts ignore all future transactions
- Invalid operations are safely ignored (no crashes)

---

## Design Overview

The system is structured into clear layers:

- **Domain** → core business logic (accounts, amounts, transactions)  
- **Engine** → transaction processing logic  
- **Ledger** → in-memory state storage  
- **I/O** → CSV parsing and output formatting  

This separation keeps the system:
- easy to reason about
- easy to test
- easy to extend

---

## Testing

All testing details can be found in:

- `docs/test-plan.md`

This includes:
- unit tests  
- integration tests  
- manual testing  

---

## Assumptions

Some assumptions were made during development.

These are documented in:

- `docs/assumptions.md`

---

## AI Usage

AI was used as a development tool throughout this project to assist with speed and clarity.

Specifically, AI helped with:

- initial system design and architecture planning  
- some business logic implementation ideas  
- CSV parsing setup and usage of the `csv` crate  
- creating the test plan and assisting with test generation  
- writing and refining documentation  

All final code decisions, structure, and validation were reviewed and adjusted manually to ensure correctness and alignment with the project requirements.

---

## Notes

- Monetary values are handled carefully to avoid floating-point issues  
- The system is deterministic and processes transactions in order  
- The architecture allows for future extensions (e.g., concurrency, different input sources)  
