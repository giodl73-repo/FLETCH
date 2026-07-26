# Pulse 06: CROP/MDLOOM publish scout

## Goal

Emit a product-neutral machine-readable status view that CROP can index and
MDLOOM or other backends can render without making generated documents the source
of truth.

## Changes

- Added `fletch.publish.v1` report type combining graph, cache status, and tips.
- Added `publish_report_from_manifest(...)` in `fletch-core`.
- Added `fletch publish from-manifest --manifest ...`.
- Updated the Justice League villain-files mock client to write
  `mock-publish.json` and report publish status count.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- CLI smoke for `fletch publish from-manifest`
- Mock client smoke with `mock-publish.json`
- `git diff --check`

## Status

Done.
