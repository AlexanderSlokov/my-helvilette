# Install meow and enable Helvilette in zsh

This guide installs the `meow` command and enables the zsh plugin.
After these steps, each new zsh terminal shows Helvilette's blue eyes in the right prompt, and `meow` opens his room.

## Prerequisites

- A Rust toolchain (`cargo`).
- zsh 5.3 or later.
- A terminal with 24-bit color and mouse reporting (for example, GNOME Terminal, Kitty, WezTerm, or Alacritty).

## Procedure

1. Install the command from the repository root:

   ```sh
   cargo install --path .
   ```

2. Make sure that `~/.cargo/bin` is in your `PATH`:

   ```sh
   command -v meow
   ```

3. Add this line to `~/.zshrc`, after the line that loads your theme or oh-my-zsh:

   ```sh
   eval "$(meow init zsh)"
   ```

4. Open a new terminal. The right prompt shows two blue eyes: `o o`.

5. To open Helvilette's room, type:

   ```sh
   meow
   ```

## What the eyes in the prompt show

| Eyes | Meaning |
|---|---|
| `o o` | Awake. The eyes move when you type. |
| `- -` | A slow blink. |
| `> <` | The last command failed. |
| `= =` | Drowsy: no input for 30 seconds. |
| `- - z` | Asleep: no input for 90 seconds. A key wakes him. |

## Controls in the room

| Input | Result |
|---|---|
| Any key | A paw taps the keyboard. |
| Mouse movement | His mouse moves, and his eyes follow the pointer. |
| Pointer moved over his head several times | He purrs. |
| Pointer moved under his chin | He lifts his chin. |
| Pointer on an ear | The ear twitches. |
| Click a food on the tray, or F1 to F5 | He eats. |
| Esc or Ctrl+C | Close the room. |

## Shared state

The time of the last meal is in `$XDG_STATE_HOME/meow/state` (default `~/.local/state/meow/state`).
All terminals use this file.
To reset the cat, delete the file.

## Remove the plugin

1. Remove the `eval "$(meow init zsh)"` line from `~/.zshrc`.
2. Remove the command: `cargo uninstall meow`.
