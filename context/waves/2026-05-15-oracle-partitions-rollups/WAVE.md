# Wave: Oracle Partitions and Rollups

## Goal

Model durable partition sets and rollups as product-neutral cache metadata so
consumers can reason about query-facing active data without embedding domain
logic in FLETCH.

## Affected crates and consumers

- `fletch-core`: partition, rollup, invalidation, folding, and active partition
  report contracts.
- `fletch-cli`: non-destructive partition and rollup report commands.
- `fletch-mock-client`: Justice League dated threat partitions and year rollup
  examples.
- Consumers: ICELINES, apportionment/BISECT, ROUTE, CROP, MDPATH, and PROOF.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Partition state contract | done | Added `fletch.partition-state.v1` manifest-derived partition rows without product semantics. |
| 02 | Rollup edge preview | done | Added `fletch.rollup-preview.v1` parent/child edges over partition state. |
| 03 | Invalidation and folding metadata | done | Added `fletch.partition-invalidation.v1` stale/folded/superseded partition reports. |
| 04 | Active partition set | pending | Produce query-facing active partition reports from aliases, labels, and rollups. |

## Validation expectations

- Every pulse runs `cargo fmt`, `cargo test --workspace`, focused CLI smokes, and
  `git diff --check` when code changes.
- Partition and rollup commands must remain read-only.
- Query-facing active state must reference cache/alias evidence; cache presence
  alone is not activation.

## Wave close gate

Before this wave can close or hand off to Speedy II, run the `.roles` review:

- Parliament: cache systems, provenance, offline release, adapter boundary,
  performance, and doc publisher.
- Editorial: scope keeper, contract checker, validation checker.
- Stakeholders: ICELINES, BISECT/apportionment, ROUTE, CROP, PROOF, CI/release.
- Panel reviewers: F-I1 through F-I6.

Blocking findings become additional Oracle pulses. Non-blocking findings must be
documented as deferred risks or next-wave inputs.

## Non-goals

- FLETCH does not interpret product-specific partition semantics such as NHL
  seasons, Census vintages, legal districts, or route scoring.
- FLETCH does not execute product queries.
- FLETCH does not mutate consumer databases or active product views directly.
