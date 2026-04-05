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
- Withdrawals decrease available funds if sufficient funds are available
- Disputes move funds from available to held
- Resolves move funds from held back to available
- Chargebacks remove funds and lock the account
- Locked accounts ignore future transactions
- Invalid operations are safely ignored

---

## Design Overview

The system is structured into clear layers:

- **Domain** -> core business logic such as accounts, amounts, and transactions
- **Engine** -> transaction processing rules
- **Ledger** -> in-memory account and transaction state
- **I/O** -> CSV parsing and output formatting

This separation keeps the code easier to reason about, test, and extend.

---

## Testing

Testing details can be found in:

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

## Design Decisions and Evaluation Criteria

This section explains how the implementation addresses the main evaluation areas.

### Basics

The application builds and runs using standard Rust tooling:

```bash
cargo run -- transactions.csv > accounts.csv
```

It reads transactions from a CSV file passed as a command-line argument and writes the final account state as CSV to stdout.

---

### Completeness

The engine supports all required transaction types:

- deposit
- withdrawal
- dispute
- resolve
- chargeback

Covered behavior includes:
- balance updates for deposits and withdrawals
- dispute lifecycle handling
- account locking after chargeback
- ignoring invalid operations safely

Invalid operations such as disputes on unknown transactions are treated as no-ops instead of crashing the application.

---

### Correctness

Correctness was validated in a few ways.

#### Unit Tests
Unit tests cover the core logic in isolation, including:
- amount parsing and formatting
- account balance transitions
- dispute, resolve, and chargeback behavior
- CSV parsing and output formatting

#### Integration Tests
Integration tests cover the full flow:
- CSV input -> parsing -> processing -> CSV output

These tests verify that the full system behaves correctly for normal flows and edge cases.

#### Manual Testing
Manual testing was also done using sample transaction data included in the repository, including `transactions.csv`.

This was used to verify:
- balance updates
- dispute lifecycle behavior
- account locking
- output formatting

The type system also helps enforce correctness through strong domain types and explicit transaction enums.

---

### Safety and Robustness

The implementation avoids unsafe behavior:

- no use of `unsafe`
- explicit error handling
- invalid operations are ignored safely
- malformed input is handled through parsing and validation logic

Examples of edge cases handled:
- duplicate transaction IDs
- insufficient funds
- disputes on missing transactions
- cross-client transaction misuse
- operations on locked accounts

This keeps the engine resilient and predictable.

---

### Efficiency

The application processes input in a streaming way using the `csv` crate.

That means:
- the entire CSV is not loaded into memory upfront
- records are processed one at a time
- only required state is stored in memory

Stored state includes:
- accounts
- transaction history required for dispute handling

Most lookups are efficient because the in-memory ledger uses hash maps.

This keeps the solution practical for larger files while still keeping the code simple and readable.

---

### Maintainability

The project was structured with readability and maintainability in mind.

This includes:
- clear separation of concerns between layers
- strongly typed domain models
- focused tests
- straightforward control flow
- explicit handling of each transaction type

The focus was on writing code that is easy to review and easy to extend later.

---

## AI Usage

AI was used as a development tool during this project.

It helped with:
- initial design and architecture planning
- some business logic implementation ideas
- CSV reader setup and usage
- test planning and some test creation
- documentation creation

All final code decisions, structure, and validation were reviewed and adjusted manually to make sure the implementation matched the project requirements.

---

## Notes

- Monetary values are handled carefully to avoid floating-point precision issues
- The system is deterministic and processes transactions in input order
- The architecture leaves room for future extensions if needed
