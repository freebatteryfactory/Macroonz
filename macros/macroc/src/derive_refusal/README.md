# `derive_refusal` — deriving a refusal family's declared facts

One refusal-family declaration in; the typed contracts the machine reads out.
There is one road, and it is the receipt-rich one.

## The callable surface

Nothing in this home knows a proc-macro exists.
[`compile_refusal`] takes a typed [`CapturedInput`] and returns either a
[`ClosedExpansion`] or a [`MacrocDiagnostic`]; everything downstream takes typed
values.
The Rust-facing shell is one caller of this function; a test is another; a
future language frontend would be a third.
A diagnostic from here names
[`crate::diagnostics::ReproductionRoute::CallableServices`] because that route is
real, and [`compile_refusal_text`] is it.

## The one road to emitted tokens

The membership-only object is [`RefusalDerivationDraft`], it has no render
method, and there is no other public value in this home that carries a token
tree.
The steps below run in order and each one refuses on its own terms:

```text
capture → plan → render → close → explain → bind
```

Delete any one of them and no [`ClosedExpansion`] exists, so nothing is emitted.
That is the property, and it is structural rather than reviewed.

Emission is not a step on this road, and that is the point.
Joining the rendered units into the tree a compiler is handed happens INSIDE
`close`, which keeps the result and commits to its digest — so there is no act
after the proof for a defect to live in.

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

## Nonclaims

This home decides no meaning. The three body shapes are band 00's; the canonical
key grammar is band 00's; the selection order's *content* is the author's; the
local keys are the author's; the `RefusalFamily` and `CauseOrderDeclaration`
contracts are band 00's. This home reads a declaration and writes down what it
already said.
