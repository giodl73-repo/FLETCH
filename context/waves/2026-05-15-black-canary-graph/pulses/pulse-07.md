# Pulse 07: Registry web browser

## Goal

Provide a local browser experience for searching a `fletch.registry-index.v1`
file, inspecting result rows, and navigating tags, metadata, and source URLs
without requiring a separate web app.

## Changes

- Added `fletch registry web --index <index.json>`.
- Serves a local HTML UI at `http://127.0.0.1:7878/`.
- Added JSON endpoints:
  - `/api/summary`
  - `/api/search?text=...&tag=...&metadata=key=value`
  - `/api/row?registry_id=...&fletch_id=...`
- Added an integration test that launches the actual CLI server and verifies
  HTML, summary, search, and detail responses return registry data.

## Validation

- `cargo fmt`
- `cargo test --workspace --quiet`
- Full MUNDUS followed-index smoke test with the web server:
  - 243 registries
  - 11,381 rows
  - `storm-foundation` search returns MUNDUS and STORM rows
- `git diff --check`

## Status

Done.
