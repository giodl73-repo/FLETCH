# Pulse 04: Publisher bundle report

## Goal

Summarize status, graph, tips, quiver, URL, and adapter publisher views for
downstream CROP/MDLOOM backends without replacing authoritative machine
contracts.

## Outcome

- Added `fletch.publisher-bundle.v1`.
- Summarized CROP row counts, MDLOOM document counts, local URL counts, and
  optional quiver/adapter counts.
- Added `fletch publish bundle --crop-index ... --mdloom-docs ... --local-url-map ...`.
- Kept publisher bundle output derived and read-only.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for publisher bundle JSON
- `git diff --check`
