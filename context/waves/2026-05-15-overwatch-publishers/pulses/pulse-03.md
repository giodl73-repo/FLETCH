# Pulse 03: Local URL map

## Goal

Map schemas and generated document rows to stable local URLs and anchors for
MDLOOM/MDCROP views without making those views authoritative.

## Outcome

- Added `fletch.local-url-map.v1`.
- Mapped MDLOOM document IDs and anchors to local paths or URL prefixes.
- Added `fletch publish local-url-map --mdloom-docs ...`.
- Preserved source schema references for every mapped URL.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for local URL map JSON
- `git diff --check`
