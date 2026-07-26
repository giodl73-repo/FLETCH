# .roles Review: Speedy Cache Execution

## Scope

Reviewed Speedy pulses 01-09 against the `.roles` gate before advancing to the
Arsenal cache-operations phase.

## Parliament findings

| Role | Finding | Disposition |
|---|---|---|
| Cache Systems Engineer | Cache identity now includes dataset, version, source kind, source URL, and generic headers; fetches use temp promotion, freshness policy, trust-aware cache hits, and ledger output upsert. | Pass |
| Provenance Auditor | Ledger entries retain source URL, dataset ID, cache key, relative path, hash, bytes, fetched timestamp, verification, and retry provenance. | Pass |
| Offline Release Operator | Offline execution distinguishes fresh hits, stale/bypassed existing objects, and missing objects; saved plans and quivers remain usable for bootstrap flows. | Pass |
| Adapter Boundary Keeper | `fletch-core` stayed product-neutral; examples mention ICELINES/ROUTE/BISECT as consumers but no product semantics entered core. | Pass |
| Performance Engineer | Fetch streams bytes while hashing; cache hits avoid live fetch; ledger upsert avoids rewriting unrelated entries semantically, though future Arsenal work should optimize large-ledger operations. | Follow-up |
| MDLOOM/CROP Publisher | Flights, tips, publish reports, ledgers, and stable pulse docs are renderable/indexable; generated docs remain views over contracts. | Pass |

## Editorial findings

| Role | Finding | Disposition |
|---|---|---|
| Scope Keeper | Fetch remains acquisition, not merge/activation; pull remains reserved for later fetch-plus-merge semantics. | Pass |
| Contract Checker | README, foundation spec, wave docs, pulse docs, and CLI surfaces agree on retry, file shafts, headers, saved plans, plan validation, and ledger upsert. | Pass |
| Validation Checker | Each code pulse recorded `cargo fmt`, `cargo test --workspace`, focused CLI smoke, and `git diff --check`; docs-only goal update ran diff validation. | Pass |

## Stakeholder findings

| Stakeholder | Finding | Disposition |
|---|---|---|
| ICELINES Maintainer | Current/historical freshness, no-live CI, focused flights, saved plans, and cache-hit trust are now represented generically. | Pass |
| BISECT/Apportionment Analyst | Large verified downloads can be ledgered and reused; missing/stale offline failures are explicit. | Pass |
| ROUTE Researcher | HTTP/file shafts, headers, local file handling, and saved plans support source catalogs without route semantics. | Pass |
| CROP Indexer | Ledgers can now accumulate entries through output upsert and publish status/tips for indexing. | Pass |
| MDLOOM Publisher | Saved plans, flights, tips, publish reports, and stable specs provide renderable source material. | Pass |
| CI/Release Engineer | Temp promotion, invalid plan rejection before live source access, offline diagnostics, and focused CLI smokes support deterministic automation. | Pass |

## Panel reviewer findings

| Reviewer | Finding | Disposition |
|---|---|---|
| F-I1 Distributed Cache | Deterministic identity and atomic promotion are covered; future cache operations should add stronger multi-entry ledger scaling. | Follow-up |
| F-I2 Reproducibility | Hashes, bytes, source identity, retry status, and trusted-ledger cache hits support auditability. | Pass |
| F-I3 Data Integration | Headers, saved plans, registries, and adapter-owned shafts remain schema-boundary friendly. | Pass |
| F-I4 Offline Packaging | Quiver staging plus offline stale/missing distinctions are sufficient for this phase; Arsenal should deepen status operations. | Follow-up |
| F-I5 Security and Trust | File path hardening, checksum failures, trusted manifest validation, and invalid plan rejection surface failures explicitly. | Pass |
| F-I6 Documentation Pipeline | CROP/MDLOOM-facing publish reports and docs are contract-derived, not source-of-truth replacements. | Pass |

## Blocking findings

None.

## Deferred to Arsenal

- Add richer cache-operation commands around existing ledgers: verification
  workflows, status aggregation, prune planning, and offline reports.
- Consider performance behavior for very large ledgers when listing, verifying,
  upserting, and publishing status.
- Keep role-review closure as a required gate before Red Arrow merge/alias work.
