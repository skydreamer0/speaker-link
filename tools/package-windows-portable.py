#!/usr/bin/env python3
"""Check/assemble a local Windows preview; never build, run or upload an executable."""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import struct
import sys
import zipfile
from pathlib import Path, PurePosixPath

TARGET = "x86_64-pc-windows-msvc"
SHA256 = re.compile(r"[0-9a-f]{64}\Z")
COMMIT = re.compile(r"[0-9a-f]{40}\Z")
TOKEN = re.compile(r"[A-Za-z0-9][A-Za-z0-9._+-]{0,99}\Z")
FIXED_FILES = {
    "speakerlink-app.exe", "LICENSE", "THIRD_PARTY_NOTICES.md",
    "build-receipt.json", "dependency-graph.json", "notice-inventory.json",
}
MAX_FILE = 256 * 1024 * 1024
MAX_TOTAL = 384 * 1024 * 1024


class PackageError(ValueError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise PackageError(message)


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def safe_name(value: object) -> str:
    require(isinstance(value, str), "notice path must be text")
    name = str(value)
    require(bool(re.fullmatch(r"[A-Za-z0-9._/-]+", name)), "unsafe notice path")
    parts = name.split("/")
    reserved = {"CON", "PRN", "AUX", "NUL"} | {
        f"{prefix}{n}" for prefix in ("COM", "LPT") for n in range(1, 10)
    }
    require(parts[0] == "notices" and len(parts) > 1, "notice must be under notices/")
    for part in parts:
        require(part not in ("", ".", "..") and not part.endswith("."), "unsafe path component")
        require(part.split(".")[0].upper() not in reserved, "Windows reserved filename")
    return name


def read_file(root: Path, name: str) -> bytes:
    path = root
    for part in PurePosixPath(name).parts:
        path /= part
        require(not path.is_symlink(), f"symlink rejected: {name}")
    require(path.is_file(), f"missing regular file: {name}")
    require(0 < path.stat().st_size <= MAX_FILE, f"empty or oversized file: {name}")
    data = path.read_bytes()
    require(0 < len(data) <= MAX_FILE, f"empty or oversized file: {name}")
    return data


def parse_json(data: bytes, name: str) -> dict:
    require(len(data) <= 1024 * 1024, f"oversized JSON: {name}")
    def pairs(items):
        obj = {}
        for key, value in items:
            require(key not in obj, f"duplicate JSON key in {name}")
            obj[key] = value
        return obj
    try:
        value = json.loads(data.decode("utf-8"), object_pairs_hook=pairs)
    except (UnicodeError, json.JSONDecodeError) as exc:
        raise PackageError(f"invalid JSON: {name}") from exc
    require(isinstance(value, dict), f"expected JSON object: {name}")
    return value


def keys(obj: dict, expected: set[str], name: str) -> None:
    require(set(obj) == expected, f"unexpected or missing fields: {name}")


def component_id(obj: dict) -> tuple[str, str]:
    name, version = obj.get("name"), obj.get("version")
    require(isinstance(name, str) and bool(TOKEN.fullmatch(name)), "invalid component name")
    require(isinstance(version, str) and bool(TOKEN.fullmatch(version)), "invalid component version")
    return name, version


def check_pe(data: bytes) -> None:
    # A bounded format sanity check, NOT a loader, signature or runtime verifier.
    require(len(data) >= 64 and data[:2] == b"MZ", "artifact is not a Windows PE image")
    offset = struct.unpack_from("<I", data, 0x3C)[0]
    require(64 <= offset <= len(data) - 26, "invalid PE header offset")
    require(data[offset:offset + 4] == b"PE\0\0", "missing PE signature")
    machine, sections = struct.unpack_from("<HH", data, offset + 4)
    optional_size, flags = struct.unpack_from("<HH", data, offset + 20)
    require(machine == 0x8664, "artifact is not AMD64/x64")
    require(1 <= sections <= 96 and optional_size >= 112, "invalid PE image header")
    require(flags & 0x0002 != 0 and flags & 0x2000 == 0, "artifact must be an EXE, not a DLL")
    require(offset + 24 + optional_size + 40 * sections <= len(data), "truncated PE section table")
    require(struct.unpack_from("<H", data, offset + 24)[0] == 0x20B, "artifact is not PE32+")



def check_source_contract(contract: dict, receipt: dict, graph: dict,
                          files: dict[str, bytes]) -> None:
    """Bind builder assertions to an independently reviewed source inventory.

    This is consistency validation, not an attestation or legal completeness proof.
    """
    keys(contract, {"schema", "source_commit", "source_repository", "version", "target",
                    "cargo_lock_sha256", "app_manifest_sha256", "features", "profile",
                    "artifact_path", "cargo_components", "supplemental_components"}, "source contract")
    require(contract["schema"] == 1, "unsupported source contract")
    require(contract["source_repository"] == "skydreamer0/speaker-link", "unexpected source repository")
    for key in ("source_commit", "version", "target", "cargo_lock_sha256"):
        require(contract[key] == receipt[key], f"source contract mismatch: {key}")
    require(contract["features"] == ["native-ui"] and contract["profile"] == "release",
            "source contract requires native-ui release profile")
    artifact_path = f"target/{TARGET}/release/speakerlink-app.exe"
    require(contract["artifact_path"] == artifact_path, "unexpected source artifact path")
    for path, field in (("Cargo.lock", "cargo_lock_sha256"), ("app-Cargo.toml", "app_manifest_sha256")):
        require(isinstance(contract[field], str) and bool(SHA256.fullmatch(contract[field])), "invalid source file hash")
        require(digest(files[path]) == contract[field], f"source file checksum mismatch: {path}")
    expected = set()
    for field in ("cargo_components", "supplemental_components"):
        require(isinstance(contract[field], list) and 0 < len(contract[field]) <= 1000, "missing contract components")
        for component in contract[field]:
            require(isinstance(component, dict), "invalid contract component")
            keys(component, {"name", "version"}, "contract component")
            identity = component_id(component)
            require(identity not in expected, "duplicate contract component")
            expected.add(identity)
    require({"wgpu", "egui-wgpu", "wgpu-core", "wgpu-hal", "wgpu-types"} <= {n for n, _ in expected},
            "source contract missing WGPU graph")
    # rust-std comes from the actual compiler, not Cargo.lock or a made-up template.
    compiler = re.fullmatch(r"rustc ([0-9]+\.[0-9]+\.[0-9]+(?:-[A-Za-z0-9.-]+)?)(?: \([A-Za-z0-9 .+-]+\))?", receipt["rustc"])
    require(compiler is not None, "invalid exact compiler version")
    compiler_version = compiler.group(1)
    expected.add(("rust-std", compiler_version))
    actual = {component_id(c) for c in graph["components"]}
    require(actual == expected, "source contract dependency graph mismatch")
    provenance = parse_json(files["artifact-provenance.json"], "artifact-provenance.json")
    keys(provenance, {"schema", "source_commit", "target", "profile", "features", "artifact_path",
                      "artifact_sha256", "source_url", "build_evidence_url", "build_command"}, "artifact provenance")
    require(provenance["schema"] == 1, "unsupported artifact provenance")
    for key in ("source_commit", "target", "profile", "features", "artifact_path"):
        require(provenance[key] == contract[key], f"artifact provenance mismatch: {key}")
    require(provenance["artifact_sha256"] == receipt["artifact_sha256"], "artifact provenance hash mismatch")
    require(provenance["source_url"] == "https://github.com/skydreamer0/speaker-link/tree/" + receipt["source_commit"],
            "artifact source URL mismatch")
    require(isinstance(provenance["build_evidence_url"], str) and bool(re.fullmatch(
        r"https://github.com/skydreamer0/speaker-link/actions/runs/[1-9][0-9]*(?:/attempts/[1-9][0-9]*)?(?:/job/[1-9][0-9]*)?",
        provenance["build_evidence_url"])), "missing exact build evidence URL")
    require(provenance["build_command"] ==
            f"cargo build --locked --release --target {TARGET} -p speakerlink-app --features native-ui",
            "artifact build command mismatch")

def validate(root: Path, source_contract: bytes | None = None) -> tuple[dict[str, bytes], dict]:
    require(not root.is_symlink() and root.is_dir(), "payload directory must be a real directory")
    required_files = FIXED_FILES | ({"Cargo.lock", "app-Cargo.toml", "artifact-provenance.json"} if source_contract is not None else set())
    files = {name: read_file(root, name) for name in sorted(required_files)}
    receipt = parse_json(files["build-receipt.json"], "build-receipt.json")
    keys(receipt, {"schema", "mode", "source_commit", "version", "target", "rustc",
                   "cargo_lock_sha256", "artifact_sha256", "dependency_graph_sha256"}, "build receipt")
    require(receipt["schema"] == 1 and receipt["mode"] == "device-free-preview", "not a preview build receipt")
    require(receipt["target"] == TARGET, "unsupported build target")
    require(isinstance(receipt["source_commit"], str) and bool(COMMIT.fullmatch(receipt["source_commit"])), "invalid source commit")
    require(isinstance(receipt["version"], str) and bool(TOKEN.fullmatch(receipt["version"])), "invalid version")
    require(isinstance(receipt["rustc"], str) and bool(re.fullmatch(r"rustc [A-Za-z0-9 .()+-]{1,100}", receipt["rustc"])), "invalid compiler identity")
    for field in ("cargo_lock_sha256", "artifact_sha256", "dependency_graph_sha256"):
        require(isinstance(receipt[field], str) and bool(SHA256.fullmatch(receipt[field])), f"invalid {field}")
    require(digest(files["speakerlink-app.exe"]) == receipt["artifact_sha256"], "artifact checksum mismatch")
    require(digest(files["dependency-graph.json"]) == receipt["dependency_graph_sha256"], "dependency graph checksum mismatch")
    check_pe(files["speakerlink-app.exe"])

    graph = parse_json(files["dependency-graph.json"], "dependency-graph.json")
    keys(graph, {"schema", "target", "cargo_lock_sha256", "components"}, "dependency graph")
    require(graph["schema"] == 1 and graph["target"] == TARGET, "dependency graph target/schema mismatch")
    require(graph["cargo_lock_sha256"] == receipt["cargo_lock_sha256"], "lockfile provenance mismatch")
    require(isinstance(graph["components"], list) and 0 < len(graph["components"]) <= 1000, "missing dependency components")
    identities = set()
    for component in graph["components"]:
        require(isinstance(component, dict), "invalid dependency component")
        keys(component, {"name", "version"}, "dependency component")
        identity = component_id(component)
        require(identity not in identities, "duplicate dependency component")
        identities.add(identity)
    # These categories cannot be omitted merely because Cargo.lock does not list them.
    required = {"speaker-link", "rust-std", "eframe", "epaint_default_fonts", "Hack-font", "Ubuntu-font", "egui-icons-font"}
    require(required <= {name for name, _ in identities}, "missing project/compiler/GUI/font inventory")

    inventory = parse_json(files["notice-inventory.json"], "notice-inventory.json")
    keys(inventory, {"schema", "artifact_sha256", "components"}, "notice inventory")
    require(inventory["schema"] == 1 and inventory["artifact_sha256"] == receipt["artifact_sha256"], "notice inventory artifact mismatch")
    require(isinstance(inventory["components"], list), "missing notice inventory components")
    covered = set()
    portable_names = {}
    total_bytes = sum(map(len, files.values()))
    for component in inventory["components"]:
        require(isinstance(component, dict), "invalid notice component")
        keys(component, {"name", "version", "license", "source", "notices"}, "notice component")
        identity = component_id(component)
        require(identity in identities and identity not in covered, "unknown/duplicate notice component")
        covered.add(identity)
        require(isinstance(component["license"], str) and 0 < len(component["license"]) <= 200, "missing license expression")
        require(isinstance(component["source"], str) and bool(re.fullmatch(r"https://[A-Za-z0-9][A-Za-z0-9./_@:+%~#?=&-]{1,500}", component["source"])), "missing public source URL")
        require(isinstance(component["notices"], list) and component["notices"], "component has no notice files")
        for notice in component["notices"]:
            require(isinstance(notice, dict), "invalid notice")
            keys(notice, {"path", "sha256"}, "notice")
            name = safe_name(notice["path"])
            require(isinstance(notice["sha256"], str) and bool(SHA256.fullmatch(notice["sha256"])), "invalid notice checksum")
            parts = name.split("/")
            for count in range(1, len(parts) + 1):
                prefix = "/".join(parts[:count])
                require(portable_names.get(prefix.casefold(), prefix) == prefix,
                        "case-colliding Windows notice paths")
                portable_names[prefix.casefold()] = prefix
            data = files.get(name)
            if data is None:
                data = read_file(root, name)
                require(len(data) <= 2 * 1024 * 1024, "notice file too large")
                total_bytes += len(data)
                require(total_bytes <= MAX_TOTAL, "payload too large")
                try:
                    data.decode("utf-8")
                except UnicodeError as exc:
                    raise PackageError("notice must be UTF-8 text") from exc
                files[name] = data
            require(digest(data) == notice["sha256"], f"notice checksum mismatch: {name}")
    require(covered == identities, "notice inventory does not cover exact dependency graph")
    require(sum(map(len, files.values())) <= MAX_TOTAL, "payload too large")
    if source_contract is not None:
        check_source_contract(parse_json(source_contract, "source contract"), receipt, graph, files)
    actual = set()
    for path in root.rglob("*"):
        require(not path.is_symlink(), "symlink found in payload")
        if path.is_file():
            actual.add(path.relative_to(root).as_posix())
    require(actual == set(files), "unexpected payload files; use a dedicated staging directory")
    return files, receipt


def package(root: Path, output: Path | None = None, source_contract: bytes | None = None) -> dict:
    files, receipt = validate(root, source_contract)
    result = {"status": "input_consistency_pass", "source_commit": receipt["source_commit"],
              "target": TARGET, "artifact_sha256": receipt["artifact_sha256"],
              "native_window_validation": "NOT_RUN_BY_PACKAGER", "speaker_operations": 0,
              "release_approval": False, "input_files": len(files)}
    result["source_contract_validation"] = "PASS" if source_contract is not None else "NOT_RUN_LEGACY_INPUTS"
    if source_contract is not None:
        files["source-contract.json"] = source_contract
        result["source_contract_sha256"] = digest(source_contract)
    if output is None:
        return result
    require(not output.resolve().is_relative_to(root.resolve()), "output must be outside staging directory")
    require(output.suffix.lower() == ".zip", "output must end in .zip")
    intro = (
        "Speaker Link: DEVICE-FREE UI PREVIEW\n\n"
        "This is a local preview assembly, not a product release or hardware acceptance.\n"
        "The packager checks supplied file/receipt consistency; it cannot prove that the\n"
        "EXE was built from the recorded source or that the supplied notice inventory is complete.\n"
        "Review provenance and notices, then run the documented Windows no-device checklist\n"
        "before sharing. No installer, auto-start, privilege change or speaker operation is included.\n"
        "Do not bypass Windows security warnings; an unsigned preview may be blocked.\n"
        "Use SHA256SUMS.txt to check unpacked files. Never run an untrusted replacement EXE.\n"
    ).encode("utf-8")
    files["PREVIEW-README.txt"] = intro
    sums = "".join(f"{digest(data)}  {name}\n" for name, data in sorted(files.items()))
    files["SHA256SUMS.txt"] = sums.encode("ascii")
    prefix = f"speaker-link-{receipt['version']}-windows-x64-preview/"
    # Exclusive creation: an existing ZIP is never replaced, including a symlink.
    try:
        with output.open("xb") as stream:
            with zipfile.ZipFile(stream, "w", compression=zipfile.ZIP_STORED) as archive:
                for name, data in sorted(files.items()):
                    info = zipfile.ZipInfo(prefix + name, date_time=(1980, 1, 1, 0, 0, 0))
                    info.create_system = 3
                    info.external_attr = 0o100644 << 16
                    archive.writestr(info, data)
    except FileExistsError as exc:
        raise PackageError("output already exists; choose a new path") from exc
    result.update(status="local_preview_assembled", zip_sha256=digest(output.read_bytes()),
                  zip_bytes=output.stat().st_size)
    return result


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("payload", type=Path, help="dedicated staging directory")
    parser.add_argument("--output", type=Path, help="new ZIP path; omit for read-only preflight")
    parser.add_argument("--source-contract", type=Path, help="reviewed exact WGPU source contract; required for current WGPU acceptance")
    args = parser.parse_args(argv)
    try:
        contract = read_file(args.source_contract.absolute().parent, args.source_contract.name) if args.source_contract else None
        result = package(args.payload.absolute(), args.output.absolute() if args.output else None, contract)
    except (PackageError, OSError) as exc:
        print(json.dumps({"status": "blocked", "reason": str(exc)}, ensure_ascii=False), file=sys.stderr)
        return 2
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
