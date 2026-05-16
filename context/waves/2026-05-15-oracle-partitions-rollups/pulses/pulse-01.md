# Pulse 01: Partition state contract

## Goal

Emit durable partition rows from existing FLETCH manifest evidence so downstream
tools can index partition-like cache members without product-specific rules.

## Outcome

- Add a named `fletch.partition-state.v1` contract.
- Represent partition IDs, dataset IDs, cache keys, hashes, paths, and optional
  grouping metadata.
- Add a read-only CLI command that derives partition state from a manifest.
- Document the contract in the foundation spec.
- Keep partition IDs product-neutral; adapters own the meaning of seasons,
  years, districts, tiles, dates, or other domain partitions.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for partition state JSON
- `git diff --check`
