# Portable registry search and inspection

Status: active; local qualification only. No Pages deployment or portfolio pointer update yet.

Goal: let a reader open an authored or own fletch.registry.v1 JSON document,
inspect native validation findings and search valid registry rows by text, exact
tags and metadata pairs. It is a derived view, never a ledger or proof that shafts
exist, bytes are verified, sources are trusted, or consumers can activate them.

Preserve entry507d222 and its four PITFALL commits beyond public masterd7bdef7.
Mechanically move registry types/index/search/validation into fletch-registry;
fletch-core reexports existing public symbols. Native cache/fetch/CLI algorithms,
schemas, version, ordering, generated_by labels and source-kind meanings remain.
No networking, filesystem, domain policy, cache verification or activation in WASM.

| Pulse | Outcome | Gates |
|---|---|---|
|01| Shared pure Rust extraction and real-WASM prototype | Full native workspace regression, mechanical source preservation, native/WASM responses, focused native CLI smokes and required ICELINES rehearsal in an isolated checkout |
|02| Reader flows and delivery | Own-file/original/current/LF-copy/report, bounds, worker deadlines/recovery/fencing, mobile/keyboard, complete source/legal packaging, exact PR/default/live artifact and browser evidence |

Each pulse must deliver a usable slice. Do not create a successor diagnostic or
custody framework. Original files and local source stay protected. Bound browser
resources separately from existing native semantics. Invalid registries must
never produce searchable rows (FLETCH-INV-03). Source references stay inert.
Source/archive downloads and startup budgets are separate and recorded explicitly.

Validation: cargo fmt --all -- --check; cargo test --workspace --locked; scoped
strict host/WASM lint; native compiled CLI smokes; ICELINES commands from
 docs/compatibility.md with actual local dependency override verified. No live
remote sources or other-session changes. Close with all parliament/editorial/
stakeholder/panel lenses and exact release evidence before a TRACKER snapshot.
