# FLETCH

**Fetch, ledger, export, trust, cache, hash.**

**Series:** [Tools & Infrastructure](https://github.com/giodl73-repo/giodl73-repo/blob/main/series/tools-infrastructure.md).

**Review roles:** This repo uses
[ROLES](https://github.com/giodl73-repo/ROLES), the `.roles` convention for
repository-local review panels.

FLETCH is a shared Rust fetch/cache layer for projects that need reproducible
data acquisition without each product reinventing download, cache, bundle,
manifest, verification, and offline behavior.

## Vocabulary

FLETCH models cached data as named, reusable fetch/cache units:

| Term | Meaning |
|------|---------|
| **fletch** | One logical fetch/cache unit: a dataset, file, generated artifact, bundle member, or adapter output. |
| **shaft** | The concrete carrier or locator for a fletch: HTTP URL, local file, GitHub release asset, generated backend output, or adapter source handle. |
| **flight** | A resolved execution plan that says what will be fetched, skipped, verified, expanded, or activated. |
| **quiver** | A named group or portable bundle of fletches that installs or activates together. |
| **ledger** | The manifest/status record of fletches, shafts, hashes, freshness, verification, and bundle membership. |
| **tip** | A small peek, sample, summary, or index at the end of a shaft/fletch so tools can inspect data before fully loading it. |
| **partition** | A durable slice of data, often scoped by date, game, state, source, or version. |
| **rollup** | A logical aggregate over partitions for folded queries and grouped activation. |
| **alias** | A mutable front door such as `current`, `today`, `latest`, or `preferred`. |

Use **source** for provenance in plain language, such as NHL API, Census, FHWA,
or MoneyPuck. Use **shaft** for the concrete thing FLETCH can fetch, read,
generate, or import.

Fletches can depend on other fletches, expand into discovered fletches, or be
satisfied together by a quiver. That keeps on-demand fetches, first-run bootstrap,
offline bundles, and feature-light-up flows in one graph instead of scattering
custom cache code across every consumer.

Tips make data easy to inspect without fully loading product logic, for example:
first rows of a CSV, schema fields from JSON, a ZIP member index, an SQLite table
list, or a PROOF-generated local status preview. Human hints can be generated
from tips, but the tip itself is structured preview/index metadata.

Partitions keep rich data manageable. For ICELINES, game boxscores can be
durable partitions, days/months/seasons can be rollups, and
`nhl:season:current` can be an alias pointing at the active season. Quivers are
portable packages that may carry those partitions, rollups, aliases, tips, and
ledger entries for offline use.

## Why FLETCH

- **One cache contract**: logical dataset IDs, source URLs, versions, hashes,
  byte counts, and verification status live in a shared manifest shape.
- **Product-neutral core**: BISECT, ICELINES, ROUTE, and CROP can eventually
  consume the same fetch/cache primitives without depending on each other.
- **Reproducible data fetches**: plans and cache keys make "what did this run
  fetch?" auditable.
- **Future offline/bundle path**: the workspace is prepared for bundle export,
  import, pruning, and stale/fresh reports.

Downstream migration status:

- ROUTE: generic source orchestration moved to FLETCH; ROUTE keeps route
  scoring, geospatial interpretation, and product outputs. ROUTE maintains a
  ROUTE-owned FLETCH cache manifest for manifest downloads and exposes
  `route fletch-cache-index --gate` evidence using FLETCH's shared manifest
  read/write, batch upsert, and cache-index gate helpers.
- BISECT: Census/TIGER/PL/EIA/LODES/ACS generic HTTP acquisition moved to
  FLETCH; BISECT keeps release adjacency, extraction, derived CSVs, and legal
  validation. `bisect fletch-cache-index --gate` maps BISECT's FLETCH cache
  manifest to compact evidence using FLETCH's shared manifest read/write,
  batch upsert, and cache-index gate helpers.
- ICELINES: roster, MoneyPuck, paged NHL stats report, Gamecenter batch,
  player landing batch, and ESPN transaction window acquisition moved to FLETCH, with
  `icelines fetch fletch-sources --gate` documenting source handoff and
  `icelines fetch fletch-partitions --gate` mapping
  leaders/player/compare, goalies, roster bios, MoneyPuck, career, and windowed
  game-line query surfaces to partition/rollup IDs.
  `icelines fetch fletch-quivers --gate` groups those partitions into query
  bootstrap and enrichment quiver handoff candidates, and `icelines fetch
  fletch-cache-index --gate` maps ICELINES' FLETCH cache manifest to compact
  cache-index evidence. ICELINES uses FLETCH's shared manifest read/write and
  batch upsert helpers for that ledger, and reuses FLETCH's shared cache-index
  gate after mapping dynamic child cachelines back to registered ICELINES
  parent sources. ICELINES keeps NHL parsing, snapshots, sealing, active
  pointers, and hockey-domain validation.
- CROP: optional indexing of FLETCH manifests and cached corpus metadata.

## Commands

```powershell
cargo run -p fletch-cli -- plan --dataset-id nhl:season:1993 --url https://example.test/1993.json --header accept=application/json
cargo run -p fletch-cli -- key --dataset-id route:tiles:demo --url https://example.test/tiles.zip --header accept=application/zip
cargo run -p fletch-cli -- fetch --dataset-id route:tiles:demo --url https://example.test/tiles.zip --max-bytes-per-second 250000 --timeout-ms 30000 --retry-attempts 2
cargo run -p fletch-cli -- fetch-plan --plan fletch.plan.json --cache-root .fletch/cache --output .fletch/cache/manifest.json
cargo run -p fletch-cli -- fetch --dataset-id route:tiles:demo --url https://example.test/tiles.zip --trusted-manifest .fletch/cache/manifest.json
cargo run -p fletch-cli -- fetch --dataset-id nhl:schedule:today --url https://example.test/schedule.json --freshness always-check
cargo run -p fletch-cli -- cache status --manifest .fletch/cache/manifest.json --freshness max-age-days --max-age-days 1
cargo run -p fletch-cli -- cache index-gate --manifest .fletch/cache/manifest.json --expected-registry fletch.registry.json --gate
cargo run -p fletch-cli -- cache prune --manifest .fletch/cache/manifest.json
cargo run -p fletch-cli -- quiver export --manifest .fletch/cache/manifest.json --quiver-id demo:pack --output-dir .fletch/quivers/demo
cargo run -p fletch-cli -- quiver import --quiver-dir .fletch/quivers/demo --cache-root .fletch/cache
cargo run -p fletch-cli -- graph export --manifest .fletch/cache/manifest.json
cargo run -p fletch-cli -- registry graph --file fletch.registry.json
cargo run -p fletch-cli -- registry flight --file fletch.registry.json --fletch-id justice-league:villains:index
cargo run -p fletch-cli -- registry index --file fletch.registry.json --output .fletch/registry-index.json
cargo run -p fletch-cli -- registry search --index .fletch/registry-index.json --tag ai-ml --metadata fetch_policy=metadata_only
cargo run -p fletch-cli -- tip from-manifest --manifest .fletch/cache/manifest.json --max-bytes 4096
cargo run -p fletch-cli -- publish from-manifest --manifest .fletch/cache/manifest.json --freshness immutable
```

`fletch plan` emits `fletch.plan.v1`, a source plan that downstream products can
check into configs or generate from their own CLI commands. Add repeatable
`--header name=value` flags for generic HTTP shafts that need explicit request
headers. `fletch key` emits the deterministic cache key for the logical
dataset/source/header identity.

`fletch fetch` acquires a HTTP/file shaft into the cache and emits a ledger
manifest. `fletch fetch-plan` executes a saved `fletch.plan.v1` file with the
same cache execution controls, preserving checked-in or generated plan details.
Saved plans are validated before execution so stale or malformed plan schemas do
not accidentally touch live sources.
`fletch-core` also exposes a product-neutral paged JSON acquisition primitive
for HTTP endpoints with `data` arrays and `total` counts; products such as
ICELINES can use it while retaining their own parsing, locks, snapshot writes,
and activation rules.
For source sets that a product has already expanded, `fletch-core` exposes a
product-neutral batch acquisition primitive over multiple fetch plans. The
consumer still owns the expansion semantics, but FLETCH owns the repeated
HTTP/file cache acquisition, verification, and manifest-ready entries.
When `--output` points to an existing manifest, fetch commands upsert the new
entry by cache key and preserve the rest of the ledger instead of replacing it
with a single-entry manifest. Consumers that fetch expanded batches through
`fletch-core` can use `upsert_cache_manifest_entries` to apply the same
cache-key merge rule to multiple fetched entries at once. `read_cache_manifest_json`
and `write_cache_manifest_json` provide reusable manifest file persistence with
schema and entry validation; they do not inspect cached bytes or activate data.
Fetching is acquisition, not activation: it verifies and records a candidate
object, but it does not merge that object into a product's active data view. In
the target model, `pull` is reserved for future fetch-plus-merge semantics
rather than a plain fetch alias.

FLETCH can use SLICE selectors over cache-index and active-partition rows before
its own gates, rollups, and quiver folding. See
[`docs/specs/slice-selectors.md`](docs/specs/slice-selectors.md).
Use `--max-bytes-per-second` to respect bandwidth-sensitive environments.
Use `--timeout-ms` and `--retry-attempts` to bound generic HTTP waits and retry
transient generic fetch/read/write failures. Ledger entries include
`fetch_attempts`, `retry_count`, and `last_retryable_error` so status publishers
can explain retry recovery.
Use `--trusted-manifest` to let a cache hit inherit verified ledger trust from a
prior manifest after FLETCH re-hashes the cached bytes and byte count.
For local file shafts, use `--source-kind file` with either a native path or a
`file://` URL such as `file:///C:/data/input.json`; empty file shafts are
rejected.
Use `--freshness immutable`, `--freshness max-age-days --max-age-days N`, or
`--freshness always-check` to say whether a shaft is effectively fixed,
periodically refreshed, or mutable on every fetch. Add `--force` to re-fetch a
fresh cache hit, or `--offline` to fail instead of touching the network. Offline
fetch errors distinguish a true missing cache object from an existing object that
is stale or explicitly bypassed.

`fletch cache index`, `fletch cache list`, `fletch cache verify`, `fletch cache status`, and
`fletch cache prune` operate on `fletch.cache-manifest.v1` ledger files.
The index command emits compact `fletch.cache-index.v1` rows for large-ledger
lookup and publisher inputs; use `--dataset-id`, `--cache-key`, `--verified`,
`--offset`, and `--limit` to focus large indexes. `fletch cache index-diff`
compares two compact indexes as `fletch.cache-index-diff.v1` without reading
object bytes. Verification compares cached object hashes and byte counts with
the ledger. Status adds freshness evaluation, and prune emits a deletion plan for
unreferenced cache objects without deleting them.

### Manifest-first consumer pattern

Consumers that expand dynamic source sets should keep their own
`fletch.cache-manifest.v1` file next to the product cache root. The product owns
the expansion semantics, fetch locks, parsing, validation, snapshots, and active
pointers; FLETCH owns generic acquisition, cache object verification, manifest
entry shape, and derived reports. A typical consumer flow is:

1. Expand product-specific work into `FetchPlan` values.
2. Execute generic fetches with `fetch_to_cache`, paged fetch, or batch fetch
   helpers.
3. Merge resulting entries into the durable ledger with
   `upsert_cache_manifest_entries`.
4. Persist and reload the ledger with `write_cache_manifest_json` and
   `read_cache_manifest_json`.
5. Feed the manifest to read-only reports such as cache index, status, verify,
   graph, publisher, partition, or quiver handoff reports.

This pattern is now used by ICELINES, BISECT, and ROUTE for cache-index
evidence and supports CROP/PROOF publisher inputs without making FLETCH
responsible for domain activation.

`fletch cache index-gate` is the product-neutral health gate for that contract:
consumers supply their own expected dataset IDs, choose whether all expected
entries are required for the current bootstrap/group/on-demand flow, and decide
whether unverified entries are allowed. FLETCH reports unexpected, missing, and
unverified rows without knowing product semantics. Expected IDs can be listed
directly with repeated `--expected-dataset-id` flags or derived from generic
HTTP/file fletches in one or more `fletch.registry.v1` files with
`--expected-registry`.

Publisher commands are read-only derived views. `fletch publish crop-index`,
`fletch publish proof-docs`, and `fletch publish local-url-map` accept
`--offset` and `--limit` for bounded output; CROP index output also accepts
`--row-type` to focus large local publisher surfaces.
Large read-only report commands also expose bounded output: partition state and
active-set rows, quiver merge-ready candidates, adapter source rows, validation
findings, and archive-preview children can be sliced with `--offset`/`--limit`
and focused with their report-specific filters.

`fletch quiver export` writes a `fletch.quiver.v1` directory with `quiver.json`
and referenced cache objects. `fletch quiver import` verifies bundled bytes,
copies through a temporary stage, then promotes into
`cache/staged/quivers/<quiver-id>/`; import is stage-first and does not activate
aliases, partitions, or product views.

`fletch graph export` emits `fletch.graph.v1` nodes and edges from a cache
manifest. The core graph includes fletch, shaft, and ledger-entry nodes plus
`satisfied-by` and `documents` edges. Product adapters can add domain edges and
registry node-kind hints, such as `partition` plus `rolls-up-to`, outside
`fletch-core`.

`fletch registry graph` reads `fletch.registry.v1` definitions and emits graph
JSON for declared fletches, shafts, partitions, rollups, format metadata, and
data-link edges before anything is fetched.

`fletch registry flight` emits a `fletch.flight.v1` dry-run plan from registry
definitions. It walks declared graph edges, reports which fletches would fetch,
which require adapters, and which are metadata-only rollups/aliases, then embeds
a graph view without touching the network or cache.

`fletch registry index` folds one or more external `fletch.registry.v1` files
into `fletch.registry-index.v1` rows for catalog search. Add `--follow` to
resolve `repo-registry` bridge rows into remote or local registry JSON files and
merge those registries into the same index. GitHub raw/blob file pointers and
GitHub contents/tree directory pointers are supported, so a MUNDUS-only checkout
can index repo registry entry points without cloning every domain repo. `fletch
registry search` filters that index by repeated `--tag`, repeated `--metadata
key=value`, and case-insensitive `--text`. FLETCH owns the generic index/search
mechanics only; catalog repos such as FONTES or MUNDUS own the actual source
registries, rights metadata, and curation policy.

`fletch registry web` serves a local browser UI at `http://127.0.0.1:7878/` for
searching registry data, clicking result rows, and inspecting tags, metadata, and
source URLs. Add `--open` to launch the bound URL in the default browser. It can
read an existing `fletch.registry-index.v1` with `--index`, or build the index in
memory from registry files with `--file` and `--follow`. Search text is
multi-term: `storm seed` matches rows that contain both terms even when the exact
phrase is not present. The UI also supports comma-separated tag filters and
comma-separated metadata filters such as `owner_repo=STORM,asset_kind=seed-fixture`.
For a one-command MUNDUS launch:

```powershell
fletch registry web --open --follow --file .fletch\registries\mundus-known-assets-seed.json --file .fletch\registries\mundus-knowledge-systems-registries.json
```

The left facet rail summarizes high-value sections of the index: owner repo,
domain, asset kind, fetch policy, and tags. Click a facet chip to filter into
that section. Click **Load preview** beside a source URL to fetch bounded source
data. Source previews include line-numbered sections, previous/next line
navigation, and a compact JSON outline when the loaded data is JSON. For followed
GitHub registry rows, FLETCH preserves the remote registry base URL so relative
source paths such as `fixtures\seed-storm.json` can be previewed from raw GitHub
without requiring that repo to be cloned locally.

`fletch tip from-manifest` emits `fletch.tip.v1` previews from cached artifacts.
The initial generic tipper samples bounded bytes and reports JSON fields, JSON
arrays/values, text samples, or opaque byte samples without interpreting product
semantics.

`fletch publish from-manifest` emits a `fletch.publish.v1` report with cache
status, graph, and tips bundled for CROP indexing, PROOF rendering, dashboards,
or other local status backends. It is a machine-readable source view, not a
generated document source of truth.

## Workspace

| Crate | Purpose |
|-------|---------|
| `fletch-core` | Fetch plans, cache policies, cache keys, and manifests. |
| `fletch-cli` | Generic command surface for plans, cache inspection, bundles, and future fetch execution. |
| `fletch-mock-client` | Justice League villain-files mock downstream app that exercises public FLETCH APIs. |

Run the mock client with:

```powershell
cargo run -p fletch-mock-client
```

It creates fake Justice League villain-index, Darkseid casefile, and dated
threat-partition fletches from local file shafts. The threat partitions carry
year rollup hints and measures such as threat count, omega events, and cities
impacted so a downstream tool can track threat measures across years. The mock
client fetches the fletches into a cache, verifies status, queries cached threat
partitions by year, city, and villain, exports/imports a stage-first quiver for
offline bootstrap, writes registry and dry-run flight files, emits graph views
from both registry and cached state, writes generic tips and a publish-ready
status report, and emits a prune plan for an orphaned trick-arrow object.

The same mock also includes a MAXIM-style source-corpus slice: a CROP view
recipe selects a frontend-framework guide, the fetched PEBBLE pack carries the
portable article context, and PROOF table/block sidecars provide structured data
for React-focused queries. FLETCH treats all four artifacts as generic cache
entries while the mock adapter owns the CROP/PEBBLE/PROOF interpretation.

## Design rule

FLETCH stays product-neutral. Adapters can know about Census, NHL, or route data,
but the core cache contract must work for any source.

## Specs

- [`docs/specs/fletch-foundation.md`](docs/specs/fletch-foundation.md) defines
  the initial plan and cache-manifest contracts.
- [`docs/specs/consumer-adapter-scout.md`](docs/specs/consumer-adapter-scout.md)
  maps ICELINES, apportionment/BISECT, ROUTE, CROP, MDPATH, and PROOF to
  migration slices and records the first completed consumer handoffs.
- `context/waves/` tracks implementation waves and pulse history.

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p fletch-cli -- plan --dataset-id nhl:season:1993 --url https://example.test/1993.json
```

## License

MIT. See [`LICENSE`](LICENSE).
