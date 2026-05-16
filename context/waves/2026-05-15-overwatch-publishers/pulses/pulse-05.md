# Pulse 05: Publisher slices

## Goal

Bound large local publisher outputs without changing authoritative machine
contracts or embedding consumer-specific dashboards in FLETCH.

## Outcome

- Added read-only slice helpers for `fletch.crop-index.v1`,
  `fletch.proof-docs.v1`, and `fletch.local-url-map.v1`.
- Added `--offset` and `--limit` to CROP, PROOF, and local URL publisher
  commands.
- Added `--row-type` filtering to CROP index publisher output for focused
  cache-status, graph, or tip views.
- Closed the Overwatch performance follow-up from the `.roles` review while
  keeping generated publisher artifacts derived from source contracts.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for sliced publisher JSON
- `git diff --check`
