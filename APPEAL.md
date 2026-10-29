# Appeal draft

We request reconsideration of Soroban-Sentinel/sentinel-contracts following substantive
security and quality improvements to the example contracts.

We fixed a token balance-conservation vulnerability: a self-transfer previously increased
the sender's balance without increasing total supply. Self-transfers now preserve balances.
We added administrator authorization at initialization and checked arithmetic for token
recipient balances and vault credits.

We replaced the vault's placeholder emergency drain with an implemented, owner-authorized
deposit pause that preserves withdrawals. We removed the ineffective lock mechanism and
explicitly documented that the vault is an educational ledger, not an asset-custody contract.

New regression tests exercise conservation across transfer sequences, self-transfers,
missing and incorrect authorization, repeated initialization, insufficient balances,
nonpositive amounts, overflow rollback, and withdrawals during a pause. We upgraded the
Soroban SDK to 22.0.11 to support these failure-case tests on our development toolchain,
and separated testing features from production contract dependencies.

The repository now describes its actual capabilities and limitations. CI runs formatting,
strict linting, unit tests and WASM compilation; we do not claim that this workflow performs
formal verification or fuzzing. These examples give Stellar developers reproducible security
regressions and authorization patterns to study and extend.

Local validation on 2026-10-29 passed: formatting, Clippy with warnings denied,
all seven workspace tests, and release WASM compilation for both contracts.

Before submission, publish the changes and attach the resulting commit and successful CI
run links. Confirm these changes postdate the rejection. This draft has not been submitted.
