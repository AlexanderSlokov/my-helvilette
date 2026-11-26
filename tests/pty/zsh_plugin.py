"""End-to-end test of the zsh plugin in an interactive zsh.

Usage: python3 tests/pty/zsh_plugin.py [shell/meow.zsh]
It takes about 25 seconds, because it waits for the cat to get drowsy and to sleep.
"""

import os
import re
import sys

from term import Checks, Session

plugin = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else "shell/meow.zsh")
EYES = re.compile(rb"\x1b\[38;2;159;230;255m([^\x1b]*)\x1b\[0m")


def frames(data):
    return [m.group(1).decode() for m in EYES.finditer(data)]


check = Checks("zsh plugin")
zsh = Session(["zsh", "-f", "-i"])
zsh.pump(0.5)
# Short thresholds, so that the test does not wait 90 seconds.
zsh.send(f"PS1='%# '; source {plugin}; _meow_drowsy_after=2; _meow_asleep_after=15\n", 1.0)
check("o o" in "".join(frames(zsh.out)), "the eyes show in the right prompt")

start = len(zsh.out)
for ch in "echo hi":
    zsh.send(ch, 0.2)
typed = frames(zsh.out[start:])
check(" o o" in typed and "o o " in typed, f"the eyes move while you type: {typed}")

start = len(zsh.out)
zsh.send("\r", 0.6)
finished = zsh.out[start:zsh.out.index(b"hi\r\n", start)]
last_line = finished[finished.rindex(b"echo hi"):]
check(not EYES.search(last_line), "Enter removes the eyes from the finished line")

start = len(zsh.out)
zsh.send("false\n", 0.6)
check("> <" in "".join(frames(zsh.out[start:])), "a failed command shows '> <'")

start = len(zsh.out)
zsh.pump(11.0)
check("= =" in "".join(frames(zsh.out[start:])), "idle: the eyes get drowsy")
start = len(zsh.out)
zsh.pump(10.0)
check("- - z" in "".join(frames(zsh.out[start:])), "idle longer: the cat sleeps")

start = len(zsh.out)
zsh.send("x", 0.5)
check("o o" in "".join(frames(zsh.out[start:])), "a key wakes the cat")

zsh.send("\x15exit 0\n", 0.3)
check(zsh.wait() == 0, "zsh exits normally")
errors = re.findall(rb"[^\r\n]*(?:command not found|parse error|bad math|no such)[^\r\n]*", zsh.out)
check(not errors, f"no zsh errors: {errors[:3]}")
check.done()
