# Wave: Manifest Maintenance

## Goal

Make manifest-first consumer workflows easier by providing product-neutral cache
manifest merge helpers and CLI behavior that let consumers maintain durable
ledgers without reimplementing FLETCH merge rules.

## Affected crates and consumers

- `fletch-core`: cache manifest merge/upsert helpers.
- `fletch-cli`: manifest write paths that exercise the shared helpers.
- Consumers: ICELINES, BISECT/apportionment, ROUTE, MDCROP, MDLOOM, and CI/release
  tooling that keep long-lived cache manifests as the fetch ledger.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Batch manifest upsert | done | Added `upsert_cache_manifest_entries` for validated multi-entry manifest merges. |
| 02 | Manifest file helpers | done | Added reusable cache manifest JSON read/write helpers without turning FLETCH into a manifest daemon. |
| 03 | Consumer smoke docs | done | Documented manifest-first consumer patterns with cache index/report handoffs. |
| 04 | Cache index gate contract | done | Added product-neutral `fletch.cache-index-gate.v1` reports and CLI gating over consumer-supplied expected IDs. |
| 05 | Registry-backed index gate | done | Let `fletch cache index-gate` derive expected IDs from `fletch.registry.v1` files. |

## Validation expectations

- Every pulse runs `cargo fmt`, `cargo test --workspace`, focused CLI smokes, and
  `git diff --check` when code changes.
- Cache manifests remain the authoritative ledger for fetched objects.
- Merge helpers must validate the resulting manifest and preserve cache-root
  consistency.

## Wave close gate

Before closing, run a `.roles` review with:

- Parliament: cache systems, provenance, offline release, adapter boundary, and
  consumer ergonomics.
- Editorial: scope keeper, contract checker, validation checker.
- Stakeholders: ICELINES, BISECT/apportionment, ROUTE, MDCROP, MDLOOM, CI/release.
- Panel reviewers: F-M1 through F-M5.

Status: complete in `ROLES_REVIEW.md`; no blocking findings. Deferred findings
feed future manifest streaming or lockfile/concurrency waves.

## Non-goals

- FLETCH does not own product activation, snapshot sealing, or domain parsing.
- FLETCH does not introduce a persistent database or background manifest daemon.
- FLETCH does not make derived indexes the source of truth.
