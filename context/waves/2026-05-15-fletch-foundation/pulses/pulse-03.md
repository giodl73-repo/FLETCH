# Pulse 03: Cache operations

## Goal

Add the first Arsenal cache operation slice: FLETCH can inspect existing
`fletch.cache-manifest.v1` ledgers, verify cached objects, report fresh/stale
state, and plan safe pruning without deleting data.

## Changes

- Added core cache status types for object verification and freshness reporting.
- Added manifest-led `cache_list`, `inspect_cache_manifest`, and
  `plan_cache_prune` primitives.
- Added path-safety validation for manifest relative cache paths.
- Added `fletch cache list`, `fletch cache verify`, `fletch cache status`, and
  `fletch cache prune` CLI commands.
- Kept prune plan-only so destructive deletion remains explicit future work.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- CLI smoke for `fletch plan`
- CLI smoke for `fletch key`
- CLI smoke for `fletch fetch --source-kind file`
- CLI smoke for `fletch cache list`
- CLI smoke for `fletch cache verify`
- CLI smoke for `fletch cache status`
- CLI smoke for `fletch cache prune`
- `git diff --check`

## Status

Done.
