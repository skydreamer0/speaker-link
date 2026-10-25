# Quick start

[English README](../README.md) · [繁體中文 README](../README.zh-TW.md) · [Troubleshooting](TROUBLESHOOTING.md)

This guide separates installing a product from inspecting development source. It is based on the public repository state checked on 2026-10-25.

繁中摘要：目前不能安裝 App 或 CLI。下方教學只讓開發者取得固定的 foundation 原始碼並檢查合成測試，不會探索藍牙，也不代表實機功能已完成。

## Install the app or CLI

There is no runnable release or published crate. There is no supported installer, package-manager command, or install-from-source procedure for an end-user application.

Do not use `cargo install speakerlink-cli`, `cargo install --path crates/speakerlink-cli`, or the proposed `speakerlink` commands in the specification as installation or usage instructions. At the snapshot below, the CLI and app packages have no binary targets.

For product availability, check the repository's [releases](https://github.com/skydreamer0/speaker-link/releases) and [compatibility record](COMPATIBILITY.md). A future release must document its Windows requirements, tested feature profiles, artifact checksums, installation steps, and limitations.

## Check the foundation from source

This path is for contributors. It checks the unpublished foundation contracts from [PR #7](https://github.com/skydreamer0/speaker-link/pull/7), merged into `main` at [`709bffe2`](https://github.com/skydreamer0/speaker-link/commit/709bffe2718fbd07fb1544d83cf3e7d774630a99). The tutorial stays pinned to the original foundation commit for reproducibility; that source is not an application or real Bluetooth adapter.

### 1. Check prerequisites

Use a machine with Git and an installed Rust stable toolchain, including rustfmt and Clippy. For Windows MSVC builds, also use the required Visual Studio C++ build tools and Windows SDK. Follow the official [Rust installation guide](https://rust-lang.org/tools/install/) and [Windows MSVC prerequisites](https://rust-lang.github.io/rustup/installation/windows-msvc.html) if anything is missing.

These are toolchain requirements, not a claim about Speaker Link's minimum supported Rust or Windows version. Neither minimum is established. You do not need a speaker or Bluetooth adapter for this foundation check.

In a terminal, confirm the tools already available:

```text
git --version
rustup run stable rustc --version --verbose
rustup run stable cargo --version
rustup component list --installed --toolchain stable
```

The installed component list should include rustfmt and Clippy. Tool installation is separate from the source check; do not disable security protections or change Bluetooth permissions to fix a compiler setup problem.

### 2. Get an exact snapshot in a new directory

Use a new directory so this example does not switch or overwrite an existing development checkout:

```text
git clone --no-checkout https://github.com/skydreamer0/speaker-link.git speaker-link-foundation
cd speaker-link-foundation
git checkout --detach 382294f407e5b6ef4dd8c3512e0c4f1b2c480b9d
git rev-parse HEAD
```

The last command must print `382294f407e5b6ef4dd8c3512e0c4f1b2c480b9d`. Detached HEAD is intentional for reading and reproducing a fixed snapshot. Stop if the commit is unavailable or different; do not silently substitute the latest branch and claim the same result.

The historical documentation baseline at `2782c396` has no `Cargo.toml`. The pinned foundation snapshot and the merged `main` foundation do. Read the snapshot's [foundation notes](https://github.com/skydreamer0/speaker-link/blob/382294f407e5b6ef4dd8c3512e0c4f1b2c480b9d/docs/FOUNDATION.md) before checking it.

### 3. Run the device-free checks

From the root of that new checkout:

```text
cargo +stable fmt --all -- --check
cargo +stable clippy --offline --locked --workspace --all-targets -- -D warnings
cargo +stable check --offline --locked --workspace --all-targets
cargo +stable test --offline --locked --workspace
```

Run each command separately and stop on failure. These match the checks in the [snapshot's CI workflow](https://github.com/skydreamer0/speaker-link/blob/382294f407e5b6ef4dd8c3512e0c4f1b2c480b9d/.github/workflows/ci.yml).

At this exact snapshot, the Cargo workspace has no external dependencies. The offline checks do not download dependencies; Git cloning and any separate tool installation require network access. Cargo writes build output under `target/`. The checks do not scan Bluetooth, open device sockets, request device access, or send speaker commands.

Expected results:

| Check | What a successful result means |
| --- | --- |
| Formatting | No formatting changes are required |
| Clippy | All targets pass the configured lint checks, with warnings treated as errors |
| Cargo check | All targets type-check; this is not a release build or installer |
| Cargo test | 18 synthetic tests pass across two integration suites: 10 core and 8 discovery-contract tests; other placeholder targets can report zero tests |

The [public CI run](https://github.com/skydreamer0/speaker-link/actions/runs/40165335679) passed on Ubuntu 24.04 and Windows Server 2022 for this commit. That is historical evidence for the pinned source, not a guarantee about your toolchain or a newer commit. Record your own commit, compiler, OS, and each result separately.

### 4. Interpret the result

Passing these checks means the foundation's synthetic contracts passed. Its `UnimplementedWindowsDiscovery` intentionally returns `Unsupported` on every OS. No speaker data, functioning CLI, GUI, complete diagnostic exporter, or hardware-support claim follows from this tutorial.

- Windows CI does not test Windows Bluetooth access.
- Ubuntu CI does not provide a Linux Bluetooth implementation.
- Synthetic test inputs are not recordings from a supported speaker.
- `0.0.0-dev` is an unpublished package version, not a release.

See [troubleshooting](TROUBLESHOOTING.md) if the result differs.

## Hardware testing is a separate stage

[Issue #2](https://github.com/skydreamer0/speaker-link/issues/2) tracks real Windows discovery. This guide does not provide a hardware command or enable device writes. Wait for a reviewed, commit-specific procedure before trying a later adapter candidate.

“No-write” discovery means no speaker-setting commands. An uncached service query can still communicate with the device, so it is not equivalent to zero radio activity. A discovered service is not evidence of battery, speaker-side device lists, or connection control. See the [evidence levels](COMPATIBILITY.md#how-to-read-evidence) and [test plan](TEST_PLAN.md).

## Contribute

Read [CONTRIBUTING](../CONTRIBUTING.md), the [specification](SPEC.md), and the relevant issue. Choose a small, reviewable change and report checks as passed, failed, or not run. Keep documentation changes separate from protocol implementation when possible.

For hardware reports, use the existing compatibility issue template with sanitized, capability-specific evidence. Do not attach raw traces or full local logs. A report is reviewed before it changes public support claims.
