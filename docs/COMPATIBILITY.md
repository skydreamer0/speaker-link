# Compatibility

[Quick start](QUICKSTART.md) · [Test plan](TEST_PLAN.md) · [Report guidance](TROUBLESHOOTING.md#report-a-problem-without-sharing-private-data)

Public status checked 2026-10-25. **No speaker capability profile has been verified by this project for release.** The first research sample is SoundLink Flex first generation, model 435910. It does not limit future validation to one host, adapter vendor, or firmware.

## How to read evidence

| Evidence | What it establishes | What it does not establish |
| --- | --- | --- |
| Planned | A specified goal or acceptance test | Working source or support |
| Synthetic checks passed | The identified commit passed the listed fixture checks | Windows Bluetooth access or behavior of a real speaker |
| User-reported observation | A contributor reports a result for a stated environment | Independently reviewed compatibility or a release guarantee |
| Reviewed hardware result | A reviewer checked evidence for an exact commit, environment, and feature | Other features, models, firmware, or host combinations |
| Released support | A release names the tested profile and capabilities | Universal support for a model family |

These are evidence labels, not new runtime capability values. [SPEC](SPEC.md) retains its `unknown`, `unsupported`, and `verified` capability policy. Unknown is not proof of impossibility, and discovery alone never authorizes writes.

## Source and platform status

| Scope | Public evidence | Bluetooth/product status |
| --- | --- | --- |
| Experimental read-only integration in this source | [Fixed handoff and integration limits](WINDOWS_DISCOVERY.md#integration-provenance-and-evidence-limits); each new commit requires its own CI | WinRT library adapter for a host-provided, already permitted device; no runnable scanner or completed hardware acceptance |
| Foundation introduced by PR #7, `709bffe2` | [PR #7](https://github.com/skydreamer0/speaker-link/pull/7) merged on 2026-10-25; [merge commit](https://github.com/skydreamer0/speaker-link/commit/709bffe2718fbd07fb1544d83cf3e7d774630a99) | Unpublished contracts; CLI/app placeholders; no working Bluetooth adapter |
| Documentation update merged in `c5baadf5` | [PR #8](https://github.com/skydreamer0/speaker-link/pull/8); [exact-commit main/push CI](https://github.com/skydreamer0/speaker-link/actions/runs/40174021328) passed on Ubuntu 24.04 and Windows Server 2022 | README and guide changes only; no new Bluetooth capability |
| Historical documentation baseline, `2782c396` | Specification and project documents before the foundation merge | No Cargo workspace or executable at that historical commit |
| Pinned foundation snapshot, `382294f4`, now included in `main` | [Exact-commit CI](https://github.com/skydreamer0/speaker-link/actions/runs/40165335679) passed on Ubuntu 24.04 and Windows Server 2022 | Synthetic checks only; this run does not certify the merge commit or later documentation changes |
| Windows hardware | Real discovery and access evidence tracked in [issue #2](https://github.com/skydreamer0/speaker-link/issues/2) | No released host/adapter/driver profile |
| Linux Bluetooth | No backend in the referenced foundation | Ubuntu fixture CI is not Linux device support |
| macOS and other platforms | No device backend or product validation recorded here | Not part of the initial Windows product scope |

The retained foundation discovery sentinel returns `Unsupported` on every OS; it does not invoke the separate experimental Windows adapter. This describes the sentinel implementation, not the inherent capabilities of a particular speaker. No minimum supported Windows or Rust version has been selected. Historical foundation CI and handoff hardware reports do not certify the current integration commit.

## Speaker feature status

| Target | Firmware profile | Battery (F1) | Speaker-side list (F2) | Connect/disconnect (F3) | Diagnostic export (F4) |
| --- | --- | --- | --- | --- | --- |
| SoundLink Flex first generation, 435910 | Not verified | Unverified | Unverified | Unverified; writes disabled by policy | Planned; foundation error preview is not full export |
| Other Bose models or firmware | No reviewed profile in this table | Unknown | Unknown | Disabled until an exact profile is verified | No hardware claim |

The historical Windows identity observation `BC60` has not been reproduced in this public evidence record. It must keep its Windows namespace. A third-party `BC59` reference does not prove equivalence; Windows identity, Bluetooth Device ID, and Bose protocol identity are separate.

Standard Bluetooth profile services do not establish a Bose-specific control or EQ channel. Cached and uncached discovery must be distinguished. No-write discovery can still communicate over Bluetooth, and successful discovery is not success for F1–F4.

## Add evidence without overstating support

Use the repository's compatibility issue template and the [report checklist](TROUBLESHOOTING.md#report-a-problem-without-sharing-private-data). Test only devices you own or are authorized to test, using the procedure for that exact candidate.

Record commit, model and identity namespace, firmware or unknown, Windows build, adapter/driver, transport, date, and each feature's passed/failed/not-tested outcome. Do not publish serials, MACs, private device names, raw traces, local paths, or full logs.

A maintainer reviews the sanitized evidence before updating this table. A feature verified for one combination does not enable all functions or all firmware. Unknown firmware remains read-only; connection control additionally requires an exact tested capability profile and readback. The full-release gate in [SPEC](SPEC.md#release-acceptance) remains unchanged.
