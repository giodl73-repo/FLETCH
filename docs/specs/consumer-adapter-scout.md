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
| MDLOOM | Registry, flight, quiver, ledger, and tip views | Generated proof/status views over FLETCH state | Release/status packs | MDLOOM renders FLETCH contracts; FLETCH remains the source of truth. |

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
   threat query summaries, tips, and quiver contents through CROP/MDLOOM-shaped
   views.
9. **MAXIM / Source corpus**: the mock now fetches a CROP view recipe, a MDPORT
   guide pack, and MDLOOM table/block sidecars for a frontend-framework guide,
   then leaves CROP/MDPORT/MDLOOM-specific querying in the adapter layer.

## First migration slices

1. ROUTE: generic source orchestration is moved to FLETCH; ROUTE keeps
   geospatial scoring, validation, and product outputs.
2. Apportionment/BISECT: generic HTTP acquisition for TIGER, PL 94-171, school
   districts, EIA 861, LODES, and ACS housing is moved to FLETCH. BISECT keeps
   GitHub release adjacency, extraction, derived artifacts, done markers,
   `--force`, and redistricting/legal validation.
3. ICELINES: `fetch fletch-sources --gate` inventories source surfaces, stable
   roster/MoneyPuck source bytes, paged NHL stats report bytes,
   schedule-expanded Gamecenter bytes, player-set-expanded landing bytes, and
   season-window-expanded ESPN transaction bytes are acquired through FLETCH, and
   `fetch fletch-partitions --gate` maps
   leaders/player/compare, goalies, roster bios, MoneyPuck, career, and
   windowed game-line queries to partition and rollup IDs.
   `fetch fletch-quivers --gate` groups those partitions into query bootstrap
   and enrichment quiver handoff candidates. ICELINES keeps dynamic source
   expansion, parsing, snapshots, sealing, active pointers, freshness, locks,
   event-stream writes, and hockey-domain validation.
4. CROP: index `fletch.cache-manifest.v1`, `fletch.quiver.v1`, `fletch.graph.v1`,
   tips, and publish reports as evidence.
5. MDPATH/MDLOOM: attach stable references and rendered views to FLETCH outputs
   without treating generated docs as source data.

## Completed consumer handoffs

| Consumer | FLETCH-owned now | Adapter-owned / product-owned remains |
|---|---|---|
| ROUTE | Generic source orchestration and source handoff/gate reporting. | Route scoring, geospatial semantics, user-facing outputs. |
| BISECT | Generic HTTP source-byte acquisition under `data/.fletch` plus handoff/gate reporting. | Release adjacency, archive extraction, derived CSVs, done markers, local manifest overrides, legal/redistricting claims. |
| ICELINES | Handoff/gate reporting, roster, MoneyPuck, paged NHL stats, schedule-expanded Gamecenter, player-set-expanded landing, and season-window-expanded ESPN transaction source-byte acquisition, query partition/rollup handoff reporting, and query quiver handoff reporting. | Schedule/player-set/window expansion semantics, snapshots, parsing, sealing, active pointers, event streams, classifiers, and hockey semantics. |
| MAXIM | Generic cache acquisition and verification for CROP view recipes, MDPORT packs, and MDLOOM sidecars. | Source-corpus authoring, CROP query semantics, MDPORT metadata interpretation, MDLOOM document parsing, and guide-specific search behavior. |
