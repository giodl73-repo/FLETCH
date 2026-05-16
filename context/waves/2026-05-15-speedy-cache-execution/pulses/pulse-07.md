# Pulse 07: Saved plan execution

## Goal

Let generic fetch execution consume saved `fletch.plan.v1` files directly, so
adapters, CROP/PROOF generated views, and checked-in configs can hand FLETCH a
complete shaft acquisition intent.

## Outcome

- Added `fletch fetch-plan --plan <path>` to execute a saved plan file.
- Reused existing generic cache execution controls for expected hash, trusted
  manifest, bandwidth, timeout, retries, force, offline, and output.
- Preserved the plan's source, headers, freshness policy, tags, and metadata
  instead of reconstructing intent from command-line flags.
- Kept activation/merge behavior out of fetch execution.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for saved plan fetch execution
- `git diff --check`
