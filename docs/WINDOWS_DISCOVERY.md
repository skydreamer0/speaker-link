# Experimental Windows read-only service adapter

This is a developer library component for issue #2, not a ready-to-run scanner. There is no discovery-host executable, discovery CLI command or device picker yet. The separate device-free App preview does not call this adapter. Tests never open a Bluetooth device. A first verification machine is a sample, not a product restriction: the adapter does not match a CPU, Bluetooth vendor, device identifier, Bose model or firmware version.

## Layers and entry point

- `speakerlink-core::discovery` defines the platform-neutral request, outcome, endpoint and asynchronous trait. Other platforms can later implement that contract; this change does not implement Linux or macOS discovery.
- `speakerlink_windows::winrt::WindowsDiscovery` exists only on Windows. A host passes an already accessible, explicitly selected `BluetoothDevice` to `from_authorized_device`. The adapter reads current access status there and again before each query. Any status other than Allowed fails closed as AccessDenied.
- Model/capability data stays separate. Every discovered endpoint yields an unobserved snapshot with unknown capabilities. A service UUID, familiar name, cached endpoint or uncached result cannot grant battery/control support or identify the speaker's own paired-device list.

The existing `UnimplementedWindowsDiscovery` remains an explicit Unsupported sentinel for callers/tests that do not supply an adapter. It is not selected as a hidden runtime fallback. Existing discovery types are re-exported from the Windows package for source compatibility.

## Host prerequisites, not implemented here

A future host must provide an appropriate initialized COM/UI context, packaging/capability declarations, explicit selection and an already permitted device object. Microsoft documents that [BluetoothDevice.FromIdAsync](https://learn.microsoft.com/en-us/uwp/api/windows.devices.bluetooth.bluetoothdevice.fromidasync) must run on the UI thread and that [RequestAccessAsync](https://learn.microsoft.com/en-us/uwp/api/windows.devices.bluetooth.bluetoothdevice.requestaccessasync) invokes consent. This adapter calls neither. It does not initialize COM, show a prompt, obtain permission, pair, connect/disconnect, open sockets or send settings/protocol commands. No access-denied fallback or privilege escalation exists. A host must not describe this component as an automatically authorized scanner.

The adapter queries [GetRfcommServicesAsync with a cache mode](https://learn.microsoft.com/en-us/uwp/api/windows.devices.bluetooth.bluetoothdevice.getrfcommservicesasync) and copies each service's discovered UUID, host and service name. It does not assume an RFCOMM channel. [Uncached mode](https://learn.microsoft.com/en-us/uwp/api/windows.devices.bluetooth.bluetoothcachemode) can communicate with the device to obtain service metadata. Thus “read-only” means no speaker-setting/protocol writes, not zero Bluetooth traffic or a guarantee that Windows performs no underlying link activity. Running it on real hardware still needs the owner's explicit approval and a suitable host. No such run is performed by CI.

## Bounds, cleanup and privacy

Callers supply a timeout greater than zero and no more than 60 seconds, and a service limit of 1–256. These are safety caps, not measured Bluetooth defaults. Uncached is the request default; callers can explicitly choose Cached, and that policy is retained on returned records without claiming independently verified freshness.

The waiter polls only the operation's status on the caller's executor. A short-lived standard-library timer thread wakes that executor at most once per 10 milliseconds and at the deadline; it never calls Windows/device APIs. Successful/error/cancelled completion releases owned operation references. Timeout or dropping a pending future requests cooperative WinRT cancellation, closes a completed operation where legal, and releases its reference without waiting indefinitely for the OS. A result observed after the deadline is rejected. The timer stops when its future is dropped.

The deadline does not forcibly preempt a synchronous OS call, and WinRT cancellation does not guarantee immediate radio quiescence. Actual cancellation latency, service-object lifetime and resource cleanup need hardware validation. Returned service objects are closed after metadata extraction; bounded owned endpoint records are not socket handles.

Device/endpoint references stay local in memory and are redacted by Debug. String conversion and result allocation have explicit limits. Errors contain normalized SPEC codes, not raw Windows error messages or identifiers. This is not a full diagnostics exporter: do not upload raw IDs, host names, device names, MAC addresses, serials, user paths or captures to the public repository. Synthetic fixtures are clearly synthetic and are not observations.

## Verification and the remaining gate

See [foundation checks](FOUNDATION.md) for compilation/tests on an existing toolchain and [dependency provenance](DEPENDENCIES.md) for registry and license details. Linux runs portable contract/waiter tests. Windows additionally compiles the real projection and tests normalized HRESULT/Bluetooth errors, bounded private strings and an in-memory WinRT error operation. Those tests perform no discovery, consent, pairing or device I/O.

Minimum remaining #2 evidence requires an explicitly authorized Windows host/device run: record toolchain and OS/SDK/adapter/driver, then demonstrate discovered host/service metadata, cached versus uncached behavior, unavailable and denied access, timeout/cancellation/cleanup, and sanitized diagnostic handling. Test more adapters/models when available; no single machine proves general hardware support. Necessary connection/consent changes are separate owner decisions, not automatic remediation. Keep #2 open and #3 blocked until their actual gates are reviewed. No battery, speaker-side list, connection control, EQ, reset, firmware or release functionality is added here.

## Integration provenance and evidence limits

This integration uses `discovery-from-pr7.patch` from the fixed [0.3.0 handoff](https://github.com/skydreamer0/speaker-link/tree/bb61d45d170c15f38964eb284018ef322c01946d/handoff/0.3.0), whose ZIP SHA-256 is `ba399da63c6871be9fa847b0419f11eca76dd661dc23fbe986aa9b049a1616a9`. All 65 manifest entries were checked. Product Rust files match source commit `5293fe849f857517f9d23e990595c40c48519d14` after line-ending normalization, including its existing WakeTimer race correction. The smaller `fix-from-ce9dfb6.patch` is an alternative for the older discovery candidate, not a second patch to apply here.

The integration base is `08aa7464b44ba080491678e303d7bfa4cce4c3d8`, after foundation PR #7 and documentation PR #9. Updated status/dependency paragraphs preserve the newer bilingual README, badges, tutorials, compatibility history, and distribution cautions. At that PR #10 integration point, the CLI/app remained placeholders. The later device-free App preview is independent of this adapter; the four-feature SPEC and no-write boundaries remain unchanged. The private selection hosts in the handoff are not installed as a product CLI or included in this workspace.

The handoff reports earlier Windows compilation and 32 workspace synthetic tests, plus separate host tests. Its earlier limited Flex read-only run found one standard `0x111E` service for each cache mode, not a private Bose control endpoint. Firmware 3.0.1 was user-reported, not read by the API. The 0.3.0 lifecycle run observed Pending followed by actual Future-owner drop, and a separate first-poll Ready Timeout; each was followed by a successful one-service requery. These are historical results for the handoff source, not fresh tests of this integration commit.

Pending-to-Timeout on real hardware remains untested. Native Cancel result, cancellation acknowledgment, Close result, OS resource cleanup, and the Timeout source were not observable. Real denied-access and unavailable-device cases were not run. Requery success, process exit, and synthetic cleanup counts cannot establish those missing outcomes. No hardware operation is performed by this integration or generic CI, and no private-channel, EQ, battery, speaker-side list, or connection-control support is inferred.

Each integration commit needs its own Ubuntu and Windows CI results. Linux cross-compilation/checking for a Windows target is not a Windows execution result. Issue #2 stays open; Issue #3 and full F1–F4 acceptance are not unlocked by this integration.
