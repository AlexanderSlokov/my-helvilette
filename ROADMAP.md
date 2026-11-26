# Roadmap

## Phase 1: Terminal cat (target: v0.1.0)

Decisions: [ADR-0001](docs/explanation/adr/0001-tui-first-no-global-input.md), [ADR-0002](docs/explanation/adr/0002-language-go-or-rust.md), [ADR-0003](docs/explanation/adr/0003-ascii-art.md).

- An ASCII cat with a keyboard and a mouse in front of it.
- The cat types when the user types in its terminal.
- The cat moves its mouse when the user moves the mouse in its terminal.
- The user can pet the head, scratch the chin and ears, and feed the cat.
- When the user is idle, the cat becomes drowsy and then falls asleep.
- The cat is present in each terminal that the user opens. The method is an open question in ADR-0001.

## Phase 2: Desktop overlay for Windows and macOS (target: not set)

- Start only if time permits.
- A new ADR must approve the input-capture method for each platform before work starts.
