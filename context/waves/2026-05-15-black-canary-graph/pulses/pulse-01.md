# Pulse 01: Manifest graph export

## Goal

Add `fletch.graph.v1` exports from cache manifests and let the Justice League
mock client attach adapter-owned graph edges.

## Changes

- Added `FletchGraph`, `GraphNode`, `GraphEdge`, `GraphNodeKind`, and
  `GraphEdgeKind` to `fletch-core`.
- Added manifest-to-graph helpers that export fletch, shaft, and ledger-entry
  nodes with `satisfied-by` and `documents` edges.
- Added `fletch graph export --manifest ...`.
- Updated the Justice League villain-files mock client to write `mock-graph.json`
  with adapter-owned `expands-to` and `rolls-up-to` edges.
- Created the Black Canary wave and made it the active wave.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- CLI smoke for `fletch graph export`
- Mock client smoke with graph output
- `git diff --check`

## Status

Done.
