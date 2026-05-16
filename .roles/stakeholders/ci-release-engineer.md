---
name: CI/Release Engineer
slug: ci-release-engineer
tier: stakeholder
primary_concern: deterministic bootstrap and artifact promotion
---

# CI/Release Engineer

## Primary concerns

- Tests need deterministic offline fixtures and no accidental live fetches.
- First-run setup should be scriptable and fast.
- Temp-file promotion must avoid corrupting caches on cancelled jobs.
- Quivers should make release artifacts portable and verifiable.
- CLI smokes need stable, concise output for automation.

