# FLETCH Product Plan

## Thesis

Rust projects in this workspace repeatedly need the same data plumbing: fetch
remote sources, cache them locally, verify hashes, pin versions, bundle caches,
profile downloads, and run offline. FLETCH centralizes those mechanics behind a
small product-neutral contract.

## Waves

1. **Foundation**: Rust workspace, fetch plan schema, cache manifest schema,
   deterministic cache keys, CLI plan/key commands. **Active.**
2. **Cache execution**: HTTP download, resumable writes, temp-file promotion,
   checksum verification, and retry policy.
3. **Cache operations**: `cache ls`, `cache verify`, `cache prune`, offline
   checks, and stale/fresh reports.
4. **Bundles**: export/import portable cache bundles with manifests and
   verification.
5. **Adapters**: Census/apportionment, NHL/icelines, route/geodata, and generic
   static archive adapters.
6. **CROP integration**: make FLETCH manifests easy for CROP to index and report
   as corpus status.

## Non-goals

- FLETCH does not own domain semantics for BISECT, icelines, route, or CROP.
- FLETCH does not replace each product's user-facing commands.
- FLETCH does not depend on BISECT.

## Naming

FLETCH = Fetch, Ledger, Export, Trust, Cache, Hash.
