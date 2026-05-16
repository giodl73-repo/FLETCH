# Pulse 04: Merge-ready bundle report

## Goal

Describe staged quiver members as candidate merge and alias inputs without
activating aliases, partitions, or product views.

## Outcome

- Added `fletch.quiver-merge-ready.v1`.
- Emitted candidate rows with dataset IDs, optional alias ID, cache keys, hashes,
  paths, verification flags, and ready/blocked status.
- Added `fletch quiver merge-ready --quiver ...`.
- Kept merge readiness as preview data only.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for quiver merge-ready JSON
- `git diff --check`
