# Pulse 01: Manifest and partition selectors

## Goal

Prove FLETCH can project cache-index and active-partition rows into SLICE for
selection while preserving FLETCH-owned folding and policy.

## Changes

- Add a dev-only `slice-core` dependency to `fletch-core`.
- Add a cache-index selector test.
- Add an active-partition selector test that derives quiver candidate dataset IDs
  after SLICE selection.
- Document the adapter boundary in `docs/specs/slice-selectors.md`.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `git diff --check`

## Status

Done.
