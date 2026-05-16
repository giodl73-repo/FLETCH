# Pulse 04: Adapter handoff report

## Goal

Summarize adapter-owned registry, source, graph, and flight inputs for downstream
tools without moving product-specific adapter behavior into `fletch-core`.

## Outcome

- Added `fletch.adapter-handoff.v1`.
- Reported validation status, fletch/source counts, adapter source counts, graph
  size, flight step count, and validation finding count.
- Added `fletch registry handoff --file ...`.
- Kept detailed registry, graph, and flight contracts as source of truth.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for adapter handoff JSON
- `git diff --check`
