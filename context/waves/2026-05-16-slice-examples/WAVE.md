# Wave: SLICE Examples

## Goal

Show how FLETCH can use SLICE selectors over cache-index and active-partition
rows without moving cache policy or quiver folding into SLICE.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Manifest and partition selectors | done | Added dev-only SLICE tests for cache-index and active-partition selection. |

## Success criteria

- SLICE is used only for row selection.
- FLETCH keeps cacheline gates, active partition sets, rollups, and quiver
  folding.
- `cargo test --workspace` passes.
