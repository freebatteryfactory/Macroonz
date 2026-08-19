# `host_wrapper` — the wrapper a bound host contract's demand composes to

The host-wrapper projection's delivery: one wrapper shell per bound host
contract, composing the components the plan's declared capability selected, in
the plane's own roster order, written as a standalone artifact into the host's
own target.

## The outside road is not open, and this home says so instead of pretending

**A host contract reaches these services only as an identity the MACHINE
minted.** The plane's one public road to an
[`OwnerIdentityRef`](crate::plane::OwnerIdentityRef) projects a commitment the
machine's identity home produced, and that home carries no public mint for a
commitment today — the seat is test-gated until digest derivation exists. So no
caller outside this workspace can hold the value a bound context requires, and
[`ProjectionPlan::planned`](crate::planning::ProjectionPlan::planned) refuses a
target-free plan for a kind whose target requirement is a bound host contract.
Between those two facts, this projection kind has no outside caller yet.

That is stated as a value rather than left for a reader to discover:

- [`wrapper_availability`] reads one shared context and answers
  [`WrapperAvailability::Bound`] with the contract, or
  [`WrapperAvailability::NoHostContract`] carrying what would open the road;
- [`HOST_WRAPPER_CONTRACT_MINT`] is that opening condition — the home that owes
  the mint and the exact seat that closes it.

**The opening condition, in one sentence: the machine's identity home publishing
a mint for a domain-tagged commitment over a declaration target.** On the day
that lands, [`WrapperContractMint::Minted`] replaces one constant in
`type_contract.rs` and nothing else in this home moves — the bound road below was
built whole and has been waiting for a caller, not for a design.

What this home does NOT do is render a wrapper against a contract it invented.
A wrapper bound to a host nobody declared is exactly the defect the target-free
refusal exists to prevent, and a "default host" would be that defect wearing a
plan's authority. The absence is typed; it is not filled.

## Where a wrapper lands, and why it is the only home that says artifact

The delivery matrix spells this projection's delivery as **host wrappers in host
targets**. A host target is a different FILE than the declaration the plan was
derived from, so the planned member is written as a standalone artifact under a
byte role — and this home is one of the two rendering homes in the services that
refuse `AtDeclarationSite` rather than requiring it, the remote surface's being
the other. A wrapper spliced beside the declaration would be a wrapper inside the
library that declared the contract, which is the one place a host target is not.

The byte role is the PLAN's ([`HostTargetLanding`]), so there is no constant
destination here the way the codec and documentation homes have one: their
answer is fixed by a shape, and this one is fixed by a seat only the plan holds.

## The composition order is the plane's, and neither list is

A plan states which components it composes and a shape states which roads answer
them. Both are membership statements whose own order says nothing. The order a
wrapper actually composes in is
[`WRAPPER_COMPONENTS`](crate::planning::WRAPPER_COMPONENTS) — the plane's own
declared roster — walked once, with the plan's selection asked per component.
Two plans that named the same components in different orders render one wrapper;
a plan that named a component twice renders it once.

That walk is also the composition pass's quantifier, in both directions: a
selected component with no stage is refused, a stage on a component nobody
selected is refused, and two stages under one component are refused. All three
co-establish freely across components, so a caller reads the whole disagreement
at once rather than repairing a wrapper one component per attempt.

## The shape arrives from the caller

The plan's kind content names a CONTRACT, the COMPONENTS composed, and the
declared CAPABILITY that selected them. It does not name a type, a road, a
signature, or an entry spelling — so [`WrapperShape`] arrives from the caller and
`plan.rs` reads only what the plan actually decided. A generator that decided
which road a host answers admission on would be declaring somebody else's calling
convention and then calling it.

The one fact this home adds per component is the local its stage's answer is
bound under, stated once as [`stage`] in `type_contract.rs`. It is a `const fn`
over the plane's closed roster rather than a second roster written beside it, so
a component admitted to the plane and not answered here stops the compiler
instead of passing as a missing row.

## What the address owes

One bill, uniform across every stage: a stage's road is an associated road on the
host contract's own type, it takes the carried value, and it answers with the
carried value or a refusal the shape's own refusal path converts from. The
emission writes `<Host>::<road>(<carried>)?` and the HOST TARGET's compiler
answers whether the road fits — which is exactly where a missing or mis-shaped
road on somebody else's type belongs. The rendering does not degrade and has no
"opaque stage" arm.

## The contract is read twice, and the readings are not folded

A plan of this kind carries a host contract in two places: the context's target
binding and the kind content's own contract seat. Nothing in the plane requires
them to agree, so `plan.rs` carries both, named for which reading each came from.
Electing one would answer a question the plan states twice with whichever answer
happened to be nearer.

## The seats

`types.rs` declares, including the three magnitude rows this home's capacities
are governed by — meaning, number, and reason on one row, stamped through the
plane's `limits!` — and the two refusal families it answers through. Its own
child `type_guard.rs` holds every road that reaches a private seat — a path's
segments and rooting, a stage's road, a shape's stages, the landing's byte role,
the surface's composition, and the refusal body's one seat — which is what makes
"every composed component was staged exactly once" a shape rather than a rule.
`type_contract.rs` states the declarative surface: the refusal family's declared
shape, the stage contract, and the mint standing. `plan.rs` reads the plan through its public
surface — the account, the context, the membership, the kind content — and states
what the wrapper will be, beside the availability reading a caller takes before
it holds a plan at all. `render.rs` is the token half: the primitives, the
roster-ordered composition, and the wrapper shell itself.
