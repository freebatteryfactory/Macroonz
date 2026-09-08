# runner — declared trials become evidence

The runner is the harness's execution engine.
It receives the complete authored world, a declared selection over that world, and one invocation, then returns a report without discovering, scanning, printing, exiting, or retaining run state.

```mermaid
flowchart LR
    accTitle: Runner assembly roads
    accDescr: The authored world and selection admit either in-process execution or a host observation, and both roads join invocation facts in one shared complete report assembler.

    world[(Complete authored world)]
    selection{{Declared selection}}
    invocation[/Invocation facts/]

    subgraph runner[runner]
        direction TB
        admission{Selected?}
        execute[In-process execution]
        record[Host observation admission]
        assemble[Shared report assembler]
    end

    report[(Complete run report)]

    world --> admission
    selection --> admission
    invocation --> execute
    invocation --> record
    admission -->|yes| execute
    admission -->|host ran it| record
    admission -->|no, keep reason| assemble
    execute --> assemble
    record --> assemble
    assemble --> report

    classDef authority fill:#1f2937,color:#f9fafb,stroke:#111827,stroke-width:2px;
    classDef choice fill:#fef3c7,color:#78350f,stroke:#f59e0b,stroke-width:2px;
    classDef operation fill:#dbeafe,color:#1e3a8a,stroke:#3b82f6,stroke-width:2px;
    classDef evidence fill:#dcfce7,color:#14532d,stroke:#22c55e,stroke-width:2px;
    class world,invocation authority;
    class selection,admission choice;
    class execute,record,assemble operation;
    class report evidence;
```

## One meaning, two admission roads

An in-process call and an external host observation are two ways to establish the attempt axis, not two report engines.
Both roads enter one assembler, which derives the standing a host cannot author from the binding, invocation, table, and selection.

A host may state only the semantic trial it ran, what became of the attempt, and the wall reading it observed.
The join refuses records that are duplicated, outside the table, outside the selection, or absent for a selected trial.
The host-record road accepts unit-input invocations only; those records establish no individual specimen join.

## Typed input

`Invocation::declared` creates the explicit unit-input road, which retains its cloneable surface.
`with_input` consumes a decoder-admitted `BoundInput<T>` and creates `Invocation<BoundInput<T>>` for the same execution and accounting engine.
The callable reads the actual value through `invocation.input().value()`.
There is no constructor pairing an arbitrary value with unrelated bytes, and cloning a caller's value cannot mint another admitted invocation.

The input's profile, case identity and decoder revision travel into the report's execution standing.
The decoder posture participates in the complete replay and cache ceiling.
Input values remain caller-owned semantics; a shared reference does not establish purity or exclude interior mutability.

Each selected trial receives the same admitted specimen once.
Before calling it, the runner requires room for one case and the specimen's actual payload bytes in that trial's invocation budgets.
An exceeded bound records a budget skip with no clock reading and cannot pass either verdict fold.
Envelope admission limits remain the input owner's separate resource bounds.
The unit-input road leaves case and byte consumption to its check, and no elapsed measurement is interpreted as a timeout.

## Saved-witness replay

`replay` receives a historical capsule and independently supplied current trial, decoder, invocation and input admission limits.
It packs the saved reached bytes under the current decoder's declared profile, decodes them once with complete consumption required, and calls the existing `run_one` road once.
An input refusal produces no trial report, while an execution-budget refusal retains the existing skipped report before clock or subject execution.
Admitted execution retains the ordinary panic, conclusion and measurement behavior of `run_one`.
The decoder remains caller-owned code under the [input contract](../input/README.md), including its effects and termination.

`ReplayedTrial` retains the historical envelope address, the admitted witness, the complete current report and the [report owner's comparison](../report/replay/README.md).
Historical source claims do not choose the current binding, and changing that binding never erases the report it actually earned.
This operation performs no reduction, mints no live replay capsule and establishes no human admission.

`replay_legacy` admits a sparse historical record's present witness through the same packing, decoding and execution operation.
Missing and null witnesses refuse separately before current decoding, while an empty witness remains eligible for the caller's decoder.
`LegacyReplayedTrial` retains the exact source address, admitted witness, complete current report and sparse-claim comparison.
Matching source claims do not supply the missing historical preimages or earn a reproduced-defect or fixed-on-witness claim.

## The complete table is the denominator

A selection chooses from the authored world and never shrinks it.
Every report accounts for every row, recording either a selected attempt or the reason that row was passed over.

```mermaid
flowchart TD
    accTitle: Complete table accounting
    accDescr: Every authored row occupies one census seat containing either its selected attempt or the declared reason it was not selected.

    row[One authored row]
    selected{Selection admits it?}
    attempt[Record one run attempt]
    passed[Record why it was not selected]
    census[(One census seat)]

    row --> selected
    selected -->|yes| attempt --> census
    selected -->|no| passed --> census
```

The selection disposition is established before execution, so a row nobody ran cannot become an attempt that failed.
A row remains data; its capture-free callable rides beside it in the binding, and no hidden registry maps rows to functions.

## Empty work is declared in advance

A selection plan states both what to choose and whether choosing nothing is an admitted result.
The ordinary posture expects at least one row.
The explicit zero-work posture carries its closed reason into the report, and no verdict reads that result as a passing trial.

## Verdicts fold typed records

The aggregate-seat fold reads the selection outcome and every selected report.
The single-lens fold reads one trial report.
Both carry typed failure facts out of the record rather than interpreting prose, and a cache-satisfied skip still refuses because the conclusion it stands in for is absent from the report being judged.

## Host facts remain declared facts

The invocation carries the target, toolchain, budgets, site, and clock.
The engine derives none of them from ambient process state and does not let elapsed-time availability change a check's conclusion.

## The panic boundary

A subject panic that unwinds is recorded as a typed subject finding.
The unwind catcher retains a safely readable payload while one process-global hook observes the origin, chains the hook that preceded it, and correlates observations per thread.

The hook is installed once.
A later process-wide replacement can remove origin capture, so the payload remains evidence while the origin becomes unavailable.

An abort or stack overflow does not unwind and therefore cannot produce a trial finding in process.
Process isolation may establish that ceiling, but hosting the process is outside this home.

## What this home does not own

The runner owns neither a command protocol nor hosting policy.
Argument parsing, output streams, exit codes, listing, filtering, sharding, and process supervision belong to the caller's host.
Comparing reports belongs to the report home, over the records this home produced.
