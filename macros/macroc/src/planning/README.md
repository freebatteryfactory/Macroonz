# planning — what the services decide before anything is rendered

The plan family.

## A family on a generic, never a mega-record

One shared spine — [`ProjectionPlan`] — carries what every plan carries: the
shared exact identities, the complete declared output set, what invalidates it,
why it was decided that way, where it came from, and what it does not claim. What
differs by kind rides [`ProjectionKind::Content`], so a new kind adds a content
type rather than another optional seat on a record everyone shares. The kind
roster is sealed: a kind is admitted here or it does not exist, because a kind
the plane cannot explain is a kind the plane must not plan.

## Plan before render, and no partial output

A plan states its complete membership up front. That is the output firewall: the
declared set is the whole set, and a sibling that is not in it was not planned.
Materializing a bundle is atomic at the publication boundary —
[`ProjectionBundlePlan`] names its members, and a partial materialization is a
refusal, never a partial success.

## Absence is explained

Where a projection was not generated, [`ProjectionDisposition`] says which kind
of absence it was and on whose fact. Silence is not one of the variants, because
silence is what the disposition exists to abolish.

## The seats

`types.rs` declares, including the `kinds!` roster whose sealed contract is part
of each kind's declaration. Its own child `type_guard.rs` holds the output
firewall and every other road that reaches a private field. `type_contract.rs`
states the rendered-role roster an implementation projection materializes,
`anchor.rs` reads a plan's footing and DERIVES the shared watch set that follows
from it — one road, reading the context's own seats, rather than a roster each
plan site keeps in step by hand — and `encode.rs` writes the bytes a plan's
transcript is taken over.

This home's qualification obligations live in the crate README's tooling-obligation
blocks.
