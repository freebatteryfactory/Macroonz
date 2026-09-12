# Real compiler and read-back example

This example builds a supplied Rust binary through the optional root compiler host, executes the reported artifact through the same process owner, and compares its count with an independently supplied expectation.
`fixture.rs` supplies a small typed computation; the count expectation remains an input rather than being derived from that source.

Run `cargo run --example compiler_workflow --features native-tooling` with a JSON object on stdin containing these fields:

| Field | Declared input |
| --- | --- |
| `rustc` | Absolute path to the selected rustc executable. |
| `directory` | Absolute source root and working directory. |
| `source` | Canonical source path relative to that root, such as `fixture.rs`. |
| `artifact` | Absolute binary output path with an existing parent directory. |
| `target` | The selected target triple, executable on the reader's host. |
| `environment` | Explicit array of `[key, value]` pairs needed by the compiler, linker and reader. |
| `expected_count` | Independently expected unsigned count; the supplied fixture is expected to produce 42. |

The example accepts at most 64 KiB of configuration, gives compilation 60 seconds and each pipe 1 MiB, and gives read-back 10 seconds and each pipe 64 KiB.
Both operations receive a five-second cleanup budget.
The library preserves unfinished cleanup ownership; this command reports it as failure and releases the value without claiming cleanup completed.
An expected count of 43 exercises the independent disagreement through the same actual build and reader.
For Cargo fixture selection, exact diagnostic refusals and native capability limits, follow the [compiler owner](../../src/native_compiler/README.md).
