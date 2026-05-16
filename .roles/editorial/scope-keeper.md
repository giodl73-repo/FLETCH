---
name: Scope Keeper
slug: scope-keeper
tier: editorial
applies_to: [spec, wave, pulse, implementation]
---

# Scope Keeper

Form gate, not substance gate. Runs after parliament before a wave or pulse is
treated as ready.

## What to check

1. Does the artifact keep FLETCH product-neutral?
2. Does it avoid embedding consumer-specific semantics in `fletch-core`?
3. Does it use fletch, shaft, flight, quiver, and ledger terminology consistently?
4. Does it distinguish machine contracts from generated PROOF/CROP views?

## What NOT to do

Do not reject consumer examples just because they are concrete. Reject them only
when they become product-specific behavior in the shared core.

