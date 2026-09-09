# `trial` — declared rows become one stamped table

This home answers how one authored trial declaration becomes a harness trial-table payload without letting the author claim producer or consumption-target facts.

The payload rides inert in the carrier's stamped seat until a consumer's test target invokes the carrier.
The ordinary build therefore compiles the exported carrier and not the stamp grammar it holds.

## Authority in

An author names the carrier, the stamped table, the suite groups, and the semantic rows the harness will receive.
The [authored grammar](#authored-grammar) gives the clause forms, while public type operations own construction contracts.

The helper attribute is the caller's word rather than a spelling owned here.
A door supplies one [`Grammar`](crate::descriptor::Grammar) to the reading, so a refusal names what the author actually typed.

## Authority that stays outside

The producer's own act arrives through the caller-declared [`Emitter`](crate::descriptor::Emitter).
The rendering composes that authority into generated provenance and origin material, so an authored declaration cannot sign an act performed by the producer.

Revision commitments, callables, budgets, target and toolchain facts, and the clock belong to the consumption target.
They arrive as expressions at carrier invocation, where that target's hygiene reaches its own items.

## Suite and namespace ownership

The suite belongs to its group and every row under that group inherits it.
A row cannot state a second suite that disagrees with the aggregate seat selecting it.

Aggregate seats and row lenses become functions in one stamped module.
The complete payload closes that shared namespace before any token is rendered.

## One parse, two uses

A generated row expression parses its subject and check once, then gives those same informed values to both the row and its executable attachment.
The harness binding can therefore establish agreement over one pair of values rather than over two coincidentally equal parses.

## Composition and ceiling

The descriptor door walks the informed payload through the ordinary request road and places the rendered table in the carrier's stamped seat.
The generated-support gate checks the published schema pair before forwarding that seat to the harness stamp.

This home captures declaration meaning and renders destination-shaped tokens.
It does not judge a trial, derive host facts, prove a published schema is current, or execute the generated table.

## Authored grammar

```text
#[<helper>(
    support = <exported name>,
    module = <stamped module name>,
    table = named("<namespace>", "<stem>"),

    suite <seat> = named("<namespace>", "<stem>") {
        <lens> {
            claim = named("<namespace>", "<stem>"),
            roles = [named("<namespace>", "<stem>"), ...],
            tags = [named("<namespace>", "<stem>"), ...],
            subject = named("<namespace>", "<stem>"),
            check = named("<namespace>", "<stem>"),
            population = named("<namespace>", "<stem>"),
        },
    },
)]
```

The helper's own spelling is the caller's, which is why `<helper>` stands where a word would: a door registers the attribute it wants and hands the same [`Grammar`](crate::descriptor::Grammar) to this reading, so a refusal names the word an author actually wrote.

`roles` and `tags` are rosters and may be left out; the other four row clauses are required.
A row that classifies itself with nothing is a lawful row, and requiring an author to write `roles = []` would be requiring a sentence that says what silence already says.

### Clauses owned elsewhere

The producer's own act — the door, the producer's name, and the projection that emitted the rows — is composed inside the rendering from the emitter the caller declares.
An author who could state one would be signing an act these services performed.

The consumption target's host facts — the two revision commitments, the callable that reaches a row's conclusion, the declared budgets, the target and toolchain, and the clock — arrive as expressions at the carrier's own invocation, inside the test target that owns them.

Every one of those keys reaches [`CaptureCause::ClauseUndeclared`](crate::descriptor::CaptureCause::ClauseUndeclared).

### Ordering

Clause order inside a body is free and is read by key.
Order between ROSTER members is meaning and is preserved: the suites in the order they were written, the rows under each seat in the order they were written, and each row's roles and tags in the order they were written.

## Carrier arguments

After the [support bindings](../../support/README.md#invocation), the carrier takes these expressions in the written order:

```text
invocation: invocation_profile_expression,
target: target_binding_expression,
clock: harness_clock_expression,
<lens>_subject_revision: subject_revision_expression,
<lens>_check_revision: check_revision_expression,
<lens>_call: trial_callable_expression,
```

Repeat the three `<lens>` clauses for every declared row, in suite and row declaration order, replacing `<lens>` with that row's lens identifier.
The `invocation` expression supplies the report owner's `InvocationProfile`; the carrier supplies the invocation site when it constructs the runner invocation.
Revision expressions supply descriptor `RevisionBinding` values and the callable supplies the runner's `TrialCall`.
