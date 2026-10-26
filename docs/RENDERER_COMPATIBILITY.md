# Renderer compatibility candidate (2026-10-25)

## Problem and bounded change

The standard Windows 2022 runner exited before a window under the original Glow
renderer. At head `9812b00db23851cea33dc050990fe383da79f966`,
[run 40205169233](https://github.com/skydreamer0/speaker-link/actions/runs/40205169233/job/121204437840)
recorded `OpenGL / opengl-version-below-2`, exit 1 at
2026-10-25 15:06:34.518 UTC. No pointer input was reached. The earlier
[run 40204568479](https://github.com/skydreamer0/speaker-link/actions/runs/40204568479)
also exited before a window, but its discarded stderr cannot establish a cause.
These remain failures, not retroactive passes.

The user reports a different symptom: a window opens, then pointer movement
makes the application unresponsive. That cause remains unknown. Switching the
renderer is a controlled compatibility candidate, not proof that local hang is
fixed, and not a reason to merge or release.

The pinned eframe 0.37.0 [default features](https://github.com/emilk/egui/blob/0.37.0/crates/eframe/Cargo.toml)
use WGPU. This candidate enables its `wgpu` feature alongside the existing Glow
comparison path. It preserves the App UI, event loop, synthetic fixtures and
hardware isolation. No automatic fallback or restart is added. No driver,
permission, real speaker, user computer or system setting is changed.

- Default / `--renderer wgpu`: WGPU using upstream adapter selection.
- `--renderer glow`: original renderer, explicit comparison only.
- Unknown or extra arguments: exit 2 before GUI startup, fixed usage message.

The version-matched egui-wgpu setup selects among surface-compatible adapters;
wgpu 30.0.1 DX12 recognizes Windows software adapters as `DeviceType::Cpu`.
The actual selection is recorded using fixed enum values only, never adapter
names, driver strings, arbitrary environment values or patient data. Software
adapter results must not be reported as hardware GPU validation. No WARP or Mesa
installer is introduced. If the runner has no usable adapter, native smoke must
fail rather than skip or substitute headless tests.

## Verification boundary

The existing native Win32 smoke still requires a real owned HWND, actual
SendInput insertion, cursor/hit-test verification and bounded WM_NULL responses.
It is not screenshot/render correctness or user-machine acceptance. Rust unit
tests cover explicit renderer selection and malformed argument rejection;
Python mock tests cover output allowlisting and existing smoke failures, not
native behavior. PR 12 remains on merge hold pending local issue validation.

## Dependency provenance

Exact direct eframe version stays 0.37.0; Cargo.lock keeps registry checksums.
The added WGPU feature uses unmodified crates.io dependencies, not copied source.
All pre-existing package versions are retained. Inventory below records newly
resolved packages; existing optional WGPU packages may also become active.
This is source compatibility work, not binary distribution approval. Existing
font notices and the complete exact-artifact license gate remain required.

| Added locked package | Version | Declared SPDX |
|---|---|---|
| [allocator-api2](https://crates.io/crates/allocator-api2/0.2.21) | 0.2.21 | MIT OR Apache-2.0 |
| [android_system_properties](https://crates.io/crates/android_system_properties/0.1.6) | 0.1.6 | MIT OR Apache-2.0 |
| [ash](https://crates.io/crates/ash/0.38.0+1.3.281) | 0.38.0+1.3.281 | MIT OR Apache-2.0 |
| [gpu-allocator](https://crates.io/crates/gpu-allocator/0.28.0) | 0.28.0 | MIT OR Apache-2.0 |
| [hashbrown](https://crates.io/crates/hashbrown/0.16.1) | 0.16.1 | MIT OR Apache-2.0 |
| [khronos-egl](https://crates.io/crates/khronos-egl/6.0.0) | 6.0.0 | MIT/Apache-2.0 |
| [objc2-core-graphics](https://crates.io/crates/objc2-core-graphics/0.3.2) | 0.3.2 | Zlib OR Apache-2.0 OR MIT |
| [objc2-io-surface](https://crates.io/crates/objc2-io-surface/0.3.2) | 0.3.2 | Zlib OR Apache-2.0 OR MIT |
| [objc2-metal](https://crates.io/crates/objc2-metal/0.3.2) | 0.3.2 | Zlib OR Apache-2.0 OR MIT |
| [objc2-quartz-core](https://crates.io/crates/objc2-quartz-core/0.3.2) | 0.3.2 | Zlib OR Apache-2.0 OR MIT |
| [ordered-float](https://crates.io/crates/ordered-float/5.5.0) | 5.5.0 | MIT |
| [pollster](https://crates.io/crates/pollster/1.0.1) | 1.0.1 | Apache-2.0/MIT |
| [presser](https://crates.io/crates/presser/0.3.1) | 0.3.1 | MIT OR Apache-2.0 |
| [range-alloc](https://crates.io/crates/range-alloc/0.1.5) | 0.1.5 | MIT OR Apache-2.0 |
| [raw-window-metal](https://crates.io/crates/raw-window-metal/1.1.0) | 1.1.0 | MIT OR Apache-2.0 |
| [spirv](https://crates.io/crates/spirv/0.4.0+sdk-1.4.341.0) | 0.4.0+sdk-1.4.341.0 | Apache-2.0 |
| [termcolor](https://crates.io/crates/termcolor/1.4.1) | 1.4.1 | Unlicense OR MIT |
| [wayland-sys](https://crates.io/crates/wayland-sys/0.31.11) | 0.31.11 | MIT |
| [wgpu-core-deps-apple](https://crates.io/crates/wgpu-core-deps-apple/30.0.1) | 30.0.1 | MIT OR Apache-2.0 |
| [wgpu-core-deps-emscripten](https://crates.io/crates/wgpu-core-deps-emscripten/30.0.1) | 30.0.1 | MIT OR Apache-2.0 |
| [wgpu-core-deps-wasm](https://crates.io/crates/wgpu-core-deps-wasm/30.0.1) | 30.0.1 | MIT OR Apache-2.0 |

WGPU/egui-wgpu/naga family metadata already present in the prior lock now participates in the selected renderer; its MIT OR Apache-2.0 paths require retained notices before distribution. No license waiver is implied by the registry inventory.

## Candidate verification receipt

- Python native-smoke control-flow/privacy tests: 14 PASS (Linux mocks only).
- cargo fmt / git diff --check: PASS.
- Official registry cargo fetch --locked and native-ui metadata: PASS.
- Local Rust check with one build job and debug symbols disabled: **INCOMPLETE**; naga 30.0.1 rustc was killed by SIGKILL, with no compiler diagnostic. Cause not established; do not interpret as OOM or source failure. No blind rerun.
- New Rust argument tests, clippy and native Windows smoke: pending exact-head Actions; not passed by this source receipt.
