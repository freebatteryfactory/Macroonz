# A declared job and independent checks

Run `cargo run --example job_workflow --no-default-features --features harness` from the repository root.
The binary checks the independent lifecycle and wire expectations, executes the generated trial table, then runs the declared benchmark and its undistinguished control.
It displays the authored declaration, all tokens emitted by the callable recipe compiler, the independent callable/vector/work/judge sources, and the complete executed trial and benchmark reports together.
The display reads the body of this example's single fixed recipe invocation from its compile-time source input; it does not maintain another declaration or parse recipe clauses.
The callable display explicitly declares `job-display.recipe` as its compiler door, so its generated provenance is a separate callable observation rather than a claim of byte identity with the proc host's expansion.
The reports come from the actually compiled recipe and support bindings and use the [complete public presentation owner](../../src/presentation/README.md).
The output includes the following result lines before returning success:

```text
all: Ok(EveryTrialConcluded { selected: 2, denominator: 2 })
codec: Ok(EveryTrialConcluded { selected: 1, denominator: 2 })
required-lane control: Err("required declared trials were not selected")
absent: Some(Unknown { position: 0 })
benchmark: Ok(())
same-work control: Some(PlantedWorseNotDistinguished)
```

The caller owns the job stages, event meanings, persisted record and independent checks.
One [recipe](../../macros/compiler/src/recipe/README.md) supplies dispatch, codec methods and the deferred trial declaration.
Its [trial bindings](../../macros/compiler/src/descriptor/trial/README.md#authored-grammar) name the separate check functions; the [root support invocation](../../macros/compiler/src/support/README.md#invocation) supplies the consuming target's facts.
The unit-input table needs no specimen decoder.
Selecting the codec suite uses [the names already admitted by that table](../../src/workflow/README.md#select-by-text), retains the unselected workflow row and refuses an unknown name before execution.
The caller's [complete-check policy](acceptance.rs) additionally requires every declared row to conclude; a successful codec-only run therefore cannot stand in for the whole declared table.
This policy reads the report's complete denominator rather than authoring another trial roster, and leaves the harness's legitimate subset verdict unchanged.
The generated suite and individual row tests also execute with `cargo test --example job_workflow --no-default-features --features harness -- --include-ignored`.

The completion and round-trip expectations are authored in `checks.rs`, independently of the declaration.
The literal stage/event population, transition outcomes and wire bytes in `ordinary.rs` supply separate expectations for requeueing completed work, cancelling queued work and retaining the attempt count.
Malformed, short, trailing and out-of-range record bytes must refuse; round-trip agreement alone does not establish those independent byte expectations.
Revision material includes the declaration and the check sources that the caller actually selected.
The target and compiler labels are explicit example inputs, not host detection, and measurement is unavailable.

The [benchmark caller](benchmark/README.md) counts actual outer dispatch calls and supplies independent literal work curves.
Its generated carrier removes direct construction of the benchmark row, attachment, binding and table; the caller still supplies the work, judge, preflight selection, invocation and report expectations.
The preflight selects the existing named completion binding through `trials::row::completion()`.
The same declaration is also consumed with identical measured and control functions, which must refuse before timing.
The declaring module uses `#[macro_use]` before those consuming modules so the generated macros remain in lexical scope within this binary.

The [Job compiler caller](../job_compiler/README.md) consumes this same declaration as a library from separately selected lawful and hostile Cargo binaries.
Its expected diagnostic code, source coordinates and lawful assertion remain independently authored caller inputs.
The [Job coverage caller](../job_coverage/README.md) observes actual record-decoder outcomes through an instrumented reader, preserving literal inputs and independent novelty/accounting expectations.
