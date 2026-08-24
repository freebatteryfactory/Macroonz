# `bench` — the kind that carries measured rows into a bench target

Two units out of one declaration: the bench table, and the one file a consumer swaps to change measurement backends.

They are two seats rather than one, because they are two independent things. The table is cargo the carrier's gate forwards; the adapter is an item beside it. A rendering that produced one and not the other is caught by the seat it left empty rather than by a count that happened to be wrong.

## The rows are the harness's field roster, mirrored as data

A row carries the workload identity, the input-size axis, the correctness preflight, the planted-worse falsifier, the declared budgets, the contention posture, the work formula where one is declared, and the neutral complexity-claim reference. [`Attachment`] carries what makes a row measurable, because a bench row is pure data and cannot measure.

Four of those seats are structural statements rather than shapes of convenience.

**The axis is a curve.** An axis of fewer than two distinct sizes is refused at the door: a growth class is read off a curve and never off a point, and a row measured at one size is a number rather than a class.

**The posture is stated always.** A measurement under an undeclared contention posture is inadmissible, and the field is what enforces it. Every row states the seat, so "unstated" is not a value and never becomes one — which holds whatever the roster's width is.

**The tolerances are spec, not vibes.** [`Budgets`] is a record of three named seats rather than a roster of counts, because the schema's roster is positional and a positional roster can be declared short. A table whose second budget silently became its third is a gate judging against the wrong tolerance. [`BUDGET_ORDER`] is the stated mapping between the named seats and the positions, written in exactly the order the rendering emits them.

**The host order is carried by the shape.** [`Attachment`] requires the measured callable, the planted-worse falsifier, and the preflight together, so a row that would be benchmarked without either gate is unwritable rather than refused.

## The adapter, and its one swap point

[`Backend`] is the single value every backend-naming token in the rendered adapter is written from — the attribute on each registered function, the black box that keeps a measured value alive, and the road that runs them.

Backend-agnostic by construction rather than by promise: there is no second place a backend name can enter the rendering, so changing backends changes one declared value. There is no default, either. A backend this compiler chose would be a dependency the consumer never asked for.

Each row renders as its own module carrying two registered functions — the measured realization and the planted-worse one — so the backend produces both curves and the gate has two to separate. A module per row rather than two suffixed names in one namespace, because a suffix is a spelling two distinct lenses can collide at.

The adapter BINDS AND REPORTS. It renders no verdict and runs no gate: the declared order — preflight, planted-worse gate, then measurement — belongs to the bench host, an adapter that ran it would be a second host, and a backend that returned a verdict would be a second authority over what the numbers mean.

## Two literals, written directly

The declared COUNTS — the axis, the budgets — as unsuffixed integers, and the declared BYTES — the work formula — as a byte string.

Each states the value and lets the tree own the spelling: an unsuffixed integer takes the type the consumer's own seat declares, and the `b`, the quotes, and every escape belong to the tree. Neither road carries a refusal for a literal it cannot spell.

## A benchmark is evidence, never a specification

What a rendering here carries is what one declaration said about one realization. Nothing in it says what any other realization must do, and no seat of it carries a measurement — measurements are the bench host's, taken by running.

## The seats

`types.rs` declares the kind, its two seats, the question it owes, and the row vocabulary; `type_guard.rs` is its own child and holds every road that reaches a private field.

`type_contract.rs` states what the kind is, the arm a posture is emitted under, the backend's roads, and the budget order.

`render.rs` writes the tokens.
