# P0 contracts and experimental Windows adapter

This slice supports [issue #2](https://github.com/skydreamer0/speaker-link/issues/2) without claiming its exit gate. It follows SPEC 0.2.0 at `2782c396f1542f3421565e36ff4d8212c14eacaf`. The specification is unchanged. P1 remains dependent on real P0 evidence; CLI and app implementation remain later stages.

## What exists

- One Cargo workspace with the four proposed packages, each marked `publish = false`. The `0.0.0-dev` version is an unpublished development placeholder, not a product release.
- `speakerlink-core`: local reference types, independent known/unknown speaker states, optional observation time and explicit freshness, validated battery percentage, capability display states, command input shapes, and SPEC error/exit codes. These are provisional Rust domain types, not a stabilized public API or CLI JSON schema.
- Platform-neutral discovery contracts live in `speakerlink-core`. `speakerlink-windows` adds a cfg(windows) WinRT service adapter for a caller-selected, already accessible device, explicit cached/uncached policy and bounded operation waiting. No device picker, access request, pairing, socket or speaker-command bytes exist. The original `UnimplementedWindowsDiscovery` still returns `Unsupported`; it does not invoke the new adapter. See [Windows integration limits](WINDOWS_DISCOVERY.md).
- A device-free diagnostic error preview containing only schema version, development app version and a normalized error code. It accepts no raw error text or identifiers. The immutable byte view is the prospective preview/write boundary, not an implemented file exporter.
- Synthetic tests and a two-OS, read-only CI workflow. The CLI and app packages are empty library placeholders: no executables, command parser, UI, eframe dependency or device operations exist.

Capability states are presentation data, not authorization to write. No reviewed capability profile or write executor exists. Discovery never populates battery or speaker-side topology, even for an uncached result or a familiar name/UUID. Local references are opaque and are not cross-namespace identity evidence. Windows identity, Bluetooth Device ID and Bose protocol identity must not be inferred from each other.

The discovery trait has no write method. The experimental adapter enforces request/result bounds and rejects late completion, with synthetic timeout/cancellation/cleanup tests. Cooperative OS cancellation and synchronous API latency still need real hardware evidence; the timer cannot forcibly preempt an OS call. Final defaults, concurrency/ownership and runtime behavior require adapter testing.

## Check the foundation

On a machine that already has Rust stable, rustfmt and clippy installed:

```text
rustc --version --verbose
cargo --version
cargo fmt --all -- --check
cargo fetch --locked
cargo clippy --offline --locked --workspace --all-targets -- -D warnings
cargo check --offline --locked --workspace --all-targets
cargo test --offline --locked --workspace
```

These commands neither scan Bluetooth nor send speaker commands. The fetch step downloads the locked official registry dependency closure; it is not a Rust/toolchain or driver installer. If the toolchain is missing, report that first rather than treating these commands as approval to install it. Windows-only dependencies and notices are recorded in [DEPENDENCIES](DEPENDENCIES.md). All device test inputs are synthetic. The CI workflow uses the standard Ubuntu and Windows runners, `contents: read`, a commit-pinned official checkout action and no saved credentials, artifact upload, hardware job, release, deployment or self-hosted runner. It records the preinstalled stable compiler rather than silently selecting a tested Windows release toolchain.

A declared workflow, a static source inspection or successful Linux compilation is not evidence of a passing Windows run. Report formatting, lint, tests and compilation for the exact candidate commit as passed, failed or not run. Even a passing Windows job only exercises synthetic operations and API projections, not device discovery or real adapter/driver behavior. No minimum supported Rust/Windows version or release toolchain is selected here.

## What must happen before P0 acceptance

1. Record the intended Windows build, Rust toolchain, MSVC, SDK, Bluetooth adapter and driver. The reviewed dependency selection is recorded separately and does not depend on one sample machine; validate it against supported hosts before claiming support.
2. Investigate access and RFCOMM service discovery on explicitly authorized, owned hardware. Use discovered host/service names, not a hardcoded channel. Do not issue Bose setting commands. Permission denial remains a failure, not permission to retry another access route.
3. Distinguish cached results from actual uncached discovery and retain sanitized evidence. Uncached service discovery can communicate with the device, despite being no-write in the speaker-settings sense. It is not a zero-radio or zero-network-traffic operation.
4. Test unavailable-device and denied-access behavior, bounded timeouts, cancellation and resource cleanup on the actual Windows adapter. Generic fixture tests cannot establish this.
5. Complete the diagnostic export privacy design: allowlisted metadata, random per-export aliases, exact preview/file bytes, chosen path, overwrite handling and manual sensitive-data review. This device-free preview deliberately omits topology rather than inventing stable or hashed aliases.
6. Obtain independent review and current-commit verification. Keep #2 open until its Windows and real no-write discovery evidence exists; do not treat this slice as #3 acceptance or F1–F4 completion.

## API research, not compatibility evidence

- Microsoft documents [RFCOMM service discovery](https://learn.microsoft.com/en-us/uwp/api/windows.devices.bluetooth.bluetoothdevice.getrfcommservicesasync?view=winrt-26100) with a cache-mode overload. [Uncached](https://learn.microsoft.com/en-us/uwp/api/windows.devices.bluetooth.bluetoothcachemode?view=winrt-26100) obtains values from the device; cached RFCOMM queries use the system cache.
- [RequestAccessAsync](https://learn.microsoft.com/en-us/uwp/api/windows.devices.bluetooth.bluetoothdevice.requestaccessasync?view=winrt-26100) invokes device consent on the UI thread. A future integration host must account for that context rather than invoking consent from an arbitrary background worker. The experimental adapter accepts a host-provided permitted device and does not request access or display consent UI.
- The [Cargo workspace documentation](https://doc.rust-lang.org/cargo/reference/workspaces.html) defines shared package metadata, path dependencies and the common lockfile used here.

No upstream Bose protocol implementation was used or translated to produce these contracts. No device captures, identifiers or private diagnostics belong in this public repository.
