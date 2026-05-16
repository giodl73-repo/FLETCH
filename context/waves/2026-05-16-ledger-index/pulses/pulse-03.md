# Pulse 03: Cache index diff

## Goal

Compare compact ledger indexes so automation can identify changed rows without
rescanning full manifests or cached object bytes.

## Outcome

- Added `fletch.cache-index-diff.v1`.
- Compared cache indexes by cache key and reported added, removed, changed, and
  unchanged counts.
- Added `fletch cache index-diff --base-index ... --candidate-index ...`.
- Kept the diff report derived and read-only; verification/status commands still
  own cached-byte inspection.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for cache index diff JSON
- `git diff --check`
