# FLETCH Product Plan

## Thesis

Rust projects in this workspace repeatedly need the same data plumbing: fetch
remote sources, cache them locally, verify hashes, pin versions, bundle caches,
profile downloads, and run offline. FLETCH centralizes those mechanics behind a
small product-neutral contract.

FLETCH's product language is intentionally concrete:

- A **fletch** is a logical cached unit a product registers.
- A **shaft** is the concrete carrier or locator for that unit.
- A **flight** is the resolved fetch/verify/skip/expand execution plan.
- A **quiver** is a named install group or portable bundle.
- The **ledger** records what exists, what shaft produced it, and whether it is
  verified, fresh, stale, missing, or activated.
- A **tip** is a small peek, sample, summary, or index for a shaft/fletch so
  tools can inspect data before fully loading it.
- A **partition** is a durable data slice, often dated or scoped.
- A **rollup** is a logical aggregate over partitions.
- An **alias** is a mutable front door pointing at a partition, rollup, or active
  view.

In prose, **source** still means provenance or authority, such as NHL API,
Census, FHWA, MoneyPuck, or a generated PROOF backend. A **shaft** is the
specific URL, path, release asset, generated artifact handle, or adapter handle
that carries bytes for a fletch.

The registry is graph-shaped. A fletch may depend on other fletches, a fetch may
expand into newly discovered fletches, and one shaft or quiver may satisfy many
fletches after extraction or adapter processing.

FLETCH should prefer immutable/versioned fletches for durable data and mutable
aliases for names like `current`, `today`, `latest`, or `preferred`. For example,
`nhl:schedule:today` is a front door pointing to
`nhl:schedule:date:2026-05-15`; merge/activation changes the pointer, not the
historical partition.

Partitions let products accumulate detail while querying at higher levels.
ICELINES can keep boxscore/game partitions for the current season, roll them up
by date, month, season, player, team, or era, and let `icelines-query` fold over
the active partition set without caring which individual cache files were fetched.
This follows the same broad idea as managed analytical partitions: physical
partitions stay small and verifiable, while queries see a logical model.

FLETCH also records links between data, not only links between fetch actions.
The registry and ledger should be directly graph-exportable for MDCROP: fletches,
shafts, quivers, flights, and generated docs become nodes; relationships become
typed edges. A cached fletch can be derived from, supersede, contain, cite,
mirror, normalize, or document another fletch. Those relationships make it
possible to explain that an icelines boxscore was discovered from a schedule, a
route CSV was extracted from a ZIP shaft, or a BISECT analysis input was
normalized from raw Census data.

Data format is explicit but optional. By default, FLETCH treats a fletch as
opaque bytes with a hash and byte count. Registries and adapters can add format
options such as media type, compression, container, schema version, record shape,
or preferred local representation. FLETCH records and negotiates those options;
products interpret the domain meaning.

## Role review principles

- **Cache correctness**: identity, freshness, atomic promotion, and verified
  reuse are core invariants, not adapter details.
- **Provenance first**: every ledger row needs a traceable shaft, hash, byte
  count, timestamp, and verification state.
- **Linked data**: ledgers should capture data relationships such as derived
  from, contains, supersedes, mirrors, cites, and normalized from.
- **MDCROP-ready graph**: every fletch/shaft/quiver/flight relationship should be
  expressible as typed nodes and edges so MDCROP can mdcrop, index, and explain data
  provenance without reverse-engineering cache files.
- **Format clarity**: opaque bytes are the safe default; declared format options
  allow adapters to choose JSON, CSV, ZIP, Parquet, SQLite, PROOF output, or
  other representations without hard-coding product semantics.
- **Offline by design**: flights must explain what works without live network
  access and what quiver or fletch is missing.
- **Adapter boundary**: FLETCH registers, resolves, fetches, verifies, bundles,
  and reports; products interpret NHL, Census, route, or corpus meaning.
- **Fast common path**: verified cache hits, dry-run planning, and ledger status
  should be cheap enough for every consumer to call routinely.
- **Publishable state**: MDCROP can index ledgers, MDPATH can address generated
  docs, and PROOF can render local Markdown/backend views from contracts.
- **Data tips**: shafts and fletches can expose structured previews such as CSV
  headers, JSON fields, ZIP member indexes, SQLite table lists, or generated
  PROOF/MDCROP status snippets without forcing product-specific loads.
- **Partition safety**: fetch adds or verifies candidate partitions; merge
  updates aliases, active views, and rollups through an auditable transaction.
- **Conflict grouping**: equivalent conflicts can be labeled as choices, e.g.
  A/B/C sources, then resolved in bulk with explicit overrides for exceptions.
- **Merge preview**: clients can ask FLETCH what a merge would activate,
  supersede, label, or conflict with before any active view changes. That lets a
  consumer merge safe groups first, resolve conflicts in stages, and re-preview
  the remaining candidates.

## Phases

FLETCH phases use Green Arrow and friends as memorable names, while each phase
keeps a plain technical contract:

1. **Green Arrow - Foundation**: Rust workspace, fletch/shaft flight schema,
   ledger schema, deterministic cache keys, CLI plan/key/fetch commands.
   Mock client: add the Justice League villain-files app as a downstream
   consumer with local file shafts. **Active.**
2. **Black Canary - Registry graph**: `fletch.graph.v1`, fletch definitions,
   dependency/expansion/satisfaction edges, data-link edges, format options, and
   dry-run flight resolution. Mock client: add graph edges from villain index to
   casefiles and dated threat partitions.
3. **Speedy - Cache execution**: HTTP/file/generated shaft acquisition,
   resumable writes, temp-file promotion, checksum verification, retry policy,
   bandwidth controls, and path safety. Mock client: fetch villain index,
   Darkseid casefile, and threat partition files.
4. **Arsenal - Cache operations**: `cache ls`, `cache verify`, `cache prune`,
   offline checks, stale/fresh reports, and ledger-backed cache hit decisions.
   Mock client: verify all cached villain files and prove prune planning with an
   orphaned trick-arrow object.
5. **Red Arrow - Merge and aliases**: `fletch.merge.v1`, active views, labels,
   pins, rollbacks, merge previews, conflict groups, and pointer updates for
   current/latest/preferred fletches. Future `pull` belongs here as fetch plus
   merge, not as a fetch alias. Mock client: add staged aliases such as
   `justice-league:threats:current-year`, preview conflicts for competing threat
   files, and then merge safe threat groups in stages.
6. **Oracle - Partitions and rollups**: partition declarations, rollup edges,
   invalidation/folding metadata, and query folding metadata for analytical
   consumers such as `icelines-query`. Mock client: add dated threat partitions,
   year rollups, and when/where/how threat queries.
7. **Speedy II - Quivers**: `fletch.quiver.v1` export/import portable packages
   with member fletches, partitions, rollups, tips, ledgers, graph edges, and
   verification. Import stages by default; merge/activate is separate. Mock
   client: export/import a Justice League villain-files quiver for offline
   bootstrap.
8. **Connor Hawke - Adapters**: Census/apportionment, NHL/icelines,
   route/geodata, and generic static archive adapters. Mock client: become the
   adapter harness that proves consumer code stays outside `fletch-core`.
   Initial downstream migrations are complete for ROUTE generic source
   orchestration, BISECT generic HTTP acquisition, and ICELINES roster,
   MoneyPuck, paged NHL stats report, Gamecenter batch, player landing batch,
   and ESPN transaction window acquisition plus query partition/rollup handoff
   reporting and query quiver handoff reporting. Product semantics remain in
   each consumer.
9. **Overwatch - Publishers**: make FLETCH ledgers easy for MDCROP to index and
   for PROOF to render as local Markdown/status/backend views. Mock client:
   publish villain-file status, threat query summaries, tips, and quiver
   contents as local views.

## Non-goals

- FLETCH does not own domain semantics for BISECT, icelines, route, or MDCROP.
- FLETCH does not replace each product's user-facing commands.
- FLETCH does not depend on BISECT.
- FLETCH does not make generated PROOF/MDCROP documents the source of truth; they
  reflect registry, flight, quiver, and ledger contracts.
- FLETCH does not silently trust unverified remote data, extracted archive paths,
  or generated artifacts.
- FLETCH does not compute product metrics such as ICELINES points leaders; it
  exposes active partition sets, tips, lineage, and format metadata so product
  query engines can do that work.

## Naming

FLETCH = Fetch, Ledger, Export, Trust, Cache, Hash.

Core nouns: fletch, shaft, flight, quiver, ledger, tip, partition, rollup, alias.
