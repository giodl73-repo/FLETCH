# Pulse 02: MDLOOM document manifest

## Goal

Emit document-ready anchors over FLETCH contracts without making generated prose
or rendered documents the source of truth.

## Outcome

- Added `fletch.mdloom-docs.v1`.
- Emitted document IDs, titles, anchors, and source schema references from MDCROP
  index rows.
- Added `fletch publish mdloom-docs --mdcrop-index ...`.
- Kept MDLOOM backend/rendering choices outside `fletch-core`.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for MDLOOM docs JSON
- `git diff --check`
