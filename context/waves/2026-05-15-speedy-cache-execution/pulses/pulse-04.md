# Pulse 04: Cache-hit ledger trust

## Goal

Let generic fetch execution treat cache hits as verified when a caller supplies a
prior trusted FLETCH ledger and the current cached object still matches that
ledger entry.

## Outcome

- Added `FetchOptions::with_trusted_manifest` to pass prior
  `fletch.cache-manifest.v1` entries into fetch execution.
- Cache hits now match trusted entries by cache key, dataset ID, and source URL,
  then verify current bytes and hash before preserving verified status.
- Trusted cache hits preserve ledger fetched timestamp, fetch attempt count,
  retry count, and last retryable error.
- Tampered trusted cache objects fail with checksum mismatch instead of silently
  succeeding.
- Added `fletch fetch --trusted-manifest <path>` for CLI-led cache-hit trust.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for trusted cache-hit verification
- `git diff --check`
