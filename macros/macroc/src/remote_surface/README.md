# `remote_surface` — the road one declared port speaks a wire contract over

The remote-surface projection's delivery:

one road per bound host contract, in which the codec pairing the surface rides opens, the port's own road answers, and the pairing's other road closes — written as a standalone artifact into the integration target.

## The outside road is not open, and this home says so instead of pretending

**A host contract reaches these services only as an identity the MACHINE minted.** The plane's one public road to an [`OwnerIdentityRef`](crate::plane::OwnerIdentityRef) projects a commitment the machine's identity home produced, and that home carries no public mint for a commitment today — the seat is test-gated until digest derivation exists.

So no caller outside this workspace can hold the value a bound context requires, and [`ProjectionPlan::planned`](crate::planning::ProjectionPlan::planned) refuses a target-free plan for a kind whose target requirement is a bound host contract.

Between those two facts, this projection kind has no outside caller yet.

That is stated as a value rather than left for a reader to discover:

- [`surface_availability`] reads one shared context and answers [`SurfaceAvailability::Bound`] with the contract, or [`SurfaceAvailability::NoHostContract`] carrying what would open the road;

- [`REMOTE_SURFACE_CONTRACT_MINT`] is that opening condition — the home that owes the mint and the exact seat that closes it.

**The opening condition, in one sentence: the machine's identity home publishing a mint for a domain-tagged commitment over a declaration target.**

On the day that lands, [`SurfaceContractMint::Minted`] replaces one constant in `type_contract.rs` and nothing else in this home moves — the bound road below was built whole and has been waiting for a caller, not for a design.

This is the same machine fact the host-wrapper home's own standing names, and the duplication is written down rather than hidden:

a shared standing belongs on the plane, beside the binding it is about, and putting it there is a decision neither home may make alone.

## What the plan actually carries

[`RemoteSurfaceContent`](crate::planning::RemoteSurfaceContent) names exactly three facts and this home codes against those three and nothing beside them:

| seat | what it is | does it reach a token? |
| ---- | ---------- | ---------------------- |
| the PORT | the declaration projected, by identity | no — the TYPE that realizes it is the caller's |
| the WIRE CONTRACT | which bytes travel, by identity | no — the ROADS that write and read them are the caller's |
| the DIRECTION | which way the surface faces | yes: it decides which pairing road opens the road |

**There is no codec seat, and that absence decides the design.**

The codec that reads and writes a wire contract's bytes is its own projection over its own plan, so the pairing a surface rides arrives from the CALLER as [`CodecPairing`] and this home derives none of it. A surface that elected a codec would be pairing somebody else's declaration with a reader nobody asked for — and the pairing would then be a fact the plan never recorded, which is the shape every "invented owner fact" in these services takes.

The port identity and the port TYPE PATH are two facts and this home joins neither to the other:

it does not derive one from the other and does not check that they correspond, because a correspondence nobody declared is not one these services may assert.

## Three calls, and the facing decides only their order

Both facings ride both of the pairing's roads and both call the port's road between them. What an inbound surface and an outbound surface disagree about is which end of the wire they stand at, and that disagreement is exactly which road runs first:

| facing | opens with | then | closes with |
| ------ | ---------- | ---- | ----------- |
| inbound | the pairing's decode road | the port's own road | the pairing's encode road |
| outbound | the pairing's encode road | the port's own road | the pairing's decode road |

That is [`facing`] in `type_contract.rs` — a constant answer over two closed rosters, so a third direction admitted to the plane stops the compiler at the table rather than falling through a rendering that guessed.

A facing says which end of the wire the road stands at and nothing about who calls it:

an inbound surface is not a server and an outbound one is not a client, because neither this home nor the plan it read says anything about who holds the road.

## Where a surface lands

The delivery matrix spells this projection's delivery as **the remote surface in its integration target**.

An integration target is a different FILE than the declaration the plan was derived from, so the planned member is written as a standalone artifact under a byte role, and this home refuses `AtDeclarationSite`.

A surface spliced beside the declaration would be a surface inside the library that declared the port — which is the one place an integration target is not.

The byte role is the PLAN's ([`IntegrationTargetLanding`]), so there is no constant destination here the way the codec and documentation homes have one:

their answer is fixed by a shape, and this one is fixed by a seat only the plan holds.

## What the address owes

[`PAIRING_CONTRACT`] in `type_contract.rs` is the bill: one row per pairing road, naming what the rendered surface calls it with and what it hands back.

The port's own road is billed on the same terms — it takes what the opening road produced and answers with what the closing road consumes, checked into the shape's own refusal by the language's own `?`. The rendering does not degrade:

it writes the calls, and the integration target's compiler answers whether the roads fit, which is exactly where a missing or mis-shaped road on somebody else's type belongs.

## One cause, and the refusal family says so

This home's composition family is SINGLE-CAUSE, and the shape is structural rather than chosen.

Every check on the road is dependent on the one before it — there is no destination to read until a member was found, no binding to read until the member lands where a surface lands, and nothing to render until the binding is there — so exactly one cause is true of any refused composition and there is no set for a body to collect.

The neighbouring host-wrapper home refuses with a collection because its component roster gives it a pass whose issues co-establish;

declaring one here would be a body shape claiming a pass that does not exist.

## The seats

`types.rs` declares, including the one magnitude row this home's capacity is governed by — meaning, number, and reason on one row, stamped through the plane's `limits!` — and the two refusal families it answers through. Its own child `type_guard.rs` holds every road that reaches a private seat — a path's segments and rooting, a pairing's two roads, a signature's three paths, a shape's port road and entry spelling, the landing's byte role, and the surface's composition — which is what makes "a pairing never spells its two roads alike" a shape rather than a rule.

`type_contract.rs` states the declarative surface: the refusal family's declared shape and selection order, the facing table, the pairing contract, and the mint standing.

`plan.rs` reads the plan through its public surface — the account, the context, the membership, the kind content — and states what the surface will be, beside the availability reading a caller takes before it holds a plan at all.

`render.rs` is the token half: the primitives, the pairing call, and the surface road itself.
