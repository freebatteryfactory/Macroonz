# testpak seat 01 — corpus (reserved)

**State: reserved.** This directory holds one file, and this is it. There is no
`mod.rs`, no type, no function, no public surface, and no obligation row standing
behind the name. `lib.rs` declares no module here, so nothing in this package can
reach this coordinate — it is a place in the map, and the map says so.

## The question this seat is reserved for

*Which populations of declared input does the machine have to survive, and where
do they live?*

A challenge that hand-writes its own input challenges what the author imagined.
This seat is where a population lives instead: the permutation hostiles that
supply an order-insensitive set in every other order, the determinism hostiles
that run one declared input twice and require one answer, and the ambient-pathway
hostiles that stand up a changed environment, clock, or working directory around
an expansion and require the answer not to move. The services' own documentation
already names all three as owed.

## What it contains today

Nothing. No corpus, no generator, no recorded population, no fixture. The seat is
a coordinate with a question written on it.

## What materializes it

The first population that is genuinely a POPULATION rather than a case: material
enumerated or generated from a stated rule, carried as its own value, and handed
to a challenge that states what it is stating over it. A second hand-written test
case in `tests/` does not materialize this seat and must not be filed here; it is
a case, and cases live beside the challenge that reads them.

## What this reservation does not claim

It does not claim a corpus exists, is designed, or is specified beyond the
question above. It does not claim any of the three hostile populations has an
owner, a method, or a date. It does not claim the seat's number reserves any
dependency position: no module is declared, so no band edge exists to point
anywhere. And it does not claim that material which fails to fit this question
may be filed here anyway — content that does not fit a reserved name comes back
for an explicit decision instead of being normalized into the nearest drawer.

```yaml
seat: 01_corpus
state: reserved
question: >
  Which populations of declared input must the machine survive, and where do
  they live — the permutation, determinism, and ambient-pathway hostiles the
  services' own documentation names as owed.
materializes_when: >
  A population enumerated or generated from a stated rule is carried as its own
  value and handed to a challenge that states what it establishes over it. A
  second hand-written case does not materialize this seat.
nonclaims: >
  No corpus exists, is designed, or is specified beyond the question above. No
  hostile population here has an owner, a method, or a date. The number reserves
  no dependency position, because no module is declared. Content that does not
  fit this question is not filed here.
```
