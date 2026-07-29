# FLETCH Showcase

**Who this is for:** someone you would hand the repo to for 15–35 minutes —
a **product integrator** who needs a shared fetch/cache ledger, or a **systems
implementer** who wants the CLI/core contract and vocabulary.

**Posture:** shared infrastructure substrate. Not a product data warehouse, not
an activation/merge layer, and not a claim that every consumer has fully migrated.
Fetching is acquisition and verification — products still own parsing, locks,
snapshots, and active pointers.

| Audience | Open this first | Time |
|---|---|---|
| Product integrator | [Integrator brief](docs/show/integrator-brief.md) | 10–20 min |
| Systems implementer | [Implementer brief](docs/show/implementer-brief.md) | 15–35 min |
| Either, vocabulary first | README “Vocabulary” + [foundation spec](docs/specs/fletch-foundation.md) | 10 min |

## One-minute pitch

FLETCH is a **product-neutral Rust fetch/cache ledger** so ROUTE, BISECT,
ICELINES, and peers stop reinventing download, cache keys, manifests, verify,
offline, and bundle behavior.

Named units:

| Term | Role |
|---|---|
| **fletch** | Logical fetch/cache identity |
| **shaft** | Concrete carrier (URL, file, asset, handle) |
| **flight** | Resolved plan: fetch / skip / verify / expand |
| **quiver** | Named portable group of fletches |
| **ledger** | Manifest/status of hashes, freshness, membership |

Family dependency stays strict:

```text
Sources → FLETCH → MDCROP → LATTICE → FLETCHER
           fetch     select     close       replay
```

## Two doors

### A. Product integrator path

**Question FLETCH answers well:** *How do I consume a shared cache ledger without
taking on another product’s domain semantics?*

| Step | What to look at | Why |
|---|---|---|
| 1 | README “Manifest-first consumer pattern” | Product owns expansion; FLETCH owns acquisition |
| 2 | [Integrator brief](docs/show/integrator-brief.md) | Safe consume path and migration posture |
| 3 | Downstream notes in README (ROUTE / BISECT / ICELINES) | What already moved vs what stayed product-local |
| 4 | `fletch cache index-gate` / consumer `--gate` helpers | Compact evidence without inventing a new manifest shape |

**Integrator takeaways:**

- Keep a product-owned `fletch.cache-manifest.v1` next to the product cache root.
- Use plan / fetch / verify / status / quiver export-import; do not treat fetch as activate.
- Optional next stage is MDCROP (select), not more fetch logic in the product.

**Do not say:** FLETCH replaces product ETL, guarantees freshness forever, or
means offline bundles are production-certified for every consumer.

### B. Systems implementer path

**Question FLETCH answers well:** *What are the CLI surfaces, ledger schemas, and
core primitives I should extend?*

| Step | What to look at | Why |
|---|---|---|
| 1 | [Implementer brief](docs/show/implementer-brief.md) | Crate/CLI map and non-goals |
| 2 | [Foundation spec](docs/specs/fletch-foundation.md) | Vocabulary and graph edges |
| 3 | [Slice selectors](docs/specs/slice-selectors.md) | Pre-gate selection over index rows |
| 4 | `cargo run -p fletch-cli -- --help` | Live command surface |

**Implementer takeaways:**

- Core is product-neutral; adapters and consumers stay outside domain logic.
- Ledger upsert is by cache key; batch helpers exist for expanded source sets.
- Tips are structured peeks — not human product UI.

## Fastest hands-on

```powershell
cargo run -p fletch-cli -- plan --dataset-id demo:pack --url https://example.test/data.json
cargo run -p fletch-cli -- key --dataset-id demo:pack --url https://example.test/data.json
cargo test --workspace
```

Use real shafts only in environments that allow network or local file access.
Example URLs in docs are shape demos, not live datasets.

## Claim packet (this showcase)

| Field | Value |
|---|---|
| Claim text | FLETCH can be shown as a shared fetch/cache ledger with separate integrator and implementer entry paths. |
| Audience | Product integrators; systems implementers. |
| Evidence | README vocabulary + commands; foundation/slice specs; consumer migration notes for ROUTE/BISECT/ICELINES. |
| Validation | Documentation + existing CLI/workspace tests; not an external multi-product certification. |
| Limitations | Not every portfolio product fully migrated; activation/merge (`pull`) is future semantics; offline/quiver path is prepared, not universally production-proven. |
| Non-claims | Product data correctness, legal archival custody, CDN/CDN-replacement, or selection/closure/replay (those are MDCROP/LATTICE/FLETCHER). |
| Review lane | Tools-infra / shared substrate; BOUNDARY if a consumer overclaims offline readiness. |

## Where not to start

| Avoid leading with… | Why |
|---|---|
| Full consumer migration | Hides the contract under product politics |
| MDCROP/LATTICE deep dive first | Wrong layer for a fetch question |
| Treating tip output as analytics | Tips are peeks, not product metrics |

## Related

- README: [`README.md`](README.md)
- Show pack index: [`docs/show/README.md`](docs/show/README.md)
- Family upstream index (portfolio): tools-infrastructure series
