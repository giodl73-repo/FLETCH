# FLETCH — product integrator brief

**Time:** 10–20 minutes. **Goal:** decide how to consume FLETCH without importing
another product’s domain code.

## Consume path

1. Own expansion, locks, parsing, validation, snapshots, and active pointers in
   your product.
2. Point generic HTTP/file acquisition at FLETCH plan/fetch/batch helpers.
3. Persist a product-owned `fletch.cache-manifest.v1` beside your cache root.
4. Publish compact evidence with cache-index / index-gate helpers when you need a
   gate artifact.
5. Optionally export/import **quivers** for portable offline packs — still not
   the same as product activation.

## What already moved (snapshot from README)

| Consumer | FLETCH owns | Product still owns |
|---|---|---|
| ROUTE | Generic source orchestration, manifest R/W, batch upsert, cache-index gate helpers | Route scoring, geospatial interpretation, product outputs |
| BISECT | Generic HTTP acquisition for Census/TIGER/PL/EIA/LODES/ACS-class fetches | Release adjacency, extraction, derived CSVs, legal validation |
| ICELINES | Roster/MoneyPuck/NHL/ESPN-class acquisition + partition/quiver/gate mapping | NHL parsing, snapshots, sealing, active pointers, hockey validation |

Re-read the live README before citing migration completeness — this brief tracks
the documented posture, not a certification matrix.

## Safe language

- “Shared fetch/cache ledger and plan surface.”
- “Acquisition and verification, not activation.”
- “Product-neutral core; domain semantics stay in the consumer.”

## Unsafe language

- “Drop-in offline product.”
- “Replaces our ETL and data quality stack.”
- “All portfolio apps fully migrated.”

## Next docs

- [`../../SHOWCASE.md`](../../SHOWCASE.md)
- [`../specs/fletch-foundation.md`](../specs/fletch-foundation.md)
- README “Manifest-first consumer pattern”
