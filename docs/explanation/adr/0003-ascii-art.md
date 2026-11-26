# ADR-0003: Generate the cat as ASCII art from code, with the artwork as the model

- **Status:** Accepted
- **Date:** 2026-11-26 (revised two times on the same day; see "History")
- **Decider:** Founder

## Context

The terminal application (see [ADR-0001](0001-tui-first-no-global-input.md)) draws text.

The cat is Helvilette, as the Founder's artwork shows it: a black cat that sits in a ring of runes, with neon cyan and violet outlines and blue eyes.
The Founder requires the cat to be **100% this cat**.
A different or simplified cat is not acceptable.

We examined these methods:

| Method | Result |
|---|---|
| Hand-drawn ASCII frames | Rejected. The frames do not show this cat accurately, and each pose needs a new drawing. |
| Sprite sheets with a terminal image protocol | Rejected. Support for image protocols is different in different terminals. |
| Sample the artwork at runtime and convert it to glyphs (the naughtian-website method) | Rejected. The application must contain the image, and only the parts that the website draws live (the eyes) can move. |
| Describe the cat as geometry in code, and convert the geometry to glyphs | **Selected.** |

The naughtian-website repository (`public/helvilette/helvilette-ascii.js`) is a **visual reference** only.
It shows how this cat looks as ASCII art.
It also contains measured geometry in pixels of the original 2752x1536 image (eyes, face, rune ring, cat outline, ears, whiskers).
We can use these measurements as a start.

## Decision

1. Describe the cat as geometry in code: outlines, ears, eyes, nose, legs, tail, and rune ring.
   Use the pixel coordinates of the original artwork, so that the code can be compared with the artwork.
2. Trace the geometry from the artwork. The artwork is the model.
3. Do not put the artwork in the application.
   Do not sample the image at runtime.
4. Convert the geometry to glyphs at runtime:
   - Calculate the intensity of each part at 2x2 points for each character cell.
     For neon lines, the intensity decreases with the distance from the line.
   - Select a line glyph (`/ \ | - _ ' , .`) when the 2x2 points show an edge.
     Otherwise, select a glyph from the ramp ` .:-=+*#%@`.
   - Show colors with 24-bit ANSI color codes.
5. Animate the cat by changes to its geometry: eyes, ears, paws, tail, and head.
6. Draw the keyboard, the mouse, and the paws with the same method.
   Show words (food names, "purr", "zZ") as text on top of the glyph grid.
   The twelve runes on the halo (one for each True Self of Helvilette) are also text: Unicode Runic characters, one cell each.
   A drawn rune would be smaller than a few cells, so it would not be legible.
   This is the only part of the art that is not ASCII. It needs a font with the Runic block (for example, Noto Sans Runic); without one, the terminal shows boxes.
7. Do not use terminal image protocols (Kitty graphics, Sixel).

## Consequences

**Good:**

- All parts of the cat can move, not only the eyes.
- The art fits each terminal size, because the geometry is scaled before the glyphs are selected.
- The application does not contain an image file.

**Bad:**

- We must trace the full geometry from the artwork by hand. This takes time.
- Accuracy depends on the trace. To compare the trace with the artwork, use a development tool that draws the geometry on the artwork. Do not ship this tool or the artwork.
- The terminal must support 24-bit color for the correct neon colors.
- Small details (rune letters, whiskers) are lost at small sizes.

## History

- First version: hand-drawn ASCII frames.
- Second version: sample the artwork at runtime, as naughtian-website does.
- Current version: geometry in code, with the artwork as the model. The website is a visual reference only.
