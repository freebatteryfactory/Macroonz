# planning — what the services decide before anything is rendered

The plan family.

## A family on a generic, never a mega-record

One shared spine — [`ProjectionPlan`] — carries what every plan carries: the
shared exact identities, the complete declared output set, what invalidates it,
why it was decided that way, where it came from, and what it does not claim.
What differs by kind rides [`ProjectionKind::Content`], so a new kind adds a
content type rather than another optional seat on a record everyone shares.
The kind roster is sealed: a kind is admitted here or it does not exist, because
a kind the plane cannot explain is a kind the plane must not plan.

## Plan before render, and no partial output

A plan states its complete membership up front.
That is the output firewall: the declared set is the whole set, and a sibling
that is not in it was not planned.
Materializing a bundle is atomic at the publication boundary —
[`ProjectionBundlePlan`] names its members, and a partial materialization is a
refusal, never a partial success.

## Absence is explained

Where a projection was not generated, [`ProjectionDisposition`] says which kind
of absence it was and on whose fact.
Silence is not one of the variants, because silence is what the disposition
exists to abolish.

## A watch set covers its context or there is no plan

A plan's watch set is derived from the context's own seats, and the derivation
fails closed.
Where a context carries a seat this watch profile cannot represent — a cause set
naming more source declarations than the trigger roster can watch — the road
refuses with a typed planning issue naming both counts, rather than emitting a
set that covers the first declaration.
A set watching one of three declarations is byte-for-byte the shape of a
complete one, so the plan over it would read as CURRENT after the other two
changed, and nothing downstream could tell the two apart.

The plan's ANCHOR is a different question and keeps naming one declaration: the
transcript commits to the whole cause set, so two plans caused by different sets
reach different identities whatever they anchor at.
An anchor naming one member of a committed set is a spelling rule; a watch
naming one member of a committed set is a claim about the others.

## The seats

`types.rs` declares, including the `kinds!` roster whose sealed contract is part
of each kind's declaration.
Its own child `type_guard.rs` holds the output firewall and every other road
that reaches a private field.
`type_contract.rs` states the rendered-role roster an implementation projection
materializes, `anchor.rs` reads a plan's footing and DERIVES the shared watch set
that follows from it, and `encode.rs` writes the bytes a plan's transcript is
taken over.
