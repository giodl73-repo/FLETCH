# Pulse 04: Quiver format

## Goal

Add the first Speedy II quiver slice: export verified cache objects into a
portable `fletch.quiver.v1` directory and import that quiver stage-first without
activating product views.

## Changes

- Added `fletch.quiver.v1` core types and directory layout:
  `quiver.json` plus referenced `objects/sha256/<cache-key>` files.
- Added `export_quiver` with verification before packaging.
- Added `import_quiver` that copies objects into
  `cache/staged/quivers/<safe-quiver-id>/` and verifies staged objects.
- Added `fletch quiver export` and `fletch quiver import` CLI commands.
- Updated the Justice League villain-files mock client to export/import a
  quiver as an offline bootstrap proof.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- CLI smoke for `fletch quiver export`
- CLI smoke for `fletch quiver import`
- Mock client smoke for stage-first quiver import
- `git diff --check`

## Status

Done.
