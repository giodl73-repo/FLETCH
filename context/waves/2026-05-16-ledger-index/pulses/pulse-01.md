# Pulse 01: Cache index report

## Goal

Provide a compact, read-only index over cache manifest entries for large-ledger
lookup and publisher workflows.

## Outcome

- Added `fletch.cache-index.v1`.
- Added compact index rows with dataset ID, version, cache key, hash, relative
  path, byte count, and verified flag.
- Added `fletch cache index --manifest ...`.
- Kept cache manifests as the authoritative ledger and left verification/status
  reports responsible for inspecting cached bytes.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for cache index JSON
- `git diff --check`
