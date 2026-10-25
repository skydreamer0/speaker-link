# Device-free native App preview

This slice adds an App entry point and a view of the existing shared `Snapshot` type. It is not the P3 acceptance milestone, does not replace P2 CLI verification, and does not establish F1–F4 speaker support. The App does not depend on or call the workspace’s PR10/PR11 Windows adapter or diagnostic-save implementations.

## Run

With Rust 1.95 or newer and a supported desktop graphics environment:

```sh
cargo run --locked -p speakerlink-app --features native-ui
```

The window always says **Device-free UI preview** and **SYNTHETIC DATA ONLY**. It starts empty. The selector shows Empty, Unknown, Stale, Unsupported and Busy layouts using fixed in-memory fixtures. Clear preview resets all displayed fixture state. It never scans, opens a device, acquires a Bluetooth connection, sends commands, writes a report or uploads data. Closing the window only closes this process; there is no background worker or device session.

The App depends on `speakerlink-core`, not `speakerlink-windows`. Scan, Refresh, Connect, Disconnect and Export diagnostics remain disabled, even if a supplied view-model contains a verified capability. This is a deliberate preview safety boundary, not a future write-authorization mechanism.

## Display contract

- Unknown battery differs from observed 0%; unknown source lists differ from observed empty lists.
- Stale values retain their timestamp and cached-state warning; they are not shown as current success.
- Paired, connected and active audio are independent observations. Sources use local display aliases, never opaque device IDs.
- Capability Unknown and Unsupported are distinct. An unknown snapshot schema fails closed.
- Busy is an explicitly selected synthetic scenario, not a claim that a real process currently owns a session.
- The current English UI uses bundled default fonts. Native screen-reader integration and CJK fonts are not enabled or accepted in this slice.
- Linux validation uses the X11 backend; this feature selection does not claim Wayland-only support. The intended product remains Windows.

## Checks

```sh
cargo fmt --all -- --check
cargo clippy --offline --locked --workspace --all-targets --features speakerlink-app/native-ui -- -D warnings
cargo check --offline --locked --workspace --all-targets --features speakerlink-app/native-ui
cargo test --offline --locked --workspace --features speakerlink-app/native-ui
```

A one-time `cargo fetch --locked` is required before offline checks in a fresh environment. Headless egui layout tests cover five preview states at 420×640 and 1000×760, plus pure view-model and reset tests. These tests do not prove native window rendering, OS focus behavior, screen-reader support, Windows runtime operation, or hardware compatibility. Those require separate recorded acceptance. The Windows CI job additionally runs a bounded [native pointer/message-pump smoke](windows-input-smoke.md) against the device-free App executable. This is not render acceptance or proof that the reported local hang is resolved. No hardware operation is part of CI.

## Dependencies and release boundary

`eframe` is pinned to 0.37.0 (minimum Rust 1.95) with defaults disabled; selected features are `glow`, `default_fonts`, and `x11`. Clipboard, URL-opening links, persistence, system-font discovery, inspection/TCP, wgpu and accesskit are not enabled. The native GUI is behind `native-ui`, so default library tests need no windowing runtime. Cargo.lock pins registry versions and checksums.

See [APP_DEPENDENCIES](APP_DEPENDENCIES.md) for exact dependency-license review and distribution obligations. This source slice is not a binary release; dependency-license declarations alone are not a release packaging audit.
