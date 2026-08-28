# Physical Linux local CI wall

This Git-tracked receipt records the physical-host Linux x64 local wall over the merged Windows Loom tree.

It does not dispatch hosted CI, merge this branch to default, or establish a cloud-host claim.

## Standing

- Source snapshot: `f3e255ec466b513e00c9496d43e3a4dc83d1db42`.
- Qualification branch: `codex/macroonz-linux-local-ci`.
- That snapshot's merge parents are `b344f057f433d1c29fffe7d68380d2bcdd91afb1` and `4b30cd415c1d71a3debc1012dd50f22262700b22`.
- The qualified tree includes the Windows Loom worker-stack correction at `57249010f8d3cee7548678b6e9b39a662a07ff5c`.
- Host plane: physical Pop!_OS 24.04 LTS on `x86_64-unknown-linux-gnu`, kernel `7.0.11-76070011-generic`.
- Host toolchain: rustc 1.98.0 commit `88d9e12ae178fab0fb5cc050a94da85685d449ea`, Cargo 1.98.0 commit `797e8a9bca276c1c9f9f738d2a20f484fa4eea9d`, LLVM 22.1.8.
- Qualification tools: cargo-nextest 0.9.132 commit `6e4a9d6f2c4964f30ff54a8cd5466f8869267daa` and cargo-deny 0.19.0.
- `Cargo.lock` had SHA-256 `471BD8BF8BAA28392CA4B14CB49C175877E6D8E06CD778E20D1AD71C21E3D586`.
- Cargo fetched the locked graph once, then ran offline.
- Compilation and test execution used `CARGO_INCREMENTAL=0`, `RUST_BACKTRACE=1`, and `RUSTDOCFLAGS=-Dwarnings`.
- The tracked worktree was clean before this receipt was written.

## Local wall

- The wall used the serial locked commands owned by `CONTRIBUTING.md`, with nextest on the hosted `ci` profile.
- `cargo +1.98.0 fmt --all -- --check` passed.
- `cargo +1.98.0 check -j1 --workspace --all-targets --all-features --locked` passed.
- `cargo +1.98.0 clippy -j1 --workspace --all-targets --all-features --locked -- -D warnings` passed with no suppression.
- `cargo +1.98.0 nextest run --profile ci -j1 --workspace --all-features --locked --no-fail-fast` started 423 tests across 60 binaries, passed all 423, and reported nine intentional skips.
- The `ci` profile wrote a parseable 95,687-byte JUnit document with 56 suites, 423 cases, zero failures, and zero errors.
- `cargo +1.98.0 test -j1 --doc --workspace --all-features --locked` passed four compiler doctests, and the other three packages retained zero doctests.
- `cargo deny --workspace check` passed advisories, bans, licenses, and sources.
- `RUSTDOCFLAGS=-Dwarnings cargo +1.98.0 doc -j1 --workspace --all-features --no-deps --locked` passed.
- `cargo +1.98.0 check -j1 --workspace --all-features --target wasm32-unknown-unknown --locked` passed.
- `cargo +1.98.0 run -j1 --example rustc_coverage --features harness --locked` passed.
- The coverage example removed its exact disposable run under `target/qualification` and left no `rustc-coverage-example-*` directory.
- `git diff --check` reported no whitespace error, and `git diff --exit-code HEAD --` reported no tracked-tree mutation.

## Focused crossing

- The branch-exhaustion preemption crossing that required a dedicated worker stack on Windows executed inside the same 423-test `ci` profile on this physical Linux host and passed.

## Custody

- The wall transcript lived at disposable `target/qualification/linux-local-ci-20260828` as one 84,443-byte log and retains no evidence authority once this receipt is committed.
- Nextest JUnit output remained under disposable `target/nextest/ci` and was not promoted into Git.

## Remaining planes

- This receipt closes the physical Linux x64 local wall for the merged snapshot only.
- Cloud Linux x64, cloud Linux ARM64, physical Linux ARM64, macOS, hosted cache behavior, JUnit ingestion, machine metrics, and a corrected hosted Windows pulse remain unexecuted here.
- A cloud runner still does not establish a physical-host claim, and this physical host does not establish a hosted pulse.
- Automatic triggers, required checks, branch governance, package publication, registry delivery, attestations, and merge to default remain outside this receipt.
