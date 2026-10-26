# Verification plan

Document version 0.2.0. The table below defines full acceptance; passing a bounded test slice does not complete a row.

Foundation, runtime, diagnostic-save and App view-model/headless UI synthetic checks have run on their recorded candidates. The integrated device-free workspace has 52 passing Linux synthetic tests plus Windows MSVC cross-checks; these are not native Windows window or hardware execution. Current-head CI must be recorded separately. Native UI, F1–F4 hardware, ownership/parity and release acceptance remain open. See [foundation evidence](FOUNDATION.md), [Windows limits](WINDOWS_DISCOVERY.md) and [App preview](APP_PREVIEW.md).

| Check | Method | Passing evidence |
| --- | --- | --- |
| Codec | Valid captured/synthetic labeled fixtures; truncated, oversized and malformed input | Bounded parsing, no panic, expected typed results |
| F1 | Compare battery to independent reading; disconnect and reconnect | Correct value or explicit unknown, timestamp and stale flag |
| F2 | Two owned sources, known pairing/connection changes | Speaker-side list distinguishes paired, connected and active audio |
| F3 | Each target and desired state; timeout, cancel, unplug and already-set cases | Readback confirms changes; uncertain writes are Indeterminate and never blindly retried |
| Policy | Unknown model/firmware, standard AVRCP only and unsupported features | Zero outbound write frames |
| F4 | Seed MACs, serials, Windows IDs, paths, Unicode names and errors | Exact exported bytes contain no prohibited data; preview equals file |
| Ownership | Run CLI while app owns device; crash owner and retry | Busy before traffic; ownership released safely |
| Interface parity | Same fixture/state-transition corpus through library, CLI and app view-model | Same values, capability decisions and normalized outcomes |
| UI | Inspect actual native app on Windows | Unknown/stale/unsupported visible, target selection explicit, UI responsive, cancel works |
| Release | Clean checkout and documented toolchain | Windows artifact built, dependencies reviewed, license notices included, checksums verified |

Hardware reports record commit, model, identity namespace, firmware, OS/build, adapter/driver, transport, date and capability outcomes. Keep raw identifiers in private local evidence only. A maintainer reviews a sanitized evidence record before enabling writes for that profile. A screenshot or codec unit test alone cannot validate a Bluetooth operation.

Initial mandatory scenarios are unavailable speaker, denied permission, unknown protocol, cached state, invalid frame, another client owner and interrupted write. All planned release tests must be reported passed, failed or not run; no skipped case counts as a pass.
