# Pulse 03: File shaft path hardening

## Goal

Tighten generic local file shaft handling while preserving product-neutral fetch
execution.

## Changes

- Added explicit `InvalidFileSource` errors for empty file shaft URLs/paths.
- Normalized common `file://` forms, including Windows-style
  `file:///C:/...` and `file://localhost/C:/...` paths.
- Kept file shafts as generic local byte sources; no product-specific source
  rules were added.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- CLI smoke for `fletch fetch --source-kind file --url file:///...`
- Mock client smoke
- `git diff --check`

## Status

Done.
