# Speaker Link

A local Rust toolkit in development for checking Bose speaker battery and connections from Windows.

[![Foundation contracts on main: view synthetic-check results](https://github.com/skydreamer0/speaker-link/actions/workflows/ci.yml/badge.svg?branch=main&event=push)](https://github.com/skydreamer0/speaker-link/actions/workflows/ci.yml?query=branch%3Amain+event%3Apush)
[![Read the MIT license for original project code and documentation](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![Development stage: experimental discovery; view milestones and exit gates](https://img.shields.io/badge/stage-experimental%20discovery-orange)](docs/DEVELOPMENT.md)
[![Windows research target: view current compatibility limits](https://img.shields.io/badge/research%20target-Windows-lightgrey)](docs/COMPATIBILITY.md)

**No installable release, published crate, working CLI, or verified speaker-control support is available yet.**

[繁體中文](README.zh-TW.md) · [Quick start](docs/QUICKSTART.md) · [Compatibility](docs/COMPATIBILITY.md) · [Troubleshooting](docs/TROUBLESHOOTING.md)

## Start here

- **Install the app or CLI:** neither is available yet. See [availability](docs/QUICKSTART.md#install-the-app-or-cli).
- **Read source or contribute:** follow the [pinned foundation tutorial](docs/QUICKSTART.md#check-the-foundation-from-source).
- **Check your speaker:** read the [per-feature compatibility record](docs/COMPATIBILITY.md); a brand or model name alone does not establish support.
- **Report a problem:** start with [troubleshooting](docs/TROUBLESHOOTING.md) and [contributing](CONTRIBUTING.md).

## What exists today

The foundation is merged into `main`. This source also includes an experimental, read-only Windows service-discovery adapter and device-free synthetic tests. It is not yet a working speaker-control engine or a runnable scanner.

| Area | Current state |
| --- | --- |
| Rust foundation | Unpublished domain, error, command-input, and diagnostic-preview contracts |
| Windows service discovery | Experimental library adapter for a caller-selected, already permitted device; [hardware and lifecycle acceptance](docs/WINDOWS_DISCOVERY.md#integration-provenance-and-evidence-limits) remains open in [issue #2](https://github.com/skydreamer0/speaker-link/issues/2) |
| Battery, speaker-side device list, connection control | No reviewed hardware-support profile |
| Diagnostic export, CLI, Windows app | No complete usable implementation |

The [Foundation CI badge](https://github.com/skydreamer0/speaker-link/actions/workflows/ci.yml?query=branch%3Amain+event%3Apush) tracks `main` push runs. It covers code and synthetic contracts, not Windows Bluetooth access or speaker functionality. Ubuntu CI is not Linux Bluetooth support. [Exact-commit evidence and history](docs/COMPATIBILITY.md#source-and-platform-status) remain separate from this changing badge.

## Quick start: check the foundation

The first available hands-on step is checking development source, not installing a product.

Use the [source-and-test tutorial](docs/QUICKSTART.md#check-the-foundation-from-source) to prepare Git and Rust, check out the fixed foundation commit, and run its checks. This path needs no speaker and does not scan Bluetooth. Passing it does not establish hardware compatibility.

## Technology and packages

A [Rust 2021 workspace](Cargo.toml) with four unpublished packages. [Cargo.lock](Cargo.lock) pins the [15-package Windows dependency closure](docs/DEPENDENCIES.md). No minimum supported Rust or Windows version has been selected.

| Package | What it contains now |
| --- | --- |
| [speakerlink-core](crates/speakerlink-core/) | Model, command-input, error, and diagnostic contracts; no protocol codec or hardware control |
| [speakerlink-windows](crates/speakerlink-windows/) | Experimental WinRT read-only service adapter with bounded waiting; explicit `Unsupported` sentinel remains available |
| [speakerlink-cli](crates/speakerlink-cli/) | Library placeholder; no executable commands |
| [speakerlink-app](crates/speakerlink-app/) | Library placeholder; no GUI |

Microsoft Windows API bindings are integrated only for the read-only adapter. eframe/egui remains an [architecture candidate](docs/ARCHITECTURE.md), not an integrated dependency. The future library, CLI, and app will share one capability policy and control implementation.

## Compatibility and limits

The first research sample is SoundLink Flex first generation, model 435910. There is no verified hardware profile, and the project is not restricted to one host, adapter vendor, or firmware. Model, firmware, Windows build, adapter/driver, and each feature need their own [evidence](docs/COMPATIBILITY.md).

Missing data stays unknown. The PC's paired-device list cannot substitute for the speaker's own list, and standard service discovery does not establish private control support. Unknown firmware stays read-only until evaluated; unsupported operations must send no commands.

The initial scope excludes internal EQ, firmware flashing, arbitrary command injection, pairing-history deletion, physical button remapping, and full Bose app parity.

## Development order

- **Now:** foundation contracts, experimental read-only service discovery, and synthetic checks; no complete CLI or app.
- **Next:** close the remaining hardware, denied/unavailable, and observable lifecycle gates without treating successful requery as proof of OS cleanup.
- **Later:** the four core capabilities, then CLI, app, and release acceptance.

See the [development gates](docs/DEVELOPMENT.md), [specification](docs/SPEC.md), [architecture](docs/ARCHITECTURE.md), and [test plan](docs/TEST_PLAN.md). The specification's CLI examples remain proposals.

## Help and contributions

Read [CONTRIBUTING](CONTRIBUTING.md) before changing behavior. Use [GitHub issues](https://github.com/skydreamer0/speaker-link/issues) for questions and sanitized reports, including the tested commit and passed, failed, or not-tested outcomes.

Do not publish MAC addresses, serials, private device names, local paths, raw captures, or full logs. Follow the [minimal report checklist](docs/TROUBLESHOOTING.md#report-a-problem-without-sharing-private-data) and [SECURITY](SECURITY.md) for sensitive disclosures.

The product is local by design: no account, cloud service, or automatic diagnostic upload is planned. Sharing a report is your choice. README badge images are served by GitHub and Shields.io; [badge notes](THIRD_PARTY_NOTICES.md#readme-badges) explain that separate web-page behavior.

## License and independence

Original project code and documentation use the [MIT license](LICENSE). See [third-party policy](THIRD_PARTY_NOTICES.md) and [distribution cautions](docs/COMPLIANCE.md).

This is an independent community project. It is not affiliated with, sponsored by, or endorsed by Bose Corporation or Bluetooth SIG. Product and company names identify compatibility targets; their owners retain their trademarks. No certification, official app equivalence, or unlocked internal EQ is claimed.
