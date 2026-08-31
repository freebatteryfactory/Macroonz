# Macroonz 0.2 Wave G local runtime and host-failure receipt

This Git-tracked receipt accepts the bounded local Windows Wave G runtime and host-failure plane.

## Authority

- Input repository HEAD: `b44dbdd7a5982f55c360d906c31cea4f44b39e18`.
- Host plane: local Microsoft Windows on `x86_64-pc-windows-msvc`.
- Rust compiler: `rustc 1.98.0 (88d9e12ae 2026-08-18)` with LLVM `22.1.8`.
- Cargo: `cargo 1.98.0 (797e8a9bc 2026-08-05)`.
- Scratch home: `target/qualification/0.2-wave-g-runtime-host-20260830`.
- The isolated scratch workspace used the current live-worktree `macroonz-harness` path dependency with default features disabled and makes no clean-package or registry-delivered claim.
- The scratch package and subject are disposable qualification material and create no product package, feature, dependency, report vocabulary, process host, or qualification architecture.

## Safe-Rust source posture

- The tracked denominator contained 571 Rust source files.
- A broad unsafe-token scan found exactly two structural-oracle documentation examples and one token-generator keyword spelling.
- Independent reading classified all three spellings as documentation or data rather than an unsafe construct.
- The root workspace and separately isolated compiler-required proc-macro observer passed their stable Rust 1.98 warnings-denied lint walls with `unsafe_code = "forbid"`.
- The tracked Rust denominator contained zero `#[allow]` and zero `#[expect]` attributes.
- The tracked Rust denominator contained zero `extern "C"` or `#[link(...)]` spelling.
- This source and lint posture applies to Macroonz-owned source and does not audit dependency, standard-library, compiler, LLVM, operating-system, or adopter-code internals.

## Logical concurrency observation

- Existing public `Strand`, `StrandSet`, `ExplorationBound`, `explored`, `concluded`, and temporal-property owners supplied the complete logical schedule road.
- The hostile subject declared one deposit and one withdrawal as two distinct one-command strands.
- The exact interleaving space was counted as two and walked exhaustively.
- The first order held while the second order retained the exact choice string `[1, 0]` at enumerated ordinal one.
- The retained finding carried the declared `wave-g-runtime-host/withdrawal-before-deposit` cause.
- A distinct two-deposit control exhausted the same two-order space and concluded passed.
- This evidence concerns command-order logic and does not establish thread execution, aliasing, data-race detection, memory-model behavior, or undefined-behavior freedom.

## Process-level observation

- The scratch driver compiled one safe-Rust subject with stable rustc `-C instrument-coverage` and no Macroonz-owned unsafe or FFI.
- The caller-owned supervisor waited for a real `std::process::abort` and classified the non-successful exit as `FuzzExecution::Crash(Some(-1073740791))` on this Windows host.
- The crashed subject emitted no admitted coverage point.
- The existing coverage corpus refused the result exactly as `CoverageAdmissionRefusal::Execution(FuzzExecution::Crash(_))` rather than retaining it as successful coverage.
- The exact method-specific crash classification remained visible beside the generic runner/report join.
- The generic host record admitted only `RunAttempt::InfrastructureFailed(BackendExecutionUnresolved)` because no subject conclusion was established.
- No crash was flattened into a subject refusal, passing verdict, coverage admission, or memory-safety conclusion.
- The caller-owned supervisor waited for the exact child to terminate, but this pilot did not establish descendant-process cleanup, timeout causality, stack exhaustion, resource enforcement, sandboxing, or signal identity beyond the retained platform exit code.

## Arithmetic boundary reuse

- Wave G cites the accepted Wave F exhaustive arithmetic and cardinality observation rather than rerunning or relabeling it.
- The cited semantic receipt is `.durafx/qualification/0.2-f-bounded-verification-20260830/receipt.md` at commit `2828e7de9652db5de8ac5fd8a555428a4b059bed`.
- The cited receipt SHA-256 is `3B3E0BAF9D194D8B2E78D00FABED18BC6F34BB1F0ADF303384BED3494FED49DE`.
- The exact cited denominator is all 256 `u8` values under the declared roundtrip property and the precise ceiling `ExhaustiveWithinDeclaredDomain`.
- The joined local Wave F closure is commit `b44dbdd7a5982f55c360d906c31cea4f44b39e18`.

## Qualification

- Stable formatting passed.
- Stable locked and offline all-target checking passed for the exact Windows target with one Cargo job.
- Stable locked and offline all-target Clippy passed under the scratch manifest's repository-equivalent warnings-denied Rust and complete `all` plus `pedantic` wall.
- The single external integration test passed and observed two byte-identical executions.
- Parent reran formatting, locked-offline all-target checking, strict Clippy, the external test, and two direct executions after the owner context completed.
- Parent independently reran the root all-target and all-feature Clippy wall plus the separately isolated compiler-required observer wall.
- Cargo metadata confirmed one isolated unpublished scratch member, one current harness path dependency, default features disabled, and a scratch-local target directory.
- Two parent-captured direct executions were byte-identical and matched the retained 1,057-byte LF UTF-8 reading exactly.
- Direct-output SHA-256: `F018C1DEFBE330568E9BF38355B4491A5A0348C06AA2C2C583F922CEC5AA9ABD`.
- The authored source and retained result have LF line endings, no byte-order mark, no reparse point, no personal identity, and no absolute user path.

## Source custody

- `source-payload.tar` contains exactly `Cargo.lock`, `Cargo.toml`, `README.md`, `rust-toolchain.toml`, `src/main.rs`, `subjects/runtime_subject.rs`, `tests/runtime_host.rs`, `direct-output.txt`, and `result.md`.
- Two independently created USTAR payloads with fixed entry order and modification time were byte-identical.
- Source-payload SHA-256: `1602B41AFC9D6964B252BF98585155C9B7CEB0B280EE8ED9B83F03C046C60BB4`.
- Source-payload size: 39,424 bytes.
- `Cargo.toml` SHA-256: `CD956F634BCFA5C7BD735E985405A9F3F12F0711F336585123B8A9A74CD3B829`.
- `Cargo.lock` SHA-256: `E1161E7FBC43EEA65FCF6A90BFCD5E4BC696604C68DD8E410B5FFD7EA30A9DEB`.
- `README.md` SHA-256: `9A3699AAFC6ABB4717FE0AEFF6B6953847436CAF34C42FD671AF2768AD525510`.
- `rust-toolchain.toml` SHA-256: `23CCFE7A3D1D73658C102BA716D14948476CB079724C4E20513CE994E350EFA7`.
- `src/main.rs` SHA-256: `A0AD3E6F4F5F8C2271DB046FC41344CEEB1A9B015E0A74C5309E1FB294E7C42D`.
- `subjects/runtime_subject.rs` SHA-256: `03ADC86A594B4D55257D59E2D828ED512A14A6EC2602947839170589AC42DB4D`.
- `tests/runtime_host.rs` SHA-256: `C712263F37863452640482D5F83F18C4899BE5963AB10EDD1ADA47CB2496209E`.
- `direct-output.txt` SHA-256: `F018C1DEFBE330568E9BF38355B4491A5A0348C06AA2C2C583F922CEC5AA9ABD`.
- `result.md` SHA-256: `B14DA25368AE80B4FD224653E912785B50D71D8D7DD4BA022F1A7B3F34D95956`.
- Raw Cargo output, compiled subjects, profiles, and other scratch material remain disposable and carry no repository authority.

## Disposition and ceilings

- Existing interleave, fuzz-host, coverage-admission, runner, report, and Wave F owners compose the complete bounded Wave G behavior without a new product seam.
- The accepted local finding is method-specific: one logical-order counterexample and one host-classified abort with an unresolved generic infrastructure standing.
- Stable Rust 1.98 is the complete toolchain denominator and no nightly component or `-Z` flag was used.
- No result is relabeled as memory safety, undefined-behavior freedom, generic correctness, dependency safety, process-tree cleanup, or another host's behavior.
- This receipt establishes no physical-host, cross-host, Linux, macOS, Wasm, architecture-wide, hosted, package-delivery, registry-delivered, or human-acceptance claim.
- Wave C remains independently blocked while Wave H and other independent work may continue.
