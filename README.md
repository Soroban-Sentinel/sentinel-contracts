# Sentinel contracts

Small Soroban security examples for Stellar developers, with executable regression tests.
These are educational contracts, not audited production assets.

## Contracts

- `sentinel-token`: fixed supply token example with authenticated initialization and transfers.
  Self transfers validate authorization and available balance without changing storage.
  Recipient additions use checked arithmetic.
- `sentinel-vault`: demonstration balance ledger with authenticated initialization,
  deposits and withdrawals, checked credit arithmetic, and an owner-authorized deposit pause.
  Withdrawals remain available while paused. **No Stellar assets are transferred or held.**
  The previous placeholder `emergency_drain` API and cosmetic reentrancy flag were removed.

Neither example implements the full Stellar Asset Contract interface. Storage TTL renewal,
asset custody, deployment administration and production lifecycle management are outside scope.
Initialization requires the proposed administrator's authorization; deployments should initialize
promptly because any authorized address can become administrator while uninitialized.

## Reproduce the checks

The workspace pins Soroban SDK 22.0.11. Install Rust with rustfmt and Clippy, plus the `wasm32-unknown-unknown` target:

```sh
rustup target add wasm32-unknown-unknown
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --locked --target wasm32-unknown-unknown --release -p sentinel-token -p sentinel-vault
```

CI runs these checks and uploads the compiled WASM artifacts. Test-only Soroban SDK
features are enabled through dev-dependencies rather than production dependencies.

## Security regression coverage

The tests exercise self-transfer inflation, 198 sequential transfers with conservation
assertions, insufficient funds, nonpositive amounts, repeated initialization, missing
authorization, vault overflow rollback, isolation of account balances, and withdrawals
while deposits are paused. These are deterministic tests, not a formal proof or fuzz campaign.

`.sentinel.toml` and `sentinel-config` provide example configuration for integrating with
Soroban Sentinel tooling. The current workflow does not invoke a Sentinel CLI, fuzzing engine,
or formal verifier. Configuration entries do not establish that an invariant is verified.

See [CHANGELOG.md](CHANGELOG.md) for the security changes and API migration notes.
