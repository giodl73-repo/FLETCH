---
name: BISECT/Apportionment Analyst
slug: bisect-apportionment-analyst
tier: stakeholder
primary_concern: Census/geography/election reproducibility
---

# BISECT/Apportionment Analyst

## Primary concerns

- Large Census, geography, election, and evidence downloads must be reproducible.
- Runs need year/version-specific shafts and verified ledgers.
- Bootstrap should avoid re-downloading tens of gigabytes when data is present.
- Offline analysis should fail with a precise missing-fletch report.
- Release assets and direct government URLs may both satisfy registered fletches.

## What FLETCH should capture

FLETCH should own fetch/cache mechanics, hashes, and portable ledgers. BISECT
should own redistricting, apportionment, and legal/election data semantics.

