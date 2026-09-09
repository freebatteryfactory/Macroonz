# Contributing

Read [`AGENTS.md`](AGENTS.md) first.
It is the working law for every person, model, and agent who edits this repository.

## Scope

Change one semantic home at a time.
Read that home's entire owner packet before editing it.

## Local wall

Develop and run affected checks in the current checkout using its ordinary Cargo target directory.
Nested Cargo consumers reuse that target unless an independent observation requires cache isolation.
Read the affected owners, implement a coherent change, run focused behavior and compile-refusal controls, then run the complete wall before acceptance.
Do not run the full wall after every intermediate edit or recreate a workspace merely to run ordinary checks.
Product source and required tests must survive cargo clean; test subjects and generated configuration are recreated from tracked source or explicitly supplied inputs.
No retained campaign folder is a prerequisite for the local wall.

Run the complete wall from the repository root before you ask for acceptance.
On Windows with the MSVC target, use an x64 Visual Studio Developer PowerShell or Native Tools command prompt with the C++ build tools and Windows SDK available.
The linker and SDK environment are prerequisites of the declared target; no private campaign initialization script is required.
Every Cargo operation is locked to the declared dependency graph, and every compilation operation uses the pinned stable Rust 1.98 toolchain.
Set `CARGO_BUILD_JOBS=1` in the environment so nested Cargo processes inherit the compilation bound, and set `CARGO_INCREMENTAL=0` for qualification.

```sh
cargo +1.98.1 fmt --all -- --check
cargo +1.98.1 check -j1 --workspace --all-targets --all-features --locked
cargo +1.98.1 nextest run -j1 -p macroonz --no-default-features --test native_policy --locked --run-ignored only --no-tests fail -E 'test(=scoped_clippy_wall)'
cargo +1.98.1 nextest run -j1 --workspace --all-features --locked --no-fail-fast --no-tests fail
cargo +1.98.1 test -j1 --doc --workspace --all-features --locked
cargo deny --workspace --all-features check
cargo +1.98.1 doc -j1 --workspace --all-features --no-deps --locked
cargo +1.98.1 check -j1 --workspace --all-features --target wasm32-unknown-unknown --locked
cargo +1.98.1 run -j1 --example rustc_coverage --features harness --locked
```

Set `RUSTDOCFLAGS` to `-Dwarnings` for the documentation command so rustdoc warnings are part of the wall.
Dependency policy checks select all features so optional native dependencies are included.
The scoped Clippy target derives a disposable profile from the canonical `clippy.toml` by removing only its `std::time::Instant` entry.
It checks compiler, proc and harness packages with the strict profile, checks the ordinary root feature postures separately with that strict profile, and checks the opt-in native root posture with the derived profile over the same source checkout.
Independent compiler subjects must distinguish that permission from strict clock refusal and still-forbidden SystemTime and environment reads.
The permitted root pass is only one part of the wall; it cannot establish the strict packages or ordinary root postures by itself.
The boundary is package and feature scoped, so review must also establish that native readings remain inside their declared semantic owner.
Do not edit source between these selections or substitute a general lint suppression for the derived profile.

The commands above cover the ordinary wall; release qualification also runs the explicit feature postures, no-harness controls, and exact long-campaign commands owned by the [hosted workflow](.github/workflows/hosted-pulse.yml).
An ignored campaign does not count as executed because the ordinary test run passed, and an empty selection is a failure rather than a successful observation.
Keep the evidence and the unproven planes in the same report as the change.

## Hosted pulse

The manual [hosted qualification](.github/workflows/README.md) observes the committed wall on declared cloud hosts after the local wall is green.
It reports host and architecture evidence without replacing local enforcement or human acceptance.

## Git boundaries

An agent may stage and commit accepted work inside an authorized task after the required checks pass.
Humans authorize pushes, merges, branch rewrites, ref movement, and recovery operations.
Do not publish or move refs without that authorization.

## Security

Report vulnerabilities through the process in [`SECURITY.md`](SECURITY.md).
