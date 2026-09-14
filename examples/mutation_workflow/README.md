# Native mutation example

This example runs an explicitly selected mutation backend against a supplied Cargo project, retains its complete manifest through bounded native storage, reloads the historical record, and compares its source claims with today's files.
The project owns its behavior and independently authored tests.

Run `cargo run --example mutation_workflow --features native-tooling` with a JSON object on stdin.
Omitting `action` selects `run`; explicitly selecting `"action":"run"` does the same.
Supply absolute `backend`, `cargo` and `rustc` executable paths, the absolute project `directory`, an array of relative literal `sources`, the selected target triple as `target`, an absolute disposable `target_directory`, and an absolute new `output` directory whose parent already exists.
The backend is cargo-mutants 27.0.0 and rustc is the supported 1.98.1 compiler.
The example passes the backend's leading `mutants` argument itself.

Also supply an existing absolute `storage` directory, a fresh portable storage `batch` name, and `environment` as an explicit array of key/value pairs suitable for the selected toolchain.
No environment variables are inherited by the native requests.
The project must already contain a lockfile and have its dependencies available offline.
The [native mutation owner](../../src/native_mutation/README.md) owns exact command construction, tool binding and effect limits.

Configuration is bounded to 64 KiB and source capture to 1 MiB.
Each native process receives 180 seconds, five seconds for cleanup and 1 MiB per output stream.
The retained archive is bounded to 4 MiB, with 1 MiB per framed field, 1024 reports, 128 arguments, sixteen source files and 1024 unread lines.
Existing output or storage batch names refuse; the example does not remove prior results.

The final line states the number of reports, caught and inconclusive readings, and historical source claims matching current files.
This is an execution/retention demonstration, not an all-mutants-caught gate: a complete observation containing missed mutants still prints its truthful result.
Missed mutants do not establish semantic survivors because this backend's console does not observe activation.
Version, baseline, capture, source, storage and current-comparison failures return an error.
An unfinished cleanup remains a reported failure rather than a completed cleanup claim.

## Compare retained evidence with current source

Invoke the same command in a fresh process with only `action: "compare"`, the existing absolute project `directory`, existing absolute `storage` and retained `batch`.
This action needs no backend, compiler, environment, output or build-cache settings, and launches no tool.
It loads the complete bounded historical manifest and delegates current-file comparison to [the native custody owner](../../src/native_mutation/README.md#observation-and-retention).
Matching source prints the number of historical source claims that match, followed by `no backend executed`.
Changing a recorded source file returns a failing command with the moved-file cause; corrupting the stored manifest refuses before source comparison.
Both operations are read-only and preserve the archived record and caller source.
Successful comparison establishes current equality with the manifest's declared source roster, not a fresh mutation run, activation or human admission.
