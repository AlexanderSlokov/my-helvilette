# Backlog

Tasks for [Phase 3 in the roadmap](ROADMAP.md): the VS Code extension.
Decision: [ADR-0005](docs/explanation/adr/0005-vscode-extension.md). The rules of [ADR-0001](docs/explanation/adr/0001-tui-first-no-global-input.md) and [ADR-0004](docs/explanation/adr/0004-shell-plugin-and-tui.md) also apply.

Do the tasks in this order: B12, B13, B14, B15, B16, B17, B18.

## Rust core

### B12: Make the cat a library that does not need a terminal

- **Files:** `Cargo.toml`, `src/lib.rs` (new), `src/main.rs`, `src/tui.rs`, `src/art.rs`, `src/cat.rs`
- **Depends on:** none
- **Work:**
  - Add `src/lib.rs` that exports `art` and `cat`. `src/main.rs` uses the library.
  - Make `crossterm` optional, behind a default feature `tui`. `state.rs` and `tui.rs` belong to `tui`, because they use the file system and the system clock (`std::time::SystemTime` panics on `wasm32-unknown-unknown`).
  - Move the code that puts the words next to the head from `Room::frame` in `tui.rs` into the library (for example, `art::compose(layout, cat) -> Vec<Cell>`), so that the TUI and the extension show the same words. The tray and the status line stay in each front end.
- **Goal:**
  - `cargo build --lib --no-default-features --target wasm32-unknown-unknown` passes.
  - `cargo test`, `cargo clippy --all-targets -- -D warnings`, and `tests/pty/*.py` pass with no change in behavior.

### B13: Hagalaz grumbles when a command fails

- **Files:** `src/cat.rs`
- **Depends on:** B12
- **Work:** Add `Cat::grumble()`. It sets the Grumpy mood (Hagalaz) for 2.5 seconds, unless he is asleep, eating, or swiping.
- **Goal:** A unit test shows that `grumble()` makes Hagalaz grumble and that the mood ends after 2.5 seconds.

### B14: WebAssembly bindings

- **Files:** `Cargo.toml`, `src/wasm.rs` (new)
- **Depends on:** B12, B13
- **Work:**
  - Add `wasm-bindgen` as an optional dependency behind a feature `wasm`. Pin its exact version: `wasm-bindgen-cli` must have the same version.
  - Export one object, for example `Room`, with: `new(seed)`, `resize(cols, rows)`, `key()`, `pointer(col, row)`, `feed(food)`, `focus(on)`, `set_hungry(on)`, `grumble()`, `update(dt)`, and `frame()`.
  - `frame()` returns flat arrays, not one object per cell: the characters as `u32` code points and the colors as `r, g, b` bytes.
  - Keep the extension ID out of the Rust code.
- **Goal:** `cargo build --lib --no-default-features --features wasm --target wasm32-unknown-unknown --release` and `wasm-bindgen --target web` make a `.wasm` file and a small JavaScript loader. Write down the size of the `.wasm` file in the pull request.

## VS Code extension

### B15: Extension skeleton and the view

- **Files:** `extension/package.json`, `extension/extension.js`, `extension/media/view.js`, `extension/media/view.html` or HTML in `extension.js`, `extension/.vscodeignore`
- **Depends on:** B14
- **Work:**
  - `package.json`: `"publisher": "naughtian"`, `"name": "myhelvilette"`, `"displayName": "My Helvilette"`, `"engines": { "vscode": "^1.93.0" }`, `"extensionKind": ["ui"]`, a view container in `panel` with one webview view, and activation on that view.
  - The webview loads the `.wasm` file and the loader from `extension/media/`, and draws the glyph grid on a canvas. A cell is two times as tall as it is wide (the `Layout` in `art.rs` expects this). Use a monospace font; the runes need a font with the Runic block.
  - The tray (five foods) and the status line are HTML in the webview, under the canvas.
  - Content Security Policy: `default-src 'none'`, scripts and styles only from `webview.cspSource`. Find out if WebAssembly needs `'wasm-unsafe-eval'` in `script-src`, add it only if it is necessary, and write the result in ADR-0005.
  - When the view is not visible, stop the drawing loop (`onDidChangeVisibility`).
  - JavaScript only as glue: about 200 lines in total, no frameworks, no runtime npm dependencies.
- **Goal:** In the Extension Development Host (F5), the bottom panel shows Helvilette. The pointer pets him, and a click on a food feeds him.

### B16: Events from VS Code and the shared state

- **Files:** `extension/extension.js`, `extension/media/view.js`
- **Depends on:** B15
- **Work:**
  - `workspace.onDidChangeTextDocument`: send a "key" event to the view.
    - Use only events with `contentChanges.length > 0`.
    - Use only documents with the `file` or `untitled` scheme. Output channels and logs also change documents, and they must not make the cat type.
    - **Never keep, log, or send the text of a change.** Count the event only.
  - `window.onDidEndTerminalShellExecution`: if `exitCode` is a number that is not 0, call `grumble()`. If `exitCode` is `undefined` (VS Code does not know), do nothing.
  - `window.onDidChangeWindowState`: send `focused` to `focus()`, for Schrödinger Cat.
  - Shared state: read and write `fed_at` in `$XDG_STATE_HOME/meow/state` (default `~/.local/state/meow/state`) with the same format as `src/state.rs`: one line `fed_at=<Unix seconds>`. Write atomically (temporary file, then rename). Calculate hunger as `meow` does (8 hours).
- **Goal:** Manual test, written in the pull request: typing in a file makes the paws type; typing in the Output panel does not; `false` in the integrated terminal makes Hagalaz grumble; a meal in VS Code shows "no bụng" in `meow` in a terminal.

## Build and CI

### B17: Package the `.vsix` in CI

- **Files:** `.github/workflows/ci.yml`, `extension/package.json` (scripts)
- **Depends on:** B16
- **Work:**
  - Add the `wasm32-unknown-unknown` target and the matching `wasm-bindgen-cli` version.
  - Build the WebAssembly, copy it to `extension/media/`, and run `npx @vscode/vsce package` in `extension/`.
  - Upload the `.vsix` file as a workflow artifact.
  - Do not commit generated files (`.wasm`, the loader, `.vsix`). Add them to `.gitignore`.
- **Goal:** CI is green and has a `.vsix` artifact that `code --install-extension` installs.

## Documentation

### B18: Install guide for VS Code and release notes

- **Files:** `docs/how-to/install-vscode.md` (new), `README.md`, `CHANGELOG.md`
- **Depends on:** B17
- **Goal:** A user can build or download the `.vsix` and install it with the guide only. The guide says which events the extension uses and that it never keeps the text that you type. `CHANGELOG.md` has the extension under `[Unreleased]`.
