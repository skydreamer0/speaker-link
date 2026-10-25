# P0 Windows dependency record

This is the bounded dependency selection for the read-only adapter, not a release or hardware compatibility claim. Dependencies come only from the official crates.io registry; there are no Git dependencies, alternate registries, vendored binaries or new build tools. Cargo.lock records the complete selected closure and archive SHA-256 checksums. CI fetches that lock once, then checks and tests offline with `--locked`.

## Direct Windows-only dependencies

Both entries are under `target.'cfg(windows)'.dependencies` in `speakerlink-windows`. Core remains standard-library-only and the other platforms do not compile the Windows adapter.

- `windows = =0.62.2`, default `std`, with `Devices_Bluetooth`, `Devices_Bluetooth_Rfcomm`, `Devices_Enumeration`, `Foundation`, `Networking`. Parent namespace features are enabled by that crate. No socket feature or UI framework is selected.
- `windows-future = =0.3.2`, default `std`, to name and inspect `IAsyncOperation` and its status. The adapter uses a bounded standard-library waiter, not an unbounded blocking `get`.

Microsoft's [published manifest](https://docs.rs/crate/windows/0.62.2/source/Cargo.toml), [published lock](https://docs.rs/crate/windows/0.62.2/source/Cargo.lock) and [published VCS record](https://docs.rs/crate/windows/0.62.2/source/.cargo_vcs_info.json) identify the upstream source commit `32c3144490c016fe496a0aed769bce60987a2e9d` in [microsoft/windows-rs](https://github.com/microsoft/windows-rs/tree/32c3144490c016fe496a0aed769bce60987a2e9d). The windows archive checksum is from its [official registry entry](https://github.com/rust-lang/crates.io-index/blob/master/wi/nd/windows). The remaining versions/checksums follow the published lock. Cargo's locked resolution must still pass for each candidate; a handwritten record alone is not validation.

## App integration reconciliation (2026-10-25)

The table below preserves the original P0 selection, not the current whole-workspace graph. Integrating the independently reviewed App lock retains all App package versions and checksums and adds the ten Windows packages absent from that lock. The existing Windows direct requirements stay exactly `windows 0.62.2` and `windows-future 0.3.2`; no adapter source changes.

Shared macro dependencies now use the App selection: proc-macro2 1.0.107, quote 1.0.47, syn 2.0.119 (the App graph also contains syn 3.0.9), and unicode-ident 1.0.27. Their cached official-registry MIT texts and unicode-ident's additional Unicode text were compared byte-for-byte with the retained notices and are unchanged. Cargo.lock is the current exact version/checksum authority; [App dependencies](APP_DEPENDENCIES.md) records the separate GUI target-graph and font obligations. This is source integration, not binary distribution or hardware acceptance.

## Historical P0 selected external closure

| Crate | Version | Upstream rust-version | SPDX expression |
| --- | --- | --- | --- |
| windows | 0.62.2 | 1.82 | MIT OR Apache-2.0 |
| windows-collections | 0.3.2 | 1.82 | MIT OR Apache-2.0 |
| windows-core | 0.62.2 | 1.82 | MIT OR Apache-2.0 |
| windows-future | 0.3.2 | 1.82 | MIT OR Apache-2.0 |
| windows-implement | 0.60.2 | 1.74 | MIT OR Apache-2.0 |
| windows-interface | 0.59.3 | 1.74 | MIT OR Apache-2.0 |
| windows-link | 0.2.1 | 1.71 | MIT OR Apache-2.0 |
| windows-numerics | 0.3.1 | 1.82 | MIT OR Apache-2.0 |
| windows-result | 0.4.1 | 1.82 | MIT OR Apache-2.0 |
| windows-strings | 0.5.1 | 1.82 | MIT OR Apache-2.0 |
| windows-threading | 0.2.1 | 1.82 | MIT OR Apache-2.0 |
| proc-macro2 | 1.0.101 | 1.56 | MIT OR Apache-2.0 |
| quote | 1.0.40 | 1.56 | MIT OR Apache-2.0 |
| syn | 2.0.106 | 1.61 | MIT OR Apache-2.0 |
| unicode-ident | 1.0.19 | 1.31 | (MIT OR Apache-2.0) AND Unicode-3.0 |

The last four sources are [proc-macro2](https://github.com/dtolnay/proc-macro2/tree/1.0.101), [quote](https://github.com/dtolnay/quote/tree/1.0.40), [syn](https://github.com/dtolnay/syn/tree/2.0.106) and [unicode-ident](https://github.com/dtolnay/unicode-ident/tree/1.0.19). Macro crates and ordinary upstream Cargo build scripts are part of this registry closure. No upstream source is modified or copied into our Rust modules.

We select the MIT option where the upstream dual license permits it and retain its text in [licenses](../licenses). unicode-ident additionally requires the Unicode-3.0 notice, retained separately. Original Speaker Link code remains under its own MIT license; that does not relicense dependencies. See [third-party notices](../THIRD_PARTY_NOTICES.md).

These are dependency minima, not Speaker Link's tested MSRV. Existing tests use `Waker::noop` (Rust 1.85 or later). CI records the installed stable compiler; no project MSRV, supported Windows build, driver list or release toolchain is promised. Generic platform compilation does not establish speaker/model/firmware compatibility.
