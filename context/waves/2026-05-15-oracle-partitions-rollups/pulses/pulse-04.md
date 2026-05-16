# Pulse 04: Active partition set

## Goal

Produce query-facing active partition reports from partition, alias, label, and
rollup evidence without treating cache presence alone as activation.

## Outcome

- Added `fletch.active-partition-set.v1`.
- Added active partition rows with alias IDs, label IDs, rollup IDs, cache keys,
  hashes, paths, and verification state.
- Added `fletch partition active-set --partition-state ...` with optional
  `--alias-state`, `--label-state`, and `--rollup-preview` inputs.
- Kept active partition sets derived and non-mutating.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for active partition set JSON
- `git diff --check`
