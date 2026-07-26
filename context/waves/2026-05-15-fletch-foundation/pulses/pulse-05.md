# Pulse 05: Consumer adapters scout

## Goal

Inventory the first migration candidates for FLETCH consumers without moving
domain-specific logic into `fletch-core`.

## Changes

- Added `docs/specs/consumer-adapter-scout.md`.
- Captured first fletch, partition, rollup, and quiver candidates for ICELINES,
  apportionment/BISECT, ROUTE, MDCROP, MDPATH, and MDLOOM.
- Tied each candidate back to the Justice League villain-files mock-client lab
  path.
- Updated README and the foundation spec to link the scout document.

## Validation

- Targeted local scout over available consumer repos.
- `git diff --check`

## Status

Done.
