# Assumptions
This document displays some assumptions I am making in this project

- Once an account is locked, that account is permanently locked.
- Disputing a transaction can only be done on a deposit transaction.
- Disputes, Resolves, and chargebacks can be done regardless of whether the account is locked.
- Account balances can go into the negatives when dispute/chargeback occurs with not enough funds.
- CSV input file is placed in root directory of this repository
- If a deposit/withdrawal doesn't have an amount associated to it. We will treat it as a $0 transaction

### Questions TBD
- How does disputing a withdrawal work?
- How do we handle errors? Case to case basis?