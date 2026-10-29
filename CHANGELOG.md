# Changes

## Unreleased — security regression improvements

- Fix token self-transfer inflation: the original implementation overwrote the debit with
  a credit computed from the original balance when sender and recipient were identical.
- Require administrator authorization during token and vault initialization.
- Check token recipient and vault deposit arithmetic before writing balances.
- Replace the vault's no-op emergency drain with owner-authorized `set_paused(bool)`.
  Deposits stop while paused; withdrawals remain available.
- Remove the vault lock flag: there are no external contract calls to protect in this ledger.
- Add deterministic conservation and rejected-operation regression coverage.
- Upgrade Soroban SDK from 20.5.0 to pinned 22.0.11 and use the current registration API
  for that SDK, enabling negative-call tests on the development toolchain.
- Lock the SDK host dependency to ed25519-dalek 2.2.0; the permissive upstream
  dependency otherwise resolves to an incompatible random-number-generator API in 3.0.
- Separate Soroban SDK test features from production dependencies.
- Make CI names, permissions and documentation match the checks actually performed.

- Pin local and CI Rust to the validated 1.96.0 toolchain and include the compiler
  version in the cache key to avoid drifting checks and stale compiler caches.

### Migration

The vault no longer exposes `emergency_drain(Address)`. Use `set_paused(bool)` with the
stored owner's authorization for the supported emergency control. This is an example
ledger and cannot custody, return or drain real tokens. These changes are intended for
fresh deployments; no deployed-contract storage migration has been implemented.
