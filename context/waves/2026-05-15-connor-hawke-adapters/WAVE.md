# Wave: Connor Hawke Adapters

## Goal

Keep product-specific source construction outside `fletch-core` while giving
adapters a stable, product-neutral handoff surface for source catalogs,
registry validation, archive expansion, and fetch planning.

## Affected crates and consumers

- `fletch-core`: adapter source reports, registry validation, archive expansion
  previews, and adapter handoff contracts.
- `fletch-cli`: read-only adapter/registry report commands.
- `fletch-mock-client`: Justice League villain-files adapter harness examples.
- Consumers: ICELINES, apportionment/BISECT, ROUTE, CROP, MDPATH, and PROOF.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Adapter source report | done | Added `fletch.adapter-sources.v1` source rows from registries without interpreting adapter semantics. |
| 02 | Registry validation report | done | Added `fletch.registry-validation.v1` missing-shaft, duplicate-ID, and adapter-source findings. |
| 03 | Archive expansion preview | pending | Preview one source expanding into many fletches without extracting archives. |
| 04 | Adapter handoff report | pending | Summarize adapter-owned registry, graph, and flight inputs for downstream tools. |

## Validation expectations

- Every pulse runs `cargo fmt`, `cargo test --workspace`, focused CLI smokes, and
  `git diff --check` when code changes.
- Adapter commands must remain read-only.
- `fletch-core` must not contain NHL, Census, route, or other product rules.

## Wave close gate

Before this wave can close or hand off to Overwatch, run the `.roles` review:

- Parliament: cache systems, provenance, offline release, adapter boundary,
  performance, and doc publisher.
- Editorial: scope keeper, contract checker, validation checker.
- Stakeholders: ICELINES, BISECT/apportionment, ROUTE, CROP, PROOF, CI/release.
- Panel reviewers: F-I1 through F-I6.

Blocking findings become additional Connor Hawke pulses. Non-blocking findings
must be documented as deferred risks or next-wave inputs.

## Non-goals

- FLETCH does not implement NHL, Census, route, or other product adapters in
  `fletch-core`.
- FLETCH does not parse product-specific archive contents.
- FLETCH does not decide product query, UI, legal, or scoring semantics.
