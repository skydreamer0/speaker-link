# Architecture proposal

Document version 0.2.0. Design only; dependencies have not been installed or audited for this implementation.

## Rust throughout

| Planned package | Responsibility | Depends on |
| --- | --- | --- |
| speakerlink-core | Domain models, framing/codec, capability policy, command engine, diagnostics redaction | Platform-neutral Rust libraries |
| speakerlink-windows | Windows discovery, access, RFCOMM sockets and exclusive device ownership | core and Microsoft windows crate |
| speakerlink-cli | Argument parsing, shared-engine calls, versioned JSON and exit codes | core and Windows adapter |
| speakerlink-app | Native desktop views, user actions and shared-engine calls | core, Windows adapter and eframe/egui |

One Cargo workspace is proposed. Do not publish placeholder packages to reserve names. Confirm availability before release. Keep transport and protocol separate with a small asynchronous interface driven by actual requirements. Avoid introducing a general multi-vendor plugin system in the first implementation.

Windows adapter investigation starts with the windows crate's BluetoothDevice service discovery, uncached refresh where supported, and StreamSocket using each discovered service's host/name. Win32 AF_BTH RFCOMM sockets are an alternative experiment if WinRT access is unsuitable. Select the transport only after a real connection experiment; do not implement both as a permanent fallback without evidence.

btleplug supports BLE, not Bluetooth Classic RFCOMM. It cannot replace an RFCOMM adapter. Linux BlueZ transports cannot be assumed portable to Windows. Reuse of a codec is subject to provenance and tests even when the crate is written in Rust.

## Why this order

First establish service discovery and a no-write diagnostic path. Then implement and validate the four library capabilities. The CLI is the first complete consumer and exposes the contract for repeatable testing. Build the app only after that contract works. The app must never implement a second Bluetooth protocol stack or quietly bypass a rejected command.

The initial App uses eframe/egui to stay within Rust. A Windows tray, hotkeys, autostart and installers are separate decisions after the four-feature flow works. No Tauri frontend, web service or account system is required for this scope.

## Alternatives and limits

A CLI-only utility would reduce UI work but does not meet the requested desktop flow. A broker service would allow concurrent clients but adds IPC authentication and lifecycle complexity; first release uses an exclusive device lock. A protocol crate from another project might reduce codec work, but its Windows support and model compatibility must be demonstrated before adoption.

See [SPEC](SPEC.md) for errors, readback, freshness and concurrency behavior. See [test plan](TEST_PLAN.md) for evidence required to change this proposal.
