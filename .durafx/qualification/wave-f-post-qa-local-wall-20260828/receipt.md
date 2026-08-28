# Post-QA local qualification receipt

This Git-tracked receipt records the cumulative stable local wall after the Wave F fuzz-owner correction, targeted mutation closure, and package refresh.

## Denominator

- Source snapshot: `1b3827a` on `codex/macroonz-repository-completion` with a clean tracked worktree.
- Host plane: local Microsoft Windows on `x86_64-pc-windows-msvc`.
- Toolchain: stable Rust 1.98.0 with Cargo 1.98.0.
- Every Cargo command was locked and serial where compilation or execution occurred.
- This receipt joins the previously committed focused correction, mutation, and package observations without changing their narrower denominators.

## Cumulative stable wall

- `cargo +1.98.0 nextest run -j1 --workspace --all-features --locked --no-fail-fast` passed 418 of 418 executed tests across 60 binaries, with nine intentionally skipped tests reported by the runner.
- `cargo deny --workspace check` passed advisories, bans, licenses, and sources.
- warnings-denied workspace documentation built successfully with all features and no dependencies.
- the all-feature `wasm32-unknown-unknown` workspace check passed.
- `cargo +1.98.0 fmt --all --check` passed.
- the root `rustc_coverage` facade example compiled and ran successfully with the `harness` feature.
- the example removed its exact disposable coverage scratch before returning.
- the tracked worktree and index were clean after the wall and example crossing.

## Joined earlier observations

- The corrected focused fuzz target passed 18 of 18 external semantic tests.
- The outside compile-refusal lane proves that an adopter cannot mint `InterestingBytes` without coverage novelty admission.
- A bounded 44-mutant campaign initially caught 28 mutants, found 13 unviable mutants, and exposed three viable boundary survivors.
- The three survivors were promoted into owning external regressions and mutation iteration caught all three with no remaining miss or timeout.
- The local package refresh retained exact package rosters, verified the compiler archive, and named the first-publication registry ceiling for dependent packages.
- The exact package target was cleaned after its receipt was committed.

## Truth ceilings

- Typed crash, timeout, and resource-exhaustion outcomes prove supervisor transport and do not claim elapsed-time precision or operating-system quota causality.
- No portable throughput, peak-memory, or process-tree resource baseline is claimed.
- Interleaving material remains deterministically reducible and directly replayable, but no generic schedule-to-replay-capsule road is claimed.
- Stable branch and MC/DC observations have zero-member denominators, so no branch or condition percentage is claimed.
- Live GitHub Mermaid rendering remains a hosted plane even though all diagrams rendered locally in light and dark contexts with accessible titles and descriptions.
- Dependent package verification, publish dry runs, registry retrieval, and as-retrieved adopter tests remain behind dependency-ordered first publication.
- The empty-composition diagnostic vocabulary remains a separate owner decision because the current public refusal misstates absence as overflow.

## Boundary

- This receipt closes the authorized post-QA local Windows qualification wall only.
- It is not hosted CI, hosted security, physical Linux, cloud Linux, macOS, ARM64, package publication, registry delivery, attestation, merge, or release acceptance.
- No hosted service, registry, ref movement, or publication operation was performed by this wall.
