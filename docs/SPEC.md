# Speaker Link specification

Document version 0.2.0 · 2026-10-25 · Proposed, implementation not started.

## Product contract

Provide four local capabilities on Windows through a Rust library, CLI and native app. The first target is Bose SoundLink Flex first generation 435910. No target capability is verified yet. Release numbers, crate names and commands below are reserved design proposals, not available products.

"Same behavior" means the same capability decisions, normalized states, command outcomes, privacy policy and error codes. It does not mean pixel parity with the official app, support for every Bose model, or guaranteed concurrent access from multiple processes.

## Feature acceptance

| ID | Feature | Required result | Gate |
| --- | --- | --- | --- |
| F1 | Battery | Optional integer 0–100, observation time, source and freshness; charging separately unknown if unavailable | Compare with an independent device reading; disconnected/cached state explicitly labeled |
| F2 | Speaker devices | Distinct paired, connected and active-audio states; each can be unknown; opaque target IDs | Verify speaker-side list with two owned sources; never substitute Windows pairing list |
| F3 | Connection changes | Explicit known target and desired connected/disconnected state; refreshed outcome | Verify both directions and interruptions on owned devices; unsupported commands send nothing |
| F4 | Diagnostics | Versioned allowlisted JSON, preview, chosen output location, no automatic upload | Redaction tests and manual sensitive-data review pass before public export |

Diagnostics may ship before private speaker controls. That does not make F1–F3 complete. A partial release lists per-feature support instead of saying simply "Flex supported".

## Capability policy

Track capability as unknown, unsupported or verified. Store evidence reference, model, firmware if known, protocol transport, OS/build and test date. Discovery can establish a transport candidate but cannot enable write commands. Enable F3 only for an exact tested capability profile; unknown firmware stays read-only until evaluated. Do not match solely by friendly name, assume BC60 equals BC59, or hardcode an RFCOMM channel.

Windows identity, Bluetooth Device ID and Bose protocol identity are separate fields. A historical Windows report BC60 is an unverified observation with its namespace preserved, not proof of a hardware variant. Absence of an RFCOMM service does not rule out other transports.

## Shared interface

The planned public operations are discover, snapshot, set_connection and diagnostic_report. A snapshot contains schema_version, device_ref, capabilities, observed_at, freshness, battery, and speaker_devices. Unknown data is null with a reason; an empty list means an observed empty list only. Paired, connected, and active_audio must not be inferred from one another. Source selection is not a first-release capability.

The library produces typed results. CLI JSON serializes those results; the app renders those results. Presentation strings and formats can differ. Device identifiers usable locally must not enter exported diagnostics.

| Core error/outcome | CLI exit | App handling |
| --- | --- | --- |
| Success | 0 | Updated observed state |
| InvalidArgument | 2 | Explain invalid input |
| Unsupported | 3 | Disable action and show reason |
| AccessDenied | 4 | Show Windows permission guidance |
| Disconnected | 5 | Show disconnected state and allow manual retry |
| Timeout | 6 | Show failed read and stale previous state |
| ProtocolError | 7 | Stop request and offer sanitized diagnostic export |
| Busy | 8 | Explain another client owns the connection |
| Indeterminate | 9 | Show unknown command result; refresh before retry |
| InternalError | 10 | Stop operation and show local diagnostic reference |

Stale state is never proof a command succeeded. After a write, read back the affected state. If delivery or readback is uncertain, return Indeterminate, preserve the request reference and reconcile before another write. Never blindly retry a timed-out write. Setting an already observed desired state can return unchanged without another write. Use bounded timeouts, cancellation and bounded frame sizes. Final values are established during adapter testing and recorded in code; they cannot be infinite.

A per-user, per-device exclusive lock prevents CLI and app opening conflicting sessions. An existing owner causes Busy without sending data. Release resources on completion/cancellation and process exit. A broker for simultaneous clients is deferred. The UI must explain this limitation.

## Proposed CLI

These commands do not exist yet. JSON goes to stdout, human diagnostics to stderr. JSON has an explicit schema version and stable error codes.

```text
speakerlink discover --json
speakerlink status --device <local-id> --json
speakerlink connections list --device <local-id> --json
speakerlink connections set --device <local-id> --target <known-id> --state connected --json
speakerlink connections set --device <local-id> --target <known-id> --state disconnected --json
speakerlink diagnose --device <local-id> --output <chosen-file>
```

The write command has no implicit target and never clears pairing history. JSON and app use the same policy; there is no force-unsupported escape hatch. The app may ask for confirmation before disrupting an audio connection. Automation explicitly requesting a target and desired state uses the same checked operation without an interactive prompt.

## Privacy and diagnostics

Allow only public product model, firmware version or unknown, app/schema versions, OS/build and adapter driver versions when available, capability states, service UUIDs, normalized error codes, timestamps and redacted topology. Replace each device reference with a new random per-export alias. Do not export MACs, serials, Windows instance IDs, user names, full paths, private friendly names, authentication data, raw packets or raw exception messages. Hashing a MAC is insufficient anonymization.

Preview exactly what will be written. Warn before replacing an existing file. No network requests are part of diagnostics. For early development, raw traces remain local and outside this public repository; a separate reviewed process is required for any sharing. Redaction failure blocks export. Do not claim all metadata is anonymous: even sanitized reports can reveal model, OS and timing.

## Release acceptance

All four features must pass shared fixtures and CLI/app parity scenarios to call the planned first full release complete. Real hardware results must identify commit, model/identity namespace, firmware if obtainable, Windows build, adapter/driver, tested capability and outcome. Unknown firmware must be documented. A full release needs an explicitly supported firmware profile; do not waive that requirement by leaving the field unknown.

No battery, device list, command or EQ support may be simulated in production. Fixtures are clearly labeled and run without a speaker. No firmware writes, reset, pairing deletion, arbitrary commands, EQ, voice prompts or auto-off settings belong to this scope.
