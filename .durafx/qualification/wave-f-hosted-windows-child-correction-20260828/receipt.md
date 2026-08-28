# Hosted Windows exact-child correction

## Standing

- Source base: `f3e255ec466b513e00c9496d43e3a4dc83d1db42`.
- Correction branch: `codex/macroonz-windows-loom-child`.
- Qualified source commit: `35dc1a34b0b8b53e4906f4ca68e8996947ef45b9`.
- Campaign-plan snapshot used for the correction: SHA-256 `131A8840B7E1C012F999CB612D021ADA46C008FCCE0B2217EEC4DC4CAF4919ED`.
- This receipt records a locally qualified external-test correction pending owner acceptance and a corrected hosted pulse.
- No corrected hosted Windows, cloud-host, push, merge, or workflow-dispatch result is claimed.

## Falsified predecessor

- First hosted pulse `33201896588` aborted the intentional Loom branch-exhaustion crossing on GitHub Windows with stack-overflow exit `0xc00000fd` under `RUST_BACKTRACE=1`.
- The first correction moved that crossing to a named safe-Rust worker with an explicitly declared 8,388,608-byte stack while retaining the same workflow diagnostic posture.
- Corrected pulse `33205616869` ran that exact source and again aborted with `0xc00000fd` after its log named `macroonz-loom-branch-exhaustion` and began printing a backtrace.
- The second run therefore proved the explicit worker was active and falsified the theory that an 8 MiB worker alone stabilized the hosted diagnostic unwind.
- Both runs retained three green seats and one incomplete hosted-Windows seat, and neither is relabeled as a complete hosted pulse.

## Correction contract

- Only `harness/tests/preemption_exploration/supported.rs` changed.
- The falsified worker-stack scaffold is removed.
- The ordinary parent test remains under the workflow's `RUST_BACKTRACE=1` posture and launches one exact ignored child through the lane's existing process helper.
- Only that child receives `RUST_BACKTRACE=0` and `RUST_LIB_BACKTRACE=0`, because its test input deliberately forces a Loom panic whose typed classification is the subject of the crossing.
- The child still forces the same branch budget of one through the same production `explored` road.
- The child still requires `ExecutionUnresolved`, `BackendExecutionUnresolved`, and retained foreign material before it can succeed.
- The child writes one completion marker only after those typed assertions hold, and the parent requires that marker so an exact-name typo cannot pass through a zero-test child.
- Child spawn failure, child abort, wrong typed classification, missing foreign material, missing completion marker, or any nonzero child status fails the ordinary parent test.
- The workflow, every other test, product source, public API, feature, dependency, identity, encoded byte, and accepted host denominator remain unchanged.
- The ordinary executed-test denominator remains 423, while the declared ignored-child roster increases from nine to ten.

## Local qualification

- Host toolchain: rustc 1.98.0 commit `88d9e12ae178fab0fb5cc050a94da85685d449ea`, Cargo 1.98.0 commit `797e8a9bca276c1c9f9f738d2a20f484fa4eea9d`, LLVM 22.1.8, `x86_64-pc-windows-msvc`.
- Qualification tools: cargo-nextest 0.9.132 commit `6e4a9d6f2c4964f30ff54a8cd5466f8869267daa` and cargo-deny 0.19.0.
- The exact ignored child passed directly with scoped backtrace generation disabled and visibly emitted Loom's branch-limit panic before returning through the typed assertions.
- The ordinary parent passed under `RUST_BACKTRACE=1` and required the child's post-assertion completion marker.
- Twenty additional fresh nextest parent processes passed with the workflow backtrace posture retained.
- Stable Rust 1.98 passed source-wide formatting, workspace every-target and all-feature checking, and strict Clippy.
- The complete serial `ci` profile ran 423 tests across 60 binaries, passed all 423, and reported ten intentional skips.
- Four compiler doctests passed, and the other three packages retained zero doctests.
- Cargo-deny reported advisories, bans, licenses, and sources green.
- Warnings-denied documentation and all-feature `wasm32-unknown-unknown` checking passed.
- The facade `rustc_coverage` example compiled and executed successfully.
- `git diff --check` reported no whitespace error.

## Remaining hosted boundary

- Local evidence cannot establish the GitHub Windows Server 2025 observation that motivated the child isolation.
- The correction branch requires an owner-authorized push and explicit merge without squash or rebase before the manual workflow can execute the corrected default-branch source.
- A later owner-authorized manual pulse must rerun the declared four-seat workflow and reach terminal state before the hosted denominator can close.
- No automatic retry, required check, workflow trigger expansion, branch governance, publication, attestation, registry mutation, or physical-host claim is authorized by this receipt.
