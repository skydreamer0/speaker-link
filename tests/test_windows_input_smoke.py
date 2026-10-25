"""Control-flow tests; these do not exercise Windows or claim native GUI acceptance."""
import importlib.util
from pathlib import Path
import unittest
import tempfile
import io
from unittest.mock import Mock, patch

SPEC = importlib.util.spec_from_file_location("smoke", Path(__file__).resolve().parents[1] / "tools/windows_input_smoke.py")
smoke = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(smoke)


class Clock:
    now = 0.0
    def monotonic(self):
        return self.now
    def sleep(self, seconds):
        self.now += seconds


class SmokeTests(unittest.TestCase):
    def setUp(self):
        self.clock = Clock()
        self.patcher = patch.object(smoke, "time", self.clock)
        self.patcher.start()
        self.addCleanup(self.patcher.stop)
        self.process = Mock(pid=123)
        self.process.poll.return_value = None
        self.api = Mock()
        self.api.find_window.return_value = 456
        self.api.points.return_value = [(10, 20), (30, 40)]
        self.emit = Mock()

    def run_smoke(self):
        smoke.exercise(self.api, self.process, self.emit, seconds=0.15)

    def test_success_requires_real_input_cursor_target_and_response_checks(self):
        self.run_smoke()
        self.assertEqual(self.api.move.call_count, 2)
        self.assertEqual(self.api.verify_cursor.call_count, 2)
        self.assertEqual(self.api.verify_target.call_count, 4)
        self.assertEqual(self.api.responsive.call_count, 3)
        self.emit.assert_called_with("native-pointer-message-pump-pass", moves=2)

    def test_missing_window_is_bounded_and_never_inputs(self):
        self.api.find_window.return_value = None
        with self.assertRaisesRegex(RuntimeError, "window-timeout"):
            self.run_smoke()
        self.api.move.assert_not_called()
        self.assertLess(self.clock.now, 20.2)

    def test_exited_process_never_inputs(self):
        self.process.poll.return_value = 1
        with self.assertRaisesRegex(RuntimeError, "exited-before-window"):
            self.run_smoke()
        self.api.move.assert_not_called()

    def test_foreground_or_hit_test_failure_never_inputs(self):
        self.api.verify_target.side_effect = RuntimeError("foreground-lost")
        with self.assertRaisesRegex(RuntimeError, "foreground-lost"):
            self.run_smoke()
        self.api.move.assert_not_called()

    def test_input_failure_never_passes(self):
        self.api.move.side_effect = RuntimeError("native-input-not-inserted")
        with self.assertRaisesRegex(RuntimeError, "not-inserted"):
            self.run_smoke()
        self.assertEqual(self.emit.call_count, 1)

    def test_cursor_mismatch_never_passes(self):
        self.api.verify_cursor.side_effect = RuntimeError("cursor-position-mismatch")
        with self.assertRaisesRegex(RuntimeError, "cursor-position-mismatch"):
            self.run_smoke()
        self.assertEqual(self.emit.call_count, 1)

    def test_hung_after_pointer_never_passes(self):
        self.api.responsive.side_effect = [None, RuntimeError("message-pump-timeout")]
        with self.assertRaisesRegex(RuntimeError, "message-pump-timeout"):
            self.run_smoke()
        self.assertEqual(self.emit.call_count, 1)

    def test_target_loss_after_pointer_never_passes(self):
        self.api.verify_target.side_effect = [None, RuntimeError("window-owner-mismatch")]
        with self.assertRaisesRegex(RuntimeError, "window-owner-mismatch"):
            self.run_smoke()
        self.assertEqual(self.emit.call_count, 1)

    def test_final_process_exit_never_passes(self):
        self.process.poll.side_effect = [None, None, 42]
        with self.assertRaisesRegex(RuntimeError, "app-exited-after-input"):
            self.run_smoke()
        self.assertEqual(self.emit.call_count, 1)

    def main_with_process(self, process):
        with tempfile.TemporaryDirectory() as directory:
            exe = Path(directory) / "speakerlink-app.exe"
            exe.write_bytes(b"mock executable; never run")
            with patch("sys.argv", ["smoke", str(exe)]), \
                 patch.dict(smoke.os.environ, {"GITHUB_ACTIONS": "true", "RUNNER_OS": "Windows"}, clear=True), \
                 patch.object(smoke, "Windows"), \
                 patch.object(smoke, "exercise"), \
                 patch.object(smoke.subprocess, "Popen", return_value=process), \
                 patch("builtins.print"):
                return smoke.main()

    def test_exit_between_exercise_and_cleanup_fails_main(self):
        self.process.poll.return_value = 42
        self.assertEqual(self.main_with_process(self.process), 1)
        self.process.terminate.assert_not_called()
        self.process.kill.assert_not_called()

    def test_cleanup_terminates_only_owned_handle_and_bounds_wait(self):
        self.process.wait.side_effect = [smoke.subprocess.TimeoutExpired("owned", 3), 0]
        self.assertEqual(self.main_with_process(self.process), 0)
        self.process.terminate.assert_called_once_with()
        self.process.kill.assert_called_once_with()
        self.assertEqual(self.process.wait.call_count, 2)
        self.process.wait.assert_called_with(timeout=3)

    def test_stderr_classifies_known_static_error_without_raw_text(self):
        raw = b'Error: OpenGL(PainterError("egui_glow requires opengl 2.0+. "))\r\n'
        result = smoke.classify_stderr(io.BytesIO(raw))
        self.assertEqual(result["stderr_category"], "OpenGL")
        self.assertEqual(result["stderr_reason"], "opengl-version-below-2")
        self.assertNotIn("PainterError", str(result))

    def test_stderr_never_exposes_arbitrary_payload(self):
        for raw in [b"secret-token C:/private/user/file", b'Error: Winit("secret-token")', b'x'*4096 + b'Error: OpenGL(secret-token)']:
            result = smoke.classify_stderr(io.BytesIO(raw))
            self.assertNotIn("secret-token", str(result))
            self.assertNotIn("private", str(result))
            self.assertLessEqual(result["stderr_prefix_bytes"], 4096)
        self.assertTrue(result["stderr_prefix_truncated"])
        self.assertEqual(result["stderr_category"], "unclassified")


if __name__ == "__main__":
    unittest.main()
