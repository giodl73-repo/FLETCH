# Pulse 01: Workspace foundation

## Goal

Make FLETCH real as a local Rust workspace with enough contract surface for
BISECT, icelines, route, and CROP to evaluate onboarding.

## Changes

- Added `fletch-core` and `fletch-cli`.
- Added `fletch.plan.v1` and `fletch.cache-manifest.v1` structs.
- Added deterministic cache keys.
- Added `fletch plan` and `fletch key`.
- Added README, product plan, foundation spec, and wave scaffolding.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- CLI smoke for `fletch plan`
- CLI smoke for `fletch key`
- `git diff --check`

## Status

Done.
