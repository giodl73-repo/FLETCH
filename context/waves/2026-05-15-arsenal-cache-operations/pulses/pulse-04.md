# Pulse 04: Prune safety expansion

## Goal

Make prune output explicit enough for operators and publishers to understand what
would be deleted before any destructive cache command exists.

## Outcome

- Added `fletch.cache-prune.v1`.
- Added generated-by, object root, `destructive: false`, keep byte totals, and
  candidate reasons to prune plans.
- Kept `fletch cache prune` plan-only and read-only.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for `fletch cache prune`
- `git diff --check`
