# Job work-count retention

This caller accumulates ordered work increments in the [shared Job record](../job_workflow/declaration.rs) and checks the independently stated bound that its count stays at most three.
The [typed trial declaration](declaration.rs) keeps the claim, population and executable binding together through the root recipe entrance.
The [check](checks.rs) owns the bound; neither the accumulator nor Macroonz chooses the expected answer.

## Execute

Run `cargo run --example job_retained --no-default-features --features native-tooling --locked` with a JSON object on stdin containing `action`, an existing absolute `storage` directory and a portable `batch` name.
The supported actions are `retain`, `inspect` and `replay`.
For example, run these commands from the repository root:

```powershell
$storage = Join-Path (Get-Location) 'target/retained-job-example'
New-Item -ItemType Directory -Path $storage -Force | Out-Null
@{ action = 'retain'; storage = $storage; batch = 'work' } | ConvertTo-Json -Compress | cargo run --example job_retained --no-default-features --features native-tooling --locked
@{ action = 'inspect'; storage = $storage; batch = 'work' } | ConvertTo-Json -Compress | cargo run --example job_retained --no-default-features --features native-tooling --locked
@{ action = 'replay'; storage = $storage; batch = 'work' } | ConvertTo-Json -Compress | cargo run --example job_retained --no-default-features --features native-tooling --locked
```

Retention checks that `[1, 2]` passes and refuses reduction as a nonfailing baseline, while `[1, 2, 4]` fails and reaches `[4]` under a budget of 32 candidate probes.
The lawful report refuses probe binding with `TrialPassed`; substituting those lawful bytes under the refused report instead yields `BaselineCaseDiffers` before any probe executes.
The original input, complete report and earned replay capsule cross the [existing workflow and storage owners](../../src/workflow/README.md#retain-and-load) in one batch.
This is the smallest witness reached by the declared search, without a global minimality promise.
Successful retention leaves the subject's refused conclusion visible in the emitted input-run JSON.

Inspection prints the complete historical stored-run JSON without decoding or executing a trial.
Replay independently selects the current generated binding, executes `[4]` once with the same cause, and prints the existing comparison JSON.
It then separately verifies that `[1]` passes; the final line distinguishes the witness's one decode and one check from that additional lawful execution.
Actual reduction-probe calls remain zero throughout the fresh replay process.
These records remain unauthenticated historical claims and current local observations, without human admission.

The shared bounded stdin reader and [versioned resource limits](../../src/configuration/v1/README.md) supply mechanical configuration.
Invocation permits one case, 64 payload bytes and the explicitly declared time budget; measurement is unavailable.
Target and toolchain spellings in `checks.rs` are explicit caller claims, not host discovery.

## Refusals and custody

Loading requires the original input convention and a capsule for the independently selected semantic trial, whose identity includes the population.
The caller's [admission](admission.rs) deliberately refuses a different trial before replay; the general replay owner remains capable of reporting coordinate movement for other callers.
Damaged or truncated input, report and capsule envelopes refuse through their owning readers.
Repeating retention with an existing batch refuses instead of replacing it.
The [outside controls](../../tests/native_policy/workflow/job_retained.rs) execute the public commands in separate processes, damage every retained member and independently change only the expected population using a second vocabulary.

All subject and check source lives in this example and the shared Job declaration.
Only native output belongs in the demonstration target directory; use caller-owned storage outside target when retained records must survive `cargo clean`.
