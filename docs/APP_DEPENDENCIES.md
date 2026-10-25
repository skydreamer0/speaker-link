# App preview dependency review

Reviewed 2026-10-25 for the source-only, device-free App slice. No binary release is approved by this document.

## Selection

- `eframe = 0.37.0`, exact pin; upstream tag: <https://github.com/emilk/egui/tree/0.37.0>.
- `egui` is used through eframe's re-export, not a second direct version.
- eframe defaults are disabled. Only `glow`, `default_fonts`, and `x11` are selected behind the App's optional `native-ui` feature.
- Upstream declares Rust 1.95 minimum; the App package records this minimum. The development toolchain used for this candidate is Rust 1.99.1, not evidence of a separately tested 1.95 build.
- App source remains Rust edition 2021; the workspace uses resolver 2. Dependencies may use edition 2024.
- `speakerlink-windows` is removed from the App dependency list. UI code cannot reach the Bluetooth adapter through its dependency graph.

The exact resolved versions and registry checksums are pinned in Cargo.lock. Review used offline `cargo tree --locked --edges normal,build` for `x86_64-pc-windows-msvc` and `x86_64-unknown-linux-gnu`, intersected with metadata. There are 96 third-party packages in the Windows graph and 112 in the Linux graph. Counts describe resolved normal/build graphs, not successful compilation or final binary linkage. The lockfile also contains optional/all-platform packages that are not in these graphs.

No selected target graph contains wgpu, egui-wgpu, arboard, webbrowser, egui_inspection, egui_system_font_provider or accesskit_winit. Clipboard, browser-link actions, persistence, system-font discovery and inspection/TCP features are not enabled. This selection also omits the native screen-reader bridge; no accessibility acceptance is claimed.

## License findings

All packages in the reviewed target graphs declare license expressions. No unavoidable GPL-only code dependency was identified. This is a source metadata and special-file review, not a legal opinion or a completed distribution audit.

| Component | Declared license / selected path | Important obligation |
| --- | --- | --- |
| eframe/egui/epaint family 0.37.0 | MIT OR Apache-2.0 | Retain the selected upstream license/copyright notices; choose a permissible path for distribution |
| epaint_default_fonts 0.37.0 | (MIT OR Apache-2.0) AND OFL-1.1 AND Ubuntu-font-1.0 | Bundled font obligations are additional, not replaced by MIT/Apache |
| Hack font bundled with default fonts | Font file notice also includes Bitstream Vera terms | Retain actual Hack/Bitstream notices and honor font renaming/modification conditions; crate SPDX alone is incomplete for packaging |
| egui-icons.ttf | Bundled emoji-icon-font MIT notice | Retain its actual copyright/license; full monochrome emoji fonts are not enabled |
| dpi 0.1.2 | Apache-2.0 AND MIT | Preserve both required notice sets |
| unicode-ident 1.0.27 | (MIT OR Apache-2.0) AND Unicode-3.0 | Preserve the additional Unicode notice |
| self_cell 1.3.0 | Apache-2.0 OR GPL-2.0-only | Select Apache-2.0; GPL is an alternative, not an unavoidable path |
| foldhash | Zlib | Preserve required license text |
| libloading | ISC | Preserve required license text |

The cached egui-family package roots do not contain their root MIT/Apache text files. Before any compiled-artifact distribution, obtain the license texts from the exact upstream 0.37.0 tag and include them with the actual font-specific notices. Do not invent copyright text or claim Cargo.lock satisfies notice retention.

## Release gate

A release must inventory the actual target artifact, compiler/runtime components and bundled font files; include every required notice; reconcile any changed dependency graph; and verify the exact supported Windows and Rust versions. No third-party implementation source or font file is copied into this repository by this slice. Registry archives are development dependencies, not a product release artifact.
