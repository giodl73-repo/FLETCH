# FLETCH

**Fetch, ledger, export, trust, cache, hash.**

FLETCH is a shared Rust fetch/cache layer for projects that need reproducible
data acquisition without each product reinventing download, cache, bundle,
manifest, verification, and offline behavior.

## Why FLETCH

- **One cache contract**: logical dataset IDs, source URLs, versions, hashes,
  byte counts, and verification status live in a shared manifest shape.
- **Product-neutral core**: BISECT, icelines, route, and CROP can all consume the
  same fetch/cache primitives without depending on each other.
- **Reproducible data pulls**: plans and cache keys make "what did this run
  fetch?" auditable.
- **Future offline/bundle path**: the workspace is prepared for bundle export,
  import, pruning, and stale/fresh reports.

Initial consumers:

- BISECT/apportionment: census, election, geography, and evidence datasets.
- icelines: NHL seasons, profiles, favorites, and bundled/offline data.
- route: geospatial/routing datasets, tiles, profiles, and on-demand pulls.
- CROP: optional indexing of FLETCH manifests and cached corpus metadata.

## Commands

```powershell
cargo run -p fletch-cli -- plan --dataset-id nhl:season:1993 --url https://example.test/1993.json
cargo run -p fletch-cli -- key --dataset-id route:tiles:demo --url https://example.test/tiles.zip
```

`fletch plan` emits `fletch.plan.v1`, a source plan that downstream products can
check into configs or generate from their own CLI commands. `fletch key` emits
the deterministic cache key for the logical dataset/source pair.

## Workspace

| Crate | Purpose |
|-------|---------|
| `fletch-core` | Fetch plans, cache policies, cache keys, and manifests. |
| `fletch-cli` | Generic command surface for plans, cache inspection, bundles, and future fetch execution. |

## Design rule

FLETCH stays product-neutral. Adapters can know about Census, NHL, or route data,
but the core cache contract must work for any source.

## Specs

- [`docs/specs/fletch-foundation.md`](docs/specs/fletch-foundation.md) defines
  the initial plan and cache-manifest contracts.
- `context/waves/` tracks implementation waves and pulse history.

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p fletch-cli -- plan --dataset-id nhl:season:1993 --url https://example.test/1993.json
```

## License

MIT. See [`LICENSE`](LICENSE).
