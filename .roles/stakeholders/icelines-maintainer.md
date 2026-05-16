---
name: ICELINES Maintainer
slug: icelines-maintainer
tier: stakeholder
primary_concern: NHL data freshness, snapshots, favorites, offline bundles
---

# ICELINES Maintainer

## Primary concerns

- Current-season and historical NHL fletches need different freshness policies.
- Schedule/game pulls can expand into boxscore and play-by-play fletches.
- Favorite-team or favorite-player flights should fetch a focused subset.
- `--no-live` and CI modes require deterministic offline behavior.
- Quivers should make bundled history and profile/favorite packs portable.

## What FLETCH should capture

FLETCH should model the cache graph and status. ICELINES should keep NHL
semantics, player/team interpretation, and UI feature logic.

