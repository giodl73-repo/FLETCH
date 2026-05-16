# Pulse 02: Registry definitions

## Goal

Add the first product-neutral `fletch.registry.v1` definition shape so products
can declare fletches, shafts, format metadata, and graph relationships before
fetching data.

## Changes

- Added `FletchRegistry`, `FletchDefinition`, `RegistryEdge`, and `DataFormat`.
- Added `fletch_registry(...)` and `graph_from_registry(...)` helpers in
  `fletch-core`.
- Added `fletch registry graph --file ...` in `fletch-cli`.
- Updated the Justice League villain-files mock client to write
  `mock-registry.json`, fetch from registry shafts, and build graph output from
  registry definitions plus cache state.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- CLI smoke for `fletch registry graph`
- Mock client smoke with registry and graph output
- `git diff --check`

## Status

Done.
