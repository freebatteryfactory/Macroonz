# runner — a table of declared trials becomes a run

This is the harness's execution engine.

Hand it the complete authored world, one selection over that world, and one invocation, and it hands back a report.
Nothing here discovers, scans, prints, or exits.
A run is a function of the values a caller declared, and its answer is a value.

## Two roads, one assembler

There are two ways in, and they are not two engines.

- **In process.** `run_one` executes one binding; `run_all` executes a selection over a whole table view.
- **From a host.** `record_one` and `record_all` take observations some external runner made and turn them into the same reports.

A host observation carries only what a host can know: which trial, what the attempt did, and the wall reading.
Everything else is joined here — from the binding, the invocation, the table, and the selection — so a host cannot author evidence it never observed.

Both roads walk one assembler, so the two spellings of a run cannot disagree about what a run means.

## The table is the denominator

The table is always the complete world.
A selection chooses from it and never shrinks it.

Each report accounts on two axes, in this order:

1. the **selection disposition** — selected, or passed over with the reason it was passed over;
2. a **run attempt**, and only where the selection admitted one.

Reading the disposition first is what keeps a row nobody ran from ever being recorded as a row that failed.
A caller narrows a run; the denominator stays whatever the table says it is.

A row is data and cannot execute.
The callable rides on the binding beside it, which is why no hidden registry from row to function exists anywhere.

## Saying in advance that you might select nothing

A selection plan is a selection joined to what the run expects that selection to match.

`SelectionPlan::of` is the ordinary road and asks for nothing but the selection, because a run expects to exercise something unless somebody says otherwise.
`SelectionPlan::allowing_empty` is the one escape, and it states the reason in the same call.
A run that then selects nothing renders a zero-work result carrying that reason, and no reading anywhere calls it passed.

## What a seat reads

A stamped test function answers with a `Result`, and the two folds that produce it live here.

- `seat_verdict` reads a whole run report and answers with a `SeatOutcome`: every selected trial concluded, or no work as stated.
- `lens_verdict` reads one trial report and answers with nothing, because a lens has one binding, no selection, and no expectation to satisfy.

Both refuse with `SeatRefusal`, the one type a seat returns instead of passing.
A construction refusal enters it unchanged through `From`, and that is the only road in, so `?` is the whole ceremony at a seat.

Neither reading reads a word of prose.
A failure is described by carrying the record's own typed fields, so a refusal that reworded its message is still the same typed arm.

They live here rather than in the stamp on purpose: a fold copied into every expansion is one calculator standing in as many places as there are invocations, and two seats disagreeing about what a passing run means is the disagreement a harness exists to make impossible.

A selected trial that was skipped refuses a seat, cache-satisfied skips included.
The conclusion a cached execution stood in for is not in the report being read, and a seat may not pass on a verdict it never saw.

## Host facts arrive; they are never derived

The target triple and the toolchain identity come in as a `TargetBinding` the invoker states.
A triple assembled out of `cfg!` predicates would be a plausible spelling entering a cache key that nothing verified.

The clock arrives the same way.
`run_one` opens it before the subject and finishes it afterwards, then records the observed, unavailable, or failed reading without concluding anything from it.
An observed zero is a real measurement, not the spelling of no measurement.

## Panics

A subject panic is a verdict about the subject, so it is caught at the trial boundary and recorded as the finding it is.
That needs two mechanisms: an unwind catch, which returns the payload but not where the panic was raised, and a process-global panic hook, which sees the origin but cannot stop the unwind.
This home installs one hook, once, and chains whatever hook was standing before it — the one process-global effect the engine performs.

Aborts and stack overflows are not unwinds.
They end the process, no finding is produced, and nothing here pretends otherwise.
Process isolation is a hosting recipe, not machinery in this home.

## What this home is not

It runs no protocol: no argument vector, no output stream, no exit code.
It keeps no memory between runs, so comparing two reports is the report home's operation over what this one wrote.
It hosts nothing — listing, filtering, sharding, and parallelism belong to whatever test harness the caller runs the stamped seats under.
