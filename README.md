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

Generative AI tools were used during development as an assistant for brainstorming, drafting, and reviewing ideas. This included help from ChatGPT on a few specific parts of the project.

### Where AI was used

AI was mainly used for:

- **Initial design discussion**
  - high-level architecture ideas
  - how to structure the project for readability and extensibility
  - how to separate concerns between domain logic, engine logic, ledger/state, and CSV I/O

- **Business logic discussion**
  - clarifying how to think about transaction flows such as deposits, withdrawals, disputes, resolves, and chargebacks
  - talking through edge cases and invalid operations
  - discussing how account locking should affect future transactions

- **CSV parsing / writing**
  - understanding how to use Rust’s `csv` crate cleanly
  - discussing how to read the input file, deserialize records, and write output records
  - talking through how to keep CSV concerns separate from domain logic

- **Testing**
  - brainstorming a unit and integration test plan
  - identifying important edge cases to cover
  - helping draft test case organization

- **Documentation**
  - refining the README, assumptions, and test plan
  - improving how design decisions and tradeoffs were explained

### Type of prompts used

The prompts I used were focused on guidance and reasoning rather than asking AI to generate a full solution. Examples include:

- how to structure a Rust payment engine project
- how to model dispute / resolve / chargeback flows
- how to use the `csv` crate for streaming input/output
- debugging help when tracing issues in transaction handling or CSV parsing
- clarifying edge cases and expected system behavior

The goal of these prompts was to explore approaches, validate understanding, and unblock specific issues rather than outsource implementation.

### How I used the output

I did not treat AI output as automatically correct.

For each area where AI was used, I reviewed the suggestions carefully, compared them against the project requirements, and then decided what to keep, what to change, and what not to use. In practice, this meant:

- checking suggested designs against the actual assessment requirements
- simplifying ideas that were too complex for the scope of the project
- adjusting code structure to match the final implementation I wanted
- validating behavior through manual testing and planned automated tests
- making final decisions myself about business rules, error handling, and maintainability tradeoffs

### Final ownership

I take ownership of the final implementation and the technical decisions in this repository.

AI was used as a support tool to help think through design options, edge cases, test coverage, and documentation, but I thoroughly reviewed the suggestions, made my own implementation choices, and ensured the final solution matched the requirements and behavior I intended.

---

## Notes

- Monetary values are handled carefully to avoid floating-point precision issues
- The system is deterministic and processes transactions in input order
- The architecture leaves room for future extensions if needed
