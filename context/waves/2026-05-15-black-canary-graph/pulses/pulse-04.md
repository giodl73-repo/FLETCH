# Pulse 04: Dry-run flights

## Goal

Resolve registered fletches into graph-shaped flight previews without fetching,
merging, or activating data.

## Changes

- Added `fletch.flight.v1` types with ordered steps and embedded graph output.
- Added `dry_run_flight(...)` in `fletch-core` to walk registry declarations and
  report `would-fetch`, `adapter-required`, `metadata-only`, and
  `missing-fletch` actions.
- Added `fletch registry flight --file ... --fletch-id ...` in `fletch-cli`.
- Updated the Justice League villain-files mock client to write
  `mock-flight.json` before fetching.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- CLI smoke for `fletch registry flight`
- Mock client smoke with `mock-flight.json`
- `git diff --check`

## Status

Done.
