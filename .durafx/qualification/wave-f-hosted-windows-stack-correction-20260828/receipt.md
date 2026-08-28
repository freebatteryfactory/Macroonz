# Hosted Windows Loom stack correction

## Standing

- Source base: `b344f057f433d1c29fffe7d68380d2bcdd91afb1`.
- Correction branch: `codex/macroonz-windows-loom-stack`.
- Qualified source commit: `57249010f8d3cee7548678b6e9b39a662a07ff5c`.
- First hosted pulse: GitHub Actions run `33201896588` against exact merged source `b344f05`.
- Campaign-plan snapshot used for the correction: SHA-256 `DAF6A6F0EB38C8F9A88DB07C074DAA5020707542DE23A12956392ACB85E9E8D1`.
- This receipt records a locally qualified test correction pending owner acceptance and a corrected hosted pulse.
- No corrected hosted Windows, cloud-host, push, merge, or workflow-dispatch result is claimed.

## Causal observation

- The first pulse passed Linux x64, Linux ARM64, and macOS ARM64.
- Hosted Windows passed graph checking, strict Clippy, and 422 of 423 executed tests before `supported::branch_exhaustion_stays_infrastructure_unresolved` aborted with Windows stack-overflow exit `0xc00000fd`.
- That test deliberately forces Loom past a branch budget of one and requires Macroonz to retain Loom's foreign unwind as typed `InfrastructureFault::BackendExecutionUnresolved` rather than a subject verdict.
- The hosted test-process stack exhausted before Rust could unwind through the existing `catch_unwind`, so the process aborted instead of returning the typed reading.
- The unchanged failing test passed locally under `RUST_BACKTRACE=1`, so the local host did not reproduce the hosted process-stack ceiling and no stronger causal claim is made.

## Correction contract

- Only the external test `harness/tests/preemption_exploration/supported.rs` changed.
- The branch-exhaustion crossing now executes on a named safe-Rust worker with an explicitly declared 8,388,608-byte stack.
- Loom must still exceed the same branch budget, return through the same production `explored` road, and produce the same typed incomplete infrastructure reading.
- The existing assertions still require `ExecutionUnresolved`, `BackendExecutionUnresolved`, and retained foreign material.
- Failure to spawn the worker, an unwind escaping the production catch boundary, or failure to produce the typed reading still fails the test.
- `RUST_BACKTRACE=1` remains active, and no test, assertion, host, feature, dependency, timeout, diagnostic, or workflow step was removed, skipped, retried, or weakened.
- No product source, API, identity, encoded byte, package, feature, dependency, workflow, or public documentation changed.

## Local qualification

- Host toolchain: rustc 1.98.0 commit `88d9e12ae178fab0fb5cc050a94da85685d449ea`, Cargo 1.98.0 commit `797e8a9bca276c1c9f9f738d2a20f484fa4eea9d`, LLVM 22.1.8, `x86_64-pc-windows-msvc`.
- Qualification tools: cargo-nextest 0.9.132 commit `6e4a9d6f2c4964f30ff54a8cd5466f8869267daa` and cargo-deny 0.19.0.
- The focused corrected crossing passed once after target checking and strict Clippy.
- Twenty additional fresh nextest processes passed the exact corrected crossing under `RUST_BACKTRACE=1` with no failure.
- Stable Rust 1.98 passed source-wide formatting, workspace every-target and all-feature checking, and strict Clippy.
- The complete serial `ci` profile ran 423 tests across 60 binaries, passed all 423, and reported nine intentional skips.
- Four compiler doctests passed, and the other three packages retained zero doctests.
- Cargo-deny reported advisories, bans, licenses, and sources green.
- Warnings-denied documentation and all-feature `wasm32-unknown-unknown` checking passed.
- The facade `rustc_coverage` example compiled and executed successfully.
- `git diff --check` reported no whitespace error.

## Remaining hosted boundary

- Local evidence cannot establish the corrected GitHub Windows Server 2025 observation that motivated the change.
- The correction branch requires an owner-authorized push and explicit merge without squash or rebase before the manual workflow can execute the corrected default-branch source.
- A later owner-authorized manual pulse must rerun the declared four-seat workflow and reach terminal state before the hosted denominator can close.
- Automatic triggers, required checks, branch governance, physical-host qualification, publication, attestations, and registry delivery remain outside this correction.
