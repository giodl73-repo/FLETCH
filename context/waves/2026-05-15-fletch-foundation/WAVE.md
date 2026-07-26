# Wave: FLETCH Foundation

## Goal

Create a neutral Rust fetch/cache workspace that BISECT, icelines, route, and
CROP can adopt without depending on each other.

## Thesis

Fetching, caching, bundling, verification, and offline data access are shared
infrastructure. The first FLETCH wave proves the common contract before any
domain adapter migrates.

## Vocabulary

- **Fletch**: logical fetch/cache unit.
- **Shaft**: concrete carrier or locator for a fletch.
- **Flight**: resolved execution plan over one or more fletches.
- **Quiver**: named group or portable bundle of fletches.
- **Ledger**: manifest/status record for cached fletches.
- **Tip**: lightweight preview, sample, summary, or index for a shaft/fletch.

The foundation should evolve toward graph-shaped fletch relationships:
dependencies, expansion, bundle satisfaction, and activation, while keeping all
product meaning in adapters.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Workspace foundation | done | Added workspace, core plan/manifest contracts, CLI plan/key commands, specs, and wave scaffolding. |
| 02 | Cache execution | done | Added HTTP/file fetch execution with temp-file promotion, SHA-256 ledger entries, optional checksum verification, bandwidth limits, and `fletch fetch`. |
| 03 | Cache operations | done | Added manifest-led cache list, verify, status, and prune-plan operations with CLI commands. |
| 04 | Quiver format | done | Added `fletch.quiver.v1` directory export/import, stage-first verification, CLI commands, and mock-client offline bootstrap. |
| 05 | Consumer adapters scout | done | Documented initial ICELINES, apportionment/BISECT, ROUTE, CROP, MDPATH, and MDLOOM migration slices. |

## Success criteria

- FLETCH has its own Rust workspace and git repo.
- `fletch-core` exposes product-neutral plan, policy, key, and manifest types.
- `fletch-cli` can emit a plan and deterministic cache key.
- Docs explain BISECT, icelines, route, and CROP onboarding paths.
- Wave and pulse scaffolding exists for follow-up work.
- `cargo fmt`, `cargo test --workspace`, CLI smokes, and `git diff --check`
  pass.
