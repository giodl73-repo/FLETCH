# Wave: Red Arrow Merge and Aliases

## Goal

Separate fetch/cache acquisition from activation by adding product-neutral merge
preview, alias, label, and rollback contracts.

## Affected crates and consumers

- `fletch-core`: merge preview contracts, conflict detection, alias/label report
  shapes, and active-view metadata.
- `fletch-cli`: non-destructive merge and alias commands.
- `fletch-mock-client`: Justice League villain-files staged active-view and
  conflict-preview smoke coverage.
- Consumers: ICELINES, apportionment/BISECT, ROUTE, CROP, MDPATH, and MDLOOM.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Merge preview conflicts | done | Added non-destructive manifest merge previews with conflict detection. |
| 02 | Active alias contract | done | Added `fletch.alias-state.v1` aliases pointing at manifest entries without moving bytes. |
| 03 | Labels and pins | done | Added `fletch.label-state.v1` labels and pin metadata over alias state. |
| 04 | Rollback preview | done | Added `fletch.rollback-preview.v1` to preview alias restoration to label targets. |

## Validation expectations

- Every pulse runs `cargo fmt`, `cargo test --workspace`, focused CLI smokes, and
  `git diff --check` when code changes.
- Merge commands must preview or stage before any activation mutation.
- Cache reports remain evidence; cache presence is not activation.

## Wave close gate

Before this wave can close or hand off to Oracle, run the `.roles` review:

- Parliament: cache systems, provenance, offline release, adapter boundary,
  performance, and doc publisher.
- Editorial: scope keeper, contract checker, validation checker.
- Stakeholders: ICELINES, BISECT/apportionment, ROUTE, CROP, MDLOOM, CI/release.
- Panel reviewers: F-I1 through F-I6.

Blocking findings become additional Red Arrow pulses. Non-blocking findings must
be documented as deferred risks or next-wave inputs.

## Non-goals

- FLETCH does not interpret product-specific merge semantics.
- FLETCH does not make fetch or cache hit equivalent to activation.
- FLETCH does not mutate consumer databases or product views directly.
