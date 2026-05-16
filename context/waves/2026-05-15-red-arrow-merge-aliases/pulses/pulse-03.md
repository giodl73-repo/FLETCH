# Pulse 03: Labels and pins

## Goal

Add product-neutral labels and pin metadata over alias state so reproducible
active views can be named without product-specific behavior.

## Outcome

- Added `fletch.label-state.v1`.
- Added label records over alias state with alias ID, dataset ID, cache key,
  hash, and `pinned` metadata.
- Added `fletch merge label-state --alias-state ... --label-id ... [--pin]`.
- Kept labels as metadata only; no cache movement or product view mutation.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for label state creation
- `git diff --check`
