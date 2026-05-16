# Pulse 04: Cache index gate contract

## Goal

Add a product-neutral cache-index gate report so consumers can reuse common
unexpected, missing, and unverified ledger checks without moving ICELINES,
BISECT, ROUTE, CROP, or PROOF policy into `fletch-core`.

## Implementation

- Added `fletch.cache-index-gate.v1` report types and
  `cache_index_gate_report` in `fletch-core`.
- Added `fletch cache index-gate` to `fletch-cli`.
- Extended `fletch-mock-client` to prove a registry-shaped expected set can gate
  a manifest-first consumer flow.
- Documented the consumer contract in the README.

## Validation

- `cargo fmt`
- `cargo test -p fletch-core --lib`
- `cargo test -p fletch-mock-client --lib`
- `cargo check -p fletch-cli`
- Focused `fletch cache index-gate --require-all-expected --gate` smoke
- `git diff --check`

## Non-goals

- No product activation, parsing, or snapshot policy in `fletch-core`.
- No persistent manifest daemon or database.
