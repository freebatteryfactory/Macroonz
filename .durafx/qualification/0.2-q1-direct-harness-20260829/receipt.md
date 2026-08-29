# Macroonz 0.2 direct-harness composition census

This Git-tracked receipt records the first Q1 composition-before-creation specimen against the published Macroonz 0.1 harness.

## Authority

- Campaign branch: `codex/macroonz-0.2-release-line`.
- Entering repository snapshot: `b71ddb2de088203489df5e99f4be10f49c137a1e`.
- Published dependency: `macroonz-harness = "=0.1.0"` from crates.io with default features disabled.
- Host plane: local Microsoft Windows on `x86_64-pc-windows-msvc`.
- Toolchain: stable Rust 1.98.0 with Cargo 1.98.0.
- Scratch home: `target/qualification/0.2-q1-direct-harness-20260829`.
- This pass performed no product-source edit, dependency edit, feature edit, push, merge, ref rewrite, publication, or registry mutation.

## Question

Can a hand-written adopter target use the published harness batteries to state and judge a bounded temporal property without a proc-macro projection, generated scaffold, Loom, or a new framework abstraction?

## Specimen

- The subject is a small bounded state machine with declared byte inputs.
- The lawful history `[1, 2]` is judged through the published `TransitionContract`, `TemporalClaim`, `TemporalDemand::Always`, and `holds_over_history` surfaces.
- The hostile history `[4]` produces the exact `FailureClass::PropertyDisagreement` result and the exact cause `("census", "state-remains-bounded")`.
- A claim-free contract refuses as `ContractRefusal::NoClaimDeclared`, proving the pass is not vacuous.
- The dependency is development-only, uses no default feature, and does not activate Loom or the preemption home.

## Scratch custody

- `Cargo.toml`: `83803C789609731705DA2411C2C9F9607A21B0FBBF50802B6EC8D2B17DD6793C`.
- `Cargo.lock`: `A7A4F1D5FD626A743603EF6B5B6DF0FEE0B13544338F2D77D8C10281870076CF`.
- `rust-toolchain.toml`: `3EBDB7479AB14F10F6724A8C1D34802EDE3A192D69E56403EF879F677E9F69F8`.
- `src/lib.rs`: `3D718EE73DA9CACD78827E88C3B0617525BD898B2CB58D27DF4E33F2B97FA7E3`.
- `tests/temporal.rs`: `8EBCC78FDE9CD754E5C311D1BB19A0174496E1E1302BE4B451FD9DB774715FBB`.
- The sealed scratch tree contained 353 files and 159,740,606 bytes before cleanup, including disposable Cargo build output.

## Stable qualification

- `cargo +1.98.0 fmt --all -- --check` passed.
- `cargo +1.98.0 check --all-targets --locked` passed.
- `cargo +1.98.0 clippy --all-targets --locked -- -D warnings` passed.
- `cargo +1.98.0 test --all-targets --locked` passed two of two semantic tests with no failure or ignored test.
- The coordinator read the authored specimen and lockfile and reran the complete declared wall independently after the owner agent completed.

## Disposition

- The temporal behavior is composition-complete on the published 0.1 surface.
- This specimen earns no new product home, public type, feature, dependency, crate, projection layer, or adapter.
- The bank's nested Cargo project is an input specimen, not authorized tracked repository architecture.
- Reduction and replay were outside this specimen's declared question and remain unproved here; that boundary is not evidence of a missing temporal-composition seam.
- This Windows x64 observation does not establish Linux, macOS, Wasm, generated-road, package, registry-delivered, performance, or human acceptance claims.

## Custody boundary

The source hashes above preserve the exact small specimen needed to audit the result.
The scratch project and its build products remain disposable and may be removed only after this receipt is committed and verified.
