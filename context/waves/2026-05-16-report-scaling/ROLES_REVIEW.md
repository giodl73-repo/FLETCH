# .roles Review: Report Scaling

## Scope

Reviewed Report Scaling pulse 01 against the `.roles` gate. This wave hardens
large derived report surfaces with bounded, read-only row slices and focused
filters.

## Parliament findings

| Role | Finding | Disposition |
|---|---|---|
| Cache Systems Engineer | Slice helpers operate on already-derived reports and do not mutate cache roots, ledgers, aliases, partitions, quivers, registries, or publisher outputs. | Pass |
| Provenance Auditor | Sliced rows retain the original schema names, IDs, hashes, cache keys, source URLs, and verification evidence carried by the source reports. | Pass |
| Offline Release Operator | All sliced reports are generated from local manifests, registries, quivers, or prior report JSON without live network access. | Pass |
| Adapter Boundary Keeper | Filters are product-neutral row selectors such as adapter-owned status, severity, active state, and candidate status; no NHL, Census, route, or dashboard semantics entered core. | Pass |
| Performance Engineer | Large registry, partition, active-set, archive-preview, validation, and quiver merge-ready reports can now emit bounded rows with `--offset` and `--limit`. | Pass |
| PROOF/MDCROP Publisher | Smaller derived report slices are easier for MDCROP/PROOF backends to index or render while retaining links to machine contracts. | Pass |

## Editorial findings

| Role | Finding | Disposition |
|---|---|---|
| Scope Keeper | Report Scaling stayed within read-only report shaping and did not add activation, product dashboard, or consumer query behavior. | Pass |
| Contract Checker | Existing schema contracts remain stable; slices reuse the same report schemas with counts recomputed for emitted rows. | Pass |
| Validation Checker | Pulse 01 ran `cargo fmt`, `cargo test --workspace`, focused CLI smoke, and `git diff --check`. | Pass |

## Stakeholder findings

| Stakeholder | Finding | Disposition |
|---|---|---|
| ICELINES Maintainer | Large season/source/query handoff reports can be sampled or focused without moving hockey semantics into FLETCH. | Pass |
| BISECT/Apportionment Analyst | Large evidence and district/source report rows can be bounded for automation and review while keeping legal/data semantics outside core. | Pass |
| ROUTE Researcher | Route/geodata source health and partition rows can be sliced without route scoring or geospatial logic in FLETCH. | Pass |
| MDCROP Indexer | MDCROP can consume smaller report chunks for local corpus/status indexing. | Pass |
| PROOF Publisher | PROOF backends can render focused sections without treating generated views as source truth. | Pass |
| CI/Release Engineer | Bounded report output reduces automation noise for release checks while preserving deterministic local inputs. | Pass |

## Panel reviewer findings

| Reviewer | Finding | Disposition |
|---|---|---|
| F-I1 Distributed Cache | Slices preserve cache-key and hash evidence and do not alter cache object state. | Pass |
| F-I2 Reproducibility | Offset/limit/filter choices produce deterministic subsets from deterministic source reports. | Pass |
| F-I3 Data Integration | Registry and adapter filters remain schema-level selectors rather than product-specific integrations. | Pass |
| F-I4 Offline Packaging | Quiver merge-ready slices support offline bundle review before import or activation. | Pass |
| F-I5 Security and Trust | Sliced reports do not bypass verification flags, archive preview safety, or unverified candidate status. | Pass |
| F-I6 Documentation Pipeline | The wave directly improves generated documentation/report pipelines by bounding source JSON artifacts. | Pass |

## Blocking findings

None.

## Deferred risks

- Extremely large reports may eventually need streaming writers or cursor tokens
  beyond offset/limit slices.
- Consumers should keep product-specific dashboards and visual grouping in their
  repos or publisher backends, not `fletch-core`.
