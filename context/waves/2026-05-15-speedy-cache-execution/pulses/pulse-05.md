# Pulse 05: Offline stale diagnostics

## Goal

Make offline fetch failures actionable by distinguishing true missing cache
objects from cache objects that exist but cannot be used because freshness policy
or `--force` requires a live refresh.

## Outcome

- Added `OfflineCacheStale` for existing cached objects that are stale or
  explicitly bypassed while offline mode prevents live acquisition.
- Preserved `OfflineCacheMiss` for true missing cache objects.
- Kept the behavior product-neutral: FLETCH reports generic cache availability
  and freshness state, while consumers decide how to message bootstrap or
  activation workflows.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for offline stale diagnostics
- `git diff --check`
