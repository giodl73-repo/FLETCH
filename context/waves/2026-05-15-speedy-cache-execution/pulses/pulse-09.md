# Pulse 09: Ledger output upsert

## Goal

Let repeated generic fetch executions build one cache ledger instead of replacing
an output manifest with a single latest entry.

## Outcome

- Added `upsert_cache_manifest_entry` to replace matching cache-key entries while
  preserving unrelated ledger entries.
- Updated `fletch fetch --output` and `fletch fetch-plan --output` to read an
  existing manifest and upsert the new entry.
- Guarded CLI output upserts so the existing manifest cache root must match the
  requested cache root.
- Kept merge/activation out of scope: this only updates a cache ledger.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for multi-entry ledger output and replacement by cache key
- `git diff --check`
