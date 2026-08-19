# `codec` — the codec that refuses on decode IS the validator

The codec projection's delivery: the ENCODE road that writes one declared shape's
canonical bytes and the DECODE road that reads them back, rendered as tokens for
the owner's own type, landing at the declaration site or inside a visibly
published module.

## There is no validator kind, and this is why

The delivery matrix admits no validator projection, and the absence is a
statement rather than a gap: a codec whose decode road refuses on malformed input
already answers every question a validator would. The rendered decode road reads
one member at a time, refuses the moment the material does not admit the member
it is standing at, and refuses again where material remains after the last
declared member — so "these bytes are a lawful `Owner`" is exactly "decode
returned a value", and nothing else has to be run to learn it.

That is why a plan whose direction is
[`CodecDirection::Encode`](crate::planning::CodecDirection::Encode) delivers no
validator, and why this home says so instead of rendering a half decode: an
encode-only codec is a lawful delivery that carries no reader, and a caller who
needed the reader needs a different direction rather than a different rendering.

## The framing is the plane's, and it is stated once

Every variable-length member is written as `u64be(len)` then the bytes — the
plane's own framing ([`crate::plane::encode_bytes`]), the same one the descriptor
and report instruments spell for their preimages. Two members can therefore never
be re-cut at a different boundary and produce one byte string, which is the whole
of what "canonical" buys. Nothing is folded on the way in and nothing is
compressed: a codec that summarized a member would be inventing a second value
for it.

A NESTED member is framed on exactly those terms — the nested codec's own output,
length-prefixed — rather than run to the end of the input, because a nested value
that consumed the remainder would make the member after it unreadable.

## The rendering writes no numeric literal, and that is deliberate

The generated-token roster carries four arms — word, punctuation, text, group —
and no numeric one. Rather than refuse, every place a number would have stood is
written as the language's own road to the same value:

| where a number would stand | what the rendering writes |
| -------------------------- | ------------------------- |
| the framing width | `::core::mem::size_of::<u64>()` |
| an absent optional member | `u8::from(false)` |
| a present optional member | `u8::from(true)` |
| a closed choice's admitted slots | the owner's own `ALL` roster, compared by `slot()` |
| a repeated member's stop | `collected.len() < count` |

The closed-choice road is the one worth reading twice: the rendering never writes
a table of slots, it walks the owner's OWN declared roster and compares each
candidate's `slot()` against the byte it read. A roster that gained an arm gains
it in the decode road too, without this home ever learning what the arms are —
and a slot no arm answers to refuses rather than electing a neighbour.

This home therefore renders no `ByteLiteralNotSpellable` and no
`CountLiteralNotSpellable`. The missing numeric arm is still a gap in the token
roster, recorded by the two crossings that DO need it; the codec simply does not
stand on it.

## The shape arrives from the caller

The plan's kind content names a SCHEMA, a BYTE ROLE, a DIRECTION, and the owner
facts the codec rests on. It does not name a type, a member, a wire shape, a
cardinality, or an assembly road — so [`CodecShape`] arrives from the caller and
`plan.rs` reads only what the plan actually decided. A generator that invented a
member's wire shape would be declaring how somebody else's value is written down
and then encoding it that way, which is the one thing these services never do.

The CARDINALITY is the machine's roster
([`FieldCardinality`](threadpak::schema::FieldCardinality)), imported rather than
restated: required, optional, repeated, and no fourth.

## What the address owes

`MEMBER_CONTRACT` in `type_contract.rs` is the complete bill: one row per wire
shape, naming exactly the roads the rendered code calls on the member's own type.
It is stated in the repository rather than in a reader's head, and nothing is
listed that the emission does not actually write.

The ASSEMBLY road is the caller's and its posture is stated, never inferred: a
total constructor is called plain, a checked one is called with `?` and the
rendered refusal carries the owner's own refusal beside a `From` implementation
this home writes — so a checked assembly costs the address nothing at all.

## Where the surface lands

Both admitted placements are expansion deliveries, so the plan's destination is
the declaration site under either: a planned member written as a standalone
artifact is a different delivery and is refused here. What the placement decides
is the SHAPE — spliced beside the owner's own item, or wrapped in a visibly
published module whose head writes the one import a wrapped surface needs.

## The seats

`types.rs` declares, including the three limit families this home's capacities are
governed by and the two refusal families it answers through. Its own child
`type_guard.rs` holds every road that reaches a private seat — a path's segments,
a member's spelling, a shape's members, the assembly road, the placement's module
spelling, the surface's composition, and the refusal body's one seat. 
`type_contract.rs` states the declarative tables: each limit family's authority
and magnitude written together, the refusal family's declared shape, the member
contract, the reserved bindings the decode road names, and which of the two roads
each declared direction covers. `plan.rs` reads the plan through its public
surface — the account, the context, the membership, the kind content — and states
what the surface will be. `render.rs` is the token half: the refusal declaration,
the encode road, the decode road, and the placement that carries them.
