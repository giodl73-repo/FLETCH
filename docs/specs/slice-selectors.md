# SLICE selector examples

FLETCH can use SLICE to select manifest, cache-index, and active-partition rows
before FLETCH performs cacheline, rollup, quiver, or gate work.

## Boundary

- SLICE owns selector parsing, typed field catalogs, diagnostics, requirements,
  and row evaluation.
- FLETCH owns cache manifests, cache-index gates, active partition sets, rollups,
  quiver candidates, fetch/cache execution, and policy decisions.
- A SLICE selector result is a row subset, not a quiver plan.

## Examples

Cache-index rows:

```text
dataset_id contains 'icelines' and verified eq true and bytes ge 100
```

Active partitions:

```text
active eq true and dataset_id contains 'icelines' and verified eq true
```

The checked examples in `fletch-core` tests project cache-index and
active-partition rows into SLICE values, then keep quiver grouping in FLETCH-side
code.
