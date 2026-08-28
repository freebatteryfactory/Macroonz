# Stable rustc coverage hardening receipt

This Git-tracked receipt retains qualified observations from the Wave F rustc-coverage hardening pass pending owner acceptance.

Git supplies history, hashing, replication, and change detection for this receipt.

No ignored warehouse, custom filesystem sealer, compiled output, raw profile, or transient corpus is retained here.

## Denominator

- The source base was `fdc8691f56eb1134c54b357344d169636ff35c1b` on `codex/macroonz-repository-completion`.
- Qualification executed against the complete source snapshot now recorded by `b5d3a5b9b482e48a75bb9d0a7f562f5ba6c33a0f`.
- The owner deliberately pushed that source commit while qualification was running, so this receipt is retained by a later evidence commit rather than folded into the source commit.
- Local commit `b83922d6eb51873fabc38599b8c9537e9a2600ff` makes the successful facade example remove its exact scratch run after the qualification audit found accumulated example output.
- The product graph remained the four existing packages plus the compiler-required narrow proc-macro test fixture.
- Stable Rust 1.98 and its matching LLVM tools were the only coverage toolchain.
- No Frida, LibAFL, TinyInst, native instrumentation dependency, nightly toolchain, unsafe source, lint escape, feature, or qualification Cargo package entered the tree.

## Contract observations

- Active preflight executes the declared absolute compiler, requires release 1.98.0, derives both LLVM tools from that compiler's reported sysroot and host, executes both tools, and requires coherent version reports.
- Hostile process doubles proved exact refusal of a wrong compiler release and disagreement between `llvm-profdata` and `llvm-cov`.
- Execution accepts only the informed preflight witness.
- Each candidate is materialized before process spawn and opened as child standard input, so a target that never reads a 16 MiB candidate remains under supervisor control rather than blocking on a pipe write before supervision.
- Every post-spawn return path terminates and reaps the child or returns a typed cleanup refusal.
- An actual abort crossed as `Crash`, and declared supervisor outcomes crossed as `Timeout` and `ResourceExhaustion`.
- The timeout and resource results establish typed supervisor transport, not elapsed-time or operating-system quota causality.
- Coverage identity contains a declared logical root and root-relative source path rather than a physical checkout path.
- Synthetic LCOV documents beneath two distinct declared checkout roots produced identical logical coverage observations, and ordinary versus verbatim Windows path spellings normalized identically.
- Mutation budgets select an exact deterministic prefix of the documented priority order.
- The root facade example compiles an instrumented subject, establishes preflight, observes novelty, evolves a corpus, and composes interesting bytes through reduction and replay.

## Windows qualification

- `cargo +1.98.0 check -j1 --workspace --all-targets --all-features` passed.
- `cargo +1.98.0 clippy -j1 --workspace --all-targets --all-features -- -D warnings` passed.
- `cargo +1.98.0 nextest run -j1 --workspace --all-features` passed 401 of 401 tests, with five intentional child fixtures skipped by direct nextest enumeration and exercised through their owning parent tests.
- `cargo +1.98.0 fmt --all -- --check` passed.
- `cargo deny --workspace check` passed advisories, bans, licenses, and sources.
- `RUSTDOCFLAGS=-Dwarnings cargo +1.98.0 doc -j1 --workspace --all-features --no-deps` passed.
- `cargo +1.98.0 check -j1 --workspace --all-features --target wasm32-unknown-unknown` passed.
- `cargo +1.98.0 run -j1 --example rustc_coverage --features harness` passed.
- The facade example passed again after its exact successful-run cleanup was added and created no new retained scratch directory.
- The focused `fuzz_compose` target passed 11 of 11 tests after the final cleanup repair.

## WSL-native qualification

- The qualified WSL run used a checkout on the invoking user's Linux-native home filesystem rather than the Windows checkout mounted under `/mnt/c`.
- Its Cargo target was the checkout-local `target/qualification/wave-f-rustc-fuzz` directory on that Linux filesystem.
- The host compiler was Rust 1.98.0 for `x86_64-unknown-linux-gnu` with LLVM 22.1.8.
- The focused `fuzz_compose` target passed 11 of 11 tests.
- The root facade `rustc_coverage` example passed.
- The root facade example passed again with successful-run cleanup active.
- Locked default-full, harness-only, diet, and renamed-dependency path adopters passed from Linux-filesystem scratch projects.
- This is WSL-native evidence and is not a physical-Linux, cloud-Linux, macOS, or ARM receipt.

## Package and adopter qualification

- `macroonz-compiler` packaged and verified successfully as 300 files and a 371.2 KiB compressed archive.
- Packaging `macroonz-harness`, `macroonz-macros`, and `macroonz` stopped truthfully because crates.io does not yet contain `macroonz-compiler = 0.1.0`.
- `cargo package -p macroonz --allow-dirty --list` included `examples/rustc_coverage.rs` and `examples/support/rustc_coverage_subject.rs`.
- Locked default-full, harness-only, diet, and renamed-dependency path adopters passed on Windows.
- The package-publication order ceiling does not establish registry, publish-dry-run, or registry-delivered artifact behavior.

## Remaining ceilings

- Physical Linux, cloud Linux, macOS, and ARM64 remain unexecuted.
- Stable repository line or region coverage, long schedules, and performance remain separate evidence planes retained by later Wave F receipts; targeted mutation remained unexecuted at this receipt's source snapshot.
- GitHub governance, hosted CI, merge, publication, attestation, and registry-delivery work remain at their explicit human boundaries.
- The exact Linux-filesystem WSL qualification checkout was removed after its retained observations were recorded.
- Host policy blocked direct recursive deletion of the exact Windows scratch directories, so they were moved recoverably to the task-created `macroonz-wave-f-scratch-20260827` archive outside the repository.
- The recoverable Windows scratch archive is disposable and has no evidence authority.
