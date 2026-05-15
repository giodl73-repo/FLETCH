# FLETCH Foundation Spec

## Goal

Create a neutral fetch/cache substrate that BISECT, icelines, route, and other
Rust repos can consume without depending on one another.

## Core contracts

### `fletch.plan.v1`

Describes intent to obtain a dataset:

- `dataset_id`: logical id, e.g. `nhl:season:1993`.
- `version`: optional source or domain version.
- `source`: source kind, URL, and optional headers.
- `cache_policy`: freshness, offline, and resumable behavior.
- `tags` and `metadata`: product-owned classification.

### `fletch.cache-manifest.v1`

Records cached artifacts:

- source URL and logical dataset id,
- deterministic cache key,
- relative cache path,
- content hash,
- byte count,
- fetched timestamp,
- verification status.

## Initial CLI

```powershell
fletch plan --dataset-id nhl:season:1993 --url https://example.test/1993.json
fletch key  --dataset-id route:tiles:demo --url https://example.test/tiles.zip
```

## Onboarding targets

| Repo | Initial FLETCH fit |
|------|--------------------|
| BISECT/apportionment | Census/geography/election source plans and cache manifests. |
| icelines | 38-season NHL source plans, profile/favorite cache bundles, offline mode. |
| route | Geodata/routing source plans, bundleable local caches, on-demand pulls. |
| CROP | Index and status over FLETCH manifests and cache docs. |

## Later extraction

RLINE should become the neutral home for reusable `r*` graph/stat/context
kernels. FLETCH should not wait on RLINE; it starts independent.
