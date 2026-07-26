# Wave: Report Scaling

## Goal

Harden large, read-only FLETCH reports with bounded slices and focused filters
while keeping source registries, manifests, quivers, and generated publisher
artifacts as the authoritative contracts.

## Affected crates and consumers

- `fletch-core`: product-neutral slice helpers for existing report schemas.
- `fletch-cli`: read-only slice/filter flags on report commands.
- Consumers: ICELINES, BISECT/apportionment, ROUTE, MDCROP, MDLOOM, and CI/release
  tooling that need smaller local report artifacts.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Report row slices | done | Added bounded slices and focused filters for registry, partition, active-set, and quiver merge-ready report rows. |

## Validation expectations

- Every pulse runs `cargo fmt`, `cargo test --workspace`, focused CLI smokes, and
  `git diff --check` when code changes.
- Slice flags must not mutate cache, registry, quiver, alias, partition, or
  publisher state.
- Sliced output remains derived JSON under the same machine contracts; consumers
  own any dashboard or product-specific presentation.

## Wave close gate

Before closing, run a `.roles` review with:

- Parliament: cache systems, provenance, offline release, adapter boundary,
  performance, and doc publisher.
- Editorial: scope keeper, contract checker, validation checker.
- Stakeholders: ICELINES, BISECT/apportionment, ROUTE, MDCROP, MDLOOM, CI/release.
- Panel reviewers: F-I1 through F-I6.

## Non-goals

- FLETCH does not build consumer dashboards or product-specific report layouts.
- FLETCH does not add activation semantics to sliced reports.
- FLETCH does not replace streaming output if future extremely large reports need
  it; this wave adds bounded derived report slices first.
