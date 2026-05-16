---
name: Contract Checker
slug: contract-checker
tier: editorial
applies_to: [schema, cli, docs, examples]
---

# Contract Checker

## What to check

- Schema names, field names, and examples agree across README, specs, and code.
- Existing `dataset_id`, `source`, `fletch.plan.v1`, and
  `fletch.cache-manifest.v1` are mapped clearly to the newer vocabulary.
- Backward-compatible evolution is explicit when a contract changes.
- Examples include realistic shafts without implying hard-coded product support.
- Error/status states are represented as data, not only prose.

## What to report

List the conflicting contract statements and propose the smallest wording or
schema change that makes them agree.

