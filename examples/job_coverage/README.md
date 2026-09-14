# Coverage of the Job record reader

This caller instruments the [record reader](../job_compiler/fixtures/coverage-record.rs) while using the existing [Job declaration](../job_workflow/declaration.rs) as its uninstrumented library dependency.
Its three literal inputs are one short byte, the sixteen-byte encoding of count 513 and attempts 7, and the same short byte again.
Independent checks require interesting, interesting and known admissions, followed by refusal of an extra attempt.
Every reported point must name the record-reader fixture, and the final corpus must account for three attempts, eighteen input bytes, two retained inputs and seventeen retained bytes.

Prepare the consumer manifest and fixtures using the [Job compiler instructions](../job_compiler/README.md).
Build this caller with `cargo build --example job_coverage --no-default-features --features native-tooling --locked --offline`.
Invoke its executable with the same explicit Cargo configuration, adding `rustc`, `declaration` and `scratch`:

```json
{
  "cargo": "/absolute/toolchain/bin/cargo",
  "rustc": "/absolute/toolchain/bin/rustc",
  "directory": "/absolute/consumer",
  "manifest": "/absolute/consumer/Cargo.toml",
  "target_directory": "/absolute/Macroonz/target",
  "package": "neutral-job-adopter",
  "target": "x86_64-pc-windows-msvc",
  "declaration": "/absolute/Macroonz/examples/job_workflow/declaration.rs",
  "scratch": "/absolute/Macroonz/target/qualification/job-coverage",
  "environment": [["RUSTC", "/absolute/toolchain/bin/rustc"]]
}
```

Supply every compiler, linker and platform environment entry required by the selected host, using the pinned compiler with matching LLVM tools.
The declaration path must select the same Job source as the consumer manifest; it joins the actual fixture and lockfile bytes in the caller-declared revision material.
The compiler's [target-only instrumentation](../../src/native_compiler/README.md#declared-compilation) preserves the fixture-only observation question without removing unexpected source points afterward.
Compiler and LLVM operations select the existing [version-one process limits](../../src/configuration/v1/README.md), which remain inspectable through their admitted tools.
Each reader has two seconds for execution, five seconds for cleanup and 64-KiB stream bounds.
The coverage campaign separately bounds export bytes to four MiB, canonical points to 4096, attempts to three, input bytes to eighteen, retained cases to two and retained bytes to seventeen.

Success prints the existing JSON readiness and corpus presentations, including exact queried tools, source mapping, every retained point and both retained inputs.
No platform-independent point count is expected.
Coverage supplies a search signal; the Job workflow's independently authored byte and lifecycle checks remain its semantic oracles.
This source consumer establishes neither packaged adoption nor coverage of every generated branch.
