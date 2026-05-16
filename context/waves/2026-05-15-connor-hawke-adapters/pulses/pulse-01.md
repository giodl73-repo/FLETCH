# Pulse 01: Adapter source report

## Goal

Emit product-neutral source rows from a registry so adapters can prove what they
constructed without moving product rules into `fletch-core`.

## Planned outcome

- Add a named `fletch.adapter-sources.v1` contract.
- Report fletch IDs, source kind, URL, header count, and adapter-owned status.
- Add a read-only CLI command over `fletch.registry.v1`.
- Document that adapter semantics remain outside core.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for adapter source JSON
- `git diff --check`
