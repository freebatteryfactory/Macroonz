# The Job compiler contract

This caller builds two explicit Cargo binaries against the [Job declaration](../job_workflow/declaration.rs).
The lawful binary must compile and execute its independent Draft/Queue assertion.
The wrong-state binary must refuse with `E0308` at `fixtures/wrong-state.rs`, line 3, columns 26 through 29 exclusive.
The caller also checks that an independently wrong `E0277`, a shifted span of columns 27 through 30, and a refusal declared for the accepted lawful twin each disagree.

Create a disposable consumer directory, copy the `fixtures` directory into it, and copy `consumer.toml` to `Cargo.toml` there.
Replace `/absolute/path/to/Macroonz` in that manifest with the absolute checkout path, using forward slashes.
The library target reads the existing declaration directly; it is not a second implementation.
Run `cargo generate-lockfile --offline --manifest-path /absolute/consumer/Cargo.toml` with the pinned toolchain after its dependencies have been fetched.
Build this caller with `cargo build --example job_compiler --no-default-features --features native-tooling --locked --offline`.
Invoke the resulting `job_compiler` executable with one JSON object on stdin:

```json
{
  "cargo": "/absolute/toolchain/bin/cargo",
  "directory": "/absolute/consumer",
  "manifest": "/absolute/consumer/Cargo.toml",
  "target_directory": "/absolute/Macroonz/target",
  "package": "neutral-job-adopter",
  "target": "x86_64-pc-windows-msvc",
  "environment": [["RUSTC", "/absolute/toolchain/bin/rustc"]]
}
```

All paths must be absolute except the fixture loci fixed in this example.
The working directory is the consumer directory and therefore the source-coordinate root.
The supplied environment must contain every host input Cargo and the selected linker require, including the selected Cargo cache and platform tool paths where applicable.
No environment is inherited by the native process entrance.
On Windows, include the executable suffix and use JSON-escaped backslashes or forward slashes in paths.
The target triple is an explicit configuration choice, not inferred by this caller.

The existing [native compiler owner](../../src/native_compiler/README.md) executes locked offline Cargo builds, selects structured diagnostics without consulting expected answers, and authorizes execution of the reported lawful artifact.
Builds select the existing [version-one process limits](../../src/configuration/v1/README.md), which remain inspectable through the admitted Cargo tool.
The lawful reader has a ten-second execution budget, a five-second cleanup budget and 64-KiB bounds on each stream.
Unfinished cleanup, missing observations and failed execution return errors.
Successful execution prints:

```text
lawful: compiled and executed
wrong-state: E0308 at fixtures/wrong-state.rs:3:26..29
controls: wrong code, shifted span and lawful twin disagree
```

This is a path consumer of the current source; it does not establish packaged adoption.
The fixtures and expected outcomes are caller semantics, while process supervision and diagnostic extraction remain existing library owners.
The same consumer manifest also supplies the [Job coverage reader](../job_coverage/README.md), which selects only its own binary for instrumentation.
