# Troubleshooting

[Quick start](QUICKSTART.md) · [Compatibility](COMPATIBILITY.md) · [Security reporting](../SECURITY.md)

These notes apply to the public foundation snapshot `382294f407e5b6ef4dd8c3512e0c4f1b2c480b9d`. For a later candidate, identify its exact commit and follow its own instructions. Do not interpret a setup error as evidence that a speaker is incompatible.

## Source and toolchain checks

| Symptom | Likely explanation | Next step |
| --- | --- | --- |
| Cargo cannot find `Cargo.toml` | You are on the historical documentation-only baseline, outside the workspace root, or in a different checkout | Follow the [new-directory checkout](QUICKSTART.md#2-get-an-exact-snapshot-in-a-new-directory); confirm the exact commit and workspace root |
| `rustup`, `cargo`, or `rustc` is not found | Toolchain installation or shell PATH is incomplete | Follow the [official Rust installer](https://rust-lang.org/tools/install/); reopen the terminal after installation |
| The stable toolchain, rustfmt, or Clippy is missing | The tutorial's prerequisites are not installed | Use the official Rust setup instructions; do not remove lint or format checks to obtain a pass |
| Windows reports a missing linker or SDK component | The MSVC build prerequisites need attention | Check the [official MSVC prerequisites](https://rust-lang.github.io/rustup/installation/windows-msvc.html); no Bluetooth settings change is required |
| Offline Cargo wants an external dependency or a lockfile update | You may be on different source or have changed the manifest/lockfile | Confirm the pinned commit and local changes; the tutorial snapshot has no external Cargo dependencies. Do not treat results from modified source as an exact reproduction |
| A check fails on your installed Rust version | Your compiler or environment may differ from the recorded CI run | Record the compiler version, exact commit, failing command, and a sanitized error excerpt. No minimum supported Rust version has been established |

Do not copy full terminal output into a public issue without reviewing it. Compiler errors can include personal filesystem paths.

## Expected limitations, not device failures

### There is no program to run

The foundation CLI and app packages are library placeholders. An error about no binary target is expected if you try to run or install them. The proposed CLI commands in [SPEC](SPEC.md) are not implemented in this snapshot.

### Discovery returns Unsupported

`UnimplementedWindowsDiscovery` returns `Unsupported` without touching hardware on Windows and other operating systems. This is the foundation's deliberate behavior, not a diagnosis of your speaker, driver, permissions, or Bluetooth adapter. Re-pairing or resetting a speaker will not implement the adapter.

### CI passed, but I cannot read battery or connections

CI checks synthetic contracts. It does not establish any F1–F4 hardware capability. The foundation diagnostic preview is also not the complete preview-and-file-export workflow. See the [compatibility table](COMPATIBILITY.md) for the separate acceptance gates.

### Windows sees my speaker, but speaker-side data is unknown

Windows discovery and the speaker's own paired/connected-device list are different observations. A friendly name, model-like string, standard service UUID, or cached result cannot enable private commands. Do not substitute the PC's device list for missing speaker-side information.

This is an interpretation rule for future hardware reports, not a runnable discovery tutorial. Do not bypass denied access, guess RFCOMM channels, clear pairing history, send arbitrary packets, or flash firmware to work around an unverified capability.

## Report a problem without sharing private data

Use [GitHub issues](https://github.com/skydreamer0/speaker-link/issues) and choose the bug or compatibility template. A useful minimal report contains:

- The exact commit or released version and the documented command or procedure used.
- Expected behavior and what happened instead, with short reproduction steps.
- Passed, failed, and not-tested checks listed separately.
- For source failures: compiler/target and OS version, plus a short sanitized error.
- For an authorized hardware test: public product model, firmware or “unknown,” Windows build, adapter/driver version, cache mode when relevant, tested feature, date, and whether a changed state was read back.

Report only fields relevant to the problem. “Unknown” is preferable to a guessed firmware version or identity. Label observations as your report until reviewed; a passing test on one host does not establish every host or firmware.

Before posting, remove MAC addresses, serials, Windows instance IDs, personal device names, account names, full paths, credentials, raw packets, raw exceptions, and unrelated devices. Replacing a MAC with a stable hash is not sufficient anonymization. Review any screenshot and any attachment as well as the text. Sanitized metadata can still reveal product, software, and timing information.

Do not attach a raw evidence ZIP, packet capture, or complete log just because it might help debugging. For vulnerabilities or information that cannot be safely sanitized, follow [SECURITY](../SECURITY.md) before sharing details. Do not assume a private reporting channel is enabled.
