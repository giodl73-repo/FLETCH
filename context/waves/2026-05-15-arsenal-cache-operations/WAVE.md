# Wave: Arsenal Cache Operations

## Goal

Make cache ledgers operationally useful after fetch: summarize, verify, report,
and plan cleanup without product-specific semantics or hidden mutation.

## Affected crates and consumers

- `fletch-core`: manifest inspection, cache-operation reports, prune/status
  planning, and ledger-backed decisions.
- `fletch-cli`: cache operation commands over `fletch.cache-manifest.v1`.
- `fletch-mock-client`: Justice League villain-files smoke coverage for generic
  cache health, orphan planning, and offline status.
- Consumers: ICELINES, apportionment/BISECT, ROUTE, MDCROP, MDPATH, and PROOF.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Cache summary report | done | Added aggregate cache health counts and byte totals over manifest status. |
| 02 | Verify report contract | done | Promoted cache verification into `fletch.cache-verify.v1` with summary and status rows. |
| 03 | Offline report command | done | Added `fletch.cache-offline.v1` readiness reports for no-live bootstrap flows. |
| 04 | Prune safety expansion | done | Added `fletch.cache-prune.v1` safety metadata while keeping prune non-destructive. |

## Validation expectations

- Every pulse runs `cargo fmt`, `cargo test --workspace`, focused CLI smokes, and
  `git diff --check` when code changes.
- Cache operations remain manifest-led and do not fetch live sources.
- Destructive cache mutation is out of scope unless a later pulse explicitly adds
  a reviewed execution command.

## Wave close gate

Before this wave can close or hand off to Red Arrow, run the `.roles` review:

- Parliament: cache systems, provenance, offline release, adapter boundary,
  performance, and doc publisher.
- Editorial: scope keeper, contract checker, validation checker.
- Stakeholders: ICELINES, BISECT/apportionment, ROUTE, MDCROP, PROOF, CI/release.
- Panel reviewers: F-I1 through F-I6.

Blocking findings become additional Arsenal pulses. Non-blocking findings must be
documented as deferred risks or next-wave inputs.

Status: complete in `ROLES_REVIEW.md`; no blocking findings. Deferred findings
feed the Red Arrow merge/aliases wave.

## Non-goals

- FLETCH does not interpret domain datasets or product-specific stale semantics.
- FLETCH does not activate aliases, partitions, or product views during cache
  operations.
- FLETCH does not delete cache objects in this wave without an explicit reviewed
  destructive command.
