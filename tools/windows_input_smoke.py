"""Hosted Windows only: native pointer/message-pump smoke, NOT render acceptance.

No clicks, device access, screenshots, credentials, or third-party dependencies.
"""
import argparse
import ctypes
from ctypes import wintypes as W
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile
import time

TITLE = "Speaker Link — Device-free UI preview"


def require(condition, reason):
    if not condition:
        raise RuntimeError(reason)


def exercise(api, process, emit, seconds=8):
    deadline = time.monotonic() + 20
    hwnd = None
    while time.monotonic() < deadline:
        require(process.poll() is None, "app-exited-before-window")
        hwnd = api.find_window(process.pid)
        if hwnd:
            break
        time.sleep(0.1)
    require(hwnd, "owned-visible-window-timeout")
    api.activate(hwnd)
    emit("window-found", pid=process.pid)
    api.responsive(hwnd, process.pid)
    # Stay well within the client area; no buttons are clicked or keys sent.
    points = api.points(hwnd)
    end = time.monotonic() + seconds
    moves = 0
    while time.monotonic() < end:
        require(process.poll() is None, "app-exited-during-input")
        for point in points:
            api.verify_target(hwnd, process.pid, point)
            api.move(point)
            time.sleep(0.1)
            api.verify_cursor(point)
            api.verify_target(hwnd, process.pid, point)
            api.responsive(hwnd, process.pid)
            moves += 1
    require(moves >= len(points), "no-input-observed")
    require(process.poll() is None, "app-exited-after-input")
    emit("native-pointer-message-pump-pass", moves=moves)


class Windows:
    def __init__(self):
        require(os.name == "nt", "windows-required")
        self.u = ctypes.WinDLL("user32", use_last_error=True)
        u = self.u
        self.callback = ctypes.WINFUNCTYPE(W.BOOL, W.HWND, W.LPARAM)
        signatures = {
            "EnumWindows": ([self.callback, W.LPARAM], W.BOOL),
            "GetWindowThreadProcessId": ([W.HWND, ctypes.POINTER(W.DWORD)], W.DWORD),
            "IsWindowVisible": ([W.HWND], W.BOOL),
            "GetWindowTextW": ([W.HWND, W.LPWSTR, ctypes.c_int], ctypes.c_int),
            "SetForegroundWindow": ([W.HWND], W.BOOL),
            "GetForegroundWindow": ([], W.HWND),
            "GetClientRect": ([W.HWND, ctypes.POINTER(W.RECT)], W.BOOL),
            "ClientToScreen": ([W.HWND, ctypes.POINTER(W.POINT)], W.BOOL),
            "WindowFromPoint": ([W.POINT], W.HWND),
            "GetAncestor": ([W.HWND, W.UINT], W.HWND),
            "IsHungAppWindow": ([W.HWND], W.BOOL),
            "SendMessageTimeoutW": ([W.HWND, W.UINT, W.WPARAM, W.LPARAM, W.UINT, W.UINT, ctypes.POINTER(ctypes.c_size_t)], W.LPARAM),
            "GetCursorPos": ([ctypes.POINTER(W.POINT)], W.BOOL),
            "GetSystemMetrics": ([ctypes.c_int], ctypes.c_int),
            "OpenInputDesktop": ([W.DWORD, W.BOOL, W.DWORD], W.HANDLE),
            "CloseDesktop": ([W.HANDLE], W.BOOL),
            "SetProcessDPIAware": ([], W.BOOL),
        }
        for name, (args, result) in signatures.items():
            fn = getattr(u, name)
            fn.argtypes, fn.restype = args, result
        require(u.SetProcessDPIAware(), "dpi-coordinate-mode-unavailable")
        desktop = u.OpenInputDesktop(0, False, 1)  # DESKTOP_READOBJECTS only
        require(desktop, "interactive-input-desktop-unavailable")
        require(u.CloseDesktop(desktop), "input-desktop-close-failed")

        class Mouse(ctypes.Structure):
            _fields_ = [("dx", W.LONG), ("dy", W.LONG), ("mouseData", W.DWORD),
                        ("dwFlags", W.DWORD), ("time", W.DWORD), ("dwExtraInfo", ctypes.c_size_t)]

        class Payload(ctypes.Union):
            _fields_ = [("mi", Mouse)]  # MOUSEINPUT is largest INPUT union member

        class Input(ctypes.Structure):
            _fields_ = [("type", W.DWORD), ("payload", Payload)]
        self.Mouse, self.Input = Mouse, Input
        u.SendInput.argtypes = [W.UINT, ctypes.POINTER(Input), ctypes.c_int]
        u.SendInput.restype = W.UINT
        require(ctypes.sizeof(Input) == (40 if ctypes.sizeof(ctypes.c_void_p) == 8 else 28), "input-abi-mismatch")

    def owned(self, hwnd, pid):
        actual = W.DWORD()
        self.u.GetWindowThreadProcessId(hwnd, ctypes.byref(actual))
        require(actual.value == pid, "window-owner-mismatch")

    def find_window(self, pid):
        found = []
        def visit(hwnd, _):
            actual = W.DWORD()
            self.u.GetWindowThreadProcessId(hwnd, ctypes.byref(actual))
            title = ctypes.create_unicode_buffer(256)
            self.u.GetWindowTextW(hwnd, title, len(title))
            if actual.value == pid and self.u.IsWindowVisible(hwnd) and title.value == TITLE:
                found.append(hwnd)
            return True
        require(self.u.EnumWindows(self.callback(visit), 0), "window-enumeration-failed")
        require(len(found) <= 1, "ambiguous-owned-window")
        return found[0] if found else None

    def activate(self, hwnd):
        require(self.u.SetForegroundWindow(hwnd), "foreground-request-denied")
        time.sleep(0.2)
        require(self.u.GetForegroundWindow() == hwnd, "foreground-not-owned-window")

    def points(self, hwnd):
        rect = W.RECT()
        require(self.u.GetClientRect(hwnd, ctypes.byref(rect)), "client-rect-failed")
        require(rect.right >= 200 and rect.bottom >= 200, "client-area-too-small")
        points = []
        for x, y in [(0.2, 0.2), (0.8, 0.2), (0.8, 0.8), (0.2, 0.8), (0.5, 0.5)]:
            point = W.POINT(int(rect.right*x), int(rect.bottom*y))
            require(self.u.ClientToScreen(hwnd, ctypes.byref(point)), "client-to-screen-failed")
            points.append((point.x, point.y))
        return points

    def verify_target(self, hwnd, pid, point):
        self.owned(hwnd, pid)
        require(self.u.GetForegroundWindow() == hwnd, "foreground-lost")
        hit = self.u.WindowFromPoint(W.POINT(*point))
        require(hit and self.u.GetAncestor(hit, 2) == hwnd, "pointer-hit-test-not-owned-window")

    def move(self, point):
        x, y = point
        left, top, width, height = [self.u.GetSystemMetrics(n) for n in (76, 77, 78, 79)]
        require(width > 1 and height > 1 and left <= x < left+width and top <= y < top+height, "point-outside-virtual-desktop")
        # ABSOLUTE | MOVE | VIRTUALDESK | MOVE_NOCOALESCE; real input queue, not WM_MOUSEMOVE.
        event = self.Input()
        event.type = 0
        event.payload.mi = self.Mouse(round((x-left)*65535/(width-1)), round((y-top)*65535/(height-1)), 0, 0xE001, 0, 0)
        require(self.u.SendInput(1, ctypes.byref(event), ctypes.sizeof(event)) == 1, "native-input-not-inserted")

    def verify_cursor(self, point):
        actual = W.POINT()
        require(self.u.GetCursorPos(ctypes.byref(actual)), "cursor-position-unavailable")
        require(abs(actual.x-point[0]) <= 1 and abs(actual.y-point[1]) <= 1, "native-cursor-position-mismatch")

    def responsive(self, hwnd, pid):
        self.owned(hwnd, pid)
        require(not self.u.IsHungAppWindow(hwnd), "windows-reports-app-hung")
        result = ctypes.c_size_t()
        # WM_NULL, SMTO_BLOCK | SMTO_ABORTIFHUNG | SMTO_ERRORONEXIT; at most 1 second.
        require(self.u.SendMessageTimeoutW(hwnd, 0, 0, 0, 0x23, 1000, ctypes.byref(result)), "message-pump-timeout-or-api-failure-or-window-exit")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("exe", type=Path)
    args = parser.parse_args()
    started = time.monotonic()
    records = []
    def emit(phase, **values):
        record = dict(phase=phase, elapsed_seconds=round(time.monotonic()-started, 3), **values)
        records.append(record)
        print(json.dumps(record), flush=True)
    process = None
    status = 1
    with tempfile.TemporaryFile() as stdout, tempfile.TemporaryFile() as stderr:
        try:
            require(os.environ.get("GITHUB_ACTIONS") == "true" and os.environ.get("RUNNER_OS") == "Windows", "hosted-windows-ci-only")
            api = Windows()
            exe = args.exe.resolve(strict=True)
            require(exe.name == "speakerlink-app.exe", "unexpected-executable-name")
            emit("launch", executable_sha256=hashlib.sha256(exe.read_bytes()).hexdigest())
            process = subprocess.Popen([str(exe)], stdin=subprocess.DEVNULL, stdout=stdout, stderr=stderr)
            exercise(api, process, emit)
            status = 0
        except Exception as error:
            # Only our fixed reason codes; never print arbitrary OS paths/environment/output.
            reason = str(error) if type(error) is RuntimeError else type(error).__name__
            emit("failed", reason=reason)
        finally:
            if process is not None:
                exit_code = process.poll()
                if exit_code is not None:
                    status = 1
                    emit("failed", reason="app-exited-before-cleanup", exit_code=exit_code)
                if exit_code is None:
                    process.terminate()  # owned Popen handle only, never a name-wide kill
                    try:
                        process.wait(timeout=3)
                    except subprocess.TimeoutExpired:
                        process.kill()
                        process.wait(timeout=3)
                emit("owned-process-cleaned", pid=process.pid, exit_before_cleanup=exit_code)
            # Capture byte counts, not unsanitized application strings or environment values.
            emit("captured-output", stdout_bytes=stdout.seek(0, 2), stderr_bytes=stderr.seek(0, 2))
            emit("result", passed=status == 0, scope="native-pointer-and-message-pump-only; rendering-and-user-machine-unverified")
            summary = os.environ.get("GITHUB_STEP_SUMMARY")
            if summary:
                with open(summary, "a", encoding="utf-8") as output:
                    output.write("\n### Speaker Link native input responsiveness smoke\n```json\n" + json.dumps(records, indent=2) + "\n```\nHosted success does not resolve the reported local hang or authorize merging.\n")
    return status


if __name__ == "__main__":
    raise SystemExit(main())
