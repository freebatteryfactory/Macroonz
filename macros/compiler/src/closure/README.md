# closure — the proof that what was rendered is what was planned

A plan is written before anything exists.
A rendering is what a renderer actually produced.
This home is where the two meet, and it does not ask the renderer whether it obeyed.

## A reconstruction, not an assertion

The membership is REBUILT out of the rendered units — seat by seat, reading each unit's own semantic key, origin, profile, address, and recomputed digest — and the rebuild is then compared against the membership the plan declared.

Every way the two can disagree is a typed issue naming the seat it disagreed at: a planned seat nothing rendered, a rendered seat nothing planned, one seat rendered twice, an origin that is not the planned one, a digest that is not the digest of the bytes actually rendered under the contract the plan stated, a unit answering to another semantic key, a unit rendered under a profile or to an address the plan did not name, and a seat the plan itself declared twice.

The last comparison is made over the whole set rather than seat by seat, because a walk comparing one member per seat would agree about two memberships that differ in their second.

## One emission per delivery

An expansion does not hand a compiler one stream.

What the consumer's normal build compiles, what a test target invokes, what a bench target invokes, and what a publication writes to its own address are four deliveries, and every seat names exactly one of them.
So a closure does not join a rendering; it PARTITIONS one, and each joined delivery is built by walking the rendered units in roster order and reading the delivery each unit's seat declares.

Artifacts are never joined: two artifacts are two addresses, and one stream claiming to be both is what the address check refuses.
A join that outgrows the token magnitude names the delivery it overran at, because three builds are three byte streams and a caller cutting the wrong one has repaired nothing.

The emission claims nothing about the vehicles.
Whether a carrier's shell has been rendered, what it is named, and whether any target invokes it are the consumption side's facts; whether a publication ever wrote an artifact is the publication road's.

## The emission is inside the proof

The closure builds every joined delivery itself, keeps them, and commits to their digests inside its own identity.

So the exact byte stream each build receives is part of what was proved rather than something assembled afterwards, and a rendering that moved one member to another delivery is a different closure rather than the same one emitted differently.
Holding a closure is the proof; there is no partial closure and no closure with a warning attached.

The road from a proof to its emissions is crate-internal.
Tokens are reached through the expansion that binds this proof to the plan it was proved against and the explanation answered over the two — a road out of here would be a road to emission that skips the binding, which is the same as no binding.

## How it says no

Closure issues are independent and co-establishable — one rendering may drop one seat and orphan another in one pass — so a refusal carries every issue its pass established rather than electing a primary one.

A body that fills its bound keeps what fits and counts the rest, which is a different statement from "no further disagreements exist".
The refusal is an ordinary error: it prints, it is a `core::error::Error`, and it projects into a diagnostic through the one contract every refusing step implements.

## The seats

`types.rs` declares, and its own child `type_guard.rs` holds every road that reaches a private field — the join, the partitioning, and the proof itself.

`prove.rs` is the per-seat pass and the readings it takes over a rendering.
`encode.rs` writes the canonical bytes the proof's transcript is taken over, and the bytes one issue is.
`type_contract.rs` states the issue roster's tables and the two contracts a closure refusal stands under.
