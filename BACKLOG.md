# Backlog

Tasks for [Phase 1 in the roadmap](ROADMAP.md).
Decisions: [ADR-0001 to ADR-0004](docs/explanation/adr/).

## Rust project

### B1: Create the Cargo project and the command line

- **Files:** `Cargo.toml`, `src/main.rs`, `.gitignore`
- **Depends on:** none
- **Goal:** `cargo build` passes. `meow` starts the TUI. `meow init zsh` prints the zsh plugin.

### B2: Trace the cat geometry and convert it to glyphs

- **Files:** `src/art.rs`
- **Depends on:** B1
- **Goal:** The renderer draws Helvilette (outline, ears, eyes, nose, legs, tail, rune ring, glitch bars), a keyboard, a mouse, and two paws as colored glyphs for each terminal size. The geometry uses pixel coordinates of the original 2752x1536 artwork. Unit tests pass.

### B3: Cat behavior

- **Files:** `src/cat.rs`
- **Depends on:** B1
- **Goal:** A state machine with these moods: awake, drowsy (after 30 s idle), asleep (after 90 s idle), startled, petted, eating. Keys lift the paws in turn. Mouse movement moves the cat's mouse. The pointer on the head, chin, or ears pets or scratches the cat. Unit tests pass.

### B4: Shared state file

- **Files:** `src/state.rs`
- **Depends on:** B1
- **Goal:** The last meal time is in `$XDG_STATE_HOME/meow/state` (default `~/.local/state/meow/state`). Writes are atomic (temporary file, then rename). A unit test reads back a written value.

### B5: TUI loop

- **Files:** `src/tui.rs`
- **Depends on:** B2, B3, B4
- **Goal:** `meow` shows the scene, a food tray, and a status line. The cat reacts to keys and to the mouse. The user feeds the cat with a click on the tray or with F1 to F5. Esc or Ctrl+C closes the TUI and restores the terminal. Only changed cells are written.

## zsh plugin

### B6: Eyes in the right prompt

- **Files:** `shell/meow.zsh`
- **Depends on:** B1
- **Goal:** The right prompt shows two blue eyes. Typing moves the eyes. A failed command shows `> <`. After 30 s idle the eyes are drowsy, after 90 s they sleep. The plugin never reads `$BUFFER`. A test in a pseudo-terminal shows these frames.

## Documentation

### B7: Installation guide

- **Files:** `README.md`, `docs/how-to/install.md`
- **Depends on:** B5, B6
- **Goal:** A user can install `meow` and enable the plugin with the guide only.
