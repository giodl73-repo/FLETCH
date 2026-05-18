# Pulse 07: Registry web browser

## Goal

Provide a local browser experience for searching a `fletch.registry-index.v1`
file, inspecting result rows, and navigating tags, metadata, and source URLs
without requiring a separate web app.

## Changes

- Added `fletch registry web --index <index.json>`.
- Added direct launch with `fletch registry web --file <registry.json> --follow`
  so users do not need to build an index file first.
- Serves a local HTML UI at `http://127.0.0.1:7878/`.
- Added JSON endpoints:
  - `/api/summary`
  - `/api/search?text=...&tag=...&metadata=key=value`
  - `/api/row?registry_id=...&fletch_id=...`
- Added an integration test that launches the actual CLI server and verifies
  HTML, summary, search, and detail responses return registry data.
- Added an integration test for direct `--file` launch.
- Added multi-term text matching and comma-separated web tag/metadata filters.
- Added `/api/facets` and UI facet chips for registry sections by owner repo,
  domain, asset kind, fetch policy, and tags.
- Added `/api/source` and UI **Load preview** buttons for bounded source data.
- Annotated followed GitHub registry rows with their raw repo base URL so relative
  source paths can be previewed without cloning the domain repo.
- Source previews now include line-numbered sections, previous/next line
  navigation, and compact JSON outlines.
- Added `--open` to `fletch registry web` so a successful bind can launch the
  local registry browser in the default browser.

## Validation

- `cargo fmt`
- `cargo test --workspace --quiet`
- `cargo run -p fletch-cli -- registry web --help`
- Full MUNDUS followed-index smoke test with the web server:
  - 243 registries
  - 11,381 rows
  - `storm seed` search returns STORM seed rows
  - source preview resolves `storm.foundation.seed-storm` to raw GitHub and loads
    2,710 bytes
  - facet smoke confirms STORM owner repo section and source line/outline data
- `git diff --check`

## Status

Done.
