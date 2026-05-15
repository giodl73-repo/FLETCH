# Wave: FLETCH Foundation

## Goal

Create a neutral Rust fetch/cache workspace that BISECT, icelines, route, and
CROP can adopt without depending on each other.

## Thesis

Fetching, caching, bundling, verification, and offline data access are shared
infrastructure. The first FLETCH wave proves the common contract before any
domain adapter migrates.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Workspace foundation | done | Added workspace, core plan/manifest contracts, CLI plan/key commands, specs, and wave scaffolding. |
| 02 | Cache execution | pending | Add HTTP/file fetch execution with temp-file promotion and checksum verification. |
| 03 | Cache operations | pending | Add cache listing, verification, stale/fresh status, and prune planning. |
| 04 | Bundle format | pending | Add export/import cache bundle contract. |
| 05 | Consumer adapters scout | pending | Inventory BISECT, icelines, and route migration candidates. |

## Success criteria

- FLETCH has its own Rust workspace and git repo.
- `fletch-core` exposes product-neutral plan, policy, key, and manifest types.
- `fletch-cli` can emit a plan and deterministic cache key.
- Docs explain BISECT, icelines, route, and CROP onboarding paths.
- Wave and pulse scaffolding exists for follow-up work.
- `cargo fmt`, `cargo test --workspace`, CLI smokes, and `git diff --check`
  pass.
