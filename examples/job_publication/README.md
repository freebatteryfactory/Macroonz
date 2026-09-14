# Shared definition and adopted values

This caller publishes one macro definition and two sites through `macroonz::native_publication::bake`.
Its [kind](type_contract.rs), [stamp](stamp.rs), values and logical address remain caller-owned.
The stamp declares its site roster once; the destination bindings read those admitted landings rather than repeating another roster.
The root [inventory](../../src/native_publication/inventory/README.md) joins the stamp to its sealed definition and requires a complete physical file set.

## Execute

Run `cargo run --example job_publication --no-default-features --features native-tooling --locked` with a JSON object on stdin.
`{"action":"prepare","values":[7,9]}` produces the canonical inventory without native effects.
The values must be exactly two integers representable as `u8`.
For `inspect`, add an existing absolute disposable `workspace` directory.
For `check` or `generate`, add an existing absolute `destination` directory outside that workspace.
Generation also requires an absolute `rustc` executable, its `target` triple and explicit `environment` key/value pairs suitable for that compiler and linker.

An unformatted generation request has this shape:

```json
{
  "action": "generate",
  "values": [7, 9],
  "workspace": "/absolute/adopter/target/publication",
  "destination": "/absolute/adopter/generated",
  "rustc": "/absolute/toolchain/bin/rustc",
  "target": "x86_64-unknown-linux-gnu",
  "environment": []
}
```

Choose paths, target and explicit tool environment appropriate to the host, as described by the [compiler example](../compiler_workflow/README.md).
The generated set is `definition.rs`, `first.rs` and `second.rs`; the authored [compilation fixture](fixture.rs) is staged privately and is never installed implicitly.
Its independent runtime assertions expect seven and nine, regardless of the requested values.
Generation compiles before installation; successful compilation alone does not establish those runtime assertions.
The returned JSON exposes the actual compiler executable under `record.value.compiler.executable.shown`, which can be run separately to execute the assertions.

Change only `action` to `check` to compare installed output without destination writes.
Checking the installed seven/nine set against `values: [7,10]` reports stale `second.rs` and returns a nonzero exit status.
Changing `first.rs` to `crate::declared_value!(8);` reports both tampered and stale material for that file.
The two findings retain its disagreement with the recorded installation and with the newly expected output.
Regeneration refuses to overwrite that damage; restore the intended source under caller authority before retrying.
The [outside control](../../tests/native_policy/publication/job_example.rs) additionally recompiles the actual installed files and observes the unchanged seven-value assertion fail on eight.
It never substitutes the earlier staged binary for that tampered-source execution.

## Configuration and custody

The [shared example adapter](../support/publication_configuration/README.md) supplies the same native fields and bounded command composition as the single-value publication example.
The caller declares one publication allowance used by generation and command preparation: three files and 64 KiB; staging adds the one authored fixture under that byte ceiling.
The native process family permits five invocations, 325 seconds of summed execution/cleanup allowances and 10 MiB of aggregate retained streams.
These are resource allowances, not an end-to-end filesystem or caller-code deadline.

Formatting is optional: supply an absolute `rustfmt`, absolute `format_configuration` file and explicit `environment` pairs to select the owning formatter operation.
Omitting rustfmt preserves unformatted output; canonical token commitments remain separate from physical bytes.
`recover` requires only `action` and `destination`; the [destination owner](../../src/native_publication/destination/README.md) resumes historical installation intent without regeneration or fresh compilation claims.
Authored neighbors remain outside the ownership roster.
Workspace material is disposable; the selected destination remains caller-owned source and is not a build-cache cleanup target.
