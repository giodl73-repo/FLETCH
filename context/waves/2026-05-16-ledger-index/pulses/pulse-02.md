# Pulse 02: Cache index lookup

## Goal

Make the compact cache index useful for large-ledger lookup without introducing a
database, daemon, or activation semantics.

## Outcome

- Added read-only cache index slicing.
- Added exact `--dataset-id` and `--cache-key` lookups to
  `fletch cache index`.
- Added `--verified`, `--offset`, and `--limit` to focus large index output.
- Kept `fletch.cache-index.v1` derived from the authoritative cache manifest.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for filtered cache index JSON
- `git diff --check`
