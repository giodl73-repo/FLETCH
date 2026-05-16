# Pulse 01: Report row slices

## Goal

Bound large registry, partition, active-set, and quiver report rows without
changing source machine contracts or mutating any FLETCH state.

## Outcome

- Added slice helpers for adapter source rows, registry validation findings,
  archive-preview children, partition state rows, active partition rows, and
  quiver merge-ready candidates.
- Added `--offset` and `--limit` to the relevant CLI report commands.
- Added focused filters: `--adapter-owned`, `--severity`, `--active`, and
  quiver merge-ready `--status`.
- Advanced the active wave to Report Scaling to close large-report follow-ups
  left by adapter, partition, and quiver reviews.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for sliced report JSON
- `git diff --check`
