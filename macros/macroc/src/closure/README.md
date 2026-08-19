# closure — what was rendered, the proof it is what was planned, and the closed expansion that proof opens

## Two values: a plan and a rendering

A plan is made before anything exists. It states what will be materialized:
under which roles, with which semantic keys, landing where, coming from where,
and whose digests will be anchored to what. A rendering is what a renderer
actually produced: token trees, their bytes, and the digests over those bytes.

Keeping them apart is what this home is for. A plan that carried its own
rendered-byte digest would either be carrying a placeholder or carrying a digest
from a rendering that already happened — and in the second case, any later
"check" compares the value against itself and passes on every input.

## A reconstruction, not an assertion

[`ProjectionClosure::proved`] does not ask the renderer whether it obeyed the
plan. It **rebuilds the membership out of the rendered units** — role by role,
reading each unit's own semantic key, destination, profile, origin, and
recomputed digest — and then compares that reconstruction against the membership
the plan declared. Every way the two can disagree is a typed refusal naming the
role it disagreed at:

- a planned role nothing rendered ([`ClosureIssue::MemberMissing`]);
- a rendered role nothing planned ([`ClosureIssue::MemberUnplanned`]);
- one role rendered twice ([`ClosureIssue::MemberDuplicated`]);
- a rendered unit whose origin is not the planned one
  ([`ClosureIssue::OriginOrphan`]);
- a digest that is not the digest of the bytes actually rendered, under the
  contract the plan stated ([`ClosureIssue::DigestMismatch`]);
- a unit standing under the right role and answering to a different semantic key
  ([`ClosureIssue::SemanticKeyMismatch`]);
- a unit rendered under a profile or to a destination the plan did not name
  ([`ClosureIssue::MaterializationMismatch`]);
- a role the plan itself declared twice ([`ClosureIssue::MemberPlannedTwice`]);
- a rebuild that is not the planned membership as a complete set
  ([`ClosureIssue::MembershipDisagreement`]);
- two published units standing at one address
  ([`ClosureIssue::ArtifactAddressDoubled`]).

## One emission per delivery

An expansion does not hand a compiler one stream. What the consumer's normal
build compiles, what a test target invokes, what a bench target invokes, and
what a publication writes to a named address are four deliveries, and every
planned member declares which one it is for. So the closure does not join a
rendering; it PARTITIONS one. Each joined emission is built by walking the
rendered units in role-roster order and reading each unit's own destination, and
the emission a unit reaches is that destination's own constant answer — which is
what makes a mutation-evaluation surface in the consumer's normal build
unwritable rather than merely discouraged. A join that outgrows the token
magnitude names the emission it overran at
([`ClosureIssue::JoinedTreeUnbounded`]), because three builds are three byte
streams and a caller cutting the wrong one has repaired nothing. Artifacts are
never joined: two artifacts are two addresses, and one stream claiming to be both
is what the address check refuses.

**No token reaches a compiler except through a proof.** The closure builds every
emission itself, keeps them, and commits to their digests inside its own identity
— so the exact byte stream each build receives is part of what was proved rather
than something assembled afterwards. Holding a closure is the proof; there is no
partial closure and no closure with a warning attached. The closure proves
agreement among values that exist when it runs — plan, declared membership,
rendering, origins, trace — never with an explanation not yet produced.

## The closed expansion is where every road ends

The explanation is answered over the PLAN and the PROVED closure, and
[`ClosedExpansion`] binds all three into the one complete account emission is
reachable from — for every projection kind, not for one family. The closure's own
road to its emissions is crate-internal, so there is no way to emit off a proof
without the plan it was proved against and the explanation answered over it.

**The three names agree or nothing binds.** Each value carries the parentage it
was produced under — the closure names the plan it was proved against, the
explanation names the plan and the closure it was answered over — and the binding
compares all three, refusing under [`ExpansionBindingRefusal`] with both
identities named. The expansion's own identity then commits to all three, so a
terminal that bound one expansion's plan and proof with another expansion's
explanation of the same kind is neither buildable nor silently identical to the
honest one. A kind is not an expansion: two plans of one kind admit the same
questions, so coverage alone never established which subject a view was about.

It states rather than invents what an expansion does not have: no carrier has
been named and nothing has been published at this seam ([`DeliveryAddressing`]),
which is an absence the account carries and never a reason to refuse. The word
"receipt" stays with that publication crossing and with the evidence crossings
the machine already owns; the terminal here is a closed expansion and carries
[`ClosedExpansionId`](crate::plane::ClosedExpansionId), which is its own name.

## The seats

`types.rs` declares; its own child `type_guard.rs` takes the digests, owns the
partitioning and the joins, builds the proof, builds the closed expansion, and
builds the refusal body, which is what keeps every one of those roads unreachable
from anywhere else. `prove.rs` is the per-role pass those roads consume, reaching
no private seat, and `type_contract.rs` states the refusal family, the issue
roster's own table, and the sealed proof contract a complete explanation is
answered over.
