# FLETCH Phases and Waves

FLETCH work is organized into Arrow-family phases, then small waves and pulses.

## Delivery goal

Deliver every Arrow phase in this file, from Green Arrow through Overwatch, as a
coherent shared fetch/cache substrate for ICELINES, apportionment/BISECT, ROUTE,
CROP, MDPATH, and PROOF.

Each wave must close with a `.roles` review before the active wave advances. The
review is part of delivery, not an optional retrospective: role findings either
become follow-up pulses in the current wave or explicit non-goals/deferred risks
in the next wave plan.

## Phase set

1. **Green Arrow - Foundation**: product-neutral contracts, workspace, cache
   keys, plan/key/fetch CLI, and first ledger shape. Mock client adds the
   Justice League villain-files app as a downstream consumer with local file
   shafts.
2. **Black Canary - Registry graph**: `fletch.graph.v1`, graph-shaped registry,
   dependency/expansion/satisfaction/data-link edges, and dry-run flights. Mock
   client adds graph edges from villain index to casefiles and threat
   partitions.
3. **Speedy - Cache execution**: fast generic shaft acquisition with temp-file
   promotion, verification, retry, bandwidth, and path safety. Mock client
   fetches villain index, Darkseid casefile, and dated threat partition files.
4. **Arsenal - Cache operations**: listing, verification, prune planning,
   stale/fresh reporting, offline status, and ledger-backed decisions. Mock
   client verifies all villain-file objects and proves prune planning with an
   orphaned trick-arrow object.
5. **Red Arrow - Merge and aliases**: `fletch.merge.v1`, active views, labels,
   pins, rollback, merge previews, conflict groups, and future `pull` as fetch
   plus merge. Mock client adds staged aliases such as
   `justice-league:threats:current-year`, conflict previews for competing threat
   files, and staged safe-group merge examples.
6. **Oracle - Partitions and rollups**: durable partitions, rollup edges,
   invalidation/folding metadata, and query-facing active partition sets. Mock
   client adds dated threat partitions, year rollups, and when/where/how threat
   queries.
7. **Speedy II - Quivers**: `fletch.quiver.v1`, portable packages, stage-first
   imports, verification, tips, ledgers, graph edges, and merge-ready bundles.
   Mock client exports/imports a Justice League villain-files quiver for offline
   bootstrap.
8. **Connor Hawke - Adapters**: product adapters for Census/apportionment,
   NHL/icelines, route/geodata, and generic archive sources. Mock client becomes
   the adapter harness that proves consumer code stays outside `fletch-core`.
9. **Overwatch - Publishers**: CROP graph/status indexing and PROOF-rendered
   local docs, dashboards, or backend views. Mock client publishes villain-file
   status, threat query summaries, tips, and quiver contents as local views.

## Active wave

- `2026-05-15-overwatch-publishers`

## Protocol

1. Read the active wave `WAVE.md`.
2. Execute the next pulse in `pulses/`.
3. Keep product logic out of `fletch-core`; use adapters for domain-specific
   sources.
4. Validate with `cargo fmt`, `cargo test --workspace`, focused CLI smokes, and
   `git diff --check`.
5. Update the wave and pulse docs before committing.
6. Before closing a wave, run `.roles` review using:
   - parliament infrastructure voices,
   - editorial quality gates,
   - stakeholder consumer lenses,
   - panel-reviewer expert checks.
7. Advance the active wave only after the `.roles` review is documented and any
   blocking findings have been resolved or explicitly deferred.
