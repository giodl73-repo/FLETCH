# Pulse 02: Manifest file helpers

## Goal

Give consumers reusable product-neutral helpers for reading and writing
`fletch.cache-manifest.v1` JSON files so they do not have to duplicate schema and
hash validation around durable cache ledgers.

## Outcome

- Added cache manifest JSON read/write helpers in `fletch-core`.
- Added explicit manifest schema validation for file reads.
- Reused the helpers from CLI manifest reads and fetch manifest writes.
- Kept helpers limited to JSON file persistence; they do not fetch sources,
  inspect object bytes, run a daemon, or activate product data.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for manifest write/read through cache index
- `git diff --check`
