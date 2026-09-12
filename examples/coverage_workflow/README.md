# Native coverage example

This example instruments a supplied Rust binary, establishes matching LLVM readiness, executes declared candidates, retains coverage-earned seeds through the existing corpus owner, and replays those seeds in fresh target processes.
Coverage remains a search signal; the example supplies no semantic oracle.

Run `cargo run --example coverage_workflow --features native-tooling` with a JSON object on stdin.
The `rustc`, `directory`, `source`, `artifact`, `target` and `environment` fields have the explicit meanings documented by the [compiler example](../compiler_workflow/README.md).
Also supply an absolute disposable `scratch` directory and `candidates`, an array containing one through eight arrays of unsigned bytes.
Place the supplied [coverage subject](../support/rustc_coverage_subject.rs) at the declared source path before running.
For that subject, `[[0], [1, 2, 3], [0]]` produces interesting, interesting and known admissions, followed by replay of the retained seeds.

Configuration is bounded to 64 KiB.
Compiler and LLVM operations receive 60 seconds and 1 MiB per output stream; target operations receive two seconds and 64 KiB per stream.
Every process receives five seconds for cleanup.
The campaign permits sixteen attempts, 8 KiB of candidate input, 1 MiB per export, ten thousand covered points, eight retained seeds and 4 KiB of retained input.
Replaying retained seeds spends the same attempt and input budgets.
The in-memory seed pack is not a filesystem storage demonstration.
An unfinished native cleanup is reported as failure, and dropping that failure does not claim cleanup completed.
For Cargo instrumentation, multiple source roots and cleanup retries, follow the [native coverage owner](../../src/native_coverage/README.md).
