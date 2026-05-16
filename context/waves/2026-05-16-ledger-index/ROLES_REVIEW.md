# .roles Review: Ledger Index

## Scope

Reviewed Ledger Index pulses 01-03 against the `.roles` gate. This wave adds
compact derived cache-ledger indexes, lookup filters, and index diffs for
large-manifest workflows without replacing cache manifests or verification.

## Parliament findings

| Role | Finding | Disposition |
|---|---|---|
| Cache Systems Engineer | Cache index and index-diff reports are read-only derived views over manifests or prior index JSON and do not mutate cache roots, ledgers, aliases, partitions, or quivers. | Pass |
| Provenance Auditor | Index rows preserve dataset IDs, versions, cache keys, object hashes, relative paths, byte counts, and verified flags; diffs retain base/candidate hash and trust evidence. | Pass |
| Offline Release Operator | Index generation, lookup, and index diff can run entirely from local manifest/index files without live network access. | Pass |
| Adapter Boundary Keeper | Index contracts are cache-ledger level primitives and do not encode NHL, Census, route, or dashboard semantics. | Pass |
| Performance Engineer | Large ledgers now have compact index rows, exact lookup filters, bounded output, and index-to-index diffs before deeper byte verification. | Pass |
| PROOF/CROP Publisher | CROP/PROOF can render or index compact ledger rows and change summaries while linking back to authoritative manifests. | Pass |

## Editorial findings

| Role | Finding | Disposition |
|---|---|---|
| Scope Keeper | Ledger Index stayed within derived report/index contracts and did not add a persistent database, daemon, activation semantics, or product dashboards. | Pass |
| Contract Checker | Foundation docs and code consistently name `fletch.cache-index.v1` and `fletch.cache-index-diff.v1`; manifests remain the source of truth. | Pass |
| Validation Checker | Pulses 01-03 ran `cargo fmt`, `cargo test --workspace`, focused CLI smokes, and `git diff --check`. | Pass |

## Stakeholder findings

| Stakeholder | Finding | Disposition |
|---|---|---|
| ICELINES Maintainer | Large season/history cache manifests can be indexed, looked up, and compared without moving hockey snapshot semantics into FLETCH. | Pass |
| BISECT/Apportionment Analyst | Large evidence ledgers can be summarized and compared by cache key/hash while legal and data-release semantics stay outside core. | Pass |
| ROUTE Researcher | Geodata ledgers can use compact lookup/diff reports without route-specific scoring or geometry logic. | Pass |
| CROP Indexer | `fletch.cache-index.v1` and diff summaries provide compact corpus-health inputs. | Pass |
| PROOF Publisher | Generated status pages can show compact ledger rows and changes without treating generated docs as authoritative state. | Pass |
| CI/Release Engineer | CI can compare compact indexes to decide which rows need deeper verify/status checks. | Pass |

## Panel reviewer findings

| Reviewer | Finding | Disposition |
|---|---|---|
| F-I1 Distributed Cache | Indexes preserve content-addressing evidence and do not alter cache object placement or ledger identity. | Pass |
| F-I2 Reproducibility | Index and diff output is deterministic from deterministic manifest/index inputs. | Pass |
| F-I3 Data Integration | The contracts are product-neutral cache-ledger reports that adapters and consumers can reuse. | Pass |
| F-I4 Offline Packaging | Compact indexes and diffs support offline bundle/bootstrap inspection before import or activation. | Pass |
| F-I5 Security and Trust | Index diffs do not bypass hash verification; they identify rows that may require deeper inspection. | Pass |
| F-I6 Documentation Pipeline | The index and diff schemas are suitable for generated CROP/PROOF/MDPATH status views. | Pass |

## Blocking findings

None.

## Deferred risks

- Extremely large ledgers may eventually need streaming index writers or cursor
  tokens beyond bounded JSON reports.
- Any future persistent index store should remain optional and derived from
  manifests, never the authoritative cache ledger.
