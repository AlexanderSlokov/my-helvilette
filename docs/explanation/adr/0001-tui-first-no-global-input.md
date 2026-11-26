# ADR-0001: Build a terminal application first, and do not capture global input

- **Status:** Accepted
- **Date:** 2026-11-26
- **Decider:** Founder

## Context

The product is a cat that lives on the computer.
The cat copies the user: it types when the user types, and it moves its mouse when the user moves the mouse.
The user can pet the cat, scratch its chin and ears, and feed it.
When the user is idle, the cat falls asleep.

To copy input from all applications, the program must read global input.
The first target system is Ubuntu with GNOME on Wayland.
Wayland does not let a client read input that goes to other clients.
The only method is to read `/dev/input/event*` directly (evdev or libinput).
This method needs membership in the `input` group.

Membership in the `input` group gives the permission to **every process of the user**, not only to this program.
Thus, any malicious process in the user session can read all keystrokes, which include passwords.
This adds a keylogger attack vector to the full session.

We examined similar projects:

| Project | Input method | Display | Result |
|---|---|---|---|
| saatvik333/wayland-bongocat | `/dev/input`, `input` group | `wlr-layer-shell` | Does not support GNOME. |
| Sivecano/layer_bongocat | libinput, `input` group | `wlr-layer-shell` | The README states: "This _is_ a keylogger." |
| ayangweb/BongoCat | libinput (July 2025) | — | Wayland support removed in July 2025. Wayland pull requests are experimental. |

No project captures global input on Wayland without the `input` group.

We also examined a privilege-separated helper daemon.
The daemon runs as a dedicated system user in the `input` group, and it sends only "key pressed" events without key codes.
This decreases the risk, but it adds a privileged component that we must maintain and audit.
It also shows keystroke timing to other processes.

GNOME (Mutter) does not support `wlr-layer-shell`.
A desktop overlay on GNOME must use XWayland.

## Decision

1. **Phase 1:** Build a terminal (TUI) application. The cat is present in the terminals that the user opens.
2. The application does **not** read global input.
   It does not read `/dev/input`.
   It does not need the `input` group or root.
   It does not install a helper daemon.
3. The cat reacts only to input that the terminal sends to it: keys, and mouse events through terminal mouse reporting.
4. **Phase 2 (optional):** Build a desktop overlay for Windows and macOS.
   Before work starts, a new ADR must approve the input-capture method for each platform.

**Reason:** We are IT operators.
An operator does not install a keylogger to make a computer look nice.
A cosmetic feature does not justify a security risk for the full session.

## Consequences

**Good:**

- The program needs no new permissions.
  The attack surface of the user session does not change.
- Phase 1 has no window-manager problems (transparency, always-on-top, layer shell).
- Petting, scratching, feeding, and sleep work without special permissions.

**Bad:**

- The cat does not copy input that goes to other applications.
  It copies only input that goes to its own terminal.
- On Linux, the cat cannot have the same features as Bongo-style overlay applications.

## Open questions

- **How the cat is present in each terminal.**
  Possible methods: a shell integration (for example, a zsh hook), a tmux pane or popup, or a small separate terminal window with "Always on Top".
  A later ADR selects the method.
- **Phase 2 input capture.**
  On macOS, global input needs an "Input Monitoring" permission that the user gives to one application.
  On Windows, a low-level keyboard hook needs no special permission.
  On both platforms, the program does not give a new permission to other processes, but the input group on Linux does.
  The Phase 2 ADR must verify these statements.

## References

- [saatvik333/wayland-bongocat](https://github.com/saatvik333/wayland-bongocat)
- [Sivecano/layer_bongocat](https://codeberg.org/Sivecano/layer_bongocat)
- [ayangweb/BongoCat, Wayland issues](https://github.com/ayangweb/BongoCat/issues?q=wayland)
