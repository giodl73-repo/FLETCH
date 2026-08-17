# FLETCH compatibility policy

FLETCH is a pre-1.0 shared acquisition and cache foundation. Compatibility is
deliberate because products depend on its fetch plans, cache identity, manifests,
registries, verification, partitions, quivers, and read-only reports.

## Protected contract

The protected surface includes:

- public `fletch-core` APIs and error meanings;
- all published `fletch.*.v1` plan, manifest, cache-index, gate, diff, registry,
  validation, graph, partition, quiver, adapter, tip, and publisher schemas;
- deterministic cache-key inputs, SHA-256 representation, relative object
  paths, byte counts, verification state, and manifest upsert behavior;
- saved-plan and registry validation before fetch or indexing;
- freshness, offline, retry, timeout, bandwidth, and trusted-cache semantics;
- stage-before-promotion and verify-before-import behavior for cache and quiver
  objects;
- deterministic graph, index, partition, rollup, alias, and bounded-report
  ordering; and
- the boundary that consumers own source expansion, parsing, domain validation,
  snapshots, activation, joins, and product policy.

Internal refactoring is compatible only when these observable contracts remain
stable.

## Versioning rules

- Additive APIs or optional report fields may remain within the current `0.y`
  line when existing plans, manifests, registries, and consumers remain
  compatible.
- Breaking APIs, schemas, cache keys, hash inputs, defaults, validation, error
  meanings, ordering, fetch controls, verification, or promotion behavior
  require a minor-version bump while the affected crate is below `1.0`.
- A breaking machine-readable record requires a new schema version rather than
  silently changing an existing `v1` shape.
- Prefer deprecation plus migration notes before removing a public item.
- A breaking change must identify affected consumers and include manifest,
  registry, cache, or adapter migration guidance.
- Downstream repositories should pin commits for reproducible evidence.
  Branch consumers must run the downstream rehearsal before updating.

## Foundation tests

From the FLETCH repository:

```powershell
cargo test -p fletch-core
cargo test -p fletch-cli
cargo test -p fletch-mock-client
```

These protect schemas, cache keys, manifests, validation, fetch controls,
verification, offline behavior, partitions, quivers, graphs, registries, and
publisher surfaces.

## Downstream breakage rehearsal

ICELINES is the required first external consumer rehearsal because it uses
FLETCH for real generic HTTP, paged, batch, and window acquisition; durable
cache manifests; verified cache reads; registries; partitions; quivers; and
cache-index gates while retaining hockey-domain activation.

From the ICELINES repository:

```powershell
python tools\repo_map.py write-cargo-config
cargo test -p icelines-fetch registry_marks_rosters_and_moneypuck_generic_http
cargo test -p icelines-fetch fetch_generic_http_bytes_uses_fletch_cache_object
cargo test -p icelines-fetch cache_manifest_upsert_atomically_replaces_existing_manifest
cargo test -p icelines-fetch fetch_paged_report_bytes_uses_fletch_paged_cache_object
cargo test -p icelines-fetch fetch_gamecenter_batch_bytes_uses_fletch_batch_cache_objects
```

The generated, ignored Cargo config patches `fletch-core` and `slice-core` to
their sibling checkouts. A compile failure exposes public API breakage.
Registry, cache-object, manifest, paged, or batch failures expose schema,
identity, acquisition, verification, or persistence drift.

FLETCH foundation changes are not ready until the affected foundation tests and
the ICELINES rehearsal pass.
