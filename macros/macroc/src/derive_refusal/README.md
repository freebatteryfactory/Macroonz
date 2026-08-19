# `derive_refusal` — deriving a refusal family's declared facts

One refusal-family declaration in; the typed contracts the machine reads out.
There is one road, and every step of it is bound before the next one runs.

## The callable surface

Nothing in this home knows a proc-macro exists.
[`compile_refusal`] takes a typed [`CapturedInput`] and returns either a
[`RefusalFamilyExpansion`] or a [`MacrocDiagnostic`]; everything downstream takes
typed values.
The Rust-facing shell is one caller of this function; a test is another; a
future language frontend would be a third.
A diagnostic from here names
[`crate::diagnostics::ReproductionRoute::CallableServices`] because that route is
real, and [`compile_refusal_text`] is it.

## The one road to every emission

The membership-only object is [`RefusalDerivationDraft`], it has no render
method, and there is no other public value in this home that carries a token
tree.
The steps below run in order and each one refuses on its own terms:

```text
capture → plan → render → close → explain → bind
```

Delete any one of them and no [`RefusalFamilyExpansion`] exists, so no emission
is reachable.
That is the property, and it is structural rather than reviewed.

Emission is not a step on this road, and that is the point.
Splitting the rendered units across the deliveries their members declared, and
joining each delivery's own stream, happens INSIDE `close`, which keeps every
one of them and commits to each one's digest — so there is no act after the
proof for a defect to live in, and no seam that could hand one build another
build's bytes.

`bind` is the generic terminal every projection kind's door ends at — a
[`ClosedExpansion`](crate::closure::ClosedExpansion) — and
[`RefusalFamilyExpansion`] is this family's VIEW over it: the identity, the plan,
the proof, the explanation, and every emission are the terminal's and are read
from it, and what this home adds is the captured surface and the cause-order
disposition, the two facts a terminal standing for every kind does not carry.

The terminal refuses unless all three values belong to one expansion: the proof
names the plan it was taken against and the explanation names the plan and the
proof it was answered over, so a compilation that crossed two expansions' values
is a typed refusal here rather than a well-formed account of the wrong thing.

## What each delivery receives

The declaration site receives the production implementations and nothing else.
The mutation-evaluation copies are planned into the TEST CARRIER, ride the
generated support shell as deferred cargo, and stand over the shell's own
private subject rather than over the type the declaration named — because a copy
of an implementation, rendered for the same type, is that implementation
declared twice beside itself, and a foreign trait implemented for a foreign type
once it reaches a consumer's test target.

The relocation is established rather than assumed: the renderer walks the tree
it produced and refuses a body that observes `Self` or names the declared type,
so a copy whose meaning would move with its subject is a typed refusal.

## Refusal vocabulary, step by step

Each `map_err` on the road is a projection rather than a collapse. A planning
body reaches the caller naming its axis and magnitude, a closure body naming its
role and the disagreement at it, a coverage body naming every seat, a rendering
refusal naming the exact bound. See [`diagnose`].

## One grammar, one citation, one site

Three things about the compiler-facing half are structural rather than reviewed.

**One grammar.** Every line this home hands a compiler is composed by
[`diagnose::composed`] — `<prefix>: <class>: <first>[<body>][<site>]` — including
the capture family's, whose two projections read the same composition back.
The prefix is [`DIAGNOSTIC_PREFIX`], the class is a [`RefusalClass`] row, and
every clause is projected from a typed value rather than restated in prose beside
one.

**One citation.** Every owner fact this home cites is a [`RefusalDeriveFact`]
row, which carries the home that declares it, the fact's declared stable name,
and the repair that fact declares. A citation and the sentence shown beside it
are one row, so neither can be shown against the other; and every capture cause
names the fact it is a violation OF, so no repair points a caller at a rule
unrelated to what was refused.

**One site.** A refusal established after capture names the offending token, and
a text read that refused BEFORE any capture names the byte it was born at — see
[`RefusalSite`]. Handle zero is never written to mean "somewhere": the second
posture exists so that it does not have to be.

## Documentation is captured, and it is its own fact

A documentation comment reaches this grammar as `#[doc = "…"]`, one attribute
per written line, and the capture reads that form on the family and on every
variant into typed rows the surface carries — the text, the token it sits at,
and the declaration it was written on. So a documented public family uses the
derive, which is what the lint wall asks of one.

**One captured surface, two authored facts.** The surface is named twice, and the
two names are two READINGS rather than two accounts:

- the SEMANTIC commitment stands over the declaration's tokens with the
  documentation attributes dropped — what the family IS, unmoved by a reworded
  sentence;
- the DOCUMENTATION commitment stands over that name and the ordered rows — what
  the family SAYS, which moves exactly when the prose does.

Which one an account carries is decided by what the projection is ABOUT. An
implementation, test, or codec projection takes its one entry account over the
semantic commitment, standing on nothing. A DOCUMENTATION projection takes its
account over the documentation commitment and DECLARES the semantic commitment as
its dependency, because what the prose says stands on what the declaration is.
That is one account over one commitment naming the one thing it stands on — the
same shape every account has — so no second account of content dependencies forms
anywhere.

Nothing here reads what the prose MEANS. `document.rs` wires the rows as far as
the documentation home's own grammar admits — the family seat's line becomes the
one plain sentence an item opens with, carried unchanged — and stops at the
election that has no lawful answer: which FACET a sentence covers is meaning, and
this derive reads a declaration under a compiler profile that reads tokens. That
stop is a typed disposition naming the profile and its version, not a silence.

Every other attribute is exactly as unread as it was: the refusal-attribute
search passes over what it does not name, and an unrecognized attribute on a
variant refuses under the cause and at the site it always did.

## Nonclaims

This home decides no meaning. The three body shapes are band 00's; the canonical
key grammar is band 00's; the selection order's *content* is the author's; the
local keys are the author's; the `RefusalFamily` and `CauseOrderDeclaration`
contracts are band 00's. This home reads a declaration and writes down what it
already said.
