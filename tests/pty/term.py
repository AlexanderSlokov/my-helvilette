"""A pseudo-terminal session for the end-to-end tests (Python standard library only)."""

import fcntl
import os
import pty
import select
import signal
import struct
import sys
import termios
import time


class Session:
    """Starts `argv` in a pseudo-terminal and keeps everything that it writes."""

    def __init__(self, argv, env=None, rows=40, cols=100):
        pid, fd = pty.fork()
        if pid == 0:
            os.environ.update(env or {})
            os.environ["TERM"] = "xterm-256color"
            try:
                os.execvp(argv[0], argv)
            finally:
                os._exit(127)
        fcntl.ioctl(fd, termios.TIOCSWINSZ, struct.pack("HHHH", rows, cols, 0, 0))
        self.pid, self.fd, self.out = pid, fd, b""

    def pump(self, secs):
        """Reads output for `secs` seconds."""
        end = time.time() + secs
        while time.time() < end:
            ready, _, _ = select.select([self.fd], [], [], 0.05)
            if ready:
                try:
                    self.out += os.read(self.fd, 65536)
                except OSError:
                    return

    def send(self, data, wait=0.2):
        os.write(self.fd, data.encode() if isinstance(data, str) else data)
        self.pump(wait)

    def wait(self, timeout=5.0):
        """Waits for the program to stop and returns its exit code."""
        end = time.time() + timeout
        while time.time() < end:
            pid, status = os.waitpid(self.pid, os.WNOHANG)
            if pid:
                return os.waitstatus_to_exitcode(status)
            self.pump(0.05)
        os.kill(self.pid, signal.SIGKILL)
        raise TimeoutError("the program did not stop")


class Checks:
    """Collects failed checks, so that one run reports all of them."""

    def __init__(self, name):
        self.name, self.failed = name, []

    def __call__(self, ok, what):
        print(("ok    " if ok else "FAIL  ") + what)
        if not ok:
            self.failed.append(what)

    def done(self):
        if self.failed:
            print(f"{self.name}: {len(self.failed)} check(s) failed")
            sys.exit(1)
        print(f"{self.name}: all checks passed")
