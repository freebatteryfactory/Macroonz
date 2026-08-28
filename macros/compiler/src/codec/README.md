# `codec`

The codec kind: one declared shape in, the Rust that writes its canonical bytes and reads them back out.

A caller says what its value is made of — the members, in the order they are written, each held at a type, each under one wire shape, each required, optional, or repeated — and this home renders the two roads over it.
Nothing here decides any of that.
A generator that chose a member's wire shape would be inventing how somebody else's value is written down and then encoding it that way.

---

## The decode road is the validator

There is no validator kind here, and the absence is a statement rather than a gap.

A decode road that refuses on malformed input already answers every question a validator would.
It reads one member at a time, refuses the moment the material does not admit the member it is standing at, and refuses once more where material remains after the last declared member.
"These bytes are a lawful value" is therefore exactly "the decode road returned one", and there is nothing else to run to learn it.

That is why a request covering only the encode direction delivers no reader, and why the surface says so instead of rendering half a decode road.

---

## The framing, stated once

Every variable-length member is written as eight big-endian length bytes and then the bytes themselves.

Two members can never be re-cut at another boundary and produce one byte string, which is the whole of what canonical buys.
Nothing is folded on the way in and nothing is summarized: a codec that summarized a member would be inventing a second value for it.
A nested member is framed on exactly those terms rather than run to the end of the input, because a nested value that consumed the remainder would make the member after it unreadable.

---

## No numeric literal is written anywhere

The generated-token roster carries a numeric arm and this home writes nothing through it.
Every place a number would have stood is the language's own road to the same value.

| Where a number would stand | What the rendering writes |
| --- | --- |
| the framing width | `::core::mem::size_of::<u64>()` |
| an absent optional member | `u8::from(false)` |
| a present optional member | `u8::from(true)` |
| a closed choice's admitted slots | the owner's own roster, walked and compared |
| a repeated member's stop | `collected.len() < count` |

The closed-choice road is the one worth reading twice.
The rendering never writes a table of slots: it walks the roster the owner declared and compares each candidate's own position against the byte it read.
A roster that gained an arm gains it in the decode road too, without this home ever learning what the arms are — and a slot no arm answers to refuses rather than electing a neighbour.

`::core::mem::size_of::<u64>()` says the width *is* the framing's, where a digit would say only what that width happens to be today.
A literal is available here, and it is the weaker sentence.

---

## What the address owes

`MEMBER_CONTRACT` is the complete bill: one row per wire shape, naming exactly the roads the rendered code calls on a member's own type.
A road named through a trait is written qualified; a road named on the member's own type is written bare.
Where a road is absent, the failure lands at the caller's site as an ordinary unresolved method, which is where a missing road on the caller's own type belongs.

The assembly road's posture is stated and never inferred.
A total constructor is called plain; a checked one is called with `?`, and the rendered refusal carries the owner's own refusal beside a `From` implementation this home writes — so a checked assembly costs the address nothing.

---

## Where the surface lands

Both placements are declaration-site deliveries, so what the placement decides is the surface's *shape*: spliced beside the owner's own item, or wrapped in a visibly published module whose head imports the scope the module sits in.

The kind's one seat names the declaration site itself, so a codec surface written anywhere else is not a member this home can refuse — it is a member nobody can plan.

---

## Ownership

This home owns the codec declaration shape, its admitted spellings and magnitudes, the canonical framing contract, the paired write and read projections, their declared placement, and the refusal when that shape cannot be established.
The caller continues to own every member's meaning, wire-shape choice, callable road, and assembly posture; the generic token home continues to own how informed Rust tokens are represented.
