# Pulse 05: Registry-backed index gate

## Goal

Make the shared cache-index gate easier for registry-first consumers by letting
the CLI derive expected dataset IDs from one or more `fletch.registry.v1` files.

## Implementation

- Added `--expected-registry FILE` to `fletch cache index-gate`.
- Merges repeated explicit `--expected-dataset-id` values with generic
  HTTP/file fletch IDs found in supplied registries.
- Deduplicates and sorts expected IDs before building the gate policy.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- Focused CLI smoke with `fletch-mock-client` output and
  `fletch cache index-gate --expected-registry ... --require-all-expected --gate`
- `git diff --check`

## Non-goals

- Registry metadata remains advisory for this gate; FLETCH does not interpret
  product-specific activation rules, fetch groups, source families, or snapshots.
