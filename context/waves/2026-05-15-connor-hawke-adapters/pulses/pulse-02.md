# Pulse 02: Registry validation report

## Goal

Report registry structure problems and adapter-owned source counts as data
without fetching, expanding, or interpreting adapter sources.

## Outcome

- Added `fletch.registry-validation.v1`.
- Reported invalid schema, duplicate fletch IDs, missing shafts, source counts,
  and adapter-owned source counts.
- Added `fletch registry validate --file ...`.
- Kept validation read-only and product-neutral.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for registry validation JSON
- `git diff --check`
