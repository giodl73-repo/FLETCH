# FLETCH Foundation Spec

## Goal

Create a neutral fetch/cache substrate that BISECT, icelines, route, and other
Rust repos can consume without depending on one another.

## Product vocabulary

FLETCH uses product-neutral nouns for shared fetch/cache mechanics:

| Term | Contract role |
|------|---------------|
| **fletch** | Logical cache identity. Existing `dataset_id` values are fletch IDs until a dedicated registry schema lands. |
| **shaft** | Concrete carrier or locator for a fletch: URL, file path, release asset, generated local artifact, or adapter-owned source handle. Existing `source` fields describe shafts. |
| **flight** | Resolved execution plan over one or more fletches: fetch, skip, verify, refresh, expand, or activate. Existing `fletch.plan.v1` is the first single-fletch flight shape. |
| **quiver** | Named group or portable bundle that satisfies multiple fletches. Future bundle contracts should use quiver terminology. |
| **ledger** | Cache manifest and status record. Existing `fletch.cache-manifest.v1` is the first ledger shape. |
| **tip** | Structured peek, sample, summary, or index for a shaft/fletch. Tips help tools inspect data without fully loading product semantics. |
| **partition** | Durable slice of a fletch family, usually scoped by date, range, geography, game, source, or version. |
| **rollup** | Logical aggregate over partitions, used for folded queries and grouped activation. |
| **alias** | Mutable front door such as `current`, `today`, `latest`, or `preferred` that points to a fletch, partition, rollup, or active view. |

Use `source` when referring to provenance or authority. Use `shaft` when
referring to the concrete URL, path, release asset, generated output, or adapter
handle FLETCH resolves.

Fletches form a graph:

- **requires**: one fletch needs another before it can be used.
- **expands-to**: fetching one fletch discovers or materializes additional
  fletches.
- **satisfied-by**: one shaft or quiver can populate many fletches.
- **activates**: verified fletches light up product-owned capabilities.
- **contains**: one fletch physically contains another, such as a ZIP member or
  release asset.
- **derived-from**: one fletch is generated, normalized, filtered, or indexed
  from another.
- **supersedes**: one fletch replaces an older version while preserving lineage.
- **mirrors**: two shafts or fletches are equivalent source alternatives.
- **cites**: one fletch uses another as evidence or documentation.
- **documents**: a generated PROOF/CROP/Markdown/backend artifact describes a
  fletch, shaft, quiver, flight, or ledger view.
- **points-to**: an alias points to the active fletch, partition, rollup, or
  view.
- **rolls-up-to**: a partition contributes to a larger date, month, season,
  geography, decade, or product-defined aggregate.
- **folds-over**: a queryable rollup or adapter view reads a partition set as one
  logical dataset.

FLETCH records the graph and state transitions; products keep domain semantics
in their own adapters.

## CROP graph contract

CROP works best when FLETCH exposes data state as a graph instead of only as a
flat manifest. FLETCH graph exports should use stable typed nodes and edges:

| Node kind | Meaning |
|-----------|---------|
| `fletch` | Logical cache/data unit. |
| `shaft` | Concrete source option for one or more fletches. |
| `quiver` | Group or portable bundle satisfying multiple fletches. |
| `flight` | Planned or executed fetch/cache operation. |
| `ledger-entry` | Observed cached artifact state. |
| `document` | Generated or handwritten status/spec artifact. |
| `partition` | Durable slice of a fletch family. |
| `rollup` | Logical aggregate over partitions. |
| `alias` | Mutable front door pointer. |

| Edge kind | Meaning |
|-----------|---------|
| `requires` | Fletch dependency. |
| `expands-to` | Fetching one fletch discovers or creates another. |
| `satisfied-by` | Shaft or quiver can satisfy a fletch. |
| `contains` | Container fletch/quiver includes another fletch. |
| `derived-from` | Fletch was transformed, normalized, indexed, or generated from another. |
| `supersedes` | Newer fletch replaces an older one. |
| `mirrors` | Alternative shafts/fletches represent equivalent data. |
| `cites` | Fletch uses another as evidence. |
| `documents` | Document/status artifact describes a graph node. |
| `points-to` | Alias points at its active target. |
| `rolls-up-to` | Partition contributes to a larger aggregate. |
| `folds-over` | Query/view evaluates over a partition set. |

`fletch.graph.v1` is the named export contract for this graph. The initial
implementation exports cache-manifest state as fletch, shaft, and ledger-entry
nodes with `satisfied-by` and `documents` edges. Consumers can add adapter-owned
domain edges, such as `expands-to` from an index to discovered fletches or
`rolls-up-to` from dated partitions to yearly rollups, without moving domain
logic into `fletch-core`.

Manifest graph export can accept registry node-kind hints so a cached partition
or rollup keeps the same graph identity it had in `fletch.registry.v1`. Ledger
edges then document the declared partition or rollup node instead of creating a
second generic fletch node.

`fletch.registry.v1` is the first registry definition contract. It declares
fletches before execution with:

- `id`: stable logical fletch id.
- `node_kind`: graph node kind such as `fletch`, `partition`, `rollup`, or
  `alias`.
- `shafts`: acceptable source options using the same source kind/url/header
  model as plans.
- `edges`: declared graph relationships such as `requires`, `expands-to`,
  `derived-from`, or `rolls-up-to`.
- `format`: optional media type, encoding, compression, container, schema,
  record shape, and preferred local representation.
- `tags` and `metadata`: product-owned labels that remain opaque to FLETCH.

Registry graph export turns those declarations into `fletch.graph.v1` without
fetching or activating data.

## Data format model

FLETCH's default data model is **opaque verified bytes**:

- `media_type`: defaults to `application/octet-stream` when unknown.
- `encoding`: optional text encoding, e.g. `utf-8`.
- `compression`: optional compression, e.g. `gzip`, `zip`, `zstd`.
- `container`: optional container shape, e.g. `zip`, `tar`, `sqlite`, `directory`.
- `schema`: optional schema label or URI, e.g. `nhl.stats.summary.v1`.
- `record_shape`: optional adapter-owned hint, e.g. `json-array`, `csv-table`,
  `geojson-feature-collection`, `shapefile-layer`.
- `preferred_local`: optional preferred cached representation when a shaft can
  be transformed, e.g. raw ZIP plus extracted CSV fletches.

Registries can declare one or more acceptable format options for a fletch.
Flights choose a satisfiable option based on local cache state, offline mode,
available quivers, and adapter capabilities. FLETCH records the chosen format in
the ledger; adapters interpret the data.

## Tip model

A tip is the lightweight inspection surface at the end of a shaft/fletch. It is
structured preview/index metadata, not a replacement for the cached artifact:

- `kind`: preview kind, e.g. `csv-header`, `json-fields`, `zip-index`,
  `sqlite-tables`, `schema-summary`, `sample-rows`, `proof-status`.
- `summary`: short human-readable preview.
- `fields`: optional field names, columns, table names, archive members, or
  schema keys.
- `sample_ref`: optional relative path or byte range for a stored sample.
- `generated_from`: fletch or shaft ID used to produce the tip.
- `truncated`: whether the tip is a partial preview.

Tips give CROP, PROOF, CLIs, and adapters a cheap way to decide what data is
inside a shaft before doing full domain-specific parsing.

`fletch.tip.v1` is the initial tip contract. Manifest-backed tips sample bounded
bytes from verified cache objects and emit generic preview kinds such as
`json-fields`, `json-array`, `json-value`, `text-sample`, or `opaque-bytes`.
Adapters can add richer tips later, but generic FLETCH tips stay product-neutral.

## Partition, rollup, and alias model

FLETCH should distinguish durable data identity from mutable front-door names:

- **partition fletches** are durable slices such as
  `nhl:boxscore:game:2025020001`,
  `nhl:schedule:date:2026-05-15`,
  `census:2020:state:WA:tracts`, or `route:nbi:2024:state:CA`.
- **rollups** group partitions for product queries, such as
  `nhl:boxscore:month:2025-10`, `nhl:season:20252026`,
  `route:nbi:2024:national`, or `census:2020:nationwide`.
- **aliases** are mutable pointers such as `nhl:schedule:today`,
  `nhl:season:current`, `route:geodata:base`, or
  `bisect:census:active-2020`.

Merge/activation should update aliases, active partition sets, labels, and
rollups; it should not rewrite historical partition identity. This lets products
fault in missing detail while retaining rollback and lineage.

For `icelines-query`, FLETCH should provide the active partition set and graph
metadata, not compute hockey stats. A query such as current-season leaders can
resolve `nhl:season:current`, follow active boxscore/stat/realtime/MoneyPuck
partition edges, fault in missing partitions when policy allows, and fold over
the resulting set inside ICELINES' query engine.

Example graph:

```text
alias: nhl:season:current
  points-to -> nhl:season:20252026

partition: nhl:boxscore:game:2025020001
  rolls-up-to -> nhl:boxscore:date:2025-10-07
  rolls-up-to -> nhl:boxscore:month:2025-10
  rolls-up-to -> nhl:boxscore:season:20252026

rollup: nhl:player-stats:season:20252026:regular
  derived-from -> nhl:boxscore:season:20252026
  folds-over   -> active nhl:boxscore:game:* partition set
```

BISECT/apportionment can use the same pattern for Census/election partitions:

```text
alias: bisect:census:active-2020
  points-to -> census:2020:nationwide

partition: census:2020:state:WA:tracts
  rolls-up-to -> census:2020:division:pacific
  rolls-up-to -> census:2020:nationwide

partition: election:2024:state:WA:precincts
  cites       -> census:2020:state:WA:tracts
  rolls-up-to -> election:2024:nationwide
```

ROUTE can use dated infrastructure partitions and stable aliases:

```text
alias: route:nbi:current
  points-to -> route:nbi:2024:national

partition: route:nbi:2024:state:CA
  derived-from -> route:nbi:2024:archive
  rolls-up-to  -> route:nbi:2024:national

rollup: route:geodata:base
  folds-over -> active route:nbi:* and route:hpms:* partition sets
```

Quivers are not partitions. A quiver is a portable package that may contain
partitions, rollups, aliases, tips, graph edges, and ledger entries so an offline
environment can satisfy or stage a set of fletches. Quiver import must stage by
default; a separate merge/activate transaction makes imported data active.
Import verifies quiver bytes against `quiver.json` before copying and promotes
from a temporary stage so failed imports do not look activated or complete.

`fletch.quiver-summary.v1` reports bundle identity, entry count, total bytes,
verified member count, and unverified member count from a `fletch.quiver.v1`
manifest without importing, copying, or activating bundle contents.

## Merge, labels, and rollback

FLETCH merge is Git-inspired but not text merging. It promotes verified
candidates into an active ledger/view through an auditable transaction:

- `fetch`: acquire and verify candidate fletches. No active state change.
- `merge`: update active aliases, partition sets, labels, rollups, and views.
- `pull`: future shorthand for fetch plus merge when policy allows; it should
  not be a plain fetch alias.
- `label`: name a ledger state, active partition set, or quiver import.
- `pin`: lock a view to a label, revision, or partition set.
- `rollback`: restore an active view to a prior label or revision.

Merge policies:

| Policy | Meaning |
|--------|---------|
| `additive` | Add new active fletches without replacing existing ones. |
| `supersede` | Replace the active target while preserving `supersedes` lineage. |
| `replace-set` | Atomically replace a coherent active partition set or quiver. |
| `overlay` | Prefer staged/local/update fletches over bundled/base fletches. |
| `no-op` | Candidate already matches active hash/state. |
| `conflict` | FLETCH cannot safely choose without policy or user resolution. |

Conflict groups should support labeled alternatives so repeated conflicts can be
resolved in bulk:

```text
conflict-group: nhl:schedule:season:20252026
  A = official NHL API
  B = release quiver
  C = local correction

resolution:
  choose A for all schedule conflicts in season 20252026
  choose C for game 2025020001
```

Merge should also support a preview mode that computes the transaction plan
without activation. A preview returns the candidate inputs, policy decision,
would-activate/would-supersede sets, alias or partition-set updates, rollback
target, and conflict groups as data. Clients can show the preview, resolve one
conflict group, merge a safe subset, then re-preview the remaining staged
changes. This keeps large imports, expanding cachelines, and quiver installs
usable in stages instead of forcing an all-or-nothing decision.

The first preview contract is `fletch.merge-preview.v1`. It compares an active
cache manifest with a candidate cache manifest and reports additions, unchanged
entries, same-source replacements, and conflicts where the same logical dataset
points at a different source. It does not mutate aliases, labels, cache objects,
or product views.

The first alias contract is `fletch.alias-state.v1`. It records product-neutral
alias IDs pointing at manifest entries by dataset ID, cache key, hash, and
relative path. Alias state names active views without moving cache objects or
embedding product semantics.

`fletch.label-state.v1` records labels over alias state. Labels name a current
alias target set for repeatable references, and `pinned: true` marks that the
label should continue pointing at the recorded cache keys until explicitly
changed by a later merge/rollback operation.

`fletch.rollback-preview.v1` compares current alias state with a target label
state and reports restore actions before mutation. A rollback preview says which
aliases would move back to pinned label targets and which aliases are already at
the target.

`fletch.partition-state.v1` records durable partition rows from manifest
evidence. Each row carries a product-neutral partition ID, dataset ID, optional
group ID, cache key, hash, byte count, source URL, relative cache path, and
verification state. FLETCH does not parse the business meaning of a partition;
adapters decide whether IDs represent seasons, years, districts, tiles, dates,
or any other domain concept.

`fletch.rollup-preview.v1` records proposed parent/child edges from a rollup ID
to partition rows. It reports selected partitions, cache keys, hashes, byte
counts, and missing child partition IDs before any rollup materialization or
activation.

`fletch.partition-invalidation.v1` reports stale, folded, and superseded
partition metadata over a partition state. It exposes counts, per-partition
flags, reasons, and missing partition IDs without deleting cache objects,
materializing rollups, or changing active aliases.

`fletch.active-partition-set.v1` reports query-facing partition rows with the
alias IDs, label IDs, and rollup IDs that make each partition active. It is a
derived view over partition, alias, label, and rollup-preview contracts; it does
not make cache presence alone equivalent to activation.

Every merge transaction should record its target view, policy, candidate inputs,
activated fletches, superseded fletches, alias updates, conflicts, optional
label, and rollback target.

`fletch.merge.v1` is the future named contract for these transactions. It should
make candidate inputs, chosen policy, conflict groups, alias updates, activated
partition sets, labels, rollback targets, and preview-only decisions
machine-readable.

Rollups should record enough invalidation/folding metadata for adapters to know
when a rollup is stale and which partitions it folds over. FLETCH owns the
lineage and freshness metadata; product query engines own the math.

## Core contracts

### `fletch.plan.v1`

Describes intent to obtain a dataset:

- `dataset_id`: logical id, e.g. `nhl:season:1993`.
- `version`: optional source or domain version.
- `source`: source kind, URL, and optional headers.
- `cache_policy`: freshness, offline, and resumable behavior.
- `tags` and `metadata`: product-owned classification.

Generic HTTP headers are part of the source identity. They are stored on the
shaft, sent during HTTP acquisition, and included in deterministic cache keys so
two requests to the same URL with different generic headers do not collide.
Saved `fletch.plan.v1` documents are executable by generic fetch tooling so
adapters, CROP/PROOF generated files, or checked-in configs can hand FLETCH a
complete acquisition intent without rebuilding it from flags.
Fetch execution validates saved or in-memory plans before acquisition: the schema
must be `fletch.plan.v1`, and required source identity fields must be present.
Invalid plans fail before cache lookup or live source access.

Freshness policy is not a promise that every fetch is one-time:

- `immutable`: reuse a verified cached object unless the caller forces a fetch.
- `max-age-days`: reuse a verified cached object until it ages past the limit.
- `always-check`: treat the shaft as mutable and fetch/check on each execution.
- `offline`: if live fetches are disabled, report missing/stale fletches instead
  of assuming the last cached value is acceptable.

When offline execution cannot use a cache entry, it reports whether the object is
missing or exists but is stale/bypassed. This keeps bootstrap and on-demand tools
from confusing "download this shaft first" with "you have a candidate, but policy
requires a refresh before activation."

Generic fetch execution can bound live source behavior with timeout, bandwidth,
and retry controls. Retries are product-neutral and only reattempt generic
HTTP/file acquisition failures; checksum mismatches and unsupported adapter
shafts remain explicit failures. Successful fetch outcomes and ledger entries
record attempt count, retry count, and the last retryable error observed before
success.

Cache hits without an expected hash are not automatically trusted. A caller can
provide a prior `fletch.cache-manifest.v1` as a trusted ledger; FLETCH matches the
logical cache key, dataset ID, and source URL, re-hashes the cached bytes, and
only preserves verified status and retry metadata when the current object still
matches the ledger hash and byte count. A mismatch is a checksum failure, not a
silent re-fetch or success-shaped fallback.

Local file shafts accept native paths and common `file://` URL forms. Empty file
shafts are invalid, and URL normalization remains generic path handling rather
than a product-specific source catalog.

Fetch/merge semantics are deliberately separate. A fetch may acquire, verify,
and record a candidate cache object, but it must not silently merge that object
into a product's active data view. Future `pull` is reserved for fetch plus
merge when policy allows. Merge, activation, or "make current" decisions belong
to later ledger/quiver operations or product adapters, where stale data,
replacement policy, and feature activation can be reviewed explicitly.

`fletch.flight.v1` records dry-run resolution before fetch execution. The first
implementation resolves `fletch.registry.v1` declarations without touching the
network or cache and emits:

- requested fletch IDs,
- dependency and expansion edges,
- data-link edges such as contains, derived-from, supersedes, mirrors, and cites,
- acceptable and chosen data format options,
- shafts that would be fetched by generic execution,
- adapter-required shafts that need product code,
- metadata-only nodes such as rollups and aliases,
- stale or missing fletches,
- quivers that can satisfy requested fletches,
- activation outcomes owned by adapters,
- tips that preview or index relevant data without replacing the artifact.

Future flight execution can add verified cache hits, skip decisions, stale/missing
cache status, and quiver satisfaction choices while preserving the same dry-run
shape.

### `fletch.cache-manifest.v1`

Records cached artifacts:

- source URL and logical dataset id,
- data format used,
- links to related fletches,
- deterministic cache key,
- relative cache path,
- content hash,
- byte count,
- fetched timestamp,
- verification status,
- fetch attempts, retry count, and last retryable error when retry recovery was
  needed.

Trusted cache-hit execution can reuse a matching ledger entry's fetched
timestamp and retry metadata, but only after verifying the current object against
that ledger entry.

Fetch execution can append to an existing output ledger by upserting the emitted
entry by deterministic cache key. This lets repeated generic fetches build one
publishable manifest while preserving unrelated entries and replacing only the
same shaft identity.

Ledger entries should remain safe to publish through CROP/PROOF. They should
include enough provenance for local status pages without requiring generated
Markdown to become the source of truth.

Initial cache operations are manifest-led:

- `cache list`: display ledger entries without touching cached objects.
- `cache verify`: emit `fletch.cache-verify.v1`, a named report that hashes
  cached objects, compares them with ledger hash and byte count, and includes
  summary counts plus per-entry status rows.
- `cache status`: report verified, missing, hash-mismatch, fresh, or stale state
  using a caller-provided freshness policy.
- `cache summary`: aggregate status rows into cache health counts and expected
  versus actual byte totals for CI, CROP, and PROOF status views.
- `cache offline-report`: emit `fletch.cache-offline.v1`, a no-live readiness
  report that counts fresh verified entries, missing entries, stale entries, and
  blocked entries using a caller-provided freshness policy.
- `cache prune`: emit `fletch.cache-prune.v1`, a non-destructive plan for
  deletion candidates under the cache object tree that are not referenced by the
  manifest. Plans include object root, keep/prune counts and bytes, candidate
  reasons, and `destructive: false`; deletion requires a later explicit
  execution command.

### `fletch.graph.v1`

Future graph export contract for typed fletch, shaft, quiver, flight,
ledger-entry, partition, rollup, alias, and document nodes plus the edge kinds
defined above.

### `fletch.flight.v1`

Dry-run contract for graph-shaped execution previews. A flight contains requested
fletch IDs, ordered steps, each step's action (`would-fetch`, `adapter-required`,
`metadata-only`, or `missing-fletch`), the chosen shaft when known, deterministic
cache key preview, declared dependencies, and an embedded graph view rooted at a
flight node. It is planning data only; it does not fetch, merge, or activate.

### `fletch.tip.v1`

Lightweight preview contract for cached artifacts. A tip records the fletch id,
cache key, preview kind, human summary, optional fields, sample reference,
ledger-entry source, and truncation state. Tips are bounded previews for
inspection and publishing; they are not authoritative data or product semantics.

### `fletch.publish.v1`

Machine-readable publish scout for CROP, PROOF, dashboards, and local status
backends. A publish report bundles the cache graph, cache status rows, and
bounded tips derived from a manifest. Generated Markdown, HTML, or other backend
views should render this contract; they should not replace the manifest, graph,
or tip contracts as source of truth.

### `fletch.merge.v1`

Future transaction contract for staged candidate inputs, active-view updates,
labels, pins, rollback targets, policy choices, and conflict-group resolution.

### `fletch.quiver.v1`

Portable package contract for member fletches, cache objects, and a `quiver.json`
manifest. The initial implementation is a directory quiver:

```text
quiver-root/
  quiver.json
  objects/sha256/<cache-key>
```

`quiver.json` records `schema_version`, `generated_by`, `quiver_id`, and copied
ledger entries. Export requires every referenced object to verify against the
source ledger. Import copies objects into
`cache/staged/quivers/<safe-quiver-id>/`, verifies the staged objects, and emits
a staged `fletch.cache-manifest.v1`; merge/activate is explicit and separate.
Future quivers can add tips, graph edges, partitions, rollups, aliases, and
compressed archive packaging while preserving stage-first import.

## Trust and safety requirements

- Never activate a fletch from an unverified hash when a hash is expected.
- Write to temporary paths and promote only after a complete fetch/write.
- Normalize archive extraction paths so quiver import cannot write outside the
  intended cache root.
- Preserve the distinction between remote shafts, local file shafts, generated
  artifact shafts, and adapter-owned shafts.
- Offline mode must report verified, stale, missing, and unverifiable fletches
  explicitly.
- Dry-run flights must show planned network, disk, quiver, and activation work
  without mutating cache state.

## Publishing surfaces

FLETCH ledgers and registry data should be shaped so local tools can publish
status without owning fetch behavior:

- CROP indexes ledgers, quivers, and cache docs as corpus/status metadata.
- MDPATH provides stable local references to generated specs and status rows.
- PROOF can render registry, flight, quiver, and ledger views as Markdown, HTML,
  dashboard, or other backend output.

## Initial CLI

```powershell
fletch plan --dataset-id nhl:season:1993 --url https://example.test/1993.json
fletch key  --dataset-id route:tiles:demo --url https://example.test/tiles.zip
fletch cache status --manifest .fletch/cache/manifest.json
```

## Onboarding targets

| Repo | Initial FLETCH fit |
|------|--------------------|
| BISECT/apportionment | Census/geography/election fletches, large shafts, and verified ledgers. |
| icelines | NHL season/game/profile/favorite fletches, on-demand expansion, quivers, offline mode. |
| route | Geodata/routing fletches, archive shafts, bundleable local caches, on-demand fetches. |
| CROP | Index and status over FLETCH ledgers, quivers, and cache docs. |

See [`consumer-adapter-scout.md`](consumer-adapter-scout.md) for the initial
adapter migration matrix and mock-client proving path.

## Later extraction

RLINE should become the neutral home for reusable `r*` graph/stat/context
kernels. FLETCH should not wait on RLINE; it starts independent.
