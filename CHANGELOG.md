# Changelog

All notable changes to this project are in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project uses [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.1.0] - 2026-11-26

The first release: Helvilette, a black cat that lives in your terminal.

### Added

- `meow` command: Helvilette's room, full screen in the terminal.
  - The cat is ASCII art that the program generates from geometry traced from the Founder's artwork: the black cat with neon outlines and blue eyes, inside a halo.
  - He types when you type, moves his mouse and follows the pointer with his eyes, purrs when you pet his head, lifts his chin, and twitches his ears.
  - His front legs bend when a paw lifts to type or goes to the mouse.
  - He eats five dishes from the lore (F1 to F5, or a click on the tray).
  - After 30 seconds without input he becomes drowsy, and after 90 seconds he falls asleep.
- The halo shows the twelve runes of Helvilette's True Selves (ᛟ Othela at the top, then ᚨ ᛗ ᛒ ᛁ ᚹ ᛚ ᚺ ᚷ ᛊ ᚱ ᛏ). The rune of the True Self that is present glows.
- Moods from the Rune personalities: Wunjo purrs when petted, Hagalaz grumbles when you wake him, Sowilo swipes when you pet him for too long, Isa sleeps, and Laguz is sad when he has had no meal for 8 hours.
- Schrödinger Cat: when the terminal comes back into focus after 2 seconds or more, the room goes dark for a moment (Voided Out) and a different True Self is there and says his name.
- zsh plugin (`eval "$(meow init zsh)"`): Helvilette's blue eyes in the right prompt of each terminal.
  - The eyes move when you type, squint (`> <`) when a command fails, and close when you are idle.
  - Only the current prompt shows the eyes; old command lines do not keep them.
  - The plugin never reads the command line, and it needs no new permissions.
- Shared state: the time of the last meal is in `$XDG_STATE_HOME/meow/state`, so the cat is the same cat in all terminals.
- Install guide: [docs/how-to/install.md](docs/how-to/install.md).
- Decisions ADR-0001 to ADR-0004: a terminal application without global input capture, Rust, ASCII art from code with the artwork as the model, and a zsh plugin with a TUI command.

[Unreleased]: https://github.com/AlexanderSlokov/my-helvilette/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/AlexanderSlokov/my-helvilette/releases/tag/v0.1.0
