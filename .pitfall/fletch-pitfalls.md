# FLETCH Pitfalls

## FLETCH-PF-01: Fetch Contract Becomes Product Activation

**Status:** OPEN

**Pattern:** FLETCH fetch, cache, manifest, registry, partition, quiver, or
publish output is treated as activating a consumer product view or validating
domain meaning.

**Domain:** ICELINES data activation, BISECT derived geography/election inputs,
ROUTE geospatial interpretation, MDCROP selection, PROOF rendering, and local
demo copy.

**Detection difficulty:** FLETCH outputs are complete and machine-readable, so
downstream tools can accidentally skip the product-owned activation step.

**Structural solution:** Keep acquisition/activation boundaries in README,
compatibility policy, role reviews, and consumer handoff docs.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `docs/compatibility.md`, and
`.roles/parliament/adapter-boundary-keeper.md`.

## FLETCH-PF-02: Derived Publisher Output Becomes Ledger Authority

**Status:** OPEN

**Pattern:** Cache indexes, registry web exports, MDCROP rows, PROOF docs,
local URL maps, tips, or publish bundles are used as source-of-truth manifests
instead of derived reports.

**Domain:** Publisher commands, registry web UI, local dashboards, proof docs,
MDCROP indexing, and research-paper evidence packets.

**Detection difficulty:** Derived views are easier to inspect than raw manifests
and often contain enough detail to look authoritative.

**Structural solution:** Preserve report-derived wording and keep manifests or
registries as the only mutable input state.

**Test:** `crates/fletch-cli/tests/registry_web.rs` cites `FLETCH-PF-02` while
checking registry-web summary, search, detail, source, CSV, JSON, and direct
registry index paths as derived views.

**Evidence:** `README.md`, `docs/show/integrator-brief.md`,
`crates/fletch-cli/src/support/misc.rs`, and
`cargo test -p fletch-cli --test registry_web`.

## FLETCH-PF-03: Local Green Tests Become Consumer Compatibility

**Status:** OPEN

**Pattern:** FLETCH-local tests passing is treated as proof that ICELINES,
BISECT, ROUTE, MDCROP, MDPATH, or PROOF consumers can accept a changed public
API, schema, cache key, manifest, registry, or report contract.

**Domain:** Compatibility policy, dependency updates, portfolio snapshots,
release notes, and child-repo adoption waves.

**Detection difficulty:** FLETCH has a broad local test suite, so downstream
rehearsal can feel redundant until a consumer-owned integration breaks.

**Structural solution:** Keep ICELINES rehearsal and affected-consumer review
as explicit compatibility gates for foundation changes.

**Evidence:** `docs/compatibility.md`, `.roles/stakeholders/icelines-maintainer.md`,
and `.roles/stakeholders/ci-release-engineer.md`.

## FLETCH-PF-04: Compiled CLI Command Surface Overflows Before Validation

**Status:** MITIGATED

**Pattern:** The large Clap command graph overflows the default Windows main
thread stack, so compiled CLI commands such as `registry web` fail before the
server can bind or validation can run.

**Domain:** Registry web UI, CLI smokes, local demos, CI, publisher workflows,
and agent validation.

**Detection difficulty:** Core tests exercise library contracts; only compiled
binary tests and CLI smokes expose Windows startup failure.

**Structural solution:** Run CLI parsing and dispatch inside a named thread with
an explicit larger stack, and keep compiled CLI smokes in validation.

**Evidence:** `crates/fletch-cli/src/main.rs`,
`crates/fletch-cli/tests/registry_web.rs`, and
`cargo test -p fletch-cli --test registry_web`.

## FLETCH-PF-05: Ordinary Tests Hide Strict Static-Analysis Debt

**Status:** MITIGATED

**Pattern:** `cargo test --workspace` passes while `cargo clippy --workspace
--all-targets -- -D warnings` fails on assertion style, test-module placement,
pointer argument shape, over-wide functions, needless returns, or nested
formatting.

**Domain:** Core tests, CLI support modules, mock-client reports, maintainability
gates, and portfolio scoring.

**Detection difficulty:** Behavior is correct and tests pass, so the debt
appears only when strict linting is part of the adoption pass.

**Structural solution:** Keep clippy with warnings denied in FLETCH validation
and group growing command/report parameters behind small value objects.

**Evidence:** `crates/fletch-core/src/lib.rs`,
`crates/fletch-cli/src/support/io.rs`,
`crates/fletch-cli/src/support/misc.rs`,
`crates/fletch-mock-client/src/lib.rs`, and
`cargo clippy --workspace --all-targets -- -D warnings`.

## FLETCH-PF-06: Cache Choice Looks Like Activation Choice

**Status:** OPEN

**Pattern:** A consumer selects a manifest, trusted manifest, offline cache,
quiver import, registry web row, partition state, alias, or publish report and
assumes that selection activates product data or establishes downstream trust.

**Domain:** `fetch-plan`, `fetch`, `cache status`, `cache index-gate`, quiver
export/import, registry search/web, partition reports, aliases, publisher
bundles, PROOF document manifests, and MDCROP indexes.

**Detection difficulty:** FLETCH intentionally exposes many precise read-only and
stage-first views; the wrong user can make the correct cache/report selection
and still skip the product-owned activation, merge, trust, or interpretation
step.

**Structural solution:** Consumer-facing examples should keep the active source
of truth, selected derived view, stage/import state, verification status, trust
source, and product-owned activation command visibly separate, and tests should
cover at least one consumer flow where selecting a row does not activate data.

**Test:** `crates/fletch-cli/tests/registry_web.rs` cites `FLETCH-PF-06` while
checking URL-selected registry rows and source previews as read-only derived
views.

**Evidence:** `README.md`, `docs/specs/fletch-foundation.md`,
`docs/show/integrator-brief.md`,
`crates/fletch-cli/tests/registry_web.rs`, and
`.roles/parliament/offline-release-operator.md`.
