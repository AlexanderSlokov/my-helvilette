# ADR-0002: Use Go or Rust for the terminal application

- **Status:** Accepted
- **Date:** 2026-11-26
- **Decider:** Founder

## Context

The terminal application (see [ADR-0001](0001-tui-first-no-global-input.md)) needs:

- A TUI library that draws text frames and reads keys.
- Terminal mouse reporting (click, drag, and motion) for petting and scratching.
- Timers for animation and for idle detection.
- Low CPU use when the cat is idle or asleep.
- One binary that is easy to install.

Go and Rust can both do all of these tasks.
The Founder has toolchains for both.
JavaScript is not a candidate.

| Language | TUI library | Notes |
|---|---|---|
| Go | Bubble Tea, Lip Gloss | The Elm model (Model, Update, View) fits a state machine. Builds are fast. |
| Rust | ratatui, crossterm | Better runtime performance. Builds are slower. |

## Decision

The language is not an architectural constraint for Phase 1.
Go and Rust are both accepted.
Select the language that gives the best development speed and runtime performance.

The first implementation task selects one language.
Record the selection in the "Selection" section of this ADR.

## Selection

**Rust** (selected by the Founder on 2026-11-26).

Use `crossterm` only. Do not use `ratatui`: the application draws its own glyph grid and does not need widgets.

## Consequences

- The architecture does not depend on the language.
- When the language is selected, all Phase 1 code uses that one language.
  Do not mix Go and Rust in Phase 1.
