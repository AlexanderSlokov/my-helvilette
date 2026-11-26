# Changelog

All notable changes to this project are in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project uses [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- `meow` command: Helvilette's room, full screen in the terminal.
  The cat is ASCII art that the program generates from geometry traced from the Founder's artwork.
  He types when you type, moves his mouse and follows the pointer with his eyes, purrs when you pet his head, lifts his chin, and twitches his ears.
  He eats five dishes from the lore (F1 to F5, or a click on the tray).
  After 30 seconds without input he becomes drowsy, and after 90 seconds he falls asleep.
- zsh plugin (`eval "$(meow init zsh)"`): Helvilette's blue eyes in the right prompt of each terminal.
  The eyes move when you type, squint (`> <`) when a command fails, and close when you are idle.
  The plugin never reads the command line, and it needs no new permissions.
- Shared state: the time of the last meal is in `$XDG_STATE_HOME/meow/state`, so the cat is the same cat in all terminals.
- Install guide: [docs/how-to/install.md](docs/how-to/install.md).
- Decisions ADR-0001 to ADR-0004: a terminal application without global input capture, Rust, ASCII art from code with the artwork as the model, and a zsh plugin with a TUI command.

### Changed

- The rune halo is a full circle. It passes in front of the body; the rune marks show only outside the body.
- The front legs look like cat legs, not stick lines. Each leg has two side lines that are wide at the shoulder and narrow at the wrist, and the lines start thin so that the leg grows out of the chest. A round paw joins the two lines and sits on the keyboard. The inside of the leg hides the lines behind it. When a paw lifts or goes to the mouse, the leg bends.

### Fixed

- Old command lines no longer keep a copy of the eyes. The plugin removes the eyes from a line when you press Enter, so only the current prompt shows them.
