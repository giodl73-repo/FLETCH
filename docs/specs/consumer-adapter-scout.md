# Consumer Adapter Scout

Pulse 05 scouts initial migration candidates for FLETCH consumers. The goal is
not to move product logic into `fletch-core`; it is to identify the fletches,
shafts, partitions, quivers, and publishing surfaces each product can register.

## Adapter boundary

FLETCH owns:

- acquisition, cache keys, manifests, verification, freshness, quivers, and
  status;
- graph records that say which fletches depend on, derive from, contain, mirror,
  or supersede other fletches;
- stage-first import and export surfaces for offline bootstrap.

Consumers own:

- domain schemas and calculations;
- query engines and UI behavior;
- product-specific source discovery;
- conflict policy defaults when a domain has multiple valid authorities.

## Initial consumer candidates

| Consumer | First fletches | Partition/rollup model | Quiver fit | Adapter notes |
|---|---|---|---|---|
| ICELINES | NHL schedule, boxscore, play-by-play, bios, MoneyPuck, contracts, playoff data | Game/date/month/season partitions with season-type rollups and aliases like `nhl:season:current` | Season quivers and CI/offline bundles | Live NHL API should remain a write path that lands snapshots; queries should read verified partitions. |
| Apportionment/BISECT | Census API rows, TIGER/Line boundaries, NHGIS files, election inputs, generated adjacency graphs | Year/state/tract partitions with nationwide and region rollups | Year/state packs for reproducible runs | FLETCH can verify raw and normalized artifacts; graph/partition algorithms stay in product code. |
| ROUTE | HPMS, NBI, FARS, NHS/RITIS-derived inputs, generated route evidence tables | Year/state/route partitions with national rollups and route aliases | Evidence quivers for reproducible route scoring | FLETCH can stage official datasets and derived indexes; scoring remains ROUTE-owned. |
| CROP | FLETCH ledgers, quiver manifests, graph exports, generated docs/status | Corpus/status partitions by repo, view, or run | Corpus health packs | CROP indexes FLETCH state as evidence; it should not infer cache semantics from raw files. |
| MDPATH | Stable references to generated specs, status rows, tips, and evidence docs | Document/path/section-addressed references | Quivers can include published docs plus `md://` references | FLETCH outputs can carry stable local references without making generated Markdown authoritative. |
| PROOF | Registry, flight, quiver, ledger, and tip views | Generated proof/status views over FLETCH state | Release/status packs | PROOF renders FLETCH contracts; FLETCH remains the source of truth. |

## Justice League mock-client proving path

The `fletch-mock-client` crate is the lab harness for these adapter ideas:

1. **Green Arrow / Foundation**: local file shafts produce villain-file fletches.
2. **Speedy / Cache execution**: villain index, Darkseid casefile, and threat
   partitions are fetched and verified.
3. **Arsenal / Cache operations**: the mock verifies cached objects, reports
   freshness, and plans pruning for an orphaned trick-arrow object.
4. **Oracle / Partitions**: dated threat partitions roll up into yearly query
   summaries by year, city, and villain.
5. **Speedy II / Quivers**: the mock exports and stage-imports a
   `justice-league:villain-files:demo` quiver for offline bootstrap.
6. **Black Canary / Registry graph**: next, the mock should emit graph edges from
   the villain index to casefiles and threat partitions.
7. **Red Arrow / Merge**: next, the mock should stage aliases like
   `justice-league:threats:current-year`, preview conflicts for competing threat
   partitions, merge safe groups first, and leave unresolved groups visible for a
   later client decision.
8. **Overwatch / Publishers**: later, the mock should publish local status,
   threat query summaries, tips, and quiver contents through CROP/PROOF-shaped
   views.

## First migration slices

1. ICELINES: register NHL schedule and boxscore partitions first, then season
   quivers.
2. Apportionment/BISECT: register Census/TIGER year-state fletches first, then
   generated adjacency graph fletches.
3. ROUTE: register HPMS/NBI year-state fletches first, then route evidence
   rollups.
4. CROP: index `fletch.cache-manifest.v1`, `fletch.quiver.v1`, and future
   `fletch.graph.v1` outputs.
5. MDPATH/PROOF: attach stable references and rendered views to FLETCH outputs
   without treating generated docs as source data.
