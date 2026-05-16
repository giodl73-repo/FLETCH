# Pulse 03: Archive expansion preview

## Goal

Preview one archive/source fletch expanding into many child fletches without
extracting archives or interpreting product-specific archive contents.

## Outcome

- Added `fletch.archive-expansion-preview.v1`.
- Reported child fletches from registry `expands-to` edges.
- Reported missing child declarations as data.
- Added `fletch registry archive-preview --file ... --archive-fletch-id ...`.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for archive expansion preview JSON
- `git diff --check`
