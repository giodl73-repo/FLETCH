# Wave: Speedy II Quivers

## Goal

Make portable quiver bundles first-class, verifiable handoff artifacts that can
stage cached bytes and metadata for offline bootstrap, merge preview, and
publisher views without product-specific install logic.

## Affected crates and consumers

- `fletch-core`: quiver summary, verification/status, graph edges, and
  merge-ready bundle reports.
- `fletch-cli`: read-only quiver report commands plus existing export/import
  flows.
- `fletch-mock-client`: Justice League villain-files quiver bootstrap examples.
- Consumers: ICELINES, apportionment/BISECT, ROUTE, CROP, MDPATH, and PROOF.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Quiver summary report | done | Added `fletch.quiver-summary.v1` bundle identity, byte, and verification totals. |
| 02 | Quiver verify report | done | Added `fletch.quiver-verify.v1` per-member verification before import or merge. |
| 03 | Quiver graph edges | done | Added quiver-to-member `fletch.graph.v1` edges for CROP/PROOF indexing. |
| 04 | Merge-ready bundle report | done | Added `fletch.quiver-merge-ready.v1` candidate merge/alias rows for bundle members. |

## Validation expectations

- Every pulse runs `cargo fmt`, `cargo test --workspace`, focused CLI smokes, and
  `git diff --check` when code changes.
- Quiver report commands must not copy, delete, or activate cached objects.
- Import remains stage-first and must verify bytes before promotion.

## Wave close gate

Before this wave can close or hand off to Connor Hawke, run the `.roles` review:

- Parliament: cache systems, provenance, offline release, adapter boundary,
  performance, and doc publisher.
- Editorial: scope keeper, contract checker, validation checker.
- Stakeholders: ICELINES, BISECT/apportionment, ROUTE, CROP, PROOF, CI/release.
- Panel reviewers: F-I1 through F-I6.

Blocking findings become additional Speedy II pulses. Non-blocking findings must
be documented as deferred risks or next-wave inputs.

## Non-goals

- FLETCH does not decide which consumer views should activate after import.
- FLETCH does not interpret product-specific bundle contents.
- FLETCH does not replace release packaging systems; it provides verifiable
  cache/ledger artifacts for them.
