# Pulse 03: Quiver graph edges

## Goal

Expose quiver-to-member graph edges so MDCROP and MDLOOM can index portable bundles
without importing or reading cache objects.

## Outcome

- Added quiver graph export over `fletch.quiver.v1`.
- Emitted quiver, fletch, and ledger-entry nodes.
- Emitted `contains` edges from quiver to member fletches and `documents` edges
  from ledger entries to fletches.
- Added `fletch quiver graph --quiver ...`.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for quiver graph JSON
- `git diff --check`
