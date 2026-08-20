# Pulse 02: Retry status reporting

## Goal

Make retry behavior visible in machine-readable fetch outputs so consumers,
MDCROP, PROOF, and CI can explain whether an object arrived on the first attempt
or after retry recovery.

## Changes

- Added `FetchAttemptStatus` to `FetchOutcome`.
- Added `fetch_attempts`, `retry_count`, and `last_retryable_error` fields to
  `CacheEntry` ledger rows.
- Recorded retry status for successful generic HTTP/file fetches and cache hits.
- Added graph ledger metadata for fetch attempt and retry counts.
- Added a local HTTP retry test that succeeds after a retryable 500 response.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- CLI smoke for `fletch fetch --retry-attempts ...`
- Mock client smoke
- `git diff --check`

## Status

Done.
