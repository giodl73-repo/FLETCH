# .roles Review: Manifest Maintenance

## Scope

Reviewed Manifest Maintenance pulses 01-03 against the `.roles` gate. This wave
adds reusable cache manifest merge and JSON persistence helpers, then documents
and smokes a manifest-first consumer workflow without changing FLETCH's
activation boundary.

## Parliament findings

| Role | Finding | Disposition |
|---|---|---|
| Cache Systems Engineer | Multi-entry upserts and JSON helpers preserve manifests as validated ledgers keyed by cache key. | Pass |
| Provenance Auditor | Helpers validate manifest schema and entry hashes before file writes or downstream reads. | Pass |
| Offline Release Operator | Manifest persistence, cache indexing, and mock-client report generation run from local files and cache roots. | Pass |
| Adapter Boundary Keeper | The consumer pattern leaves expansion, parsing, snapshots, and active pointers outside `fletch-core`. | Pass |
| Consumer Ergonomics | Consumers now have shared helpers for durable ledgers instead of open-coding repeated single-entry upserts and JSON parsing. | Pass |

## Editorial findings

| Role | Finding | Disposition |
|---|---|---|
| Scope Keeper | The wave did not add a persistent database, daemon, scheduler, product parser, or activation state. | Pass |
| Contract Checker | README and pulse docs consistently describe `fletch.cache-manifest.v1` as the source of truth and cache indexes as derived reports. | Pass |
| Validation Checker | Pulses ran workspace tests, focused CLI smokes, formatting, and diff checks. | Pass |

## Stakeholder findings

| Stakeholder | Finding | Disposition |
|---|---|---|
| ICELINES Maintainer | ICELINES can keep its FLETCH cache manifest and cache-index evidence while preserving NHL snapshot ownership. | Pass |
| BISECT/Apportionment Analyst | Evidence ledgers can be persisted and indexed without legal/data-release semantics moving into FLETCH. | Pass |
| ROUTE Researcher | Geodata fetch ledgers can reuse manifest helpers while route scoring and geometry remain outside core. | Pass |
| CROP Indexer | Manifest-first ledgers provide stable inputs for CROP status and publisher indexes. | Pass |
| MDLOOM Publisher | Durable manifests and read-only reports can feed generated docs without docs becoming authoritative state. | Pass |
| CI/Release Engineer | CI can smoke manifest writes, reloads, and cache-index rows from local files. | Pass |

## Panel reviewer findings

| Reviewer | Finding | Disposition |
|---|---|---|
| F-M1 Manifest Contract | Schema validation is explicit for cache manifest JSON reads and writes. | Pass |
| F-M2 Merge Semantics | Batch upsert reuses the same cache-key replacement rule as single-entry upsert. | Pass |
| F-M3 Consumer Boundary | Mock-client expansion and query logic remain adapter-side examples, not core behavior. | Pass |
| F-M4 Offline Workflow | The persisted manifest can feed cache index/report commands without live source access. | Pass |
| F-M5 Documentation | README now describes the reusable manifest-first workflow and its non-activation boundary. | Pass |

## Blocking findings

None.

## Deferred risks

- Extremely large manifests may eventually need streaming JSON writers or
  chunked manifest shards.
- Any future manifest lockfile/concurrency helper should remain optional and
  product-neutral.
