# Pulse 02: Active alias contract

## Goal

Define product-neutral active alias state that names a cached ledger entry
without moving bytes or applying consumer-specific activation semantics.

## Outcome

- Added `fletch.alias-state.v1`.
- Added alias records containing alias ID, dataset ID, cache key, hash, and
  relative path.
- Added `fletch merge alias-state --manifest ... --alias-id ... --dataset-id ...`.
- Missing alias targets fail explicitly instead of creating dangling aliases.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for alias state creation and missing-target failure
- `git diff --check`
