# .roles Review: Overwatch Publishers

## Scope

Reviewed Overwatch pulses 01-04 against the `.roles` gate. This closes the
final Arrow phase listed in `PHASES.md`.

## Parliament findings

| Role | Finding | Disposition |
|---|---|---|
| Cache Systems Engineer | Publisher reports are read-only derived views and do not alter cache, ledger, alias, quiver, or adapter state. | Pass |
| Provenance Auditor | MDCROP, PROOF, URL, and publisher bundle rows retain source schema references back to authoritative machine contracts. | Pass |
| Offline Release Operator | Publisher reports can be generated from local manifests/indexes without live fetches. | Pass |
| Adapter Boundary Keeper | Publisher contracts remain backend-neutral and do not embed product UI or adapter semantics. | Pass |
| Performance Engineer | Reports summarize existing rows and read-only MDCROP/PROOF/URL outputs now support bounded slices for large local publisher surfaces. | Pass |
| PROOF/MDCROP Publisher | Overwatch directly provides index rows, document anchors, URL maps, and bundle summaries for local rendering/indexing. | Pass |

## Editorial findings

| Role | Finding | Disposition |
|---|---|---|
| Scope Keeper | Generated views remain derived from contracts and do not become the source of truth. | Pass |
| Contract Checker | Foundation spec and code consistently name `fletch.mdcrop-index.v1`, `fletch.proof-docs.v1`, `fletch.local-url-map.v1`, and `fletch.publisher-bundle.v1`. | Pass |
| Validation Checker | Every pulse ran `cargo fmt`, `cargo test --workspace`, focused CLI smoke, and `git diff --check`. | Pass |

## Stakeholder findings

| Stakeholder | Finding | Disposition |
|---|---|---|
| ICELINES Maintainer | ICELINES can render cache/status/favorite-pack views while keeping NHL UI logic outside FLETCH. | Pass |
| BISECT/Apportionment Analyst | Evidence and partition statuses can be published with machine-contract provenance. | Pass |
| ROUTE Researcher | Geodata and route-source health can be indexed/rendered without route-specific code in core. | Pass |
| MDCROP Indexer | `fletch.mdcrop-index.v1` provides a direct local corpus index input. | Pass |
| PROOF Publisher | PROOF document and URL contracts support multiple renderer/backends without core coupling. | Pass |
| CI/Release Engineer | Publisher bundle summaries are concise automation artifacts for release/status checks. | Pass |

## Panel reviewer findings

| Reviewer | Finding | Disposition |
|---|---|---|
| F-I1 Distributed Cache | Publisher contracts reference cache status and ledger evidence without mutation. | Pass |
| F-I2 Reproducibility | Source schema references preserve auditability from generated views back to JSON contracts. | Pass |
| F-I3 Data Integration | MDCROP/PROOF outputs remain backend-neutral and adapter-friendly. | Pass |
| F-I4 Offline Packaging | Local publisher reports complement quiver/bootstrap workflows without network access. | Pass |
| F-I5 Security and Trust | Generated URLs and docs do not bypass verification status or promote untrusted bytes. | Pass |
| F-I6 Documentation Pipeline | The wave delivers the requested local documentation/indexing contracts. | Pass |

## Blocking findings

None.

## Deferred risks

- Extremely large publisher surfaces may eventually need streaming output beyond bounded slices.
- Future PROOF backends should keep generated Markdown/HTML linked to these
  source contracts instead of duplicating state.
- Consumer-specific dashboards should live in consumer repos or publisher
  backends, not `fletch-core`.
