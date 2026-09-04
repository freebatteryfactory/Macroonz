# Contributing

Read [`AGENTS.md`](AGENTS.md) first.
It is the working law for every person, model, and agent who edits this repository.

## Scope

Change one semantic home at a time.
Read that home's entire owner packet before editing it.

## Local wall

Run the complete wall from the repository root before you ask for acceptance.
Every Cargo operation is locked to the declared dependency graph, and every compilation operation uses the pinned stable Rust 1.98 toolchain.
Set `CARGO_BUILD_JOBS=1` in the environment so nested Cargo processes inherit the compilation bound, and set `CARGO_INCREMENTAL=0` for qualification.

```sh
cargo +1.98.1 fmt --all -- --check
cargo +1.98.1 check -j1 --workspace --all-targets --all-features --locked
cargo +1.98.1 clippy -j1 --workspace --all-targets --all-features --locked -- -D warnings
cargo +1.98.1 nextest run -j1 --workspace --all-features --locked --no-fail-fast --no-tests fail
cargo +1.98.1 test -j1 --doc --workspace --all-features --locked
cargo deny --workspace check
cargo +1.98.1 doc -j1 --workspace --all-features --no-deps --locked
cargo +1.98.1 check -j1 --workspace --all-features --target wasm32-unknown-unknown --locked
cargo +1.98.1 run -j1 --example rustc_coverage --features harness --locked
```

Set `RUSTDOCFLAGS` to `-Dwarnings` for the documentation command so rustdoc warnings are part of the wall.
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
