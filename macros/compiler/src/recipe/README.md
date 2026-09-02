# recipe — one informed structural account through every projection host

This home owns the compiler contract beneath `macroonz::recipe!` and the callable road used by a caller-owned projector.
It reads one inline Rust module, preserves its authored items, informs only the structure named by its final `bake!` declaration, and sends every selected projection through the existing request, plan, render, closure, explanation, and expansion owners.

## The declaration

The authored module remains ordinary Rust.
Its final `bake!` declaration names only the structural families Macroonz must account over and the projections it must answer.

The generic grammar admits zero or more authored enum vocabularies, named same-roster or cross-roster relations, caller-declared answers to structural questions, codec declarations delegated to the existing codec owner, requested projections, evidence declarations, and one support address where evidence cargo needs it.
An absent family is absent rather than represented by a placeholder, so an authored-item recipe need not invent a relation and a codec-only recipe need not invent an enum.
Clause families occur in that order when present because the reader settles references before consumers and refuses an unknown or repeated seat before rendering.

Vocabulary members are read from the authored enums themselves.
The caller does not restate a roster, and Macroonz does not reconstruct an enum as a parallel Rust model.
The current vocabulary ceiling is one unit-variant enum, and its generated declared-order companion uses Rust's own `_VARIANTS` vocabulary.
Record-shaped or field-bearing types remain ordinary authored structs and enter the existing codec owner when canonical bytes are requested.

A relation names its left and right vocabularies and contains zero or more caller-authored endpoint rows.
Every row in one relation is uniformly unlabeled, carries one ordinary Rust path, carries one exact Rust fragment, or belongs to the transition lowering.
Macroonz settles boundedness, endpoint membership, repetition, payload uniformity, and the structural questions the caller chose to answer without assigning meaning to either endpoint or payload.

Structural posture belongs to the relation account exactly once.
The caller may require empty or nonempty rows, allowed or refused repetition, open or closed membership, partial or total endpoint coverage, sparse or dense occupancy, allowed or refused absence, self-relations, and cycles where each question applies.
Macroonz computes those generic answers and enforces the caller's declared requirement; it does not decide which requirement a domain ought to have.

A codec declaration names one record-shaped struct authored in the recipe module and supplies the existing codec owner's direction, refusal type, assembly road, and member shapes.
The recipe home owns only the declaration bridge and projection selection.
Canonical encoding, decoding, member cardinality, assembly posture, and generated method contracts remain owned by `codec/`.

The `transitions(State, Event)` and `absence(refused)` clauses are ergonomic lowerings into a caller-named relation and its structural posture inside the same generic account.
A transition names one left member, one right member, one target member, and one ordinary Rust effect path, while rustc retains authority over that path, types, ownership, exhaustiveness, and the final generated program.

The optional `typestate(Vocabulary)` projection treats the selected authored members as caller-declared type-level stages.
It generates one marker per member, one structural `RecipeStage` trait carrying the caller-authored spelling, and one generic `Stage<Marker>` phantom carrier with conventional construction inside `baked::typestate`.
The projection assigns no runtime transition meaning to those types.
Newtypes, markers and phantom carriers are ordinary data-item compositions rather than separate compiler ontologies.

## Projection disclosure

A standard projection has one progressively disclosed seat rather than separate beginner and expert APIs.
The preset spelling names only the role, a parenthesized spelling carries flat mechanical configuration, and a braced spelling carries exact Rust material.

For dispatch, those levels are `dispatch;`, `dispatch(apply);`, and a braced semicolon-terminated function signature.
The exact signature preserves caller-authored attributes, visibility, qualifiers, name, generics, two simple caller-named parameter bindings and exact types, result, and where clause.
The standard projector generates only the body accounted from the informed transition rows.
A caller-authored function body is refused at that seat because an arbitrary body belongs to the caller-owned projector road.

A `relation_tables` projection selects one or more caller-named relations inside a single role-owned block.
An unlabeled row set accepts `relation;` for the borrowed `contains` preset or `relation(function_name);` for the same typed membership body under a configured name.
A payload-bearing row set requires `relation { exact_signature; };` because only the caller can state the payload type; the standard projector fills a complete `Some(payload)` or `None` body without invoking a path or interpreting exact Rust.
Each selected relation owns one public relation-named module inside `baked`, so several relations compose without inventing several projection roles.
The transition lowering remains owned by dispatch and refuses generic relation-table selection rather than exposing two bodies for one transition account.

Every effective projection reads back whether its value came from a preset, named configuration, or exact Rust.
The exact signature enters the recipe's canonical content, while producer-local spans remain outside semantic identity.

## Projection authority

Every possible recipe role receives one explicit standing before rendering.
Only generated roles enter the request's selected membership, and an unavailable requested evidence role refuses the recipe rather than producing a placeholder.

A projector receives a read-only recipe view, one already selected role, and a consuming sink bound to that role.
It can offer one `GeneratedTree` and cannot choose another role, destination, plan, identity, sidecar, or completion marker.
The existing output and closure owners retain admission, missing-output, doubled-output, delivery, and completion authority.

Built-in and caller-owned projectors use this same capability boundary.
The paved proc host executes only projectors shipped with Macroonz, while an arbitrary caller-owned algorithm runs through this callable compiler home or a caller-owned proc host.

## Evidence projections

An evidence projection carries caller-declared facts and exact Rust fragments through the existing descriptor adapter rather than teaching the recipe home a second harness vocabulary.
Trial-form material covers compile contracts, properties and temporal claims, generation and fuzz populations, fault and schedule claims, and package or publication challenges as ordinary harness rows.
Mutation material uses the existing mutation-surface declaration, benchmark material uses the existing benchmark declaration, and network or concurrency material uses the existing direct descriptor projections.

Every carrier keeps one explicit address, one parent expansion, one destination form, and one consumer invocation.
Test and benchmark cargo remain separate because the harness gates them under different forms, and an absent family retains an explicit unrequested or unavailable standing instead of acquiring placeholder output.

The compiler may render calls to public harness constructors and carry target-owned callables, but it never invokes them or decides what their result means.
An external target invokes the carrier and the harness owns the resulting judgment, report, corpus, reduction, replay, mutation, benchmark, and failure standing.

## Caller-owned projectors

A caller-owned projector implements `RecipeProjector` and is selected for one already requested role through `bake_with`.
It receives a copyable read-only `RecipeView`, a copyable `ProjectionRequest` carrying the role's effective mechanical configuration, and one consuming `ProjectionSink`.

The view exposes only informed recipe facts and exact captured fragments required to generate that role.
The request exposes the selected role, its role-owned destination, and its effective configuration.
The sink accepts one `GeneratedTree`; the returned `ProjectionOffered` cannot be constructed any other way.

No capability exposes the plan, closure, identity framing, output membership, sidecars, filesystem, or completion mint.
The role roster and mechanical lowering provenance are non-exhaustive because later standard projector families may add capabilities without turning exhaustive matching into the extension contract.

The shipped `custom_recipe_projector` example replaces the standard companions seat with a domain-neutral structural-dimensions projection using only `macroonz-compiler`.
An application cannot name downstream executable projector code inside `macroonz::recipe!`; arbitrary algorithms run in the callable compiler or a caller-owned proc host, not inside Macroonz's already compiled proc carrier.

## Output

Authored items stay in the recipe module.
Declaration-site companions and direct builder modules are assembled inside one generated `baked` child module, and no ordinary item is reexported automatically.
Names generated by one standard recipe are accounted within their Rust namespace before rendering, so a support address, direct evidence module, or conventional companion cannot silently double another name from that recipe.
Names outside the recipe input remain ordinary rustc resolution rather than an ambient scan.

An explicitly addressed support carrier is emitted beside the recipe module because Rust exports its macro at the crate root and its hidden helper must share that holder.
The address is caller-authored evidence material rather than an automatic reexport, and an unrequested carrier emits nothing.
Evidence cargo remains inert inside each existing support carrier until an external target invokes its explicitly named address.
The support home proves each cargo came from its descriptor expansion and keeps the schema gate, parentage, form, and destination crossing in their existing owners.

## Evidence ceiling

An accepted recipe expansion proves structural informing, selected-role accounting, projector authority, rendered closure, and carrier parentage.
It does not prove that a payload or effect path resolves, that generated Rust type-checks, that a carrier is invoked, or that a declared relation is correct for any domain.
Those claims belong to rustc and to independent harness or compile-contract crossings.
