# Pulse 01: Retry and timeout controls

## Goal

Add small, product-neutral controls for generic fetch execution so consumers can
bound network waits and retry transient fetch/read/write failures.

## Changes

- Added `FetchOptions::with_timeout_ms(...)` for generic HTTP request timeouts.
- Added `FetchOptions::with_retry_attempts(...)` to retry retryable generic fetch
  failures after the initial attempt.
- Added CLI flags `--timeout-ms` and `--retry-attempts` to `fletch fetch`.
- Added validation for zero timeout values and tests for timeout/retry options.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- CLI smoke for `fletch fetch --timeout-ms ... --retry-attempts ...`
- Mock client smoke
- `git diff --check`

## Status

Done.
