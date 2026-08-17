# muterprater — the proof-pressure engine

Muterprater is a mutation interpreter and the brain over every adversarial
lane: it builds facts about what exists, plans which pressure is worth
running, runs it under declared budgets, explains every survivor, synthesizes
candidate descriptors, and promotes only the ones that earn it.

The promotion rule: no oracle, no promotion; no killed mutant or new proof
delta, no promotion; no report of the run, no trust in its result.

## The lanes

**mutation** — compiled mutation wraps `cargo-mutants`, retained for
high-assurance passes; its output is parsed into reports and every survivor is
explained against the declared properties. The artifact-mutation mode — damaging a
rendered artifact to prove the readers notice — is the harness rehearsing its
own alarm, and its mutation roster doubles as this lane's seed material.
Sequences are a mutation subject of their own: dropping, duplicating,
reordering, or delaying commands in a generated history must trip the
temporal laws — the cheapest event-order pressure there is. The
compile-once interpreter is the rapid loop this instrument is named for; it
opens on the evidence that the property suites kill mutants, not on a date.

**fuzz** — structure-aware generation over `arbitrary` with budgeted
minimization. A minimized find is promoted into a regression descriptor row
carrying its reproduction seed; the seed-packs under `corpus/` warm-start the
search and hold nothing the promotion road has not already made durable.

**chaos** — campaigns over the fault adapters: schedules, stacked faults, and
budgets. The adapters are the fault instrument's typed values; this lane only
orchestrates them. A schedule is a fault placed at a sequence position, so the
strongest claim in the harness is one campaign: the temporal laws hold under
the faults.

Mutation families expressed as rewrite descriptors join the plan once the
interpreter is the execution substrate that makes them cheap.
