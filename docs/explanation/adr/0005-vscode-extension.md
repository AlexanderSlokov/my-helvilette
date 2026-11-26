# ADR-0005: Add a VS Code extension, with the cat compiled to WebAssembly

- **Status:** Accepted
- **Date:** 2026-11-26
- **Decider:** Founder

## Context

The Founder works mostly in VS Code.
In VS Code, Helvilette is visible only as the eyes in the prompt of the integrated terminal, or as `meow` in a terminal tab.

[ADR-0001](0001-tui-first-no-global-input.md) forbids global input capture.
It does not forbid input that a host application already gives to the program.
A terminal emulator, a terminal multiplexer, and VS Code all receive the keys that the user types into them.
Thus, if the cat lives inside such a host, it can react to typing without a keylogger.

We examined three hosts:

| Option | Result |
|---|---|
| A. Our own terminal emulator (GUI) | Rejected. To replace the Founder's daily terminal, it needs fonts, scrollback, selection, clipboard, tabs, and an input method for Vietnamese on Wayland. Most of the work is not about the cat. |
| B. `meow shell`: a PTY host, like a small tmux, that shows the shell and the cat side by side in any terminal | Deferred. It is a consideration in the [roadmap](../../../ROADMAP.md). Risk: a rendering bug breaks the programs that run inside it (vim, htop). |
| C. A VS Code extension | **Selected.** |

We checked the stable API in `vscode.d.ts` (microsoft/vscode, branch `main`, 2026-11-26):

| Need | Stable API | Available |
|---|---|---|
| Typing in the editor | `workspace.onDidChangeTextDocument` (`contentChanges`) | Yes |
| A command in the integrated terminal failed | `window.onDidEndTerminalShellExecution` (`exitCode`), from VS Code 1.93 | Yes |
| Window focus, for Schrödinger Cat | `window.onDidChangeWindowState` (`focused`) | Yes |
| A view for the cat in the sidebar or the panel | `window.registerWebviewViewProvider` | Yes |
| Keys typed in the integrated terminal | `window.onDidWriteTerminalData` | **No.** It is a proposed API; installed extensions cannot use it. The zsh plugin covers the terminal. |

An extension can be installed without the Marketplace: `vsce package` makes a `.vsix` file, and `code --install-extension <file>.vsix` installs it.

## Decision

1. Build a VS Code extension that shows Helvilette in a webview view.
   Put it in the bottom panel by default. The user can move it.
2. **Use the same cat.** Compile the art and the behavior (`art.rs`, `cat.rs`) to WebAssembly.
   The `meow` command and the extension use one Rust core.
   The webview draws the glyph grid on a canvas.
3. **JavaScript or TypeScript is glue only:** activate the extension, send events to the webview, draw the grid, and keep the state.
   Target: about 200 lines.
   No frameworks and no runtime npm dependencies.
4. The cat reacts to these events:
   - Typing in the editor (`onDidChangeTextDocument`): the paws type. Count events only. **Never keep, log, or send the text of a change.**
   - A failed command in the integrated terminal (`exitCode` is not 0): Hagalaz grumbles.
   - The pointer in the webview: petting, scratching, and the mouse.
   - Window focus (`onDidChangeWindowState`): Schrödinger Cat.
5. The extension uses the same state file as `meow` ([ADR-0004](0004-shell-plugin-and-tui.md)), so the cat in VS Code and the cat in the terminal are the same cat.
6. The extension runs on the local machine (`"extensionKind": ["ui"]`), also in remote windows (SSH, WSL, containers). The editor events and the state file are local.
7. When the view is not visible, the webview stops drawing.
8. Security:
   - The extension sends no data out of VS Code.
   - The webview Content Security Policy is `default-src 'none'`, and scripts load only from the extension.
   - WebAssembly needs `'wasm-unsafe-eval'` in `script-src`. The implementation must verify this, because the VS Code webview guide does not state it.
9. Distribution: a `.vsix` file, installed with `code --install-extension`. Do not publish to the Marketplace now.

## Consequences

**Good:**

- Helvilette is visible where the Founder works, all the time.
- The art and the behavior are written one time, in Rust.
- No new permissions. The extension uses only events that VS Code gives to all extensions.

**Bad:**

- The repository gets a second toolchain: Node.js, `@vscode/vsce`, the `wasm32-unknown-unknown` Rust target, and `wasm-bindgen`.
- The Rust code must become a library that the `meow` binary and the WebAssembly build both use.
- CI must also build the WebAssembly and the `.vsix` file.
- The extension does not see keys typed in the integrated terminal. In the terminal, the prompt eyes stay the way to see typing.
- VS Code forks (for example, VSCodium) can probably install the `.vsix`, but we do not test them.

## Names and placement

Decided by the Founder on 2026-11-26:

- Publisher `naughtian`, extension name `myhelvilette`. The extension ID is `naughtian.myhelvilette`.
- The default place of the view is the bottom panel. The user can move it to the sidebar; the layout scales to the size of the view.
