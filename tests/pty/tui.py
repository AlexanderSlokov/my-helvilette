"""End-to-end test of the `meow` TUI in a pseudo-terminal.

Usage: python3 tests/pty/tui.py [target/debug/meow]
"""

import os
import sys
import tempfile

from term import Checks, Session

binary = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else "target/debug/meow")
NAMES = ["Othela", "Ansuz", "Mannaz", "Berkano", "Isa", "Wunjo", "Laguz",
         "Hagalaz", "Gebo", "Sowilo", "Raidho", "Tiwaz"]

check = Checks("meow TUI")
state_home = tempfile.mkdtemp(prefix="meow-test-")
room = Session([binary], env={"XDG_STATE_HOME": state_home}, rows=50, cols=100)

room.pump(3.0)
check("meo...".encode() in room.out, "a cat that never ate is sad (Laguz)")
check(all(r.encode() in room.out for r in "ᛟᚨᛗᛒᛁᚹᛚᚺᚷᛊᚱᛏ"), "the halo shows the twelve runes")

room.send("abc", 0.3)
for col in range(30, 70, 4):
    room.send(f"\x1b[<35;{col};14M", 0.02)

room.send("\x1b[O", 2.6)  # the terminal loses focus
start = len(room.out)
room.send("\x1b[I", 2.0)  # and gets it back
seen = [n for n in NAMES if n.encode() in room.out[start:]]
check(len(seen) > 0, f"Schrödinger Cat: another True Self says his name: {seen}")

room.send("\x1bOP", 0.5)  # F1: feed
# Only changed cells are written, so look for a part of the words that is new.
check("đậu xanh".encode() in room.out, "F1 feeds the cat: bánh cam nhân đậu xanh")
room.send("\x1b", 0.5)    # Esc
check(room.wait() == 0, "Esc closes the room with exit code 0")

for name, seq in [("alternate screen", b"\x1b[?1049l"), ("mouse capture", b"\x1b[?1000l"),
                  ("focus reporting", b"\x1b[?1004l")]:
    check(seq in room.out, f"the terminal is restored: {name} off")
check(b"panicked" not in room.out, "no panic")
state = os.path.join(state_home, "meow", "state")
saved = open(state).read() if os.path.exists(state) else ""
check(saved.startswith("fed_at="), f"the meal is saved in the state file: {saved.strip()!r}")
check.done()
