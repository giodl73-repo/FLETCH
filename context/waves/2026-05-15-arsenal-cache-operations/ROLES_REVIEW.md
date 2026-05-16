# .roles Review: Arsenal Cache Operations

## Scope

Reviewed Arsenal pulses 01-04 against the `.roles` gate before advancing to Red
Arrow merge and aliases.

## Parliament findings

| Role | Finding | Disposition |
|---|---|---|
| Cache Systems Engineer | Cache operations are manifest-led and read-only; summary, verify, offline, and prune reports expose cache state without hidden mutation. | Pass |
| Provenance Auditor | Verification and offline reports retain per-entry source identity, hashes, bytes, paths, and status rows. | Pass |
| Offline Release Operator | `fletch.cache-offline.v1` gives no-live readiness counts and blocked status rows for bootstrap flows. | Pass |
| Adapter Boundary Keeper | Reports stay product-neutral and do not interpret NHL, Census, route, or other consumer semantics. | Pass |
| Performance Engineer | Summary and reports reuse streamed hash inspection; future large-ledger indexing may need incremental/status-cache support. | Follow-up |
| PROOF/CROP Publisher | Named report contracts (`cache-verify`, `cache-offline`, `cache-prune`) are directly indexable/renderable. | Pass |

## Editorial findings

| Role | Finding | Disposition |
|---|---|---|
| Scope Keeper | Arsenal did not add live fetch, merge, activation, or destructive delete behavior. | Pass |
| Contract Checker | Foundation spec and wave docs name the new report schemas and command semantics consistently. | Pass |
| Validation Checker | Every pulse ran `cargo fmt`, `cargo test --workspace`, focused CLI smoke, and `git diff --check`. | Pass |

## Stakeholder findings

| Stakeholder | Finding | Disposition |
|---|---|---|
| ICELINES Maintainer | Offline and summary reports support CI/no-live checks for current and historical data packs. | Pass |
| BISECT/Apportionment Analyst | Verify reports and prune plans support large reproducible cache audits without re-fetching. | Pass |
| ROUTE Researcher | Cache status reports can cite local/geodata inputs generically without ROUTE-specific code. | Pass |
| CROP Indexer | Named reports are stable corpus-health inputs for CROP indexing. | Pass |
| PROOF Publisher | Reports are suitable for generated status pages while ledgers remain source of truth. | Pass |
| CI/Release Engineer | Non-destructive prune and offline reports make automation safer. | Pass |

## Panel reviewer findings

| Reviewer | Finding | Disposition |
|---|---|---|
| F-I1 Distributed Cache | Manifest-led summaries and prune plans preserve cache invariants; destructive execution remains deferred. | Pass |
| F-I2 Reproducibility | Verification reports capture audit evidence without mutating cache state. | Pass |
| F-I3 Data Integration | Report schemas remain generic and adapter-friendly. | Pass |
| F-I4 Offline Packaging | Offline readiness reports are sufficient input for later quiver/bootstrap workflows. | Pass |
| F-I5 Security and Trust | Prune is explicitly non-destructive; hash mismatches and missing files remain visible. | Pass |
| F-I6 Documentation Pipeline | Named report contracts are CROP/PROOF-ready. | Pass |

## Blocking findings

None.

## Deferred to Red Arrow

- Merge/alias work must keep fetch/cache reports as evidence and avoid treating
  cache presence as activation.
- Conflict previews should reference cache-operation reports and verification
  status before any alias or active-view change.
- Large-ledger incremental indexing remains a performance follow-up for later
  publisher or cache-operation waves.
