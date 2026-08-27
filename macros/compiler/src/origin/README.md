# origin — where a generated thing came from

Generated material must account for how it was derived from authored material.
An origin trail records that derivation as a directed walk from producing nodes toward produced nodes, and a decision trace records the choices made while planning it.

## Location is not derivation

A span says where bytes sat.

An origin says which semantic nodes produced which later nodes and what each step stood for.
A generated unit that offers only a span has answered a different question than the one it was asked.

## The orphan law is structural

Every generated unit carries a structurally non-empty [`OriginTrail`].
A unit with no origin is therefore a value nobody can build rather than a shape something downstream has to check for.

A trail is a walk and not a set: each edge starts where the one before it ended.
A sequence that does not join is two walks presented as one, and whichever end a reader trusts, the other end is provenance nobody established — so it is refused where the trail is drawn, ahead of the ceiling and before anything is measured.
A walk that outgrows the ceiling refuses too, because truncating a walk is how an origin quietly becomes a span.

## Decision order is meaning

A [`DecisionTrace`] keeps its entries in the order the decisions were made, never sorted and never tidied.
Two plans that decided the same things in a different order decided differently, and the trace is what says so.

A selection and an omission each cite the owner fact that decided them.
A check that did not run cites nothing, so absence of a decision never reads as one that ran and passed.

## Trust ceiling

This home carries node and subject identities, relation meanings, and owner-fact citations.
It carries no source material or source spelling, and it does not establish that a caller's declared relation or fact is semantically correct.

[`Nonclaim`] is the other half of that honesty: one thing a plan explicitly does not claim, and the fact that leaves it unclaimed.
Stated nonclaims are what keep a trace from reading as a stronger promise than the plan made.
