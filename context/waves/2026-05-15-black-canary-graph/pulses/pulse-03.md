# Pulse 03: Role-review hardening

## Goal

Address the first role-review findings that affect graph trustworthiness before
adding dry-run flights.

## Changes

- Cache-hit reuse now preserves a nonzero file timestamp and only marks the
  returned entry as verified when the caller supplied an expected hash.
- Temp-file promotion no longer deletes an existing object before attempting to
  promote the new object.
- Quiver import validates `quiver.json` entries, verifies source bytes, copies
  into a temporary stage, verifies the staged manifest, then promotes the staged
  directory.
- Manifest graph export can use registry node-kind hints so cached partitions
  and rollups keep the same node identity as registry graph export.
- The mock client now merges registry and manifest graph views without duplicate
  logical partition nodes.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- Focused CLI and mock-client smokes
- `git diff --check`

## Status

Done.
