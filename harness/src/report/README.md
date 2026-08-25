# report

What a run leaves behind.

A trial executes once and then it is gone.
What survives are the records this home defines, in two separately earned values: the report — which trial ran, under which revisions of the subject and the check, on which target, what it concluded — and, where a completed reduction earned one, a [`ReplayCapsule`] carrying the smallest reproducing input reached under the declared reducers and budget.
The two join on one execution key; an ordinary report does not carry the capsule, and never claims what only a reduction established.

Everything here is a value.
Nothing in this home runs a trial, reads a clock, or touches a file.
The runner hands it facts; it hands back records that can be compared, counted, and — through the capsule's own road — replayed.

## Two rails that never cross

A trial has two names, and they answer different questions.

`TrialId` is what the trial **means** — its claim, its subject, its check contract, its population, its profile.
Move the file, rename the function, reorganize the module: the identity does not change.

`TrialSite` is where the trial is **written** — module path, file, line, display name.
That is what a person filters on and jumps to, and it is deliberately not identity.

A report joins both.
A path-spelled name is a site; it is never an identity.

## What a key is made of

`ExecutionKey` names one execution rather than one trial: the trial identity, the subject revision, the check revision, the invocation profile, and the target and toolchain — the last unconditionally.
A cache hit across two targets asserts something nobody verified, and refusing it costs reruns.
Cost is a price.
A false claim is not.

`RowRevisionId` is the bookkeeping key beside it.
Editing a row's tags moves it, aggregation recomputes, and no execution is owed: nothing about what the row runs has changed.

## What a reproduction may claim

An attachment binds two revisions — one for the subject, one for the check — and every posture sentence reads over the weaker of the two.

Only a pair of harness-derived revisions can authorize a cache hit.
A declared or untracked half always executes again; its address still names the historical standing without claiming the harness derived it from canonical material.

- `ExactDerived` is the one posture that earns the phrase "replay exactly".
- `DeclaredByAuthor` inherits the ceiling of a hand-written declaration and says so.
- `UnavailableBecauseUntracked` states plainly that reproduction is not exact; the run and its input are still evidence, and no rendering pretends otherwise.

`ReplayCapsule` is the closed shape of a reproduction account, and it has exactly one mint: completed reduction evidence bound to a real refused report.
No caller assembles its seats by hand.

## The census

A `RunReport` is stated over a denominator — one entry per row of the table the run stood on, selected or not.
That is what makes claim coverage a computation instead of a hand count, and what makes a shrinking census a visible fact rather than a smaller number nobody noticed.

A run whose selection matched nothing is a complete report, not a missing one.
`SelectionExpectation::AtLeastOne` is what a caller gets without asking, because a run that exercised nothing is not a run that passed.
Admitting zero is a declaration made in advance, with a typed reason attached.

A report comparison has a population half and an execution-standing half.
The population half compares membership, authored-row revision, and denominator; the execution half compares shared trials across subject revision, check revision, and normalized outcome, then compares the runs across case, byte, and time budgets plus the exact target and toolchain pair.
An empty selection still exposes a budget or target move because those facts live on `RunReport`, not behind the first executed row somebody happened to find.

## Text from outside

A subject's panic payload, a decoder's message, an external tool's output: all of it rides `ForeignText`, bounded at `FOREIGN_TEXT_MAX_BYTES`, marked when it was cut and marked when rendering it loses bytes.
It travels one way.
Nothing here reads it back, matches on it, or decides from it — a finding is a typed value first and prose second.

## What this home does not do

It does not run anything; the runner owns execution and hands records here.
It does not judge; a `TrialConclusion` arrives already reached.
It does not persist; writing a report somewhere is the caller's.
It does not read a clock, a target triple, or a toolchain name — those are declared at the invocation and carried, never guessed.
And it never interprets a `FindingCause`: the family and the local key are the caller's own spelling, stored, hashed into every fingerprint, and handed back unread.
