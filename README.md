# Speaker Link

Your speaker's battery and connections, from your Windows desktop.

Speaker Link is a planned local Rust toolkit for inspecting and managing supported Bose speakers. It will provide a reusable library, a scriptable CLI, and a native Windows app that share the same device logic.

**Status: P0 contract preparation. There is no runnable release, published crate, working Bluetooth adapter, or verified SoundLink Flex support yet.** The [foundation workspace](docs/FOUNDATION.md) contains unpublished contracts and synthetic tests, not device support.

[繁體中文](README.zh-TW.md) · [Specification](docs/SPEC.md) · [Development order](docs/DEVELOPMENT.md) · [Compatibility](docs/COMPATIBILITY.md)

## What we are building

| Feature | Why use it | Release condition |
| --- | --- | --- |
| Battery status | Check charge without opening a phone app | Actual value verified on the target speaker |
| Speaker connection list | See the speaker's paired and connected devices separately | Speaker-side protocol verified |
| Connection control | Connect or disconnect a known device with a clear result | Explicit target and readback verified |
| Private diagnostics | Export useful troubleshooting evidence locally | Identifier redaction and preview tested |

The first research target is SoundLink Flex first generation, model 435910. Standard Windows Bluetooth discovery alone does not establish support for speaker-side controls. A PC's paired-device list is not the speaker's paired-device list.

## Who this is for

- Windows users who want battery and connection information on the desktop.
- Developers who want structured results for scripts and integrations.
- Device owners who can help document model and firmware differences with sanitized reports.

The project is local by design. No account, cloud service, or automatic diagnostic upload is planned. You choose whether to share a report.

## One engine, three interfaces

Build order: Rust library and Windows adapter, then CLI, then Windows app. All clients will use one capability policy, state model, command implementation, and error model. Unsupported functions will remain unavailable rather than report invented values.

The initial scope excludes internal EQ, firmware flashing, arbitrary command injection, physical button remapping, and full Bose app parity. A future capability needs its own evidence and specification.

## Try it or contribute

There are no installation commands yet. Start with the [SPEC](docs/SPEC.md), review [open issues](https://github.com/skydreamer0/speaker-link/issues), or contribute a sanitized compatibility report using the issue template. Read [CONTRIBUTING](CONTRIBUTING.md) before submitting code and [SECURITY](SECURITY.md) before sharing captures.

Planned CLI examples in the SPEC are design proposals, not working commands. A release will include supported Windows versions, hardware evidence, installation instructions, checksums, and known limitations.

## License and independence

Original project code and documentation use the [MIT license](LICENSE). See [third-party policy](THIRD_PARTY_NOTICES.md) and [distribution cautions](docs/COMPLIANCE.md).

This is an independent community project, unaffiliated with or endorsed by Bose Corporation or Bluetooth SIG. Product and company names identify compatibility targets; their owners retain their trademarks. No certification, official app equivalence, or unlocked internal EQ is claimed.
