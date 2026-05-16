# Pulse 04: Rollback preview

## Goal

Preview rolling current alias state back to a prior label target before any alias
or active-view mutation.

## Outcome

- Added `fletch.rollback-preview.v1`.
- Added restore action rows comparing current alias cache keys with target label
  cache keys.
- Added `fletch merge rollback-preview --alias-state ... --label-state ...`.
- Kept rollback preview non-destructive and product-neutral.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for rollback preview actions
- `git diff --check`
