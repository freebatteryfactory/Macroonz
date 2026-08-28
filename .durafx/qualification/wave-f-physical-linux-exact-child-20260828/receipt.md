# Physical Linux exact-child local wall

This Git-tracked receipt records the physical-host Linux x64 local wall over the exact-child source now on default `main`.

The predecessor physical Linux receipt at `710b881` qualified merge `f3e255e` and does not speak for this source.

The solo-governance receipt named physical Linux for the current exact-child source as unestablished, and the green hosted pulse is a cloud-host observation.

## Standing

- Source snapshot: `1f2929264d74cecd97b161e93b646300274cf26e`.
- Qualification branch: `codex/macroonz-linux-child-local-ci`.
- That snapshot's merge parents are `be990fb11b8968ab13944c2aee746b200ce929c8` and `615bb6d6a57b5f463d8b03c4fead7f883c3601f3`.
- The qualified tree includes the exact-child diagnostic isolation at `35dc1a34b0b8b53e4906f4ca68e8996947ef45b9`.
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
- `cargo +1.98.0 nextest run --profile ci -j1 --workspace --all-features --locked --no-fail-fast` started 423 tests across 60 binaries, passed all 423, and reported ten intentional skips.
- The `ci` profile wrote a parseable 95,685-byte JUnit document with 56 suites, 423 cases, zero failures, and zero errors.
- `cargo +1.98.0 test -j1 --doc --workspace --all-features --locked` passed four compiler doctests, and the other three packages retained zero doctests.
- `cargo deny --workspace check` passed advisories, bans, licenses, and sources.
- `RUSTDOCFLAGS=-Dwarnings cargo +1.98.0 doc -j1 --workspace --all-features --no-deps --locked` passed.
- `cargo +1.98.0 check -j1 --workspace --all-features --target wasm32-unknown-unknown --locked` passed.
- `cargo +1.98.0 run -j1 --example rustc_coverage --features harness --locked` passed.
- The coverage example removed its exact disposable run under `target/qualification` and left no `rustc-coverage-example-*` directory.
- `git diff --check` reported no whitespace error, and `git diff --exit-code HEAD --` reported no tracked-tree mutation.

## Focused crossing

- The ordinary parent `supported::branch_exhaustion_stays_infrastructure_unresolved` executed under the wall's `RUST_BACKTRACE=1` posture and passed.
- That parent launches the ignored child `supported::branch_exhaustion_is_typed_child` with backtrace generation disabled and requires the child's post-assertion completion marker.
- Direct nextest enumeration therefore reports ten intentional skips, matching the hosted exact-child denominator rather than the earlier nine-skip worker-stack wall.

## Custody

- The wall transcript lived at disposable `target/qualification/linux-child-local-ci-20260828` as one 76,251-byte log and retains no evidence authority once this receipt is committed.
- Nextest JUnit output remained under disposable `target/nextest/ci` and was not promoted into Git.

## Remaining planes

- This receipt records the physical Linux x64 local wall for exact-child snapshot `1f29292` only.
- Physical Linux ARM64, cloud-host cache behavior, JUnit ingestion, machine metrics, package publication, registry delivery, attestations, and merge to default remain outside this receipt.
- A cloud runner still does not establish a physical-host claim, and this physical host does not replace the hosted pulse already sealed for `be990fb`.
