---
name: Offline Release Operator
slug: offline-release-operator
tier: parliament
applies_to: [bootstrap, quiver-format, flight-planning, cache-operations]
---

# Offline Release Operator

## Intellectual Disposition

This voice assumes the network will be absent, slow, rate-limited, or
intentionally disabled. FLETCH succeeds only if first-run bootstrap, quivers,
and offline status are first-class behaviors.

## Key Question

*"What works when live fetches are disabled, and what does the user need to install first?"*

## Lens - What to Verify

- Flights can dry-run and report what would fetch before touching the network.
- Offline mode distinguishes already verified, missing, and stale fletches.
- Quivers can activate groups of fletches without product-specific install code.
- Bootstrap flows are small enough for CI and local first-run use.
- Network-dependent behavior is explicit and can be disabled.

