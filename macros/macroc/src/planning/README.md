# planning — what the services decide before anything is rendered

The plan family.

## The two rails and the three identity layers

Semantic identity is content-addressed (canonical bytes through the
admitted digest profile); diagnostic identity is location-addressed
(spans); the two never mix — spans are ephemeral by nature, which is
exactly why they may never leak into a semantic identity — and the origin
graph is the join between the rails. Three layers: the intent identity
(kind plus the owner-provided content commitment) → the plan identity
(intent, context, membership, invalidation, trace, origin, nonclaims) →
the rendered-unit identity (derived from the exact rendered bytes). The
plan identity commits only to the semantic origin projection — owner
identities, relations, authored-fact identities; location-bearing origin
data attaches beside it and never enters the transcript. Content walks in
the door carrying the commitment it already has, plus its dependency set —
the services never invent encodings for machine values. The four batteries
this buys: equivalence-by-compare; invalidation IS identity change (the
watch set consumes identities — no dirty flags, ever); unchanged content
is detected by identity and skipped by publication and cache mechanisms
that explicitly consume it (the compiler always re-runs macro expansion;
nothing claims otherwise); and the user-facing identity derive —
domain-separated, versioned, canonical-bytes derivation for consumers'
types with its honesty contract. The rails' first outside customer is the
harness's selective-rerun cache, which keys on the execution key the
report instrument owns — the rails expose identities a consumer can
compare.

## The seam, from this side

The harness owns the public descriptor vocabulary and its schema;
hand-written descriptors are lawful; these services emit conforming DATA
and never import the harness — the producer writes letters to an address;
it does not own the mailbox. The wall has exactly three named crossings —
generated support into the construction contract; the mutation-evaluation
surface into the mutation-point vocabulary; benchmark rows into the bench
row vocabulary — every crossing under the same law, and one generated-
support schema pin governing all three. The physical carrier is the
generated support shell, emitted at the declaration site as deferred
tokens and invoked by the consumption target: constructor-calling
expressions for the row vocabularies, the evaluation copy for the
mutation crossing; the shell is a collision-free, mangled, hidden export
with a stable path-addressable invocation, correct under the rename twins
— the consumer's names are declared once in the consumer's test target
and passed to each invocation as its arguments, and the shell's tokens
splice those paths.

These services carry their OWN independently held expectation of the
generated-support schema identity — intentionally never derived during an
invocation from the harness's supplied or current id, because collapsing
the two ids into one input rebuilds a comparison of a value with itself.
The two values live in two crates and their independence is across
upgrade time: the comparison detects a version-mixed consumer, a partial
publication, or a hand edit to one side; a jointly stale pair is outside
the comparison's claim and dies at the compiler or in the harness's
conformance lane — the mailbox side owns the disposal routes. Both
checked-in sides are written by one explicit publication operation at
schema-change time, under a receipt; the first pair is hand-authored
under the declared-bootstrap posture and becomes verified-derived at the
first toolchain contact, the flip itself a receipted publication act. The
expectation updates only as an explicit compatibility change. Ownership
stays clean: the harness owns the schema; these services own exactly one
fact — "I know how to emit against generated-support schema X."

## The projections and the derived matrix

Explanation is a station every kind answers to, not a kind; no validator
kind exists — a codec that refuses on decode IS the validator. Every
unmaterialized projection carries a disposition, so every empty cell
explains itself. The delivery matrix is DERIVED, never authored: kinds
carry typed attributes, surfaces state offers, a projection demand
composes on top like overlays, and the matrix is computed with every cell
self-explaining. The paved roads: the derive-implementation projection's
production surface at the declaration site and its mutation-evaluation
surface, planned as a member of the same plan under its own rendered role
and consumed in the consumer test target after riding the shell — one
implementation meaning, two surfaces, both inside the closure's membership
proof, a direct parity relationship, and production carries no selector
ever; codec at the declaration site or a visibly published module;
documentation through rustdoc and the explanation station; test-descriptor
rows in a consumer test target and benchmark rows in a bench target, both
carried by the shell; the pattern stamp in its owning source module; host
wrappers in host targets; the remote surface in its integration target.
Guaranteed by shape: no machine-to-harness cycle, no harness types in
production expansion, no normal-build bench tax, no ambient registry. The
test-descriptor content binds exactly the closed row field set the
descriptor instrument owns, via public constructors, against the
independently held expectation; the benchmark content instantiates the
bench instrument's field set — both defer to their owners and enumerate
nothing here.

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

## One member, one delivery

An expansion hands a compiler no single stream. What the consumer's normal build
compiles, what a consumption target invokes later, and what a publication writes
to a named address are three different deliveries, and every planned member says
which one it is for: [`MemberDestination`] is that seat, and
[`EmissionPartition`] is the closed roster it reads to. The reading is total and
it is the destination's own constant answer, so the join that emits, the proof
that closes, and a consumption target routing cargo all take one answer instead
of three that agree until one is edited. The production implementation is
written at the declaration site; the mutation-evaluation copy is written into
the test carrier and rides the shell as deferred cargo — a copy spliced beside
the production implementation would be a selector-bearing surface inside the
normal build, which is the tax the wall exists to refuse.

## Absence is explained

Where a projection was not generated, [`ProjectionDisposition`] says which kind
of absence it was and on whose fact.
Silence is not one of the variants, because silence is what the disposition
exists to abolish.

## A watch set covers its context or there is no plan

A plan's watch set is derived from the context's own seats and the entry
account's commitments, and the derivation
fails closed.
Where the account names more commitments than this watch profile can
represent — more watched declarations than the trigger roster can watch —
the road
refuses with a typed planning issue naming both counts, rather than emitting a
set that covers the first declaration.
A set watching one of three declarations is byte-for-byte the shape of a
complete one, so the plan over it would read as CURRENT after the other two
changed, and nothing downstream could tell the two apart.

The plan's ANCHOR is a different question and keeps naming one declaration: the
transcript commits to the whole account — the commitment and every
dependency it declares — so two plans caused by different accounts
reach different identities whatever they anchor at.
An anchor naming one member of a committed set is a spelling rule; a watch
naming one member of a committed set is a claim about the others.

## The seats

`types.rs` declares, including the `kinds!` roster whose sealed contract is part
of each kind's declaration.
Its own child `type_guard.rs` holds the output firewall and every other road
that reaches a private field.
`type_contract.rs` states the rendered-role roster an implementation projection
materializes — each contract's production implementation, each production
implementation's evaluation copy, which role pairs with which, and which
delivery a member under each one is written into — together with the destination
roster's own constant answer, which is the emission each destination reads to.
`anchor.rs` reads a plan's footing and DERIVES the shared watch set
that follows from it, and `encode.rs` writes the bytes a plan's transcript is
taken over.
