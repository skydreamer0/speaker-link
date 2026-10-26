"""Synthetic files only: these tests never build or run a Windows executable."""
import importlib.util
import json
import struct
import tempfile
import unittest
import zipfile
from pathlib import Path

SPEC = importlib.util.spec_from_file_location(
    "packager", Path(__file__).resolve().parents[1] / "tools/package-windows-portable.py"
)
P = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(P)


def write_json(path, value):
    path.write_text(json.dumps(value, sort_keys=True), encoding="utf-8")


class PackageTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.base = Path(self.temp.name)
        self.root = self.base / "payload"
        self.root.mkdir()
        # Header-only synthetic fixture. Passing PE sanity is not a runnable-EXE claim.
        image = bytearray(512)
        image[:2] = b"MZ"
        struct.pack_into("<I", image, 0x3C, 64)
        image[64:68] = b"PE\0\0"
        struct.pack_into("<HH", image, 68, 0x8664, 1)
        struct.pack_into("<HH", image, 84, 112, 2)
        struct.pack_into("<H", image, 88, 0x20B)
        (self.root / "speakerlink-app.exe").write_bytes(image)
        (self.root / "LICENSE").write_text("Synthetic project notice\n")
        (self.root / "THIRD_PARTY_NOTICES.md").write_text("Synthetic inventory; not real license evidence.\n")
        (self.root / "notices").mkdir()
        notice = b"Synthetic test notice, not distribution approval.\n"
        (self.root / "notices/test.txt").write_bytes(notice)
        names = ["speaker-link", "rust-std", "eframe", "epaint_default_fonts",
                 "Hack-font", "Ubuntu-font", "egui-icons-font"]
        self.graph = {"schema": 1, "target": P.TARGET, "cargo_lock_sha256": "a" * 64,
                      "components": [{"name": n, "version": "0-test"} for n in names]}
        write_json(self.root / "dependency-graph.json", self.graph)
        self.receipt = {
            "schema": 1, "mode": "device-free-preview", "source_commit": "b" * 40,
            "version": "0.0.0-dev", "target": P.TARGET, "rustc": "rustc 1.99.1",
            "cargo_lock_sha256": "a" * 64, "artifact_sha256": P.digest(image),
            "dependency_graph_sha256": P.digest((self.root / "dependency-graph.json").read_bytes()),
        }
        self.inventory = {"schema": 1, "artifact_sha256": P.digest(image), "components": [
            {**c, "license": "MIT", "source": "https://example.invalid/test",
             "notices": [{"path": "notices/test.txt", "sha256": P.digest(notice)}]}
            for c in self.graph["components"]
        ]}
        self.save()

    def save(self):
        write_json(self.root / "build-receipt.json", self.receipt)
        write_json(self.root / "notice-inventory.json", self.inventory)

    def assertBlocked(self, pattern):
        output = self.base / "blocked.zip"
        with self.assertRaisesRegex(P.PackageError, pattern):
            P.package(self.root, output)
        self.assertFalse(output.exists())

    def test_preflight_is_read_only_and_explicitly_not_release(self):
        before = sorted(p.relative_to(self.root).as_posix() for p in self.root.rglob("*"))
        result = P.package(self.root)
        self.assertEqual(result["status"], "input_consistency_pass")
        self.assertFalse(result["release_approval"])
        self.assertEqual(result["native_window_validation"], "NOT_RUN_BY_PACKAGER")
        self.assertEqual(before, sorted(p.relative_to(self.root).as_posix() for p in self.root.rglob("*")))

    def test_deterministic_zip_and_complete_checksums(self):
        first, second = self.base / "a.zip", self.base / "b.zip"
        one, two = P.package(self.root, first), P.package(self.root, second)
        self.assertEqual(first.read_bytes(), second.read_bytes())
        self.assertEqual(one["zip_sha256"], two["zip_sha256"])
        with zipfile.ZipFile(first) as archive:
            self.assertIsNone(archive.testzip())
            prefix = "speaker-link-0.0.0-dev-windows-x64-preview/"
            sums = archive.read(prefix + "SHA256SUMS.txt").decode("ascii").splitlines()
            covered = set()
            for line in sums:
                sha, name = line.split("  ", 1)
                self.assertEqual(sha, P.digest(archive.read(prefix + name)))
                covered.add(prefix + name)
            self.assertEqual(covered, set(archive.namelist()) - {prefix + "SHA256SUMS.txt"})
            for item in archive.infolist():
                self.assertEqual(item.date_time, (1980, 1, 1, 0, 0, 0))
                self.assertEqual(item.compress_type, zipfile.ZIP_STORED)

    def test_missing_artifact_blocks(self):
        (self.root / "speakerlink-app.exe").unlink()
        self.assertBlocked("missing regular file")

    def test_elf_or_text_cannot_be_mislabeled_as_exe(self):
        data = b"\x7fELF" + bytes(508)
        (self.root / "speakerlink-app.exe").write_bytes(data)
        self.receipt["artifact_sha256"] = P.digest(data)
        self.inventory["artifact_sha256"] = P.digest(data)
        self.save()
        self.assertBlocked("not a Windows PE")

    def test_arm64_and_dll_rejected(self):
        for offset, value in ((68, 0xAA64), (86, 0x2002)):
            with self.subTest(offset=offset):
                path = self.root / "speakerlink-app.exe"
                original = path.read_bytes()
                data = bytearray(original)
                struct.pack_into("<H", data, offset, value)
                path.write_bytes(data)
                self.receipt["artifact_sha256"] = P.digest(data)
                self.inventory["artifact_sha256"] = P.digest(data)
                self.save()
                self.assertBlocked("not AMD64|not a DLL")
                path.write_bytes(original)

    def test_artifact_checksum_mismatch(self):
        self.receipt["artifact_sha256"] = "0" * 64
        self.save()
        self.assertBlocked("artifact checksum")

    def test_graph_checksum_and_lock_pin(self):
        self.receipt["dependency_graph_sha256"] = "0" * 64
        self.save()
        self.assertBlocked("dependency graph checksum")
        self.receipt["dependency_graph_sha256"] = P.digest((self.root / "dependency-graph.json").read_bytes())
        self.receipt["cargo_lock_sha256"] = "0" * 64
        self.save()
        self.assertBlocked("lockfile provenance")

    def test_missing_component_notice_blocks(self):
        self.inventory["components"].pop()
        self.save()
        self.assertBlocked("does not cover")

    def test_font_and_std_inventory_cannot_be_omitted(self):
        self.graph["components"] = [c for c in self.graph["components"] if c["name"] != "rust-std"]
        write_json(self.root / "dependency-graph.json", self.graph)
        self.receipt["dependency_graph_sha256"] = P.digest((self.root / "dependency-graph.json").read_bytes())
        self.save()
        self.assertBlocked("missing project/compiler/GUI/font")

    def test_notice_tampering_blocks(self):
        (self.root / "notices/test.txt").write_text("changed")
        self.assertBlocked("notice checksum")

    def test_paths_fail_closed(self):
        for name in ("../secret", "notices/../secret", "/notices/a", "notices/C:a", "notices/a\\b", "notices/NUL.txt", "notices/COM1", "notices/a.", "notices//a"):
            with self.subTest(name=name):
                self.inventory["components"][0]["notices"][0]["path"] = name
                self.save()
                self.assertBlocked("unsafe|notice must|reserved")

    def test_symlink_and_unlisted_files_block(self):
        extra = self.root / "private-trace.txt"
        extra.write_text("must never be packaged")
        self.assertBlocked("unexpected payload")
        extra.unlink()
        notice = self.root / "notices/test.txt"
        notice.unlink()
        notice.symlink_to(self.root / "LICENSE")
        self.assertBlocked("symlink rejected")

    def test_existing_output_never_overwritten(self):
        output = self.base / "existing.zip"
        output.write_bytes(b"keep")
        with self.assertRaisesRegex(P.PackageError, "already exists"):
            P.package(self.root, output)
        self.assertEqual(output.read_bytes(), b"keep")

    def test_duplicate_or_unknown_fields_rejected(self):
        path = self.root / "build-receipt.json"
        path.write_text('{"schema":1,"schema":1}')
        self.assertBlocked("duplicate JSON key")
        self.receipt["private_machine_path"] = "C:/Users/name"
        self.save()
        self.assertBlocked("unexpected or missing fields")

    def test_notice_case_collision_rejected(self):
        (self.root / "notices/TEST.txt").write_bytes((self.root / "notices/test.txt").read_bytes())
        self.inventory["components"][1]["notices"][0]["path"] = "notices/TEST.txt"
        self.save()
        self.assertBlocked("case-colliding")

    def test_output_under_staging_rejected(self):
        with self.assertRaisesRegex(P.PackageError, "outside staging"):
            P.package(self.root, self.root / "out.zip")
        self.assertFalse((self.root / "out.zip").exists())

    def test_case_colliding_parent_directories_rejected(self):
        for directory, filename in (("A", "one.txt"), ("a", "two.txt")):
            folder = self.root / "notices" / directory
            folder.mkdir()
            (folder / filename).write_text("notice")
        for component, path in zip(self.inventory["components"], ("notices/A/one.txt", "notices/a/two.txt")):
            component["notices"] = [{"path": path, "sha256": P.digest(b"notice")}]
        self.save()
        self.assertBlocked("case-colliding")



class WgpuContractTests(unittest.TestCase):
    """Strict source/notice handoff checks with synthetic artifacts, never real EXEs."""
    def setUp(self):
        PackageTests.setUp(self)
        names = ["wgpu", "egui-wgpu", "wgpu-core", "wgpu-hal", "wgpu-types"]
        additions = [{"name": n, "version": "1-test"} for n in names]
        self.graph["components"] += additions
        for c in additions:
            self.inventory["components"].append({**self.inventory["components"][0], **c})
        for c in self.graph["components"] + self.inventory["components"]:
            if c["name"] == "rust-std":
                c["version"] = "1.99.1"
        self.contract = {
            "schema": 1, "source_commit": self.receipt["source_commit"],
            "source_repository": "skydreamer0/speaker-link", "version": self.receipt["version"],
            "target": P.TARGET, "cargo_lock_sha256": P.digest(b"synthetic lock"),
            "app_manifest_sha256": P.digest(b"synthetic manifest"), "features": ["native-ui"],
            "profile": "release", "artifact_path": f"target/{P.TARGET}/release/speakerlink-app.exe",
            "cargo_components": additions,
            "supplemental_components": [c for c in self.graph["components"] if c["name"] not in names + ["rust-std"]],
        }
        (self.root / "Cargo.lock").write_bytes(b"synthetic lock")
        (self.root / "app-Cargo.toml").write_bytes(b"synthetic manifest")
        self.receipt["cargo_lock_sha256"] = self.contract["cargo_lock_sha256"]
        self.graph["cargo_lock_sha256"] = self.contract["cargo_lock_sha256"]
        self.provenance = {k: self.contract[k] for k in ("schema", "source_commit", "target", "profile", "features", "artifact_path")}
        self.provenance.update(artifact_sha256=self.receipt["artifact_sha256"],
            source_url="https://github.com/skydreamer0/speaker-link/tree/" + self.receipt["source_commit"],
            build_evidence_url="https://github.com/skydreamer0/speaker-link/actions/runs/123/job/456",
            build_command=f"cargo build --locked --release --target {P.TARGET} -p speakerlink-app --features native-ui")
        self.sync()

    save = PackageTests.save

    def sync(self):
        write_json(self.root / "dependency-graph.json", self.graph)
        self.receipt["dependency_graph_sha256"] = P.digest((self.root / "dependency-graph.json").read_bytes())
        write_json(self.root / "artifact-provenance.json", self.provenance)
        self.save()

    def run_package(self, output=None):
        return P.package(self.root, output, json.dumps(self.contract).encode())

    def blocked(self, pattern):
        with self.assertRaisesRegex(P.PackageError, pattern):
            self.run_package(self.base / "bad.zip")
        self.assertFalse((self.base / "bad.zip").exists())

    def test_strict_receipt_and_contract_in_zip(self):
        result = self.run_package(self.base / "strict.zip")
        self.assertEqual(result["source_contract_validation"], "PASS")
        self.assertEqual(result["native_window_validation"], "NOT_RUN_BY_PACKAGER")
        with zipfile.ZipFile(self.base / "strict.zip") as z:
            name = next(n for n in z.namelist() if n.endswith("source-contract.json"))
            self.assertEqual(P.digest(z.read(name)), result["source_contract_sha256"])

    def test_wrong_source_and_lock_rejected(self):
        for key in ("source_commit", "cargo_lock_sha256", "version"):
            original = self.contract[key]
            self.contract[key] = "wrong"
            self.blocked("source contract mismatch")
            self.contract[key] = original

    def test_actual_source_files_must_match(self):
        for name in ("Cargo.lock", "app-Cargo.toml"):
            path = self.root / name
            old = path.read_bytes()
            path.write_bytes(b"tampered")
            self.blocked("source file checksum")
            path.write_bytes(old)

    def test_glow_only_or_partial_graph_rejected(self):
        self.graph["components"] = [c for c in self.graph["components"] if c["name"] != "wgpu-hal"]
        self.inventory["components"] = [c for c in self.inventory["components"] if c["name"] != "wgpu-hal"]
        self.sync()
        self.blocked("dependency graph mismatch")

    def test_contract_cannot_omit_wgpu(self):
        self.contract["cargo_components"].pop()
        self.blocked("missing WGPU")

    def test_wrong_std_version_rejected(self):
        self.receipt["rustc"] = "rustc 1.98.0"
        self.save()
        self.blocked("dependency graph mismatch")

    def test_bad_compiler_is_controlled_failure(self):
        self.receipt["rustc"] = "rustc   "
        self.save()
        self.blocked("exact compiler")

    def test_debug_or_unpinned_build_rejected(self):
        for key, bad in (("profile", "debug"), ("artifact_path", "target/debug/speakerlink-app.exe"),
                         ("build_command", "cargo build"), ("artifact_sha256", "0" * 64),
                         ("source_url", "https://github.com/skydreamer0/speaker-link/tree/main"),
                         ("build_evidence_url", "https://example.invalid/unknown")):
            old = self.provenance[key]
            self.provenance[key] = bad
            self.sync()
            self.blocked("provenance|command|source URL|evidence URL")
            self.provenance[key] = old

    def test_missing_provenance_blocks(self):
        (self.root / "artifact-provenance.json").unlink()
        self.blocked("missing regular file")

    def test_missing_notice_still_blocks_strict_mode(self):
        self.inventory["components"].pop()
        self.save()
        self.blocked("does not cover")


if __name__ == "__main__":
    unittest.main()
