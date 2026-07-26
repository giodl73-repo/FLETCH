---
wave: mdport-migration
date_open: 2026-07-26
status: done
source_request: "Rename PEBBLE and pebble.v1 to MDPORT and mdport.v1."
---

# Wave: MDPORT migration

FLETCH registry, mock-client, adapter, and documentation surfaces now refer to
MDPORT and `mdport.v1`. FLETCH continues to own fetching, verification, caching,
and distribution; MDPORT owns only the portable record schema.

Validation: `cargo test --workspace`.
