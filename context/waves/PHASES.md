# FLETCH Waves

FLETCH work is organized into small waves and pulses.

## Active wave

- `2026-05-15-fletch-foundation`

## Protocol

1. Read the active wave `WAVE.md`.
2. Execute the next pulse in `pulses/`.
3. Keep product logic out of `fletch-core`; use adapters for domain-specific
   sources.
4. Validate with `cargo fmt`, `cargo test --workspace`, focused CLI smokes, and
   `git diff --check`.
5. Update the wave and pulse docs before committing.
