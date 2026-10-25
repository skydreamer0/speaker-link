# Architecture

Document baseline 0.2.0; documentation revised 2026-10-25. The [P0 foundation](FOUNDATION.md) is merged: four unpublished packages, shared contracts, and synthetic tests. The responsibilities below describe the target design, not completed features. The experimental read-only adapter uses the [locked Windows dependency closure](DEPENDENCIES.md); the device-free preview uses optional pinned [eframe/egui dependencies](APP_DEPENDENCIES.md). No Bose protocol dependency is integrated.

## Rust throughout

| Package | Planned responsibility | Planned dependencies |
| --- | --- | --- |
| speakerlink-core | Domain models, framing/codec, capability policy, command engine, diagnostics redaction | Platform-neutral Rust libraries |
| speakerlink-windows | Windows discovery, access, RFCOMM sockets and exclusive device ownership | core and Microsoft windows crate |
| speakerlink-cli | Argument parsing, shared-engine calls, versioned JSON and exit codes | core and Windows adapter |
| speakerlink-app | Device-free native preview of shared snapshots; real controls disabled | core and optional eframe/egui; no Windows adapter |

The four packages exist in one Cargo workspace with `publish = false`. The Windows package provides an experimental read-only WinRT service adapter for a caller-selected, already permitted device and retains the explicit `Unsupported` sentinel. The CLI remains a placeholder. The App has a device-free native preview backed by shared snapshot models, without an adapter dependency. See [adapter boundaries and remaining evidence](WINDOWS_DISCOVERY.md). Do not publish placeholder packages to reserve names. Confirm availability before release. Keep transport and protocol separate with a small asynchronous interface driven by actual requirements. Avoid introducing a general multi-vendor plugin system in the first implementation.

The read-only adapter uses the windows crate's BluetoothDevice service discovery and explicit cached/uncached queries. Socket transport remains a later investigation using each discovered service's host/name; no StreamSocket or speaker-command path is implemented. Win32 AF_BTH RFCOMM sockets are an alternative experiment if WinRT access is unsuitable. Select the transport only after a real connection experiment; do not implement both as a permanent fallback without evidence.

btleplug supports BLE, not Bluetooth Classic RFCOMM. It cannot replace an RFCOMM adapter. Linux BlueZ transports cannot be assumed portable to Windows. Reuse of a codec is subject to provenance and tests even when the crate is written in Rust.

## Why this order

First establish service discovery and a no-write diagnostic path. Then implement and validate the four library capabilities. The CLI is the first complete consumer and exposes the contract for repeatable testing. An isolated device-free App layout may be built earlier with explicitly synthetic snapshots; real App control integration still waits for that accepted contract. The app must never implement a second Bluetooth protocol stack or quietly bypass a rejected command.

The device-free preview uses pinned eframe/egui 0.37.0 in Rust. Its GUI is an opt-in `native-ui` feature, and device controls remain disabled. This does not complete P3 acceptance. A Windows tray, hotkeys, autostart and installers are separate decisions after the four-feature flow works. No Tauri frontend, web service or account system is required for this scope.

## Alternatives and limits

A CLI-only utility would reduce UI work but does not meet the requested desktop flow. A broker service would allow concurrent clients but adds IPC authentication and lifecycle complexity; first release uses an exclusive device lock. A protocol crate from another project might reduce codec work, but its Windows support and model compatibility must be demonstrated before adoption.

See [SPEC](SPEC.md) for errors, readback, freshness and concurrency behavior. See [test plan](TEST_PLAN.md) for evidence required to change this proposal.
