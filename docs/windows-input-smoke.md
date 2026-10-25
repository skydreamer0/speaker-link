# Hosted Windows native input responsiveness smoke

The existing Windows 2022 contracts job builds the actual `speakerlink-app.exe`
and launches its default device-free fixture with the default WGPU renderer. No device selection, clicks, keys,
audio APIs, user computer, extra packages, secrets, or paid/GPU runners are used.

The Python standard-library script finds a visible top-level window by the
spawned PID **and** exact title. It requires an accessible input desktop,
foreground ownership and native point hit-testing. It sends real absolute
`SendInput` mouse movements across five client-area positions for eight seconds,
checks the actual cursor position, and checks `IsHungAppWindow` and bounded
`SendMessageTimeout(WM_NULL)` before and after input. Missing desktop, denied
foreground activation, obscured window, failed insertion, mismatched cursor,
process exit, or unresponsive message pump fail the job, not skip/pass.

This is narrower than GUI acceptance: a successful message pump does **not**
prove painting, GPU correctness, delivery of every egui hover event, or resolution
of the user's local pointer-triggered hang. No screenshot or visual comparison
is claimed. Keep PR 12's merge hold until the reported native problem has been
separately diagnosed and validated. A hosted failure can also be runner desktop
or foreground restrictions; inspect its phase/reason before attributing it to
application code. No official Glow comparison app is added in this change.

A window must appear within 20 seconds; each response call waits at most one
second. The workflow step has a two-minute outer limit. Finally, cleanup uses
only the launched process's retained `Popen` handle, never a process-name kill.
This preview does not launch child processes. Output is captured privately in
temporary files and reported as byte counts plus allowlisted eframe error
categories from at most the first 4096 stderr bytes. The exact upstream
`egui_glow requires opengl 2.0+.` error has a fixed classification code; arbitrary
messages remain unclassified and are never echoed. No raw application logs,
environment values or desktop captures are published. Compact JSON phase,
PID, executable SHA-256 and timing receipts go to the job log and step summary.
The renderer candidate also reports only complete allowlisted startup lines from
the first 4096 stdout bytes: renderer, backend and device-type enums. A `Cpu`
adapter is explicitly software, not hardware GPU acceptance. Unknown output stays
unverified. Upstream adapter selection remains unchanged; no software driver is
installed or implicit renderer retry performed.
There are no added artifacts, caches, storage retention or permission changes.

`python -m unittest discover -s tests -p test_windows_input_smoke.py -v` validates
control-flow failure handling with mocks on any platform. These tests are not
evidence that Windows input APIs or a real window worked. The actual smoke is
restricted to the Windows Actions job; do not use it to operate a user's desktop.

The original Glow failures are retained as historical evidence in
[renderer compatibility](RENDERER_COMPATIBILITY.md); a later WGPU pass does not
turn those failures into passes.
