# Pulse 02: Quiver verify report

## Goal

Report per-member quiver verification state before import, merge preview, or
activation.

## Outcome

- Added `fletch.quiver-verify.v1`.
- Reported verified, missing, and hash-mismatch bundle members as data.
- Added `fletch quiver verify --quiver-dir ...`.
- Kept verification read-only: no import, promotion, or activation.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for quiver verify JSON
- `git diff --check`
