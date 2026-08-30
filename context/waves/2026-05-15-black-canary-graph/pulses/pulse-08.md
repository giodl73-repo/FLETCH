# Pulse 08 - PITFALL Boundary Coverage

## Intent

Convert the PITFALL pass into retained coverage for the registry-web and
consumer-handoff surfaces.

## Changes

- Added use-case-first actor, task, surface, likely mistake, consequence, and
  owner fields to the open FLETCH pitfalls.
- Added `crates/fletch-cli/tests/pitfall_policy.rs` to cite
  `FLETCH-PF-01`, `FLETCH-PF-02`, `FLETCH-PF-03`, and `FLETCH-PF-06`.
- Kept the Windows CLI stack mitigation for `FLETCH-PF-04` and strict clippy
  cleanup for `FLETCH-PF-05` in the same validation pass.

## Validation

- `cargo test -p fletch-cli --test pitfall_policy`
- `cargo test -p fletch-cli --test registry_web`
- `cargo test --workspace --locked`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `pitfall-cli validate C:\src\TRACKER\repos\tools-infra\fletch --format json`
- `python C:\src\TRACKER\repos\standards-protocols\pitfall\tools\check_pitfall.py C:\src\TRACKER\repos\tools-infra\fletch`

## Notes

The compatibility policy still treats affected-consumer rehearsal as required
for foundation changes. This portfolio pass did not run ICELINES because the
current repository sweep excludes ICELINES.
