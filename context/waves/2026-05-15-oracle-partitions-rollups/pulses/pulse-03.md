# Pulse 03: Invalidation and folding metadata

## Goal

Report stale, folded, and superseded partition metadata without mutating cached
objects, rollups, aliases, or product views.

## Outcome

- Added `fletch.partition-invalidation.v1`.
- Added per-partition stale, folded, and superseded flags with reason rows.
- Added missing partition reporting for invalidation inputs that do not match
  the partition state.
- Added `fletch partition invalidation-report --partition-state ...`.
- Kept invalidation/folding metadata non-destructive and domain-neutral.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for partition invalidation JSON
- `git diff --check`
