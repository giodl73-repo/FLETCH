# FLETCH Invariants

## FLETCH-INV-01: Cache Keys Include Generic Source Headers

**Status:** MITIGATED

**Claim:** Deterministic cache keys include logical dataset identity, source
URL, source kind, and generic request headers.

**Why it matters:** Two shafts that differ by required header can return
different bytes and must not collide in cache.

**Enforcement:** Core tests assert stable plan schema and header-sensitive cache
keys.

**Evidence:** `crates/fletch-core/src/lib.rs` and
`cargo test -p fletch-core cache_key_includes_generic_source_headers`.

## FLETCH-INV-02: Manifests Validate Before Persistence

**Status:** MITIGATED

**Claim:** Manifest read/write and upsert helpers reject invalid schema, hash,
cache-root, duplicate, or malformed ledger state before persistence.

**Why it matters:** Long-lived consumer ledgers become evidence only when merge
helpers cannot silently preserve invalid rows.

**Enforcement:** Manifest JSON helper and upsert tests validate resulting
manifests.

**Evidence:** `crates/fletch-core/src/lib.rs`,
`crates/fletch-cli/src/support/io.rs`, and `cargo test --workspace`.

## FLETCH-INV-03: Registry Validation Stops Invalid Indexing

**Status:** MITIGATED

**Claim:** Direct and followed registry inputs must validate before registry
index or web surfaces build searchable rows.

**Why it matters:** Search and local browser surfaces can make broken registry
metadata look authoritative.

**Enforcement:** CLI tests reject missing-shaft registries before indexing and
core tests report duplicate IDs and missing shafts.

**Evidence:** `crates/fletch-cli/src/support/io.rs`,
`crates/fletch-core/src/lib.rs`, and `cargo test --workspace`.

## FLETCH-INV-04: Quiver Import Stages Before Activation

**Status:** MITIGATED

**Claim:** Quiver import verifies bundled bytes and promotes into a staged
cache location without activating aliases, partitions, rollups, or product
views.

**Why it matters:** Offline bootstrap packages must not silently change active
consumer state.

**Enforcement:** Core tests cover quiver export, tamper rejection, verification,
summary, and stage-first import.

**Evidence:** `crates/fletch-core/src/lib.rs` and `cargo test --workspace`.

## FLETCH-INV-05: Registry Web Must Start In Compiled CLI Smokes

**Status:** MITIGATED

**Claim:** `fletch-cli registry web` starts from the compiled binary on Windows
and serves summary/search/detail/source APIs under integration tests.

**Why it matters:** Library tests can pass while the real Clap command graph
overflows before the web server binds.

**Enforcement:** CLI startup uses an explicit stack thread and registry-web
integration tests execute the compiled binary.

**Evidence:** `crates/fletch-cli/src/main.rs`,
`crates/fletch-cli/tests/registry_web.rs`, and
`cargo test -p fletch-cli --test registry_web`.

## FLETCH-INV-06: Consumer Handoff Boundaries Stay Machine-Readable

**Status:** VERIFIED

**Claim:** FLETCH records acquisition, verification, cache selection, product
activation, domain interpretation, and compatibility acceptance ownership in
`docs/consumer-boundaries.v1.json`.

**Why it matters:** Fetchable, verified, selected, or locally green artifacts
look complete enough to skip the consumer-owned parse, merge, trust, activation,
domain validation, and rehearsal steps.

**Enforcement:** The CLI PITFALL policy test parses the boundary manifest and
checks that acquisition is not activation, local FLETCH tests are not consumer
compatibility, and cache or registry selection is not downstream trust.

**Evidence:** `docs/consumer-boundaries.v1.json` and
`crates/fletch-cli/tests/pitfall_policy.rs`.

**Test:** `cargo test -p fletch-cli --test pitfall_policy`.
