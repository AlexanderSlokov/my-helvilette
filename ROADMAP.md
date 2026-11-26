# Roadmap

## Phase 1: Terminal cat (released in v0.1.0)

Decisions: [ADR-0001](docs/explanation/adr/0001-tui-first-no-global-input.md), [ADR-0002](docs/explanation/adr/0002-language-go-or-rust.md), [ADR-0003](docs/explanation/adr/0003-ascii-art.md), [ADR-0004](docs/explanation/adr/0004-shell-plugin-and-tui.md).

The cat is Helvilette, drawn as ASCII art from the Founder's artwork.

zsh plugin, in each terminal (the prompt shows the cat's two blue eyes):

- When the user types, the cat types.
- When a command fails, the cat grumbles.
- When the user is idle, the cat becomes drowsy and then falls asleep.

`meow` command (full TUI):

- The full scene: the cat, a keyboard, and a mouse.
- The cat's eyes follow the mouse pointer.
- The user can pet the head and scratch the chin and ears with the mouse.
- The user can feed the cat five dishes from the lore: bánh cam nhân đậu xanh, bánh tai yến, chè nước cốt dừa, mỡ cá kho tộ, and bánh cuốn chấm nước mắm.

## Phase 1b: Lore features (released in v0.1.0)

- The twelve runes of the True Selves on the halo.
- Moods from the Rune personalities (Othela, Wunjo, Hagalaz, Sowilo, Isa, Laguz).
- Schrödinger Cat: when the terminal comes back into focus, a different True Self is present. Voided Out is the transition effect.
- Not planned: the Triquetra and the background cats of the artwork. They are too large for a terminal and would make Helvilette smaller.

## Phase 2: Desktop overlay for Windows and macOS (target: not set)

- Parked: the Founder has no Windows or macOS computer for tests.
- Start only if time permits.
- A new ADR must approve the input-capture method for each platform before work starts.
