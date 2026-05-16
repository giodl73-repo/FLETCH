# Pulse 01: Quiver summary report

## Goal

Emit a read-only quiver summary so consumers and automation can inspect bundle
identity, member counts, byte totals, and verification totals before import.

## Planned outcome

- Add a named `fletch.quiver-summary.v1` contract.
- Report quiver ID, entry count, byte total, verified count, and unverified
  count.
- Add a read-only CLI command that summarizes an existing quiver manifest.
- Document the contract in the foundation spec.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- focused CLI smoke for quiver summary JSON
- `git diff --check`
