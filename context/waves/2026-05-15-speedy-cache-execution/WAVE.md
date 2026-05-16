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
| 03 | File shaft path hardening | pending | Tighten local file shaft handling and status messages for local-only sources. |
| 04 | Cache-hit ledger trust | pending | Prepare persistent ledger-backed skip decisions beyond single-command manifests. |

## Validation expectations

- Every pulse runs `cargo fmt`, `cargo test --workspace`, focused CLI smokes, and
  `git diff --check` when code changes.
- Mock-client pulses must keep domain query logic outside `fletch-core`.
- Fetch execution pulses must include offline/no-live behavior checks when live
  source behavior changes.

## Non-goals

- FLETCH does not interpret NHL, Census, route, or villain-file semantics.
- FLETCH does not activate aliases or product views during fetch execution.
- FLETCH does not make generated CROP/PROOF views the source of truth.
