# Speaker Link

A local Rust toolkit in development for checking Bose speaker battery and connections from Windows.

Speaker Link is being built as one shared engine with three interfaces: a Rust library, a scriptable CLI, and a native Windows app. It is intended for Windows users, developers, and device owners who want to help test compatibility.

**There is no installable release, published crate, working CLI, or verified speaker-control support yet.** Start with the path below that matches what you want to do.

[繁體中文](README.zh-TW.md) · [Quick start](docs/QUICKSTART.md) · [Compatibility](docs/COMPATIBILITY.md) · [Troubleshooting](docs/TROUBLESHOOTING.md)

## Start here

| I want to… | Where to go | What is available |
| --- | --- | --- |
| Install a Windows app or CLI | [Availability](docs/QUICKSTART.md#install-the-app-or-cli) | Not available yet; no installer or `cargo install` command |
| Read or check the Rust foundation | [Source and test tutorial](docs/QUICKSTART.md#check-the-foundation-from-source) | A pinned development snapshot with device-free tests |
| Check whether my speaker works | [Compatibility and evidence](docs/COMPATIBILITY.md) | Per-feature status; no verified hardware profile yet |
| Contribute or report a problem | [Contributing](CONTRIBUTING.md) and [troubleshooting](docs/TROUBLESHOOTING.md) | Documentation, code review, and sanitized reports |

## Current public status

Checked 2026-10-25. A proposal, a passing test, and a released feature are different milestones.

- [PR #7](https://github.com/skydreamer0/speaker-link/pull/7) was merged into `main` at [`709bffe2`](https://github.com/skydreamer0/speaker-link/commit/709bffe2718fbd07fb1544d83cf3e7d774630a99) on 2026-10-25. The repository now includes unpublished Rust contracts and synthetic tests. Its CLI and app packages remain library placeholders, and its discovery adapter returns `Unsupported` without contacting a device.
- [`2782c396`](https://github.com/skydreamer0/speaker-link/tree/2782c396f1542f3421565e36ff4d8212c14eacaf) is the historical documentation-only baseline. The source tutorial uses the fixed foundation snapshot [`382294f4`](https://github.com/skydreamer0/speaker-link/tree/382294f407e5b6ef4dd8c3512e0c4f1b2c480b9d), which is now included in `main`.
- [CI for that exact foundation commit](https://github.com/skydreamer0/speaker-link/actions/runs/40165335679) passed on Ubuntu and Windows. This is evidence for `382294f4`, not a CI claim for the merge commit or later documentation changes. It checks code and synthetic contracts, not Windows Bluetooth access or speaker functionality.
- Real Windows discovery and hardware acceptance are tracked in [issue #2](https://github.com/skydreamer0/speaker-link/issues/2). The [compatibility table](docs/COMPATIBILITY.md) records what evidence can establish.

The source tutorial deliberately uses that fixed foundation commit, not unreviewed local changes. Merging the foundation does not make it a released application.

## Planned capabilities

| Feature | Intended result | Required before claiming support |
| --- | --- | --- |
| Battery status | Charge level with source, observation time, and freshness | Comparison with an independent device reading |
| Speaker connection list | Separate paired, connected, and active-audio states | Verified speaker-side data, not the PC's pairing list |
| Connection control | Connect or disconnect an explicitly selected known device | Tested capability profile and confirmation by readback |
| Private diagnostics | Preview and save an allowlisted report locally | Redaction and exact preview/export checks |

The first research target is SoundLink Flex first generation, model 435910. It is a starting sample, not a restriction to one computer, Bluetooth adapter, or firmware version. Each model, firmware, and host combination needs its own evidence. Standard Bluetooth discovery alone does not establish speaker-side control support.

Missing data must remain unknown. Unknown firmware stays read-only until evaluated; unsupported operations must not send commands. The initial scope excludes internal EQ, firmware flashing, arbitrary command injection, pairing-history deletion, physical button remapping, and full Bose app parity.

## Design and documentation

The library and Windows adapter come first, then the CLI, then the app. They share capability decisions, state, commands, and errors instead of maintaining separate control implementations.

- [Specification](docs/SPEC.md): feature acceptance, privacy, errors, and proposed interfaces.
- [Architecture](docs/ARCHITECTURE.md): package responsibilities and transport boundaries.
- [Development order](docs/DEVELOPMENT.md): staged work and exit gates.
- [Test plan](docs/TEST_PLAN.md): fixture, hardware, and release verification.

The CLI examples in the specification are proposals, not runnable commands. Ubuntu CI does not mean a Linux Bluetooth backend exists.

## Help and contributions

Read [CONTRIBUTING](CONTRIBUTING.md) before changing behavior. Use [GitHub issues](https://github.com/skydreamer0/speaker-link/issues) for questions, documentation gaps, and sanitized bug or compatibility reports. Include the tested commit and what actually passed, failed, or was not tested.

Keep MAC addresses, serials, private device names, local paths, raw captures, and full logs out of public issues. See [troubleshooting](docs/TROUBLESHOOTING.md#report-a-problem-without-sharing-private-data) for a minimal report and [SECURITY](SECURITY.md) for sensitive disclosures.

The project is local by design. No account, cloud service, or automatic diagnostic upload is planned. Sharing a report is your choice.

## License and independence

Original project code and documentation use the [MIT license](LICENSE). See [third-party policy](THIRD_PARTY_NOTICES.md) and [distribution cautions](docs/COMPLIANCE.md).

This is an independent community project, unaffiliated with or endorsed by Bose Corporation or Bluetooth SIG. Product and company names identify compatibility targets; their owners retain their trademarks. No certification, official app equivalence, or unlocked internal EQ is claimed.
