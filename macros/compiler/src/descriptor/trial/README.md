# `trial` — declared rows become one stamped table

This home turns one authored trial declaration into a harness trial-table payload without inventing producer or consumption-target facts.

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
Revision and callable expressions may be co-located in a row's `binding` clause or supplied at carrier invocation.
They remain inert until invocation and resolve in the consuming target, where Rust checks their types.
Budgets, target/toolchain facts and the clock still arrive at carrier invocation.

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
    input = { <decoded Rust type template> },

    suite <seat> = named("<namespace>", "<stem>") {
        <lens> {
            claim = named("<namespace>", "<stem>"),
            roles = [named("<namespace>", "<stem>"), ...],
            tags = [named("<namespace>", "<stem>"), ...],
            subject = named("<namespace>", "<stem>"),
            check = named("<namespace>", "<stem>"),
            population = named("<namespace>", "<stem>"),
            binding = {
                subject_revision = { <Rust expression> },
                check_revision = { <Rust expression> },
                call = { <Rust expression> },
            },
        },
    },
)]
```

The helper's own spelling is the caller's, which is why `<helper>` stands where a word would: a door registers the attribute it wants and hands the same [`Grammar`](crate::descriptor::Grammar) to this reading, so a refusal names the word an author actually wrote.

`roles` and `tags` are optional rosters; claim, subject, check and population are required.
A row that classifies itself with nothing is a lawful row, and requiring an author to write `roles = []` would be requiring a sentence that says what silence already says.

`input` is optional and selects the decoded Rust type for the entire table.
It requires a nonempty brace-delimited type template, which may use `$consumer` for the consuming crate just as an attachment does.
When present, every callable receives `Invocation<BoundInput<DecodedType>>`, and the invocation supplies one `specimen` expression returning `Result<BoundInput<DecodedType>, InputRefusal>`.
The input owner admits the specimen and the stamp joins it through the runner's existing typed-input road once per suite or lens; no separate input registry or decoder is generated.
Omitting `input` preserves the unit-input entrance and accepts no `specimen` clause.

`binding` is optional and, when present, requires all three nonempty brace-delimited expression templates shown above.
The templates may use `$consumer` for the crate identifier supplied once at carrier invocation, so `$consumer::checks::law` can name a function absent from the declaring library.
Use this explicit binding for consumption-target paths; literal `crate` inside an exported macro names the invocation crate without declaring that dependency and fails strict macro-definition linting.
Paths must resolve inside the generated module; qualify a local import alias from the consuming crate root rather than relying on an enclosing module's imports.
An expression can reference a shared target-owned profile without restating that profile's values in every row.
These are caller-supplied expressions, not independently verified revisions or compiler-inferred oracles.
Missing, unknown and duplicate binding clauses refuse capture of the complete declaration.
After macro substitution, Rust syntax, visibility and type errors in the retained fragments are checked by the consuming compiler.

### Clauses owned elsewhere

The producer's own act — the door, the producer's name, and the projection that emitted the rows — is composed inside the rendering from the emitter the caller declares.
An author who could state one would be signing an act these services performed.

The declared budgets, target/toolchain and clock arrive at carrier invocation.
Revision and callable expressions belong only inside the optional `binding` clause or at carrier invocation; bare host-fact keys in a descriptor row still reach [`CaptureCause::ClauseUndeclared`](crate::descriptor::CaptureCause::ClauseUndeclared).

### Ordering

Clause order inside a body is free and is read by key.
Order between ROSTER members is meaning and is preserved: the suites in the order they were written, the rows under each seat in the order they were written, and each row's roles and tags in the order they were written.

## Carrier arguments

After the [support bindings](../../support/README.md#invocation), the carrier takes these clauses in the written order:

```text
consumer: crate_identifier,
invocation: invocation_profile_expression,
target: target_binding_expression,
clock: harness_clock_expression,
specimen: admitted_specimen_expression,
<lens>_subject_revision: subject_revision_expression,
<lens>_check_revision: check_revision_expression,
<lens>_call: trial_callable_expression,
```

Supply `consumer` exactly once when `input` is declared or any row has a co-located `binding`, and omit it otherwise.
It takes one identifier, normally `crate` or an explicitly imported crate alias; module segments remain in the template after `$consumer`.
Supply `specimen` only when `input` is declared.
Supply the three `<lens>` clauses only for rows without a co-located `binding`, in suite and row declaration order, replacing `<lens>` with that row's lens identifier.
Supplying a second binding for a co-located row does not match the carrier.
The `invocation` expression supplies the report owner's `InvocationProfile`; the carrier supplies the invocation site when it constructs the runner invocation.
Revision expressions supply descriptor `RevisionBinding` values and the callable supplies the runner's `TrialCall`.
