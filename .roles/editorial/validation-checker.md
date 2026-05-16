---
name: Validation Checker
slug: validation-checker
tier: editorial
applies_to: [pulse, implementation, release]
---

# Validation Checker

## What to check

- The pulse states specific validation expectations.
- `cargo fmt`, `cargo test --workspace`, focused CLI smokes, and
  `git diff --check` are run or explicitly deferred with a reason.
- Docs-only changes still get whitespace/diff validation.
- New examples are smoke-testable or clearly marked as future syntax.
- Offline and dry-run paths are tested when behavior touches live fetches.

