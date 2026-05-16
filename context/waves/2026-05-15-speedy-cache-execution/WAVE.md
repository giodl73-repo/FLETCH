# Wave: Speedy Cache Execution

## Goal

Make generic shaft acquisition faster and more resilient while keeping FLETCH
focused on product-neutral fetch/cache execution.

## Affected crates and consumers

- `fletch-core`: fetch execution options, source acquisition, temp promotion, and
  verification behavior.
- `fletch-cli`: generic fetch execution controls.
- `fletch-mock-client`: Justice League villain-files smoke coverage for generic
  local file shafts.
- Consumers: ICELINES, apportionment/BISECT, ROUTE, CROP, MDPATH, and PROOF.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Retry and timeout controls | done | Added generic request timeout and retry-attempt fetch options plus CLI support. |
| 02 | Retry status reporting | done | Added attempt counts, retry counts, and last retryable error to fetch outcomes and manifest entries. |
| 03 | File shaft path hardening | done | Normalized common `file://` shaft paths and rejected empty local file shafts. |
| 04 | Cache-hit ledger trust | done | Added trusted-manifest cache-hit verification with ledger metadata preservation. |
| 05 | Offline stale diagnostics | done | Distinguished missing offline cache entries from stale or bypassed cached objects. |
| 06 | Header-aware shafts | done | Added CLI header flags and included generic source headers in cache identity. |
| 07 | Saved plan execution | done | Added `fetch-plan` so generated or checked-in `fletch.plan.v1` files can execute directly. |
| 08 | Saved plan validation | done | Validated plan schema and required identity fields before cache lookup or live fetch. |

## Validation expectations

- Every pulse runs `cargo fmt`, `cargo test --workspace`, focused CLI smokes, and
  `git diff --check` when code changes.
- Mock-client pulses must keep domain query logic outside `fletch-core`.
- Fetch execution pulses must include offline/no-live behavior checks when live
  source behavior changes.

## Wave close gate

Before this wave can close or hand off to Arsenal, run the `.roles` review:

- Parliament: cache systems, provenance, offline release, adapter boundary,
  performance, and doc publisher.
- Editorial: scope keeper, contract checker, validation checker.
- Stakeholders: ICELINES, BISECT/apportionment, ROUTE, CROP, PROOF, CI/release.
- Panel reviewers: F-I1 through F-I6.

Blocking findings become additional Speedy pulses. Non-blocking findings must be
documented as deferred risks or next-wave inputs.

## Non-goals

- FLETCH does not interpret NHL, Census, route, or villain-file semantics.
- FLETCH does not activate aliases or product views during fetch execution.
- FLETCH does not make generated CROP/PROOF views the source of truth.
