# Generated structural mutations

This example turns declared codec members or recipe rows into complete unchanged and alternative Rust sources, then uses the existing [compiler workflow](../compiler_workflow/README.md) to execute an independently authored check.
The generator uses the callable compiler through the root facade.
The [mutation owner](../../macros/compiler/src/descriptor/mutation/README.md) owns the structural completion and source-selection limits.

Build the two public callers:

```sh
cargo build --example structural_mutation --example compiler_workflow --no-default-features --features native-tooling --locked
```

Run the generator with a JSON object on stdin:

```sh
cargo run --quiet --example structural_mutation --no-default-features --features harness --locked
```

```json
{"family":"codec","selection":"unchanged"}
```

The output is a complete Rust binary source, including the same separately authored observer for every alternative of that family.
Save stdout as UTF-8 `subject.rs` in an explicitly chosen working directory.
An unknown family or selection refuses before source output; the configuration reader admits at most 64 KiB.

| Family | Unchanged selection | Alternative indices | What the independent observer requires |
| --- | --- | --- | --- |
| `codec` | `unchanged` | `0` | Exact bytes for the ordered values one and two, and matching decode. |
| `transition` | `unchanged` | `0`, `1` | Start reaches Busy with one work unit; the other row and absent transition retain their expected behavior. |
| `effect` | `unchanged` | `0` | The same independently stated state and work expectations. |

For example, `{"family":"effect","selection":"0"}` substitutes the complete effect already authored on the sibling row.
No replacement effect or expected answer is inferred.
The observer reports one when all its demands hold and zero when they disagree.

Run the existing compiler caller with the following fields on stdin, replacing the paths, target and environment with the explicit values for your host:

```sh
cargo run --quiet --example compiler_workflow --no-default-features --features native-tooling --locked
```

```json
{
  "rustc": "/absolute/path/to/rustc",
  "directory": "/absolute/work",
  "source": "subject.rs",
  "artifact": "/absolute/work/subject-binary",
  "target": "your-executable-target-triple",
  "environment": [],
  "expected_count": 1
}
```

On Windows the artifact needs its `.exe` suffix and the explicit environment must include the compiler/linker requirements described by the compiler workflow.
The unchanged source prints `compiled count agrees with the independent expectation` through that workflow.
Each listed alternative compiles but makes the unchanged observer return zero, so the same expected count of one produces a non-success exit naming `MemberValue` for `count`.
Keep the expected count at one when challenging an alternative; changing it to zero would accept the disagreement as the expected behavior.

Generation alone establishes neither compilation nor behavioral damage.
This paired workflow executes these exact alternatives and their authored checks; it does not mint harness activation, equivalence, survivor qualification or human admission.
The [mutation assessment example](../mutation_assessment/README.md) owns the separate public execution-qualification, witness-assessment and unadmitted-proposal journey.
Other caller declarations can produce alternatives that fail compilation or agree with a given check; the producer does not predict those outcomes.
