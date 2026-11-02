# FLETCH registry explorer build and delivery

Rust1.95.0 with wasm32-unknown-unknown, clippy, rustfmt and rust-docs;
wasm-bindgenCLI0.2.127; Node22; Playwright1.61.1; Python3.11+.
See browser/inspector/README.md in source.zip for exact native/companion/browser
commands. CARGO_TARGET_DIR, if set, must be absolute. The prototype builder
freshly builds native and WASM outputs before staging; it never reuses an
unverified native executable.

source.zip contains the entire exact first-party Git tree, both Cargo locks,
all tests/includes/fixtures/specs and the original MIT grant. source-files.json
pins each file, and the ZIP comment records the source commit. Cargo obtains
pinned registry/Git dependencies; this is not a vendored offline compiler pack.
The extracted serde-only registry crate is the same code reexported by native
fletch-core, not a JavaScript implementation of native behavior.

From clean source after building:

```text
python browser/inspector/package-site.py
python browser/inspector/verify-site.py target/registry-release/FLETCH --commit <HEAD>
python browser/inspector/verify-source.py --bindgen /absolute/path/to/wasm-bindgen
```

The source replay extracts outside Git into a path containing spaces, runs full
native workspace and adapter tests, rebuilds both companions, compares every
request/response and compiled native CLI scenario, and repeats browser behavior
checks. Cross-directory WASM binary identity is not established; exact responses
must match. Hosted deployment verifies the tested transferred artifact again.
Fresh live files/browser checks remain a separate release gate.

Budgets: initial runtime2MiB, optional complete source ZIP20MiB, complete site25MiB.
The optional source archive is not startup data. Original dependency and Rust
library copyright/notices accompany the software. Private QA responses, raw
metadata/logs, screenshots and scratch requests are never public site files.
Registry search/validation cannot establish source/cache/trust/activation status.
