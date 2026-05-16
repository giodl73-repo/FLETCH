# .roles Review: Oracle Partitions and Rollups

## Scope

Reviewed Oracle pulses 01-04 against the `.roles` gate before advancing to
Speedy II quivers.

## Parliament findings

| Role | Finding | Disposition |
|---|---|---|
| Cache Systems Engineer | Partition, rollup, invalidation, and active-set reports are manifest/state-led and read-only. | Pass |
| Provenance Auditor | Partition and active-set rows carry dataset IDs, cache keys, hashes, paths, byte counts, and verification evidence. | Pass |
| Offline Release Operator | Oracle contracts are local JSON reports, so offline query/readiness flows can inspect partitions without live fetches. | Pass |
| Adapter Boundary Keeper | Partition IDs and group IDs are generic; NHL seasons, Census vintages, geodata tiles, and other semantics remain adapter-owned. | Pass |
| Performance Engineer | Reports operate over state rows rather than object bytes; future very large partition sets may need indexed summaries. | Follow-up |
| PROOF/CROP Publisher | Named partition, rollup, invalidation, and active-set schemas are directly renderable/indexable. | Pass |

## Editorial findings

| Role | Finding | Disposition |
|---|---|---|
| Scope Keeper | Oracle stayed within product-neutral partition metadata and did not add product query execution. | Pass |
| Contract Checker | Foundation spec and code consistently name `fletch.partition-state.v1`, `fletch.rollup-preview.v1`, `fletch.partition-invalidation.v1`, and `fletch.active-partition-set.v1`. | Pass |
| Validation Checker | Every pulse ran `cargo fmt`, `cargo test --workspace`, focused CLI smoke, and `git diff --check`. | Pass |

## Stakeholder findings

| Stakeholder | Finding | Disposition |
|---|---|---|
| ICELINES Maintainer | Season/game/date partitions can be represented by IDs and groups without NHL logic in core. | Pass |
| BISECT/Apportionment Analyst | Year/vintage/district evidence sets can be pinned and inspected as partition rows with hashes. | Pass |
| ROUTE Researcher | Geodata archive members and rollups can be tracked generically without route-specific scoring. | Pass |
| CROP Indexer | Partition and active-set reports are stable corpus-health and query-state inputs. | Pass |
| PROOF Publisher | PROOF can render partition, rollup, invalidation, and active-set views from machine JSON. | Pass |
| CI/Release Engineer | Read-only CLI reports are suitable for automation and release promotion checks. | Pass |

## Panel reviewer findings

| Reviewer | Finding | Disposition |
|---|---|---|
| F-I1 Distributed Cache | Partition reports reference deterministic cache keys and never mutate cached bytes. | Pass |
| F-I2 Reproducibility | Active sets and invalidation reports preserve audit evidence for repeated runs. | Pass |
| F-I3 Data Integration | Contracts accept adapter-defined IDs without schema churn or core product logic. | Pass |
| F-I4 Offline Packaging | Partition and rollup metadata provide inputs for later quiver/bootstrap activation planning. | Pass |
| F-I5 Security and Trust | Reports surface missing partition IDs and preserve verification flags instead of silently succeeding. | Pass |
| F-I6 Documentation Pipeline | The named schemas are suitable for generated query-facing status pages. | Pass |

## Blocking findings

None.

## Deferred to Speedy II

- Quiver export/import should preserve partition, rollup, invalidation, and
  active-set evidence as portable metadata.
- Large partition sets may need index-friendly summaries before publisher waves.
- Future activation mutations must verify cache keys and hashes before changing
  aliases or active partition sets.
