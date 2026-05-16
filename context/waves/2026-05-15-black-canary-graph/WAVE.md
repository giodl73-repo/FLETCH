# Wave: Black Canary Registry Graph

## Goal

Add product-neutral graph exports so FLETCH state can be inspected by CROP,
rendered by PROOF, and extended by consumer adapters without moving domain logic
into `fletch-core`.

## Affected crates and consumers

- `fletch-core`: graph node/edge contracts and manifest/quiver graph helpers.
- `fletch-cli`: graph export command.
- `fletch-mock-client`: Justice League villain-files graph proof.
- Consumers: ICELINES, apportionment/BISECT, ROUTE, CROP, MDPATH, and PROOF.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Manifest graph export | done | Added `fletch.graph.v1` nodes/edges from manifests, CLI export, and mock-client adapter edges. |
| 02 | Registry definitions | pending | Add first fletch registry definition shape with dependency and format metadata. |
| 03 | Dry-run flights | pending | Resolve registered fletches into graph-shaped flight plans without fetching. |
| 04 | Tips | pending | Add lightweight structured previews for cached artifacts. |
| 05 | CROP/PROOF publish scout | pending | Emit graph/status views ready for CROP indexing and PROOF rendering. |

## Validation expectations

- Every pulse runs `cargo fmt`, `cargo test --workspace`, focused CLI smokes, and
  `git diff --check` when code changes.
- Documentation-only pulses run `git diff --check`.
- Mock-client pulses must keep domain query logic outside `fletch-core`.

## Non-goals

- FLETCH does not compute product metrics or threat/hockey/route/census stats.
- FLETCH does not activate aliases as part of graph export.
- FLETCH graph export does not replace product registries or CROP indexes; it
  provides stable typed state they can consume.
