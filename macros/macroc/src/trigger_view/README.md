# `trigger_view` — which wrapper components a plan selected, and why

The wrapper-trigger view: which host-wrapper components a plan selected, which it
left out, and on whose declared fact each way.

## A derived summary, never a second truth table

The view reads decisions the machine's owners already made and reports them with
citations — "the suspension wrapper was selected because the execution posture
permits PEND". It answers no capability question of its own. If this view and an
owner disagree, the owner is right and the view is broken; there is no second
table here to consult.

## Absence is explained on the same footing as presence

A selection cites at least one owner fact and so does an omission. Both are
decisions somebody's declaration caused, and a component that appears in neither
list is not "off by default" — it is undecided, which
[`WrapperTriggerView::composed`] refuses.

## The two capabilities that are not modeled here

Benchmark intent and the host-conformance requirement have no owner declaration
to cite yet. They are deliberately absent from this view rather than modeled on a
guess: their owner declarations land with the qualification plane, under a named
owner. Sequencing the work is not deferring the architecture — a citation-free
trigger would be exactly the second truth table this view exists to refuse.

## The seats

`types.rs` declares. Its own child `type_guard.rs` holds every road that reaches
the view's private seats, which is what makes exhaustive disposition structural:
a view exists only where the disposition pass agreed, so there is no partial view
for a reader to mistake for a complete one. `type_contract.rs` states the refusal
family's declared shape. `establish.rs` is the per-component disposition pass and
the body the established issues amount to — the component roster is the
quantifier, so "every component was examined" is a fact about the loop rather
than a claim about it.
