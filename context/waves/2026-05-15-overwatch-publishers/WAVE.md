# Wave: Overwatch Publishers

## Goal

Expose MDCROP/PROOF-friendly publisher contracts that render or index FLETCH
machine state as local status, graph, tip, quiver, and adapter views without
making generated artifacts the source of truth.

## Affected crates and consumers

- `fletch-core`: MDCROP index, PROOF document manifest, local URL map, and
  publisher bundle contracts.
- `fletch-cli`: read-only publisher report commands.
- `fletch-mock-client`: Justice League villain-file status and bundle examples.
- Consumers: ICELINES, apportionment/BISECT, ROUTE, MDCROP, MDPATH, and PROOF.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | MDCROP index report | done | Added `fletch.mdcrop-index.v1` rows over cache status, graph nodes, graph edges, and tips. |
| 02 | PROOF document manifest | done | Added `fletch.proof-docs.v1` document anchors over MDCROP index rows. |
| 03 | Local URL map | done | Added `fletch.local-url-map.v1` stable local URLs over PROOF document anchors. |
| 04 | Publisher bundle report | done | Added `fletch.publisher-bundle.v1` summary over MDCROP, PROOF, URL, quiver, and adapter views. |
| 05 | Publisher slices | done | Added bounded MDCROP/PROOF/URL publisher slices for large local surfaces. |

## Validation expectations

- Every pulse runs `cargo fmt`, `cargo test --workspace`, focused CLI smokes, and
  `git diff --check` when code changes.
- Publisher commands must remain read-only.
- Generated/document views must point back to machine contracts.

## Wave close gate

Before all Arrow phases are considered delivered, run the `.roles` review:

- Parliament: cache systems, provenance, offline release, adapter boundary,
  performance, and doc publisher.
- Editorial: scope keeper, contract checker, validation checker.
- Stakeholders: ICELINES, BISECT/apportionment, ROUTE, MDCROP, PROOF, CI/release.
- Panel reviewers: F-I1 through F-I6.

Blocking findings become additional Overwatch pulses. Non-blocking findings must
be documented as deferred risks.

## Non-goals

- FLETCH does not require a specific PROOF backend, renderer, or web framework.
- FLETCH does not make Markdown/HTML/docs the source of truth.
- FLETCH does not embed consumer UI logic in `fletch-core`.
