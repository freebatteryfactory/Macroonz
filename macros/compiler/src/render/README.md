# render — what a renderer actually materialized

A plan says what will exist.
A rendering is what does.

This home is one step of the road and nothing more: the sink a renderer writes into, the units it writes, the whole rendering, and the one way rendering says no.
It proves nothing, which is the reason nothing here is shaped like a proof.

## A unit is a planned member, materialized

One rendered unit is built from one planned member and one token tree, and from nothing else.

The key it answers to, where it came from, the profile expected to render it, and the address a publication writes it to are facts the plan already states, so they are read off the member rather than restated at the call.
Nothing at a renderer's call site can pair one seat's key with another seat's origin, because there is no seat to write either into.

Its own identity and the digest of its bytes are both taken here, over the tree's own canonical bytes, under that key at that seat's roster position.
No caller supplies either.
A renderer that could hand in a digest could name bytes it never emitted, and every reading downstream would agree with it.

## Where a unit lands is not stored

A unit's delivery is its seat's own answer, read through the role.

There is no seat on a rendered unit for a destination, so no unit can disagree with its own role about which build compiles it, and no seam decides a delivery a second time.

## The sink

`Output` is what a renderer writes into: one seat and its tokens, once per unit, and nothing else.

It holds the plan, so naming the seat is enough.
A seat the plan declares no member for refuses right there, because there is no honest key to materialize against.

## What it does not do

It does not prove, join, partition, or emit.

It never asks what the tokens mean, and it reads nothing out of them but their canonical bytes.
A seat left unfilled and a seat filled twice are both representable here on purpose: those are disagreements between a rendering and a plan, and a sink that refused them would be a proof written in the wrong place.

## How it says no

One refusal, at the first thing that goes wrong.

A unit that cannot be materialized is not a unit and the units after it were never written, so no pass here co-establishes anything and there is nothing to enumerate.
The refusal is an ordinary error: it prints, it is a `core::error::Error`, and it projects into a diagnostic through the one contract every refusing step implements.

## The seats

`types.rs` declares, and its own child `type_guard.rs` holds every road that reaches a private field — the digest, the rendering's non-emptiness, and the sink.

`encode.rs` writes the canonical bytes a proof commits to.
`type_contract.rs` states the refusal roster's positions and the contracts a refusal stands under.
