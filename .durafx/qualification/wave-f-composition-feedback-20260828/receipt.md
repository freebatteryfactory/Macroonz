# Composition feedback correction receipt

This Git-tracked receipt records the complete local Windows qualification of the composition feedback correction.

## Authority and source

- Entering repository snapshot: `8c184279189b83e5a94d0cba3108ac561227bd87` on `codex/macroonz-repository-completion`.
- Qualified source snapshot: `e49c206d216bb896918b9a2f5730e5620cb22494`.
- Host: local Microsoft Windows on `x86_64-pc-windows-msvc`.
- Toolchain: rustc 1.98.0 with LLVM 22.1.8 and Cargo 1.98.0.
- Mutation tool: cargo-mutants 27.0.0.

## Owner rulings carried into the correction

- `Composition::declared(Vec<Provider>)` remains the ergonomic dynamic declaration road.
- `Composition::of_one(Provider)` remains the zero-ceremony total road.
- `CompositionIssue::slot()` remains public because append-only public issue slots are established Macroonz compatibility vocabulary and directly discriminate canonical bytes.
- The unreachable second composition-issue cap is removed rather than retained as public posture.
- Every lawful duplicate finding is retained after the sixty-four-provider magnitude admits the roster.
- Empty, oversized, and doubled-provider refusals use the existing typed diagnostic channel and the descriptor meaning fact.
- The repository README owns the composition model while constructor mechanics remain in item documentation and outside tests.
- The obsolete statement that the repository has no CI was removed from `AGENTS.md`; local checks still report and a human still decides.

## Structural result

- A successful `Composition` remains privately non-empty and bounded by `PROVIDER_LIMIT`.
- `CompositionError` is privately non-empty through one required primary issue and a complete ordered tail.
- No capping method, composition issue limit, or truncation rendering remains on the public composition refusal surface.
- Empty input repairs to `state at least one provider of descriptor material`.
- Oversized input repairs to `state no more than the declared provider magnitude`.
- A doubled identity repairs to `state each provider identity once`.
- Exact independent canonical vectors pin both public slots, nested declaration rows, framing, provider subjects, and provider identity bytes.
- A maximum-width external crossing retains all thirty-two possible distinct doubled identities from one admitted sixty-four-provider roster.
- No crate, dependency, feature, backend, qualification package, output channel, or unsafe path was added.

## Stable local wall

- `cargo +1.98.0 check -j1 --workspace --all-targets --all-features --locked` passed.
- `cargo +1.98.0 clippy -j1 --workspace --all-targets --all-features --locked -- -D warnings` passed with no suppression.
- `cargo +1.98.0 nextest run -j1 --workspace --all-features --locked --no-fail-fast` passed 423 of 423 executed tests across 60 binaries, with nine intentionally skipped tests.
- `cargo +1.98.0 test -j1 --doc --workspace --all-features --locked` passed all four doctests.
- `cargo +1.98.0 fmt --all -- --check` passed.
- `cargo deny --workspace check` passed advisories, bans, licenses, and sources.
- Warnings-denied workspace documentation built with all features, no dependencies, and the locked Rust 1.98 graph.
- The locked all-feature `wasm32-unknown-unknown` workspace check passed.
- The root `rustc_coverage` facade example compiled and ran through the `harness` feature.
- `git diff --check` passed before the source commit.
- Focused searches found no composition capping holder and no new unsafe or lint-suppression site.

## Targeted mutation

- The filtered denominator covered the composition duplicate scan, issue slots, canonical bytes, diagnostic classification and projection, complete issue iteration, issue counting, and repair dispatch.
- Forty-one mutants were tested in one serial cargo-mutants campaign.
- Twenty mutants were caught by the outside behavior lanes.
- Twenty-one mutants were unviable because the changed program could not satisfy the informed type surface.
- Zero mutants were missed.
- Zero mutants timed out.
- The exact disposable campaign path contains 91 files and 2,642,629 bytes before cleanup.
- The exact disposable path is `target/qualification/wave-f-composition-polish-mutants-20260828`.
- After this receipt was committed, the exact validated path was removed with Cargo's cache-directory guard.
- Cleanup removed 93 files totaling 2.5 MiB, including the temporary cache tag required for guarded cleanup.
- A direct existence check confirmed that the exact disposable path no longer exists.

## Boundary

- This receipt qualifies the composition feedback correction on the local Windows and Wasm compile planes only.
- It is not hosted CI, physical Linux, macOS, ARM64, package publication, registry delivery, merge, or release acceptance.
- No push, merge, rebase, squash, ref movement, hosted workflow, or publication operation was performed.
