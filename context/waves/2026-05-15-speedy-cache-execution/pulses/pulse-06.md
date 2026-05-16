# Pulse 06: Header-aware shafts

## Goal

Make generic HTTP shaft headers first-class enough for plans, fetches, and cache
identity without introducing product-specific adapter behavior.

## Outcome

- Included `SourceSpec.headers` in deterministic cache keys so same-URL shafts
  with different generic headers do not collide.
- Added repeatable `--header name=value` flags to `fletch plan`, `fletch key`,
  and `fletch fetch`.
- Added CLI validation for malformed header flags before fetch execution.
- Added core coverage that HTTP fetch sends configured headers.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for header-bearing plans and keys
- `git diff --check`
