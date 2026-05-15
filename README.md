# FLETCH

**Fetch, ledger, export, trust, cache, hash.**

FLETCH is a shared Rust fetch/cache layer for projects that need reproducible
data acquisition without each product reinventing download, cache, bundle,
manifest, verification, and offline behavior.

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

## Workspace

| Crate | Purpose |
|-------|---------|
| `fletch-core` | Fetch plans, cache policies, cache keys, and manifests. |
| `fletch-cli` | Generic command surface for plans, cache inspection, bundles, and future fetch execution. |

## Design rule

FLETCH stays product-neutral. Adapters can know about Census, NHL, or route data,
but the core cache contract must work for any source.
