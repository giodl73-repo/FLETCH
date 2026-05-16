# FLETCH

**Fetch, ledger, export, trust, cache, hash.**

**Series:** [Tools & Infrastructure](https://github.com/giodl73-repo/giodl73-repo/blob/main/series/tools-infrastructure.md).

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

Initial consumers:

- BISECT: census, election, geography, and evidence datasets.
- ICELINES: NHL seasons, profiles, favorites, and bundled/offline data.
- ROUTE: geospatial/routing datasets, tiles, profiles, and on-demand fetches.
- CROP: optional indexing of FLETCH manifests and cached corpus metadata.

## Commands

```powershell
cargo run -p fletch-cli -- plan --dataset-id nhl:season:1993 --url https://example.test/1993.json
cargo run -p fletch-cli -- key --dataset-id route:tiles:demo --url https://example.test/tiles.zip
cargo run -p fletch-cli -- fetch --dataset-id route:tiles:demo --url https://example.test/tiles.zip --max-bytes-per-second 250000 --timeout-ms 30000 --retry-attempts 2
cargo run -p fletch-cli -- fetch --dataset-id nhl:schedule:today --url https://example.test/schedule.json --freshness always-check
cargo run -p fletch-cli -- cache status --manifest .fletch/cache/manifest.json --freshness max-age-days --max-age-days 1
cargo run -p fletch-cli -- cache prune --manifest .fletch/cache/manifest.json
cargo run -p fletch-cli -- quiver export --manifest .fletch/cache/manifest.json --quiver-id demo:pack --output-dir .fletch/quivers/demo
cargo run -p fletch-cli -- quiver import --quiver-dir .fletch/quivers/demo --cache-root .fletch/cache
cargo run -p fletch-cli -- graph export --manifest .fletch/cache/manifest.json
cargo run -p fletch-cli -- registry graph --file fletch.registry.json
cargo run -p fletch-cli -- registry flight --file fletch.registry.json --fletch-id justice-league:villains:index
cargo run -p fletch-cli -- tip from-manifest --manifest .fletch/cache/manifest.json --max-bytes 4096
cargo run -p fletch-cli -- publish from-manifest --manifest .fletch/cache/manifest.json --freshness immutable
```

`fletch plan` emits `fletch.plan.v1`, a source plan that downstream products can
check into configs or generate from their own CLI commands. `fletch key` emits
the deterministic cache key for the logical dataset/source pair.

`fletch fetch` acquires a HTTP/file shaft into the cache and emits a ledger
manifest. Fetching is acquisition, not activation: it verifies and records a
candidate object, but it does not merge that object into a product's active data
view. In the target model, `pull` is reserved for future fetch-plus-merge
semantics rather than a plain fetch alias.
Use `--max-bytes-per-second` to respect bandwidth-sensitive environments.
Use `--timeout-ms` and `--retry-attempts` to bound generic HTTP waits and retry
transient generic fetch/read/write failures. Ledger entries include
`fetch_attempts`, `retry_count`, and `last_retryable_error` so status publishers
can explain retry recovery.
Use `--freshness immutable`, `--freshness max-age-days --max-age-days N`, or
`--freshness always-check` to say whether a shaft is effectively fixed,
periodically refreshed, or mutable on every fetch. Add `--force` to re-fetch a
fresh cache hit, or `--offline` to fail instead of touching the network.

`fletch cache list`, `fletch cache verify`, `fletch cache status`, and
`fletch cache prune` operate on `fletch.cache-manifest.v1` ledger files.
Verification compares cached object hashes and byte counts with the ledger.
Status adds freshness evaluation, and prune emits a deletion plan for
unreferenced cache objects without deleting them.

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

## Design rule

FLETCH stays product-neutral. Adapters can know about Census, NHL, or route data,
but the core cache contract must work for any source.

## Specs

- [`docs/specs/fletch-foundation.md`](docs/specs/fletch-foundation.md) defines
  the initial plan and cache-manifest contracts.
- [`docs/specs/consumer-adapter-scout.md`](docs/specs/consumer-adapter-scout.md)
  maps ICELINES, apportionment/BISECT, ROUTE, CROP, MDPATH, and PROOF to first
  FLETCH migration slices.
- `context/waves/` tracks implementation waves and pulse history.

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p fletch-cli -- plan --dataset-id nhl:season:1993 --url https://example.test/1993.json
```

## License

MIT. See [`LICENSE`](LICENSE).
