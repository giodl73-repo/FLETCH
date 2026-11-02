# FLETCH Principles

## FLETCH-P-01: Acquisition Is Not Activation

**Decision rule:** FLETCH may fetch, verify, cache, bundle, index, and report
source material, but consumers own parsing, selection, activation, snapshots,
legal interpretation, product scoring, and domain policy.

**Rationale:** The shared cache layer stops being reusable if ICELINES, BISECT,
ROUTE, MDCROP, PROOF, or another consumer moves product semantics into
`fletch-core`.

**Test:** Adapter Boundary Keeper review, compatibility policy, workspace tests,
and consumer handoff docs preserve product-neutral boundaries.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `docs/compatibility.md`,
`.roles/parliament/adapter-boundary-keeper.md`, and `cargo test --workspace`.

## FLETCH-P-02: Ledger Bytes Must Be Verifiable

**Decision rule:** Cache entries record source identity, cache key, byte count,
SHA-256, verification state, freshness, retries, and object status before
downstream tools can treat them as usable evidence.

**Rationale:** A fetch/cache substrate is valuable only if later agents can
audit exactly which bytes were acquired and whether they still match the
ledger.

**Test:** Core tests cover hash validation, trusted manifests, cache hits,
verification, freshness, retries, and status reports.

**Evidence:** `crates/fletch-core/src/lib.rs`, `docs/specs/fletch-foundation.md`,
`.roles/parliament/provenance-auditor.md`, and `cargo test -p fletch-core`.

## FLETCH-P-03: Offline Paths Are First-Class

**Decision rule:** Quivers, offline cache reports, staged imports, and dry-run
flights must explain what can proceed without live network access and what
object or bundle is missing.

**Rationale:** CI, customer demos, field work, and reproducible research cannot
depend on silent live fetches.

**Test:** Quiver, offline, import, and dry-run tests preserve stage-first and
no-network semantics.

**Evidence:** `README.md`, `crates/fletch-core/src/lib.rs`,
`.roles/parliament/offline-release-operator.md`, and `cargo test --workspace`.

## FLETCH-P-04: Derived Views Are Not The Source Of Truth

**Decision rule:** Cache indexes, registry web pages, PROOF docs, MDCROP rows,
tips, local URL maps, and publish bundles are read-only reports over manifests
or registries, not replacement ledgers.

**Rationale:** Generated views are useful for humans and tooling but can drift
from the source contract if treated as authoritative state.

**Test:** Publisher, slice, registry-web, and manifest tests keep report output
bounded and derived from inputs.

**Evidence:** `README.md`, `docs/show/integrator-brief.md`,
`.roles/parliament/doc-publisher.md`, and
`cargo test -p fletch-cli --test registry_web`.

## FLETCH-P-05: Foundation Changes Need Consumer Rehearsal

**Decision rule:** Public API, schema, cache identity, manifest, registry,
fetch-control, verification, ordering, or error-meaning changes are not ready
until affected FLETCH tests and required downstream rehearsal pass or are
explicitly scoped out.

**Rationale:** The first failure often appears in ICELINES, BISECT, ROUTE, or
MDCROP after FLETCH-local tests stay green.

**Test:** Compatibility policy names foundation tests and focused ICELINES
rehearsal commands.

**Evidence:** `docs/compatibility.md`,
`.roles/stakeholders/icelines-maintainer.md`, and
`.roles/stakeholders/ci-release-engineer.md`.
