# FLETCH - Role Index

Four tiers of review roles. Read this before opening any role file.

---

## Parliament roles (6 voices)

Adversarial infrastructure voices for FLETCH design reviews. They keep the
shared fetch/cache layer useful across consumers without letting any one product
turn `fletch-core` into domain code.

| File | Voice | Primary tension |
|---|---|---|
| `parliament/cache-systems-engineer.md` | Cache Systems Engineer | Correct cache identity, freshness, atomicity, and promotion vs. feature pressure |
| `parliament/provenance-auditor.md` | Provenance Auditor | Traceable shafts, hashes, and ledgers vs. convenient undocumented pulls |
| `parliament/offline-release-operator.md` | Offline Release Operator | Bootstrap, quivers, and offline use vs. live-network assumptions |
| `parliament/adapter-boundary-keeper.md` | Adapter Boundary Keeper | Product-neutral core vs. leaking NHL/Census/route semantics |
| `parliament/performance-engineer.md` | Performance Engineer | Fast verifies, skips, and bulk pulls vs. slow universal abstractions |
| `parliament/doc-publisher.md` | PROOF/CROP Publisher | Human-readable generated docs/status vs. machine-only manifests |

---

## Editorial roles (3 voices)

Quality gates before a wave or pulse is considered ready. Run after parliament,
not instead of it.

| File | Role | Checks |
|---|---|---|
| `editorial/scope-keeper.md` | Scope Keeper | FLETCH stays infrastructure, not product logic |
| `editorial/contract-checker.md` | Contract Checker | Schemas, names, examples, and compatibility remain coherent |
| `editorial/validation-checker.md` | Validation Checker | Pulse docs state concrete fmt/test/smoke expectations |

---

## Stakeholder roles (consumer lenses)

These are not reviewers; they are lenses for understanding how a shared
fetch/cache primitive serves real downstream tools.

| File | Stakeholder | Primary concern |
|---|---|---|
| `stakeholders/icelines-maintainer.md` | ICELINES Maintainer | On-demand NHL data, snapshots, favorites, no-live CI, offline bundles |
| `stakeholders/bisect-apportionment-analyst.md` | BISECT/Apportionment Analyst | Large Census/election/geography pulls, checksums, reproducibility |
| `stakeholders/route-researcher.md` | ROUTE Researcher | Geodata archives, source catalogs, cacheable research inputs |
| `stakeholders/crop-indexer.md` | CROP Indexer | Indexable ledgers, status pages, corpus health, broken/missing data |
| `stakeholders/proof-publisher.md` | PROOF Publisher | Generated Markdown/backend views from FLETCH contracts |
| `stakeholders/ci-release-engineer.md` | CI/Release Engineer | Deterministic offline tests, bootstrap speed, artifact promotion |

---

## Panel reviewer roles (6 domain experts)

Local expert-style reviewers for FLETCH technical proposals. These are
role-based infrastructure reviewers, not product stakeholders.

| File | Reviewer | Expertise |
|---|---|---|
| `panel-reviewer/F-I1.md` | Distributed Cache Reviewer | cache invalidation, manifests, content addressing |
| `panel-reviewer/F-I2.md` | Reproducibility Reviewer | provenance, deterministic runs, audit ledgers |
| `panel-reviewer/F-I3.md` | Data Integration Reviewer | source registries, adapters, schema boundaries |
| `panel-reviewer/F-I4.md` | Offline Packaging Reviewer | bundles/quivers, import/export, bootstrap installs |
| `panel-reviewer/F-I5.md` | Security and Trust Reviewer | checksums, path safety, untrusted archives, transport risk |
| `panel-reviewer/F-I6.md` | Documentation Pipeline Reviewer | PROOF/CROP/MDPATH integration and generated status docs |

