# .roles Review: Red Arrow Merge and Aliases

## Scope

Reviewed Red Arrow pulses 01-04 against the `.roles` gate before advancing to
Oracle partitions and rollups.

## Parliament findings

| Role | Finding | Disposition |
|---|---|---|
| Cache Systems Engineer | Merge preview, alias, label, and rollback contracts are manifest/state-led and do not confuse cache acquisition with activation. | Pass |
| Provenance Auditor | Alias and label rows preserve cache keys, hashes, relative paths, and generated schema names so activation evidence remains traceable. | Pass |
| Offline Release Operator | Rollback previews and labels are local JSON state, so offline release flows can reason about active targets without live fetches. | Pass |
| Adapter Boundary Keeper | Red Arrow kept product semantics out of `fletch-core`; aliases and labels use generic IDs and cache metadata only. | Pass |
| Performance Engineer | Preview operations compare manifest/state rows without touching object bytes; future large active sets may need indexed state lookups. | Follow-up |
| PROOF/CROP Publisher | Named merge, alias, label, and rollback-preview schemas are renderable/indexable as local status artifacts. | Pass |

## Editorial findings

| Role | Finding | Disposition |
|---|---|---|
| Scope Keeper | The wave stayed within non-destructive activation metadata and did not add product-owned merge behavior. | Pass |
| Contract Checker | Code and foundation spec consistently name `fletch.merge-preview.v1`, `fletch.alias-state.v1`, `fletch.label-state.v1`, and `fletch.rollback-preview.v1`. | Pass |
| Validation Checker | Each pulse ran `cargo fmt`, `cargo test --workspace`, focused CLI smoke, and `git diff --check`. | Pass |

## Stakeholder findings

| Stakeholder | Finding | Disposition |
|---|---|---|
| ICELINES Maintainer | Current-season aliases, favorite-pack labels, and rollback previews can be modeled without NHL logic in core. | Pass |
| BISECT/Apportionment Analyst | Release-year or evidence-set labels can pin reproducible cache keys before active dataset changes. | Pass |
| ROUTE Researcher | Geodata active views can cite generic alias state and labels instead of ad hoc local paths. | Pass |
| CROP Indexer | Red Arrow state contracts are stable CROP inputs for active-view and rollback-readiness indexes. | Pass |
| PROOF Publisher | PROOF can render merge/alias/label/rollback previews while machine JSON remains the source of truth. | Pass |
| CI/Release Engineer | Non-destructive previews and JSON outputs are automation-friendly for promotion checks. | Pass |

## Panel reviewer findings

| Reviewer | Finding | Disposition |
|---|---|---|
| F-I1 Distributed Cache | Activation metadata references deterministic cache keys and does not mutate cached bytes. | Pass |
| F-I2 Reproducibility | Labels and rollback previews make active target restoration auditable from saved state. | Pass |
| F-I3 Data Integration | Contracts leave adapter-owned source construction outside `fletch-core`. | Pass |
| F-I4 Offline Packaging | Label and rollback state provide inputs for later quiver/bootstrap activation planning. | Pass |
| F-I5 Security and Trust | Red Arrow remains preview-only; hashes are carried forward before any future activation execution. | Pass |
| F-I6 Documentation Pipeline | Schemas and docs are suitable for generated active-view status pages. | Pass |

## Blocking findings

None.

## Deferred to Oracle

- Partition and rollup state should build on alias/label contracts without making
  cache presence equivalent to query activation.
- Large active partition sets may need indexed state summaries before publisher
  or adapter waves scale them up.
- Future mutation commands must verify cache keys and hashes before changing
  active aliases.
