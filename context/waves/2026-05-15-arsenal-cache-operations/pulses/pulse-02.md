# Pulse 02: Verify report contract

## Goal

Give cache verification a named machine contract that CROP, MDLOOM, CI, and
humans can consume without guessing what an anonymous status array means.

## Outcome

- Added `fletch.cache-verify.v1`.
- Added `CacheVerifyReport` with generated-by metadata, cache root, aggregate
  summary, and per-entry status rows.
- Updated `fletch cache verify` to emit the named report.
- Kept verification manifest-led and read-only: no live fetch and no mutation.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for `fletch cache verify`
- `git diff --check`
