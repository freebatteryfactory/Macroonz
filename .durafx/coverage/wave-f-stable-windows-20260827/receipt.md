# Stable Windows repository coverage receipt

This Git-tracked receipt retains the stable Windows line and region coverage observation over source commit `9f2679e1bb8ae084dee2bb75aa026c117e729de7`.

Git supplies history, hashing, replication, and change detection for this receipt.

The raw profile, instrumented objects, test executables, and exported summary remain disposable build output.

## Denominator

- The host was Microsoft Windows 11 Home build 26200 on `x86_64-pc-windows-msvc` with an Intel Core i7-8700 and 12 logical processors.
- The compiler was stable Rust 1.98.0 commit `88d9e12ae178fab0fb5cc050a94da85685d449ea` with LLVM 22.1.8.
- Cargo was 1.98.0 commit `797e8a9bc`.
- `cargo-llvm-cov` was exactly 0.9.0 and remained external qualification tooling rather than a product dependency.
- `Cargo.lock` had SHA-256 `471BD8BF8BAA28392CA4B14CB49C175877E6D8E06CD778E20D1AD71C21E3D586`.
- The qualified command used no branch flag, nightly toolchain, threshold, or ignored long-campaign test.

## Observation

- With the absolute repository root elided, `cargo +1.98.0 llvm-cov nextest -j1 --workspace --all-features --locked --no-fail-fast --json --summary-only --output-path target/q/cw/summary.json` completed successfully with `CARGO_TARGET_DIR=target/q/cw/t`.
- The ordinary nextest denominator passed 401 of 401 tests across 60 binaries, while opt-in long campaigns remained separately executed under their own receipt.
- Function coverage was 2,850 of 3,366, or 84.67023172905526 percent.
- Line coverage was 21,043 of 24,747, or 85.03252919545803 percent.
- Region coverage was 28,853 of 34,454, or 83.74354211412319 percent.
- Instantiation coverage was 5,625 of 13,342, or 42.160095937640534 percent.
- Stable branch and MC/DC reporting each returned a zero-member denominator, so no branch or condition percentage is claimed.
- The disposable JSON summary was 114,120 bytes with SHA-256 `FB49188274E3E8B74951287774094A5FC604192F1C3EED563E764C39C84E767C`.

## Post-observer census

- A second stable census executed the clean tracked snapshot `fb1307bc5a2216a9ed77aea20646ffab75d6486d`, including the observer source committed at `53cc0df59cdce64aa524051c96888746e5a801b7`.
- With the absolute repository root elided, `cargo +1.98.0 llvm-cov nextest -j1 --workspace --all-features --locked --no-fail-fast --json --summary-only --output-path target/q/c2/summary.json` completed successfully with `CARGO_TARGET_DIR=target/q/c2/t`.
- The second ordinary nextest denominator passed 411 of 411 tests across 60 binaries, while nine intentional child or ignored tests remained outside direct nextest enumeration.
- Function coverage rose to 2,915 of 3,366, or 86.60130718954248 percent.
- Line coverage rose to 21,487 of 24,747, or 86.82668606295712 percent.
- Region coverage rose to 29,455 of 34,454, or 85.49079932663841 percent.
- Instantiation coverage rose to 5,710 of 13,350, or 42.77153558052434 percent.
- Stable branch and MC/DC reporting again returned a zero-member denominator, so no branch or condition percentage is claimed.
- The second disposable JSON summary was 114,317 bytes with SHA-256 `17F0F603ED298EC53CCA560A44DC37BDB1A0F7665149C89DA2BACF0AB90D0570`.
- The new external observers moved `harness/src/bench/type_contract.rs` to 9 of 9 functions and 27 of 27 lines, `macros/compiler/src/explanation/project.rs` to 1 of 1 function and 13 of 13 lines, and `macros/compiler/src/descriptor/composition.rs` to 3 of 3 functions and 20 of 20 lines.
- The new support observers moved `macros/compiler/src/support/encode.rs` to 2 of 2 functions and 8 of 8 lines and `macros/compiler/src/support/type_contract.rs` to 9 of 9 functions and 42 of 42 lines.
- The closure contract observer moved `macros/compiler/src/closure/type_contract.rs` to 12 of 12 functions and 157 of 159 lines, so the file is strongly observed but not represented as wholly line-covered.
- The published-module codec crossing increased observation of `macros/compiler/src/codec/render.rs`, which remains partially covered at 1 of 2 functions and 21 of 31 lines.
- Coverage remains a routing radar rather than topology or deletion authority.

## Windows path observation

- The first run used the longer exact target `target/qualification/wave-f-stable-coverage-windows-20260827`.
- Its nested generated-support scratch caused Microsoft's `ml64.exe` to refuse the BLAKE3 assembly output argument with `A1009: line too long`.
- Repeating the identical stable command from the short exact target `target/q/cw` removed that host-tooling obstruction and completed successfully.
- The obstruction establishes that Windows coverage choreography needs a bounded short target spelling when nested package-adopter tests invoke MASM.
- It does not establish a Macroonz product, dependency, or generated-support defect.

## Evidence ceiling

- This is a local Windows line and region receipt only.
- Stable branch reporting remains unavailable and no floating-nightly result may substitute for it.
- Proc-macro, child-`rustc`, generated-package, compile-refusal, and opt-in long-campaign observations retain their separate evidence boundaries even when their parent tests contribute executed lines.
- Physical Linux, cloud Linux, macOS, ARM64, hosted, mutation, publication, attestation, and registry planes remain unexecuted here.
- The exported summary is a radar for bounded observer work rather than deletion or topology authority.

## Cleanup

- After the post-observer receipt was committed, Cargo refused to clean `target/q/c2/t` because that coverage layout carried no valid `CACHEDIR.TAG`, and host policy blocked a direct recursive deletion.
- The validated exact `target/q/c2` tree was moved recoverably to the `c2` leaf of the task-created `macroonz-night-scratch-20260827` archive outside the repository.
- The live repository coverage path no longer exists, and the recoverable archive remains disposable with no evidence authority.
