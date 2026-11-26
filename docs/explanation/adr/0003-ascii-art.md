# ADR-0003: Draw the cat with ASCII art

- **Status:** Accepted
- **Date:** 2026-11-26
- **Decider:** Founder

## Context

The terminal application (see [ADR-0001](0001-tui-first-no-global-input.md)) draws text.
We examined these options for the cat graphics:

- **ASCII art:** Each animation frame is plain text.
- **Sprite sheets:** Each frame is an image. Somebody must draw or generate the images, and the terminal must support an image protocol (for example, Kitty graphics or Sixel). Support is different in different terminals.
- **Unicode block or Braille characters:** These look like small pixel art, but each frame is more difficult to edit by hand.

## Decision

Draw the cat with ASCII art.
Keep each animation frame as plain text.
Do not use sprite sheets or terminal image protocols.

The art is original.
Do not copy art from other cat applications.

## Consequences

**Good:**

- Frames are easy to create and edit in a text editor.
- Git shows frame changes as text diffs.
- All terminals can show the frames.

**Bad:**

- The cat has less detail and less expression.
- The animation changes frame by frame. It is not smooth.

Unicode block characters are a possible upgrade in a later phase.
A new ADR must approve that change.
