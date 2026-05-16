# Pulse 01: Batch manifest upsert

## Goal

Let consumers merge multiple fetched cache entries into a durable FLETCH manifest
through one product-neutral helper instead of open-coding repeated single-entry
upserts.

## Outcome

- Added `upsert_cache_manifest_entries` to `fletch-core`.
- Reused the batch helper from the CLI manifest write path.
- Kept manifests as validated ledgers keyed by cache key; the helper does not read
  files, fetch sources, inspect objects, or activate product data.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for manifest upsert
- `git diff --check`
