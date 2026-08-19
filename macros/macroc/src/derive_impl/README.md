# `derive_impl` — one implementation meaning, two surfaces

The derive-implementation projection's delivery: the PRODUCTION SURFACE that lands
at the declaration site, and the MUTATION-EVALUATION SURFACE that travels the shell
road to the consumer's test target. One meaning, rendered twice, with a typed
statement of what the two share.

## The two surfaces are explicitly different, and the difference is one-directional

The production surface carries **no selector, no test switch, and no
configuration arm — ever**. It is the implementation the consumer's normal build
compiles, and nothing about mutation is representable in it: there is no seat on
[`ProductionSurface`] for a selector, so a production rendering that consulted one
is not a value anybody can hold. The delivery matrix's guarantee — no harness
types in production expansion, no normal-build tax — is that absence, stated
structurally rather than reviewed.

The evaluation surface is the one that carries the selector. It is the same
implementation with every admitted mutation point wrapped in a selection among
that point's admitted alternatives, keyed by an active-point parameter the caller
declares. It compiles once, carrying every point at once, which is what makes the
rapid loop rapid: **runtime is SELECTION among admitted points, never
interpretation of arbitrary source** — an interpreter over arbitrary source would
mint a second meaning authority, and this home renders a `match` over a closed
enum instead. Every arm of that `match` is written out; there is no wildcard, so a
variant added without an arm stops the compiler rather than falling through to
something.

## The control is structural, not a rule

Every evaluation surface contains the no-mutation control, at the table's first
position, and it is there because [`MutationPointTable::over`] PUTS it there. The
road takes the admitted points and returns a table; there is no road that takes a
control, and no road that returns a table without one. A surface missing its
control is therefore unwritable rather than refused — which is the difference
between a law and a check somebody has to run.

With the control selected, every point renders its original operation unchanged,
so the evaluation copy under `NoMutation` emits exactly the production surface's
own operations.

## Parity, and what it is silent about

[`SurfaceParity`] states what the two surfaces SHARE: one declaration — the
address the entry account walked in with — and one rendering engine, the
generator identity the plan's context names. Both surfaces are rendered by this
home from that one plan, so the parity is derived from seats that exist rather
than asserted about a comparison nobody made.

**And it is silent about both of them.** Agreement across a shared substrate is
silence about that substrate: a declaration that says the wrong thing says it to
both surfaces, and a rendering engine that writes the wrong tokens writes them
twice. The parity proves the evaluation copy faithful to the RENDERED PRODUCTION
SURFACE — never that either surface matches the owner's intent, and never that
the mutants the points admit are meaningful damages. Those are the harness's
questions and they are answered by running, not by holding this value.

## Where each surface lands

The production surface's rendered tree is what the plan's membership declared: it
carries the planned member's semantic key, its expected profile at its version,
and its origin trail, so it is exactly the material a closure rebuilds a planned
member out of, and its destination is the declaration site by construction rather
than by a seat that could say otherwise. A planned member that lands anywhere else
is not a derive-implementation production surface, and this home refuses it.

The evaluation copy is NOT a planned member of that plan. Its identity is
therefore derived here, over its own rendered bytes, under the rendered-unit role
and anchored on the production member's semantic key — the contract
([`EvaluationIdentityContract`]) is stated at planning time and honoured at
rendering time, never a digest of bytes nobody has produced yet. It reaches the
consumer's test target as deferred tokens inside the generated support shell; the
shell's own crossing — constructor-calling expressions against the harness's
mutation-point vocabulary, under the two-sided schema pin — is the shell's
rendering and not this home's. This home names no harness type and imports
nothing from the harness: it holds the points as DATA in the harness's field
shape, and the mailbox belongs to whoever owns the address.

## What a mutation point carries

Its own identity, the owner claim it stands under, the original operation it is
about, the alternatives admitted against that operation, and the activation site
as a route into the captured declaration. Each of the two operations — original
and alternative — carries both its declared spelling, which is what the harness
reads, and the tokens that write it, which is what the rendering substitutes. A
point admitting no alternative is refused: a selection among one thing selects
nothing.

The points arrive from the caller. Nothing here decides that an operation is worth
damaging, which family a damage belongs to, or what a survivor means — those are
the harness's declarations, and a generator that invented them would be its own
oracle.

## The seats

`types.rs` declares, including the three limit families this home's capacities are
governed by and the two refusal families it refuses through. Its own child
`type_guard.rs` holds every road that reaches a private seat — a name's parts, an
operation's tokens, a point's alternatives, the table's control, both surfaces'
renderings, the parity, and the refusal body — which is what makes the control
structural and keeps a surface from existing that the passes did not agree on.
`type_contract.rs` states the declarative tables: each limit family's authority
and magnitude written together, the composition family's declared shape, and the
two-surface roster with the one fact that separates them. `plan.rs` reads the plan
through its public surface — the account and the membership — and states what the
two surfaces will be. `render.rs` is the token half: the active-point enum, one
point's selection, and the single walk that turns the production tree into the
evaluation copy.
