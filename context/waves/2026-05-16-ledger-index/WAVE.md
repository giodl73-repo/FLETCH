# Wave: Ledger Index

## Goal

Add product-neutral derived indexes over FLETCH cache ledgers so large manifests
can feed lookup, publisher, and automation workflows without making indexes the
source of truth.

## Affected crates and consumers

- `fletch-core`: cache ledger index contract and helpers.
- `fletch-cli`: read-only cache index report command.
- Consumers: ICELINES, BISECT/apportionment, ROUTE, CROP, PROOF, and CI/release
  tooling that need compact ledger views.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Cache index report | done | Added `fletch.cache-index.v1` compact rows over manifest entries. |
| 02 | Cache index lookup | done | Added product-neutral filters and bounded output for cache index rows. |
| 03 | Cache index diff | done | Added `fletch.cache-index-diff.v1` comparisons for compact index changes. |

## Validation expectations

- Every pulse runs `cargo fmt`, `cargo test --workspace`, focused CLI smokes, and
  `git diff --check` when code changes.
- Index reports must remain read-only derived views over manifests.
- Cache manifests remain the authoritative ledger; indexes must not replace
  verification, status, merge, or activation evidence.

## Wave close gate

Before closing, run a `.roles` review with:

- Parliament: cache systems, provenance, offline release, adapter boundary,
  performance, and doc publisher.
- Editorial: scope keeper, contract checker, validation checker.
- Stakeholders: ICELINES, BISECT/apportionment, ROUTE, CROP, PROOF, CI/release.
- Panel reviewers: F-I1 through F-I6.

## Non-goals

- FLETCH does not introduce a persistent database or background index daemon.
- FLETCH does not treat derived indexes as activation state.
- FLETCH does not remove or weaken hash verification against cached bytes.
