# Pulse 03: Offline report command

## Goal

Give no-live and bootstrap workflows a manifest-led readiness report that says
what is usable offline and what still blocks execution.

## Outcome

- Added `fletch.cache-offline.v1`.
- Added `CacheOfflineReport` with ready, missing, stale, and blocked counts plus
  underlying status rows.
- Added `fletch cache offline-report` with the same freshness policy inputs as
  `cache status`.
- Kept the operation read-only and manifest-led: no source access and no cache
  mutation.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for `fletch cache offline-report`
- `git diff --check`
