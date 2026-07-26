# Pulse 01: MDCROP index report

## Goal

Emit indexable MDCROP rows over existing FLETCH status, graph, and tip contracts
without creating a separate source of truth.

## Outcome

- Add a named `fletch.mdcrop-index.v1` contract.
- Report row counts and rows for cache statuses, graph nodes, graph edges, and
  tips.
- Add a read-only CLI command over a manifest.
- Document that the index points back to machine contracts.
- Keep generated indexes derived from authoritative FLETCH JSON contracts.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for MDCROP index JSON
- `git diff --check`
