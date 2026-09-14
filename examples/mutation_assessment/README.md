# Independent mutation assessment

This example compiles and executes a nonzero predicate and its explicitly permitted comparison mutation, then applies two independently authored checks to the same observed results.
For the positive sample `7`, the original answers `1` and the mutation answers `0`.
The weak check accepts either well-formed result and misses the damage; the stronger check requires acceptance of the positive input and catches it.
Changing the check executes neither the evaluator nor the compiled subject again.
The stronger check then demonstrates its rejection in a staged table, yielding an unadmitted proposal whose saved selected result can be judged in a fresh process.

The caller declares its claim, policy, mutation, source materializer and checks in ordinary Rust.
The [interpretation owner](../../harness/src/muterprater/interpretation/README.md#scoped-observation-and-assessment) supplies observation, correspondence qualification and witness assessment.
The [native compiler](../../src/native_compiler/README.md#compiled-read-back) supplies actual compilation and bounded read-back, with explicit host settings carried beside the immutable numeric sample.
This is the handwritten mutation entrance; generated structural source uses the [descriptor producer](../../macros/compiler/src/descriptor/mutation/README.md) and still owes actual execution.

## Run

Run from an initialized developer shell with the selected compiler and linker available.
Create separate existing absolute directories for disposable compiled subjects and retained records.
The source directory must have no `baseline` or `selected` child, and the storage directory must have no `item-assessment` batch.
This PowerShell example explicitly passes the selected environment entries into the library:

```powershell
$work = Join-Path $PWD 'target/mutation-assessment-subjects'
$records = Join-Path $PWD 'target/mutation-assessment-records'
New-Item -ItemType Directory -Path $work,$records -ErrorAction Stop | Out-Null
$compiler = (rustup which --toolchain 1.98.1 rustc).Trim()
$target = (& $compiler -vV | Select-String '^host: ').ToString().Substring(6)
$entries = @('PATH','SystemRoot','WINDIR','TEMP','TMP','INCLUDE','LIB','LIBPATH') | ForEach-Object {
    $value = [Environment]::GetEnvironmentVariable($_)
    if ($null -ne $value) { ,@($_,$value) }
}
@{
    action = 'assess'
    rustc = $compiler
    directory = $work
    target = $target
    toolchain = 'rustc 1.98.1'
    environment = @($entries)
    storage = $records
    sample = 7
} | ConvertTo-Json -Depth 4 -Compress | cargo run --example mutation_assessment --no-default-features --features native-tooling --locked
```

The first two output lines are complete historical JSON presentations, weak then strong, including source bytes, input `00000007`, five meanings and five actual judgments each.
The third is the historical proposal, retaining its candidate, complete staged table, actual rejection and replay capsule.
The final line is:

```text
weak=Survived strong=Killed equivalence=Refuted; evaluations=2 compiled-executions=2 admissions=0
```

Inspect those stored assessments in a fresh process with only their storage location:

```powershell
@{ action = 'inspect'; storage = $records } | ConvertTo-Json -Compress | cargo run --example mutation_assessment --no-default-features --features native-tooling --locked
```

The same three historical presentations precede:

```text
loaded historical assessments and proposal; evaluations=0 compiled-executions=0 witnesses=0 admissions=0
```

The [assessment archive](../../harness/src/muterprater/interpretation/archive/README.md#complete-mutation-assessment) and [proposal archive](../../harness/src/muterprater/proposal/archive/README.md) own their historical consistency, and [native storage](../../src/native_storage/README.md) owns the bounded three-artifact transaction.
Choose storage outside `target` when records must survive `cargo clean`.

## Replay the proposed check

The proposed check is ordinary caller code already authored in `checks.rs`.
The [proposal owner](../../harness/src/muterprater/proposal/README.md) synthesizes its candidate descriptor and proves it in a staged view over the complete authored parent, without changing that parent or admitting a row.
The captured input explicitly encodes the selected source address, original numeric sample and actual selected result as forty bytes.
Its caller-owned decoder consumes that complete convention through the existing input binding.
One bounded reduction probe challenges a smaller encoding; this example retains the original forty-byte witness and claims no global minimality.

Replay supplies current target/toolchain declarations and selects the current checker independently of the archive:

```powershell
@{ action = 'replay'; storage = $records; target = $target; toolchain = 'rustc 1.98.1' } | ConvertTo-Json -Compress | cargo run --example mutation_assessment --no-default-features --features native-tooling --locked
```

The current trial and historical comparison are emitted as JSON, followed by:

```text
DefectReproduced; decodes=1 checks=1 compiled-executions=0 admissions=0
```

Change only the current checker to the weak well-formedness rule:

```powershell
@{ action = 'replay-weakened'; storage = $records; target = $target; toolchain = 'rustc 1.98.1' } | ConvertTo-Json -Compress | cargo run --example mutation_assessment --no-default-features --features native-tooling --locked
```

That current trial passes, but the comparison records a moved check and unchanged subject:

```text
NotReproduced(PassedWithoutRepairStanding); decodes=1 checks=1 compiled-executions=0 admissions=0
```

This replay judges saved selected output; it does not rerun the original subject or demonstrate a repair to it.
The [saved-witness runner](../../harness/src/runner/README.md#saved-witness-replay) and [comparison owner](../../harness/src/report/replay/README.md) own those separate execution and outcome contracts.
Inspection and replay need neither the compiler setting nor the disposable subject directory.

## Refusals and boundaries

A zero sample refuses the positive-input scenario; missing tools, compiler failure, interrupted read-back, malformed output and unfinished cleanup cannot become a mutation verdict.
Use a new explicit source directory for another execution; exclusive creation preserves existing source and artifacts.
Use a fresh storage directory for another assessment batch; a collision preserves the existing batch, and storage failure returns a failing command.
Inspection and replay refuse corrupt records and require an absolute existing storage root.
These operations never recover or delete a source directory automatically.

Compilation uses [version-one process limits](../../src/configuration/v1/README.md); each reader receives ten seconds, five seconds for cleanup, 64 stdout bytes and 64 KiB stderr bytes.
The command reports pending cleanup as failure and releases custody without claiming cleanup completed.
The native process owner defines its operating-system enforcement limits.

The target and toolchain strings are caller declarations; the example does not authenticate a compiler or protect source and artifacts against concurrent replacement.
The source files and sample passed to the actual compiler and reader are explicit, and outside controls execute the binaries on a second input to challenge canned answers.
Road comparisons declare the shared predicate and numeric encoding; they do not claim independent source derivation.
The archived input convention preserves the immutable numeric sample, excluding host configuration and observation counters, and grants no input-aware execution key or arbitrary interior-state snapshot.
Historical inspection performs no replay, compilation, witness judgment or human admission.
The captured source address is a historical claim; fresh replay does not authenticate its producer or reestablish compilation or mutation activation.
Current check and decoder revision labels are explicit caller declarations, not claims of a complete source digest.
No action invokes human admission or changes an authored table.
Each ordinary unit attachment executes the in-process predicate on its authored positive sample; mutation assessment calls the separately bound meaning check over each retained result.
