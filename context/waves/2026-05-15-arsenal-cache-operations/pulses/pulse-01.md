# Pulse 01: Cache summary report

## Goal

Add a compact, product-neutral aggregate report over existing manifest inspection
results so humans, CROP, PROOF, and CI can see cache health without parsing every
status row themselves.

## Outcome

- Added a `CacheSummary` report with object status counts, freshness counts, and
  expected/actual byte totals.
- Added `summarize_cache_manifest` over existing manifest inspection behavior.
- Added `fletch cache summary` with the same freshness policy inputs as
  `fletch cache status`.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for `fletch cache summary`
- `git diff --check`
