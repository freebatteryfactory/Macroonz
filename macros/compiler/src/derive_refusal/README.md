# `derive_refusal` — deriving a refusal family's declared facts

One refusal-family declaration in; the typed contracts the machine reads out. There is one road, and every step of it is bound before the next one runs.

## The callable surface

Nothing in this home knows a proc-macro exists. [`compile_refusal`] takes a typed [`CapturedInput`] and returns either a [`RefusalFamilyExpansion`] or a [`MacrocDiagnostic`]; everything downstream takes typed values. The Rust-facing shell is one caller of this function;

a test is another;

a future language frontend would be a third.

A diagnostic from here names [`crate::diagnostics::ReproductionRoute::CallableServices`] because that route is real, and [`compile_refusal_text`] is it.

## The one road to every emission

The membership-only object is [`RefusalDerivationDraft`], it has no render method, and there is no other public value in this home that carries a token tree. The steps below run in order and each one refuses on its own terms:

```text
capture → plan → render → close → explain → bind
```

Delete any one of them and no [`RefusalFamilyExpansion`] exists, so no emission is reachable. That is the property, and it is structural rather than reviewed.

Emission is not a step on this road, and that is the point. Splitting the rendered units across the deliveries their members declared, and joining each delivery's own stream, happens INSIDE `close`, which keeps every one of them and commits to each one's digest — so there is no act after the proof for a defect to live in, and no seam that could hand one build another build's bytes.

`bind` is the generic terminal every projection kind's door ends at — a [`ClosedExpansion`](crate::closure::ClosedExpansion) — and [`RefusalFamilyExpansion`] is this family's VIEW over it:

the identity, the plan, the proof, the explanation, and every emission are the terminal's and are read from it, and what this home adds is the captured surface and the cause-order disposition, the two facts a terminal standing for every kind does not carry.

The terminal refuses unless all three values belong to one expansion: the proof names the plan it was taken against and the explanation names the plan and the proof it was answered over, so a compilation that crossed two expansions' values is a typed refusal here rather than a well-formed account of the wrong thing.

## What each delivery receives

The declaration site receives the family implementation and, for a single-cause family, the typed cause-order implementation. A declared mutation helper produces one generated mutation module inside the support carrier; its production branch reads the declaration's actual typed order, its no-mutation branch delegates to production, and its active branch selects a separately rendered adjacent permutation.

The derive frontend constructs one complete `MutationProjectionRequest` before calling the mechanical mutation renderer. The module seam is review-enforced: the renderer consumes that request and does not import the derive frontend or rediscover owner meaning.

## The joined road: the vehicle is a second projection

Planning cargo into a carrier says where the tokens are compiled and nothing about how they get there. The vehicle is the generated support shell, and it is a PROJECTION — its own plan, its own rendering, its own proof, its own explanation, its own terminal — so [`compile_declaration`] walks the same six steps a second time for it, over the same captured surface.

```text
capture → plan → render → close → explain → bind      (the implementation)
                       ↓ the assembly
         plan → render → close → explain → bind        (the carrier)
```

What sits between the two is not a step of either: `generated_support` reads the implementation terminal's proved test-carrier cargo, verifies that the axes compose into one carrier — one root, one published expectation, every carried unit consumed once, no unit reaching a second destination — and hands back the assembly the shell is rendered from. The carrier's own composition road is crate-internal, so there is no way to an exported shell that skips it.

The one road from a plan and an assembly to a shell establishes the join neither road holds alone: this carrier plan and this assembly name ONE declaration. A plan for a second declaration would agree with every reading downstream, because the unit is born wearing that plan's own metadata — so the comparison happens at the public seam and refuses in the assembly family, and this door projects that body through the assembly family's own projection rather than restating it.

Reading a terminal's cargo into an axis is crate-internal for the matching reason: `carry.rs` reads the terminal's proved mutation-module tokens and the optional public support address from the mutation declaration that owns it, then binds both into one `EvaluationCargo`.

The trials axis carries what the DECLARATION states. A declaration may write its own trial rows beside its refusal declaration, in the `#[threadpak_trials(...)]` helper attribute this door declares and the carrier's own home reads — the claim, the suite, the roles, the tags, the subject, the check, and the population being the caller's words throughout, and none of the producer's act or the consumption target's host facts having a clause at all. A declaration that states none leaves the axis ABSENT under the disposition that says why, and what the door delivers is then a MUTATION-ONLY carrier:

an empty trials seat beside carried deferred cargo, which is exactly the delivery the carrier's grammar renders both seats for.

One declaration, three readings, and the third is why a trial edit is cheap. The semantic commitment sets both the prose and the trial attribute aside, so rewording a sentence moves the documentation name alone and editing a row moves the trial commitment alone — every implementation member keeps the name it had, while the carrier plan's complete content account changes because its dependency set changed.

Inside what [`compile_declaration`] hands back stand both terminals and the assembly. Their two declaration-site cargos are exactly the two terminals' declaration-site partitions — the implementations, and the shell definition — and an emitter writes both. [`compile_refusal`] is unchanged and its callers stand: the difference between the two roads is what is added, never a different first one.

## The whole roster answers, and none of it is a fake seat

Two kinds are produced here and the sealed roster names five, so the three that produce nothing say so. [`compile_declaration`] hands back an account carrying one typed disposition per kind: the implementation projection and the carrier name the output they were planned as, while codec, benchmark descriptor, and pattern stamp each carry this door's typed disposition.

The citation is what makes the answer readable without coming back here. Each unavailable road cites one door-owned fact whose declared stable name states the exact blocking conjunction for its kind:

- the byte role;

- the work currency;

- the authored pattern application and the publication posture.

Where several seats are independently blocked the name says all of them, because electing one as the primary blocker would tell a caller that closing it opens the road. The facts remain distinct because a byte role, a work currency, and a pattern application have different owners.

The remaining unavailable plans require owner facts a captured declaration and this compiler context do not carry. `account.rs` states each kind's answer, so a reader is told which fact could not be furnished rather than only that nothing was generated.

**No anchoring was invented to close any of them.** Several of those records carry one seat of exactly the shape the descriptor's obligation seat has — a SUBJECT the captured declaration could honestly stand in for, the way a descriptor stands over what it challenges.

Every one of those records carries a second seat that is not a subject at all, and no declaration stands for a byte role, a currency, or a pattern application under any posture.

A record that still cannot be filled is a record no plan is made from, so growing its subject seat would be machinery nothing pulls — and the posture would have been decided by a door instead of by the home that owns the seat. The verdicts are recorded at each kind's road in `account.rs`, where the ground travels with the answer.

## Refusal vocabulary, step by step

Each `map_err` on the road is a projection rather than a collapse.

A planning body reaches the caller naming its axis and magnitude, a closure body naming its role and the disagreement at it, a coverage body naming every seat, a rendering refusal naming the exact bound. See [`diagnose`].

## One grammar, one citation, one site

Three things about the compiler-facing half are structural rather than reviewed.

**One grammar.** Every line this home hands a compiler is composed by [`diagnose::composed`] — `<prefix>: <class>: <first>[<body>][<site>]` — including the capture family's, whose two projections read the same composition back.

The prefix is [`DIAGNOSTIC_PREFIX`], the class is a [`RefusalClass`] row, and every clause is projected from a typed value rather than restated in prose beside one.

**One citation.** Every owner fact this home cites is a [`RefusalDeriveFact`] row, which carries the home that declares it, the fact's declared stable name, and the repair that fact declares.

A citation and the sentence shown beside it are one row, so neither can be shown against the other;

and every capture cause names the fact it is a violation OF, so no repair points a caller at a rule unrelated to what was refused.

**One site.** A refusal established after capture names the offending token, and a text read that refused BEFORE any capture names the byte it was born at — see [`RefusalSite`].

Handle zero is never written to mean "somewhere": the second posture exists so that it does not have to be.

## Documentation is captured, and it is its own fact

A documentation comment reaches this grammar as `#[doc = "…"]`, one attribute per written line, and the capture reads that form on the family and on every variant into typed rows the surface carries — the text, the token it sits at, and the declaration it was written on.

So a documented public family uses the derive, which is what the lint wall asks of one.

**One captured surface, two authored facts.** The surface is named twice, and the two names are two READINGS rather than two accounts:

- the SEMANTIC commitment stands over the declaration's tokens with the documentation attributes dropped — what the family IS, unmoved by a reworded sentence;

- the DOCUMENTATION commitment stands over that name and the ordered rows — what the family SAYS, which moves exactly when the prose does.

The implementation, test, and codec roads take their entry accounts over the semantic commitment. The documentation commitment remains an inspectable capture reading and is not a projection kind in the current roster.

Nothing here reads what the prose means.

Every other attribute is exactly as unread as it was: the refusal-attribute search passes over what it does not name, and an unrecognized attribute on a variant refuses under the cause and at the site it always did.

## Nonclaims

This home decides no meaning. The three body shapes and the canonical key grammar belong to the public contracts package;

the selection order's *content* is the author's;

the local keys are the author's; the `RefusalFamily` and `CauseOrderDeclaration` contracts belong to the public contracts package.

This home reads a declaration and writes down what it already said.
