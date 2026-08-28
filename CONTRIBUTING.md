# Contributing

Read [`AGENTS.md`](AGENTS.md) first.
It is the working law for every person, model, and agent who edits this repository.

## Scope

Change one semantic home at a time.
Read that home's entire owner packet before editing it.

## Local wall

Run the complete wall from the repository root before you ask for acceptance.
Every Cargo operation is locked to the declared dependency graph, and every compilation operation uses the pinned stable Rust 1.98 toolchain.

```sh
cargo +1.98.0 fmt --all -- --check
cargo +1.98.0 check -j1 --workspace --all-targets --all-features --locked
cargo +1.98.0 clippy -j1 --workspace --all-targets --all-features --locked -- -D warnings
cargo +1.98.0 nextest run -j1 --workspace --all-features --locked --no-fail-fast
cargo +1.98.0 test -j1 --doc --workspace --all-features --locked
cargo deny --workspace check
cargo +1.98.0 doc -j1 --workspace --all-features --no-deps --locked
cargo +1.98.0 check -j1 --workspace --all-features --target wasm32-unknown-unknown --locked
cargo +1.98.0 run -j1 --example rustc_coverage --features harness --locked
```

Set `RUSTDOCFLAGS` to `-Dwarnings` for the documentation command so rustdoc warnings are part of the wall.
Keep the evidence and the unproven planes in the same report as the change.

## Git boundaries

An agent may stage and commit accepted work inside an authorized task after the required checks pass.
Humans authorize pushes, merges, branch rewrites, ref movement, and recovery operations.
Do not publish or move refs without that authorization.

## Security

Report vulnerabilities through the process in [`SECURITY.md`](SECURITY.md).
