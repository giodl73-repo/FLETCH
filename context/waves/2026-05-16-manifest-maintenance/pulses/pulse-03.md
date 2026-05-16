# Pulse 03: Consumer smoke docs

## Goal

Document the manifest-first consumer workflow and back it with the mock client so
consumer teams have an executable example for durable FLETCH cache ledgers and
read-only cache-index handoffs.

## Outcome

- Added README guidance for manifest-first consumers.
- Updated the mock client to persist and reload its manifest through shared
  FLETCH helpers.
- Added mock-client assertions that the persisted manifest feeds cache-index
  evidence.
- Kept consumer-owned expansion and query semantics in the mock adapter layer,
  not in `fletch-core`.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for manifest-first cache-index flow
- `git diff --check`
