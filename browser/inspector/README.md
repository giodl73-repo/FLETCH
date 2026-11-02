# FLETCH registry explorer

The companion implements a bounded Rust/WASM registry task. Release status is
recorded in the wave; implementation and local checks alone are not live
qualification. PRs qualify read-only; only tested default-master artifacts deploy.

The new serde-only fletch-registry crate contains mechanically moved native
registry definitions, validation, indexing and search. fletch-core reexports the
same names. Generated labels and v1 records remain unchanged. Native validation
gates all indexing: invalid schema, duplicate IDs or missing shafts return native
findings without searchable rows. This does not verify referenced sources, graph
relationships, cache bytes, freshness, downstream trust or activation. Native
unknown registry fields are ignored in derived reports; opened original JSON is
kept separately and never represented as roundtripped lossless source.

Rust 1.95.0 with wasm32-unknown-unknown/clippy/rustfmt, wasm-bindgen CLI0.2.127,
Node22, Playwright1.61.1 and Python3.11+ are required. From the repository root:

```text
cargo +1.95.0 fmt --all -- --check
cargo +1.95.0 test --workspace --locked
cargo +1.95.0 test --manifest-path browser/inspector/Cargo.toml --locked
cargo +1.95.0 clippy -p fletch-registry --target wasm32-unknown-unknown --locked -- -D warnings
python browser/inspector/build-prototype.py --bindgen /absolute/path/to/wasm-bindgen
cargo +1.95.0 build -p fletch-cli --locked
python browser/inspector/verify-native-cli.py --binary /absolute/target/debug/fletch-cli
```

Use the platform executable suffix on Windows. CARGO_TARGET_DIR, when set, must
be absolute. The prototype builder freshly builds native and WASM companions and
writes target/registry-prototype/FLETCH plus private QA responses under .cache.
It never copies a stale native executable. Browser companion admission is
separate from native registry findings: registry1MiB,500fletches/4000shafts/5000edges,
query/filter1024bytes,30tags/pairs,pages1..100,report4MiB,WASM64MiB. Request framing
is3MiB. Original/current/LF-copy/report exports stay distinct; URLs remain inert.
Exact paged reports contain the current page and full matched count, not every
matching row. No cache acquisition or consumer capabilities are available.

From browser/inspector:

```text
npm ci --ignore-scripts
npx playwright install chromium
npm run test:pages
```

The local server tests /FLETCH/. Thirteen packaged browser cases include forty-four exact executing
WASM responses, typed invalid-registry gate, own UTF-8/BOM/CRLF preservation,
pagination/full current-page report, inert metadata, startup deadline/recovery,
mobile/keyboard, file/read/revision/generation fencing, actual 5/12/10-second
deadlines, crash/restart and memory ceiling. Two delivery checks verify exact
source/complete hashes and independent source/build/legal routes under unavailable
WASM. Source replay repeats eleven behavior cases outside Git in a path with spaces. Timing samples are descriptive and
include JS/message/serialization overhead. No comparative performance is claimed.

See context/waves/2026-11-01-registry-browser/portability.json for Windows native76tests,
adapter4tests, compiled native CLI scenarios and five required ICELINES checks.
That consumer rehearsal used an isolated unchanged Git archive, actual candidate
path verified through Cargo metadata. Its initial unrestricted build exhausted
Windows paging memory compiling unrelated integration binaries; the exact five
library tests pass with one build job. Other owner sessions/data are untouched.


Clean-source release staging and independent replay commands are in BUILD.md.
The complete first-party Git archive pins every tracked file and retains the MIT
grant. Dependency/Rust notices are copied verbatim. Separate budgets are2MiB
runtime,20MiB optional source archive,25MiB site. Source archives do not load at
startup. Private QA outputs/metadata/screenshots stay outside public staging.
Request downloads preserve the current registry and exact filters/page; original
opened bytes stay separate from current text and LF copies. No browser operation
resolves declared source URLs or certifies source/cache/trust/activation status.
