# Retained input and fresh witness execution

This example declares one typed trial through the root recipe and support entrances, runs an independent check, reduces its failure, retains the original input and reached witness, and loads or replays them in a separately invoked process.
The caller's lossy counter overlooks bytes equal to one; the independent oracle uses slice length.
The corrected implementation advances once per byte and keeps the same oracle.
Neither implementation nor the oracle is generated from the other's answer.

## Run

Run each request from the repository root with `cargo run --example retained_workflow --no-default-features --features native-tooling --locked`, supplying a JSON object on stdin.
The fields are `action`, an existing absolute `storage` directory and a fresh portable `batch` name.
The shared example reader bounds configuration to 64 KiB.
The [version-one resource limits](../../src/configuration/v1/README.md) bound input, archives and storage; invocation permits one case and 64 payload bytes, and reduction permits 32 candidate probes.
Target and toolchain labels are explicit illustrative caller claims, and measurement is unavailable.

For example, these PowerShell commands create disposable demonstration storage and retain one failure:

```powershell
$storage = Join-Path (Get-Location) 'target/retained-count-example'
New-Item -ItemType Directory -Path $storage -Force | Out-Null
@{ action = 'retain'; storage = $storage; batch = 'count' } | ConvertTo-Json -Compress | cargo run --example retained_workflow --no-default-features --features native-tooling --locked
```

The command first checks a lawful input, then observes the defect on `[7, 1, 9]` and reaches `[1]` through the existing reducer.
It prints the complete original input-run JSON and ends with `retained original [7, 1, 9]; reached witness [1]`.
This is the smallest witness reached by this declared search, not a global minimality claim.
The successful command reports successful retention of an observed failure; its report still says that the subject check refused.

Each following command starts another process:

```powershell
@{ action = 'inspect'; storage = $storage; batch = 'count' } | ConvertTo-Json -Compress | cargo run --example retained_workflow --no-default-features --features native-tooling --locked
@{ action = 'replay'; storage = $storage; batch = 'count' } | ConvertTo-Json -Compress | cargo run --example retained_workflow --no-default-features --features native-tooling --locked
@{ action = 'replay-fixed'; storage = $storage; batch = 'count' } | ConvertTo-Json -Compress | cargo run --example retained_workflow --no-default-features --features native-tooling --locked
```

| Action | Record and final line |
| --- | --- |
| `inspect` | Complete historical stored-run JSON, then `loaded historical records; decodes=0 checks=0 probes=0`. |
| `replay` | Existing replay-comparison JSON, then `DefectReproduced; decodes=1 checks=1 probes=0`. |
| `replay-fixed` | Existing replay-comparison JSON, then `FixedOnWitness; decodes=1 checks=1 probes=0`. |

The counters observe actual decoder, checker and reduction-probe calls within that process.
The replay actions select their current callable and revision from authored code, while loaded records supply only the saved witness and historical comparison coordinates.
The corrected result means that the current implementation passes this witness; it does not prove correctness for every input.
The original input remains `[7, 1, 9]`, and replay executes `[1]` without repeating reduction.

## Refusals and custody

Repeating `retain` with the same batch refuses without replacing the saved input; choose a fresh batch to retain another run.
Relative storage paths, malformed batch names such as `../escape`, and unknown actions refuse with the responsible input named in the error.
A missing or damaged retained batch refuses loading; preserve the original complete batch and use its original input convention instead of editing archive bytes.
Storage failure remains separate from the subject's refused trial.

The [root workflow](../../src/workflow/README.md) owns execution, retention joins and replay composition, and the [storage owner](../../src/native_storage/README.md) owns physical custody and recovery.
This example neither discovers a host environment nor installs a second runner, archive, reducer or process supervisor.
The historical record is unauthenticated and cannot choose current code or confer human admission.
The demonstration directory is Cargo output and may be cleaned; choose storage outside build output when the retained records must survive `cargo clean`.

The [outside-process control](../../tests/native_policy/workflow/example.rs) executes all actions, observes original and reached bytes separately, checks current revision movement, and supplies collision, path and damaged-input cases.
