# FLETCH — systems implementer brief

**Time:** 15–35 minutes. **Goal:** know the CLI/core surface and what not to
stuff into the substrate.

## Layout to expect

| Surface | Role |
|---|---|
| `fletch-cli` | Plan, key, fetch, fetch-plan, cache\*, quiver\*, graph, registry\*, tip, publish |
| `fletch-core` | Product-neutral acquisition, ledger R/W, batch upsert, paged JSON helper, gates |
| Ledger schemas | `fletch.plan.v1`, `fletch.cache-manifest.v1`, `fletch.cache-index.v1`, diffs |
| Specs | Foundation vocabulary; slice selectors over index/partition rows |

## Contract rules

- **Fetch ≠ activate.** Verified cache objects are candidates until the product merges.
- **Upsert by cache key** when writing into an existing manifest.
- **Freshness** is explicit (`immutable` / `max-age-days` / `always-check`), not implied.
- **Offline** fails closed instead of touching the network; errors distinguish missing vs stale/bypassed.
- **Tips** are structured peeks (sample/index), not product reports.

## Family boundary

```text
FLETCH (acquire) → MDCROP (select) → LATTICE (close) → WITNESS (replay)
```

Do not grow FLETCH into selection, semantic closure, or harness replay.

## Hands-on

```powershell
cargo run -p fletch-cli -- --help
cargo test --workspace
```

## Next docs

- [`../../SHOWCASE.md`](../../SHOWCASE.md)
- [`../specs/fletch-foundation.md`](../specs/fletch-foundation.md)
- [`../specs/slice-selectors.md`](../specs/slice-selectors.md)
- [`../specs/consumer-adapter-scout.md`](../specs/consumer-adapter-scout.md)
