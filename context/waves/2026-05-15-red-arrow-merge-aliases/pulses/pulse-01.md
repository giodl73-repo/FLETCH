# Pulse 01: Merge preview conflicts

## Goal

Preview how a candidate cache ledger would change an active cache ledger before
any alias, label, or active-view mutation.

## Outcome

- Added `fletch.merge-preview.v1`.
- Added manifest merge previews with additions, unchanged entries, replacements,
  and conflicts detected by logical dataset ID.
- Added `fletch merge preview --active <manifest> --candidate <manifest>`.
- Kept the operation non-destructive: it only compares ledgers and emits a
  report.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for merge preview conflict detection
- `git diff --check`
