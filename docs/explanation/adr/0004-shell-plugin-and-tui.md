# ADR-0004: Put the cat in each terminal with a zsh plugin and a TUI command

- **Status:** Accepted
- **Date:** 2026-11-26
- **Decider:** Founder

## Context

[ADR-0001](0001-tui-first-no-global-input.md) states that the cat must be present in each terminal that the user opens.
It must do this without global input capture.

A zsh plugin can be present in each terminal, but it has these limits:

- The zsh line editor does not receive mouse events. Thus, the user cannot pet or scratch the cat in the prompt.
- The prompt has space for a few characters only. The full scene (cat, keyboard, and mouse) does not fit.

A full-screen TUI has mouse events and space, but it is not present in each terminal.

## Decision

Use two parts that show the same cat:

1. **zsh plugin (`meow.zsh`)**, in each terminal:
   - It shows a small cat in the right prompt (`RPROMPT`).
   - When the user types, the cat types. A zle hook counts keystrokes.
   - When a command fails (exit status is not 0), the cat grumbles.
   - When the user is idle, the cat becomes drowsy and then sleeps (a recurring `sched` event).
     Do not use `TMOUT`: if `TRAPALRM` is not defined, `TMOUT` closes the shell.
2. **`meow` command**, a full TUI, when the user wants to play with the cat:
   - It shows the full scene: the cat, a keyboard, and a mouse.
   - The user pets and scratches the cat with the mouse, and feeds it.
   - The user opens it with a zle key binding or with `tmux display-popup -E meow`.
3. **One state file** in `$XDG_STATE_HOME/meow/` (default `~/.local/state/meow/`):
   - It keeps the cat state that all terminals share: last meal time, mood.
   - The program calculates hunger from timestamps. A background process is not necessary.
   - Write the file atomically: write a temporary file, then rename it.

Rules for the plugin:

- Use only zsh built-in features in hooks that run on each keystroke.
  The plugin code does not start an external process.
- To show a new eye frame, the plugin must redraw the prompt (`zle reset-prompt`).
  This expands the user's prompt again, and some themes start processes (for example, oh-my-zsh themes start `git`).
  Thus, redraw the prompt one time in 150 ms, at most.
  Measured cost with the oh-my-zsh `robbyrussell` theme: 0.8 ms for each redraw.
- Do not read `$BUFFER` or the command text.
  Do not record keys or commands.
  The plugin uses only keystroke counts, timing, and exit status.

## Consequences

**Good:**

- The cat is present in each zsh terminal, and it needs no new permissions.
- Each part does only what it can do well.
- Shared state makes the cat in all terminals the same cat.

**Bad:**

- Phase 1 supports zsh only. Bash has no hook that runs on each keystroke.
- The prompt cannot show the full artwork (see [ADR-0003](0003-ascii-art.md)).
- Two terminals can write the state file at the same time. The last write wins. This is acceptable for cat state.

## Prompt appearance

The full cat does not fit in a few characters.
Thus, the prompt shows only the two blue eyes of Helvilette, in 24-bit color.
The eyes blink. They become half closed when the cat is drowsy, and they close when the cat sleeps.
To see the full cat, the user runs `meow`.
