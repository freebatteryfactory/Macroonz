# Wave F composition correction

This Git-tracked receipt records the local stable-Rust correction of descriptor composition construction and diagnostic feedback and awaits owner acceptance and publication authority.

## Authority

- Implementation snapshot: `459548e43a8c92df3dca593ca90ea88c509a3b30` on `codex/macroonz-repository-completion`.
- Entering synchronized base: `afbebb3aad6ff5e8fe4f6ddee2095996c2abf548`.
- Host plane: local Microsoft Windows on `x86_64-pc-windows-msvc`.
- Toolchain: stable Rust 1.98.0 with Cargo 1.98.0.
- This pass performed no push, merge, ref rewrite, hosted-CI activation, publication, or registry mutation.

## Corrected contract

- `Composition::declared(Vec<Provider>)` remains the ergonomic dynamic-input road.
- `Composition::of_one(Provider)` is a total one-provider road with no caller-visible bounded-container ceremony.
- Every successful composition stores the existing private `NonEmpty<Provider, PROVIDER_LIMIT>` shape.
- Empty input refuses as `DeclarationError::Absent { seat: Seat::Provider }`.
- Input beyond `PROVIDER_LIMIT` refuses as `DeclarationError::Unbounded { seat: Seat::Provider, ... }`.
- Absence and magnitude are settled before the duplicate scan, so only an admitted bounded roster reaches pairwise work.
- Duplicate identities remain explicit `ProviderDoubled` issues and are reported once at their first occurrence.
- Composition issues retain distinct canonical shape and identity material and project through the existing `Refused` and `Diagnostic::refused` roads.
- The library performs no stderr or `compile_error!` side effect.
- `CompositionError` minting is private, and an outside compile-refusal fixture proves that callers cannot forge the declaration pass's answer.
- No crate, package, dependency, feature, builder subsystem, or output channel was added.

## Focused and mutation observations

- The external descriptor-content target passed nine of nine semantic tests.
- The compiler compile-refusal target passed every fixture, including the new private composition-mint crossing.
- `cargo-mutants 27.0.0` generated 41 mutants over the composition constructor, duplicate pass, canonical contract, and diagnostic projection.
- The initial campaign caught 17 mutants, classified 22 as unviable through the informed private types, and exposed two viable slot-constant survivors with no timeout.
- Exact public slot assertions were promoted into the owning external semantic target.
- Mutation iteration retested those two survivors and caught both, leaving no viable miss or timeout in the declared 41-mutant denominator.

## Cumulative stable wall

- Workspace check passed with all targets and all features under the locked graph.
- Strict workspace Clippy passed with all targets, all features, warnings denied, and no suppression.
- Nextest passed 422 of 422 executed tests across 60 binaries, with nine intentional skips reported by the runner.
- Formatting and diff checks passed.
- Cargo-deny passed advisories, bans, licenses, and sources.
- Warnings-denied workspace documentation built with all features and no dependencies.
- The all-feature `wasm32-unknown-unknown` workspace check passed.
- The changed source and tests contain no `unsafe`, `#[allow]`, or `#[expect]` marker.

## Custody and boundaries

- The two mutation survivors are retained as ordinary external regressions rather than raw campaign output.
- Raw mutant copies, build output, logs, and diffs were disposable beneath `target/qualification/wave-f-composition-mutants-20260828` and were removed after this receipt was written.
- The host blocked direct recursive removal before it ran; after exact resolved-path validation, a temporary cache tag admitted Cargo's own target cleanup, which removed 106 files and 2.4 MiB and left the exact directory absent.
- This local Windows observation is not hosted CI, hosted security, physical Linux, cloud Linux, macOS, ARM64, package publication, registry delivery, attestation, merge, or release acceptance.
