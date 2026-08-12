# `derive_refusal` — deriving a refusal family's declared facts

One road, and it is the receipt-rich one.

## Ordinary callable Rust, and nothing else

Nothing in this home knows a proc-macro exists. [`compile_refusal`] takes a typed
[`CapturedInput`] and returns either a [`ClosedExpansion`] or a
[`MacrocDiagnostic`]; everything downstream takes typed values. The Rust-facing
shell is one caller of this function; a test is another; a future language
frontend would be a third. A diagnostic from here names
[`crate::diagnostics::ReproductionRoute::CallableServices`] because that route is
real, and [`compile_refusal_text`] is it.

## There is exactly one road to emitted tokens

There used to be two. A caller could capture a declaration, fix its membership,
and take a rendering straight off the draft — no plan, no identities, no origin
graph, no trace, no explanation, no closure. That road was shorter than the
receipt-rich one, which is another way of saying every receipt on the
receipt-rich road was optional.

It is closed. The membership-only object is [`RefusalDerivationDraft`], it has no
render method, and there is no other public value in this home that carries a
token tree. The steps below run in order and each one refuses on its own terms:

```text
capture → plan → render → close → explain → bind → emit
```

Delete any one of them and no [`ClosedExpansion`] exists, so nothing is emitted.
That is the property, and it is structural rather than reviewed.

The emit step is not a step on this road any more, and that is the point. Joining
the rendered units into the tree a compiler is handed happens INSIDE `close`,
which keeps the result and commits to its digest — so there is no act after the
proof for a defect to live in.

## Every step refuses in its own vocabulary, and it survives the crossing

Each `map_err` on the road is a projection rather than a collapse. A planning
body reaches the caller naming its axis and magnitude, a closure body naming its
role and the disagreement at it, a coverage body naming every seat, a rendering
refusal naming the exact bound. See [`diagnose`].

## What this home does not decide

It decides no meaning. The three body shapes are band 00's; the canonical key
grammar is band 00's; the selection order's *content* is the author's; the local
keys are the author's; the `RefusalFamily` and `CauseOrderDeclaration` contracts
are band 00's. This home reads a declaration and writes down what it already
said.
