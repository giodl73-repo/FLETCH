# Pulse 02: Cache execution

## Goal

Add the first generic FLETCH cache execution slice: HTTP/file shafts can be
fetched into a deterministic cache path using temp-file promotion, SHA-256
hashing, and optional checksum verification.

## Changes

- Added generic `fetch_to_cache` execution for `SourceKind::Http` and
  `SourceKind::File`.
- Added deterministic object cache paths under `objects/sha256/<cache-key>`.
- Added temp-file writes, flush/sync, promotion, byte counts, and SHA-256 ledger
  entries.
- Added optional expected-checksum validation with temp cleanup on mismatch.
- Added `fletch fetch` CLI that emits a `fletch.cache-manifest.v1` manifest.
- Added `--max-bytes-per-second` for bandwidth-sensitive fetches.
- Added freshness-aware execution for immutable, max-age, always-check, forced,
  and offline fetches.
- Kept fetch semantics acquisition-only: a fetch caches and emits a manifest but
  does not merge or activate a product's active data view. Future `pull`
  semantics are reserved for fetch plus merge.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- CLI smoke for `fletch plan`
- CLI smoke for `fletch key`
- CLI smoke for `fletch fetch --source-kind file`
- CLI smoke for `fletch fetch --source-kind file --max-bytes-per-second`
- `git diff --check`

## Status

Done.
