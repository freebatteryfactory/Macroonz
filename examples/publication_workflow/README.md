# Publication workflow example

This executable prepares, inspects, checks, generates and recovers a caller-owned Rust constant through `macroonz::native_publication::bake`.
The example owns its kind, renderer, value, logical address and compilation fixture.
The root library owns the [command composition](../../src/native_publication/command/README.md); an adopter supplies its own generation operation without copying formatting, staging, installation or process supervision.
The [shared example adapter](../support/publication_configuration/README.md) also serves the [definition-and-sites example](../job_publication/README.md), while each caller keeps its own fixture and publication allowance.

Run `cargo run --example publication_workflow --features native-tooling` with a JSON object on stdin.
`{"action":"prepare","value":42}` returns the sealed inventory without native effects.
For `inspect`, add an existing absolute disposable `workspace` directory.
For `check` or `generate`, also supply an existing absolute `destination`, such as the adopter's dedicated generated-source directory beside its disposable target directory.
The workspace must be outside the destination tree.
For `recover`, only `action` and `destination` are needed; recovery uses retained historical installation intent and does not regenerate or claim a fresh compilation.

Formatting is optional: omit `rustfmt` or set it to null to preserve unformatted source.
To select formatting, supply the absolute `rustfmt` executable, an absolute existing TOML file as `format_configuration`, and `environment` as explicit key/value pairs suitable for that toolchain.
The formatter owner requires the supported rustfmt 1.9.0 stable profile and enforces LF output independently of configuration.
For `generate`, also supply the absolute `rustc` executable, its `target` triple and the explicit `environment` pairs; no environment is inherited by native requests.
The compiled fixture and binary live in the disposable workspace, while only `value.rs` and the library's ownership controls are installed in the destination.

For example, after creating the directories and selecting tool paths, an unformatted generation request has this shape:

```json
{
  "action": "generate",
  "value": 42,
  "workspace": "/absolute/adopter/target/publication",
  "destination": "/absolute/adopter/generated",
  "rustc": "/absolute/toolchain/bin/rustc",
  "target": "x86_64-unknown-linux-gnu",
  "environment": []
}
```

Use paths, target and declared linker/tool environment appropriate to the actual host.
The [compiler example](../compiler_workflow/README.md) describes the native toolchain prerequisite.
`check` returns a nonzero exit status for missing, stale, tampered or extra owned files and prints every reported discrepancy.
It performs no destination writes; optional formatter scratch writes remain in the separate workspace.
`generate` actually compiles before installation, preserves authored neighbors and refuses unowned collisions or tampered owned files.
Command results are printed through `macroonz::presentation::bake_output`; command errors use `bake_error` after the explicit cleanup retry.
The [publication presentation owner](../../src/presentation/native/publication/README.md) defines the common envelope and the distinct canonical and physical fields.
These projections can also produce Markdown or HTML through the same value's methods, as the [trial example](../trial_workflow/README.md) demonstrates.
Repeated identical declarations may reuse exactly compared staged source, but still invoke the compiler.

Input configuration is bounded to 64 KiB, generated output to one file and 64 KiB, and staged source to two files and 64 KiB.
Each selected process receives sixty seconds, five seconds for cleanup and 1 MiB per stream; the complete selection admits at most three processes, 195 seconds of declared process allowances and 6 MiB of retained streams.
These bounds do not claim an end-to-end deadline for caller Rust code or filesystem operations.
A native failure receives one explicit five-second cleanup retry and remains a command error; cleanup never silently resumes installation.
Destination ownership, interrupted recovery and concurrent-writer limits remain owned by the [destination contract](../../src/native_publication/destination/README.md).
