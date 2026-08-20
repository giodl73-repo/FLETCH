# Pulse 02: PROOF document manifest

## Goal

Emit document-ready anchors over FLETCH contracts without making generated prose
or rendered documents the source of truth.

## Outcome

- Added `fletch.proof-docs.v1`.
- Emitted document IDs, titles, anchors, and source schema references from MDCROP
  index rows.
- Added `fletch publish proof-docs --mdcrop-index ...`.
- Kept PROOF backend/rendering choices outside `fletch-core`.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for PROOF docs JSON
- `git diff --check`
