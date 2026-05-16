# Pulse 01: CROP index report

## Goal

Emit indexable CROP rows over existing FLETCH status, graph, and tip contracts
without creating a separate source of truth.

## Planned outcome

- Add a named `fletch.crop-index.v1` contract.
- Report row counts and rows for cache statuses, graph nodes, graph edges, and
  tips.
- Add a read-only CLI command over a manifest.
- Document that the index points back to machine contracts.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for CROP index JSON
- `git diff --check`
