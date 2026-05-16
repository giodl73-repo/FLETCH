# Pulse 08: Saved plan validation

## Goal

Prevent malformed or stale saved plans from reaching cache lookup or live source
execution.

## Outcome

- Added `validate_fetch_plan` for product-neutral `fletch.plan.v1` validation.
- `fetch_to_cache` now validates schema version, dataset ID, and source URL
  before cache-key derivation, cache hits, offline checks, or source access.
- Added `InvalidPlanSchema` so callers can distinguish unsupported saved-plan
  versions from missing required fields.
- Documented that generated and checked-in plans are executable only when they
  match the current generic plan contract.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for invalid saved-plan rejection
- `git diff --check`
