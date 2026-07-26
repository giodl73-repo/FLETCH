# Pulse 05: Tips

## Goal

Add a product-neutral preview surface so CLIs, CROP, MDLOOM, and adapters can
inspect cached artifacts without loading full product semantics.

## Changes

- Added `fletch.tip.v1` types for bounded preview metadata.
- Added `tips_from_manifest(...)` in `fletch-core`.
- Added `fletch tip from-manifest --manifest ... --max-bytes ...`.
- Updated the Justice League villain-files mock client to write
  `mock-tips.json` and report tip counts.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- CLI smoke for `fletch tip from-manifest`
- Mock client smoke with `mock-tips.json`
- `git diff --check`

## Status

Done.
