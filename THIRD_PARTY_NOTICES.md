# Third-party sources and provenance

The original specification baseline contained project documents and a standard MIT license. The merged foundation adds original Rust contracts, and the experimental read-only adapter uses the registry dependencies documented below. This is not a binary release. No third-party implementation source, firmware, binary, logo or packet capture is bundled.

| Reference | Use so far | Adoption rule |
| --- | --- | --- |
| [aaronsb/bosectl](https://github.com/aaronsb/bosectl) | Protocol/project research; MIT repository | Verify exact files/version and retain copyright and license if reused |
| [bose-connect Rust docs](https://docs.rs/bose-connect/latest/bose_connect/) | API research | Audit exact crate source, license and transitive dependencies before use |
| [BoseConnect-Linux_based-connect](https://github.com/bosefirmware/BoseConnect-Linux_based-connect) | Historical GPL project reference | No copied, translated or linked code in this MIT-only distribution without a separate licensing decision |
| Microsoft Windows API documentation | API design reference | Review selected windows crate version and license before dependency adoption |
| egui/eframe 0.37.0 | Device-free native UI preview | Exact target dependency screen recorded in [App dependencies](docs/APP_DEPENDENCIES.md); binary notice packaging remains a release gate |

For every adopted dependency or source fragment, record version/commit, upstream URL, SPDX expression, files used, modifications and notice obligations. A permissive top-level license does not settle every bundled file or transitive dependency. This table is a research inventory, not a completed dependency audit.

## P0 read-only adapter dependencies and App integration

The historical P0 15-package closure and the current App integration changes are recorded in [DEPENDENCIES](docs/DEPENDENCIES.md); Cargo.lock pins the combined registry checksums. All dependencies are unmodified. The App now adds eframe/egui as described below; its target graph does not link the Windows adapter. No third-party Rust source is copied into project modules, and no GPL code is copied, translated or linked.

- Microsoft windows-rs packages: MIT OR Apache-2.0, MIT option selected. Copyright (c) Microsoft Corporation. Retained [Microsoft MIT notice](licenses/MICROSOFT-MIT.txt), from the upstream repository's license-mit.
- proc-macro2 1.0.107: MIT OR Apache-2.0, MIT option selected. Retained [upstream notice](licenses/PROC-MACRO2-MIT.txt), unmodified.
- quote 1.0.47: MIT OR Apache-2.0, MIT option selected. Retained [upstream notice](licenses/QUOTE-MIT.txt), unmodified.
- syn 2.0.119 and 3.0.9: MIT OR Apache-2.0, MIT option selected. Retained [upstream notice](licenses/SYN-MIT.txt), unmodified.
- unicode-ident 1.0.27: (MIT OR Apache-2.0) AND Unicode-3.0. MIT option selected and the additional Unicode license retained: [MIT notice](licenses/UNICODE-IDENT-MIT.txt), [Unicode-3.0 notice](licenses/UNICODE-3.0.txt). The latter includes its Unicode copyright attribution and must not be dropped from distribution.

Notice files preserve upstream text rather than inventing copyright statements absent from those files. The project MIT license does not replace any dependency's obligations. Before distributing compiled artifacts, include these notices and review the actual final dependency closure again.

CI references [actions/checkout](https://github.com/actions/checkout/tree/11d5960a326750d5838078e36cf38b85af677262), pinned to commit `11d5960a326750d5838078e36cf38b85af677262` (v4 tag checked when introduced), under [MIT](https://github.com/actions/checkout/blob/11d5960a326750d5838078e36cf38b85af677262/LICENSE). It is used unmodified as CI tooling; no action source is copied into this distribution. Its upstream copyright and license remain with that tooling. CI uses the runner's preinstalled Rust stable toolchain and records its version; this is not a release toolchain pin or a supported-version declaration.

## Before distributing binaries

Before distributing binaries, audit the actual release contents, including third-party crates, bundled Rust standard-library components, runtimes, and other redistributed assets. Include the project LICENSE and all notices required for those exact versions.

Record the release artifact, component/version, source, license, required notice, and where that notice is included. A Cargo.lock with no third-party crates does not replace this artifact-level check. Build tooling is not automatically part of the redistributed payload. See the [Rust copyright inventory](https://github.com/rust-lang/rust/blob/main/COPYRIGHT) and the [MIT license](https://opensource.org/license/mit); use the sources for the versions actually shipped, not just their latest branch.

## README badges

The README links to the native GitHub Actions badge and three text-only Shields.io badges. No additional brand logos, icon packages, or third-party implementation are bundled. The URLs contain only public project information.

The CI image is scoped to `main` push runs; static license, stage, and research-target labels must be updated when those facts change. Images can be cached and are not permanent exact-commit evidence. Rendering the README can request images from GitHub or Shields.io, depending on the renderer; this is separate from the product's local-only diagnostics policy. See [GitHub status badges](https://docs.github.com/en/actions/how-tos/monitor-workflows/add-a-status-badge) and [Shields static badges](https://shields.io/badges/static-badge).

## Device-free App preview

The App now selects pinned eframe/egui 0.37.0 with a minimal optional feature set. The [App dependency review](docs/APP_DEPENDENCIES.md) records target-graph provenance, Rust minimum and special font/license obligations. Default font licensing includes OFL, Ubuntu Font License and actual Hack/Bitstream Vera notices in addition to MIT/Apache options. No third-party implementation or font files are copied into this source slice. Binary distribution remains blocked on a complete exact-artifact notice package and platform acceptance; this source review is not release approval.
