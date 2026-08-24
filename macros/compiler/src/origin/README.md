# origin — where a generated thing came from

Every unit this compiler renders carries a walk back to the material a person authored, and every plan carries the decisions it made on the way there.

## A span is not an origin

A span says where bytes sat.

An origin says which authored declaration, which pattern instantiation, which profile selection, and which rendering act stand between that declaration and this unit.
A generated unit that offers only a span has answered a different question than the one it was asked.

## The orphan law is structural

An [`OriginTrail`] holds a [`NonEmpty`](crate::bounded::NonEmpty) list of edges, so a trail with nothing in it is a value nobody can build rather than a shape something downstream has to check for.
Every generated unit carries one, and that is the whole of what makes a generated unit with no origin unrepresentable.

A trail is a walk and not a set: each edge starts where the one before it ended.
A sequence that does not join is two walks presented as one, and whichever end a reader trusts, the other end is provenance nobody established — so it is refused where the trail is drawn, ahead of the ceiling and before anything is measured.
A walk that outgrows the ceiling refuses too, because truncating a walk is how an origin quietly becomes a span.

## A record, not an inventory

A [`DecisionTrace`] keeps its entries in the order the decisions were made, never sorted and never tidied.
Two plans that decided the same things in a different order decided differently, and the trace is what says so.

A check that did not run is [`TraceDecision::NotRun`], and it never reads as one that ran and passed.
A selection and an omission each cite the fact that decided them; a check that did not run cites nothing, because there is nothing to cite.

## What it does not carry

No source material, and no spelling of one.
An entry names the subject it decided about by identity, and names its reason by the declared fact of whoever owns it.

[`Nonclaim`] is the other half of that honesty: one thing a plan explicitly does not claim, and the fact that leaves it unclaimed.
Stated nonclaims are what keep a trace from reading as a stronger promise than the plan made.
