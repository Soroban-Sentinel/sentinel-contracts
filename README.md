# sentinel-contracts

Example Soroban contracts wired up to Soroban Sentinel for automated fuzzing and formal verification.

## Structure

```
contracts/
  token/   — fungible token (balance_conservation, no_overflow invariants)
  vault/   — vault with access-control and reentrancy guard
sentinel-config/  — shared config-parsing crate
.sentinel.toml    — one-line invariant config (no harness code needed)
```

## Quick start

```bash
# Build all contracts
stellar contract build

# Run unit tests
cargo test --workspace

# Run Sentinel locally (requires sentinel-backend running)
sentinel-cli run --config .sentinel.toml
```

## Adding invariants to your own contract

1. Add `.sentinel.toml` to your workspace root.
2. List your contract and choose invariant templates:

```toml
[[contracts]]
name = "my-contract"
path = "contracts/my-contract"
invariants = ["balance_conservation", "access_control"]
```

3. Push a PR — the GitHub Action blocks the merge if any invariant is violated.
