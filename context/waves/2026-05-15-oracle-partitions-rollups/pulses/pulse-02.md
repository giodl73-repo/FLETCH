# Pulse 02: Rollup edge preview

## Goal

Preview product-neutral rollup edges from a parent rollup ID to child partition
rows before any rollup materialization or activation.

## Outcome

- Added `fletch.rollup-preview.v1`.
- Added edge rows carrying rollup ID, partition ID, dataset ID, cache key, hash,
  byte count, and relative path.
- Added missing child partition reporting for subset previews.
- Added `fletch partition rollup-preview --partition-state ... --rollup-id ...`.
- Kept rollup preview non-destructive and domain-neutral.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for rollup preview JSON
- `git diff --check`
