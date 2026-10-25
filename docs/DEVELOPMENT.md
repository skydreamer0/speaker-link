# Development order and handoff

Document baseline 0.2.0; documentation revised 2026-10-25. This is the public development contract. Local Codex process receipts remain outside the repository.

| Stage | Work | Exit gate | State |
| --- | --- | --- | --- |
| D0 | SPEC, README, license, governance and issue tracking | Reviewed documents and consistent four-feature scope | Documentation baseline merged; maintained as development progresses |
| P0 | Check local Rust/MSVC/SDK; create workspace; service discovery and redacted diagnostics | Compile on recorded Windows build; real no-write discovery evidence | Foundation workspace and synthetic CI merged; Windows and hardware gates open |
| P1 | Core models/codec and Windows adapter; F1, F2, F3, F4 | Shared fixture tests and target hardware evidence; no unsupported writes | Not started |
| P2 | CLI as first complete core consumer | Stable JSON/exit codes; four functions and failure cases verified | Not started |
| P3 | Windows app using the same engine | CLI/app parity and cancellation/resource behavior verified | Not started |
| P4 | Public release | License/dependency review, compatibility table, reproducible build instructions, Windows artifact checksums and release notes | Not started |

Do not publish a library before its API, license and tested compatibility are ready. A diagnostic-only preview is allowed if clearly marked partial and read-only. The first full release remains gated by all four features.

## Working rules

1. Read project instructions, SPEC and the complete tracking issue before changing code. Record a falsifiable acceptance condition, baseline and intended files.
2. Implement one verifiable unit per branch/PR. Keep discovery separate from writes. Preserve evidence of failures as well as successes.
3. Add meaningful codec, capability-policy, redaction and command-state tests. Hardware tests must be explicitly enabled and must never run unattended in generic CI.
4. Maintain formatting, linting, compilation and synthetic-test CI; add adapter integration tests when an implementation is available. Fixture tests are not evidence of speaker support.
5. Review dependency licenses and source provenance before importing code. Commit Cargo.lock for shipped binaries. Pin release toolchain and validate supported Rust/Windows versions before documenting them.
6. Check behavior on actual hardware, have someone independently review the same commit, then release. A changed commit invalidates the earlier acceptance until relevant checks are repeated.
7. Publish capability-specific release notes, checksums and limitations. Do not describe untested models as supported.

The original documentation-only baseline had no builds, tests, package installations or hardware operations. The [P0 foundation](FOUNDATION.md) now supplies an unpublished four-package workspace, synthetic contract tests and a CI definition. It does not supply a working Windows adapter, CLI or app. A passing run is evidence for its exact commit and checks only; see the [source and platform record](COMPATIBILITY.md#source-and-platform-status) for recorded runs. No hardware operation is part of generic CI.
