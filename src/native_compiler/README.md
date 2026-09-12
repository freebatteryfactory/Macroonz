# Native compiler fixtures

This home executes real Rust compiler fixtures and extracts observations for the existing [compiled oracle](../../harness/src/oracle/compiled/README.md).
Enable `native-tooling`, supply an explicit `native_process::ProcessTool`, admit a `CompilerRequest`, and call `compile`.
The harness retains diagnostic and member comparison meaning; this home owns invocation, structured extraction and observation provenance.

## Declared compilation

The rustc entrance builds one Rust 2024 binary with an explicit source, target triple and output path.
It requests JSON diagnostics and artifact notifications and requires the selected link artifact before reporting successful compilation.
The Cargo entrance builds one selected package library or binary with an explicit manifest, target triple and target directory, using locked, offline resolution and one compilation job.
The caller supplies the lockfile and all required compiler, linker and environment configuration through the process owner.
Cargo's offline resolution is not a network sandbox for build scripts or compiler plugins.
Every effective command remains available through the request and result.
`CompilerRequest::instrumented` explicitly selects Rust source coverage without changing diagnostic or artifact interpretation.
For Cargo it supplies `CARGO_ENCODED_RUSTFLAGS` for the target and its Rust dependencies, refusing caller `RUSTFLAGS` or `CARGO_ENCODED_RUSTFLAGS` entries in any letter case instead of silently replacing them.
This encoded selection takes precedence over Cargo-configured Rust flags; project configuration and compiler wrappers remain caller trust inputs.

The process working directory is the declared physical root for relative diagnostic paths.
For Cargo it must match the workspace root used as rustc's path basis, even when the selected manifest is below that root.
The independent diagnostic locus is a canonical `RelativeSourcePath` below that root.
Primary coordinates retain rustc's code, source, lines and columns; diagnostic selection never consults the expected error code.
Paths outside the root, ambiguous paths, missing codes and multiple relevant diagnostics or primary spans refuse observation establishment.
Expansion context does not silently replace a primary span with a different call-site span.

## Observation standing

Both compiler pipes use the [native process contract](../native_process/README.md).
Timeout, truncated output, observation failure and unfinished cleanup cannot become an expected compiler refusal.
`CompilerRun::Pending` retains the exact request and process custody and can be retried with an explicit cleanup budget.
A finished process retains its raw exit and captured prefixes even when structured interpretation fails.

Rustc diagnostics are read from stderr and Cargo messages from stdout using their documented JSON protocols.
The JSON reader rejects malformed values, duplicate fields and trailing material, retains the standard parser's depth bound and tolerates unknown protocol fields and message kinds.
Non-JSON lines on the selected diagnostic stream refuse interpretation, including unrelated plugin chatter.
Cargo messages are selected by manifest and target; the final build status must agree with the process exit and successful builds must identify the selected artifact.
`cargo_fresh` preserves Cargo's artifact reuse report: `true` establishes a successful Cargo check of existing artifacts, not a newly executed rustc invocation.
The selected executable and project configuration are caller trust inputs; these observations do not authenticate a tool binary, source tree or artifact against concurrent replacement.

## Compiled read-back

For a successful binary, `CompilerOutput::read_back` accepts only that build's reported executable and runs it under a separately explicit process request.
Arguments, stdin, environment, output bounds and deadlines for the reader remain caller inputs.
After native cleanup finishes, `compared_read_back` decodes the actual successful reader's stdout through a caller-owned function and delegates the resulting `ObservedMember` roster to the existing comparator.
The caller owns value encoding and independent expectations; no downstream type or universal read-back wire format is invented here.
Duplicate observed members remain visible to the comparator.
A nonzero reader exit, incomplete output or decoder refusal produces no semantic verdict.
