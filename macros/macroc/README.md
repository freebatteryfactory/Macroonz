# macroc — the metaprogramming services

The services are ordinary callable Rust — capture, planning, rendering, closure,
inspection, explanation — reached the same way by any caller. They depend inward
on the machine and never back outward: nothing here knows a proc-macro exists.

The crate's own doc comment carries the charter and the dependency order; this
file carries what a README owes that rustdoc does not: **the mechanisms this
tooling admits, and the qualification obligations it stands under.**

## The admitted digest, and what it is admitted for

The services derive their own identities — plans, closures, rendered units,
generated units, origin nodes, bundles, closed expansions — and those identities
are handed out. A receipt that names a plan is only as good as the name, so the
derivation is a **BLAKE3 identity profile**, versioned and domain-separated, over
complete transcripts.

**The dependency.** `blake3`, pinned exact at `=1.8.6`, with
`default-features = false`. The default `std` feature buys an `io::Write` adapter
and error-trait impls the services name nowhere. `rayon` is not a default feature
and is never named: an expansion running inside `rustc` must not stand up a
thread pool to hash a few kilobytes. `serde`, `zeroize`, `mmap`, and the
digest-trait previews buy surfaces the plane has no seat for. What is left is
`Hasher` and `derive_key`, which is the whole mechanism the profile uses. The
crate carries its own build script and C/assembly fast paths on the platforms
that have them; that is a property of the admitted dependency and is disclosed
here rather than discovered by whoever first builds without a C toolchain.

**This admission is the TOOLING PLANE's, and it is not band 07's.** Band 07's
digest-family law proposes blake3-256 for the machine's commitments, under the
machine's domain-tag register, and admitting it is a separate mechanism ruling
with a separate owner. The two admissions share an algorithm and nothing else:
different preimages, different domain separation, different claims, different
owners. Neither one licenses the other, and a plane identity is never accepted
where a machine commitment is required.

**What the profile claims.** For a transcript as specified beside
`ProjectionTranscript`, under the declared profile version, collision resistance
is claimed AS BLAKE3's — finding two transcripts that derive one identity is as
hard as finding a BLAKE3 collision.

**What it does NOT claim.** It does not claim that two things the plane considers
different always have different transcripts; that is each mint site's
completeness, documented at each mint site. It does not claim anything across
profile versions, which derive under different contexts and are different name
spaces. It does not make a plane identity into a machine commitment. And it makes
no claim at all about keyed use, authentication, or protection: the profile uses
`derive_key` for domain separation and holds no secret.

**What it replaced.** An in-house four-lane FNV-shaped fold that explicitly
claimed no collision resistance at any width, and that reduced both the anchor
and the content to eight bytes before hashing. That scheme is retired outright —
deleted, not re-hashed. Hashing a lost-information fold with a strong digest
would have produced a strong-looking value carrying a weak preimage, which is a
worse position than the honest nonclaim it started from.

## The working law: a failed required seat is a typed refusal

**A required seat is never repaired with an empty, default, or neighbouring
value after construction fails — a failed required seat is a typed refusal.**

This is a law about what the services do at the moment something impossible
happens. A checked seam returns a `Result`; a caller that has no honest value for
the failing case reaches for the nearest one it can see, and the nearest one is
always wrong in the same three ways:

- **empty** — a rendering that did not fit becomes a blank explanation;
- **default** — an owner fact nobody cited stands in for the one the plan
  declares;
- **neighbouring** — the first member of a set stands in for the member under a
  role, the first rendered unit's digest stands in for the digest of the unit
  this seat is about.

Each of those produces a value that is well-formed, complete-looking, and about
something else. That is strictly worse than a refusal, because everything
downstream then proves the wrong claim correctly: a membership shortened to one
member is closed over, at one member, and the closure is honest about a plan that
is not.

The law has two halves, and the first is the one that does the work.

**Where the failing case cannot happen, the road must not have it.** A complete
set fixed by a shape, a static rendering whose length is a compile-time fact, a
roster with a known arity — none of these has a runtime count to read, so none of
them returns a `Result`. `PlannedMembership::complete`,
`RenderedProjection::complete`, `NonEmptyBounded::from_array`, and the seam behind
`human_projection!` are *total structural* constructors: the bound is settled by
const evaluation and there is no error branch for a caller to fill. This is the
half that removes the temptation rather than policing it.

**Where the failing case can happen, the refusal is typed and it propagates.** It
names the seat (`ExplanationBindingRefusal::RequiredOutputAbsent`), the role
(`ClosureIssue::MemberPlannedTwice`), the axis and magnitude
(`ProjectionPlanningIssue::BoundExceeded`), or the bound it overran
(`CaptureBound`). It reaches a caller as a diagnostic that keeps those
distinctions — one related identity per established issue, the first issue's own
classification, and a summary composed from the typed values.

Saturating a numeric conversion to `MAX` while REPORTING a count is not this
defect and is not touched by the law: it is a rendering of a number too large to
render, inside a refusal that has already been established.

## Tooling obligations are their own category

A core semantic obligation is a claim about the MACHINE — what a home's types
make unrepresentable, what a law proves, what a reversal breaks. Its denominator
is the red-twin ledger.

A **tooling qualification obligation** is a claim about a TOOL — what a service
refuses, what a check catches, what a judge is rehearsed against. It has its own
denominator and its own reversals, and the two are never added together. A
repository that reported "178 obligations, 5 discharged" over both populations at
once would be reporting a number nobody can act on: the two populations are
challenged by different methods, owned by different homes, and are complete on
different days. `cargo xtask check` prints them apart, always, on every run.

Each block below binds seven things: the CLAIM, the OWNER module, the POSITIVE
control, the REVERSAL, the ACTIVATION route, the METHOD, and the NONCLAIMS.

```yaml
tooling-obligation: macroc.capture-refuses-a-malformed-declaration
  claim: >
    A declaration the authored grammar does not admit reaches the compiler as a
    refusal naming the established cause and the offending token — never as a
    silent empty expansion and never as a smaller success.
  owner: macros/macroc/src/derive_refusal/capture.rs
  positive: xtask/fixtures/macro-consumer/src/lib.rs
  method: compile-refusal
  activation: cargo test -p threadpak-testpak --test compile_refusals
  tooling-red: testpak/tests/compile-fail/a-malformed-refusal-declaration-refuses.rs
  nonclaims: >
    It does not claim the grammar admits every well-formed Rust enum, and it does
    not claim the refusal text is stable across releases.

tooling-obligation: macroc.the-receipt-rich-road-is-the-only-road
  claim: >
    Tokens are emitted only from a closed expansion, and a closed expansion
    exists only once the plan, the origin graph, the trace, the rendering, the
    closure, and the explanation have all been produced and have agreed.
  owner: macros/macroc/src/derive_refusal/mod.rs
  positive: macros/macroc/src/laws.rs
  method: executable-law
  activation: cargo test -p threadpak-macroc
  tooling-red: testpak/tests/compile-fail/ — one fixture per unwritable road: a
    receipt bound without a closure, a rendering taken off the membership-only
    draft, a closure minted without proving. Deleting the plan, the origin graph,
    the trace, the invalidation set, or the explanation dies at the same private
    constructor as deleting the closure, and one fixture proves that gate.
  nonclaims: >
    It does not claim the rendering is correct Rust; that is lane C's claim, and
    it is made by the consumer fixtures.

tooling-obligation: macroc.the-crate-binding-travels
  claim: >
    A consumer that renamed its dependency gets a rendering naming the crate by
    the name that consumer uses, because the binding is captured, planned,
    explained, and rendered rather than assumed.
  owner: macros/macroc/src/derive_refusal/render.rs
  positive: xtask/fixtures/renamed-consumer/src/lib.rs
  method: compiled-behaviour
  activation: cargo test -p threadpak-renamed-consumer
  tooling-red: owed-to-testpak — a renderer hardcoding the default binding
  nonclaims: >
    It does not claim the machine is reachable under an arbitrary path; only
    under a crate name the consumer's own manifest declares.

tooling-obligation: macroc.the-identity-profile-is-pinned-and-separated
  claim: >
    Every plane identity is BLAKE3 over a complete transcript under a versioned,
    domain-separated context; three golden vectors pin the exact derivation, one
    bit anywhere in the transcript moves the identity, two members swapped move
    it, and one transcript under two roles or two subjects derives two
    identities.
  owner: macros/macroc/src/plane.rs
  positive: macros/macroc/src/laws.rs
  method: executable-law
  activation: cargo test -p threadpak-macroc identity_profile
  tooling-red: testpak/tests/independent_identity_transcript.rs
  nonclaims: >
    It claims nothing keyed and nothing authenticated, nothing across profile
    versions, and nothing about a machine commitment. It does not claim any mint
    site's transcript is complete for that site's question; that is stated at
    each mint site.

tooling-obligation: macroc.the-transcript-specification-is-complete
  claim: >
    The published transcript specification says enough for an implementation
    sharing none of the producer's encoding to derive the same identity,
    including for a captured declaration the services actually read.
  owner: macros/macroc/src/plane.rs
  positive: testpak/tests/independent_identity_transcript.rs
  method: independent-reconstruction
  activation: cargo test -p threadpak-testpak --test independent_identity_transcript
  tooling-red: testpak/tests/independent_identity_transcript.rs
  nonclaims: >
    It does not claim BLAKE3 itself is independently implemented; both sides call
    the same admitted digest, because the digest is not what is under judgement.

tooling-obligation: macroc.a-failed-required-seat-is-a-typed-refusal
  claim: >
    No required seat in the services is repaired with an empty, default, or
    neighbouring value after construction fails. Where the failing case cannot
    happen the road carries no refusal at all; where it can, the refusal is typed,
    names its seat, and survives into the diagnostic as its own classification and
    its own related identity.
  owner: macros/macroc/src/derive_refusal/diagnose.rs
  positive: macros/macroc/src/laws.rs
  method: executable-law
  activation: cargo test -p threadpak-macroc failure_path_closure
  tooling-red: testpak/tests/failed_seat_refusals.rs — the plane restores each
    killed repair itself, out of the values a lawful compilation hands back, and
    shows the repaired value is about another subject while the seam refuses
  nonclaims: >
    It does not claim every seat in the services is required; a declared default,
    such as the crate binding a consumer did not rename, is a stated posture and
    not a repair. It does not claim a saturating numeric conversion inside a
    refusal body is covered: rendering a count too large to render is not
    answering a seat with a neighbouring value.

tooling-obligation: macroc.the-emitted-tree-is-inside-the-closure-proof
  claim: >
    The exact token stream an expansion emits is joined by the closure, owned by
    it, and named by its identity: the closure transcript commits to the joined
    tree's digest, and a closed expansion emits the closure's own tree rather than
    a second concatenation performed after the proof returned.
  owner: macros/macroc/src/closure.rs
  positive: macros/macroc/src/laws.rs
  method: executable-law
  activation: cargo test -p threadpak-macroc failure_path_closure
  tooling-red: testpak/tests/compile-fail/a-post-proof-join-outside-the-closure.rs
    — a mutant re-opening a public post-proof join has to make the join
    reachable, and while it is not, the fixture does not compile
  nonclaims: >
    It does not claim the joined tree is correct Rust; that is lane C's claim and
    the consumer fixtures make it.

tooling-obligation: macroc.a-declared-input-stands-under-four-magnitudes
  claim: >
    Every producer of captured input — the expansion shell and the callable text
    reader alike — walks under one declared nesting depth, one per-level token
    magnitude, one whole-tree token magnitude, and one capture-work budget, and
    exceeding any of them refuses naming that bound before any partial tree
    exists. Each captured token carries the index route from the root, which
    locates exactly one token.
  owner: macros/macroc/src/token.rs
  positive: macros/macroc/src/laws.rs
  method: executable-law
  activation: cargo test -p threadpak-macroc failure_path_closure
  tooling-red: testpak/tests/declared_magnitudes.rs — the plane implements the
    killed depth-and-index coordinate itself and shows two distinct tokens
    colliding under it, and drives every magnitude both directions
  nonclaims: >
    It does not claim the budget binds before the whole-tree magnitude for
    today's two producers; both keep every token they examine, so the tree
    magnitude bites first, and the budget is what bounds a producer that reads
    material it discards.

tooling-obligation: macroc.the-generator-identity-is-a-deliberate-fact
  claim: >
    A plan names the generator that produced it by a declared profile name and a
    deliberately bumped schema version, so the identity a plan watches for
    invalidation is a value that moves when the rendered shape moves.
  owner: macros/macroc/src/plane.rs
  positive: macros/macroc/src/laws.rs
  method: executable-law
  activation: cargo test -p threadpak-macroc identity_profile
  tooling-red: owed-to-xtask — a declared generator-source digest joined by a check
  nonclaims: >
    It does not claim the generator's SOURCE is committed to. The schema version
    is authored, so a rendered-shape change that nobody bumped it for is not
    caught by anything here.
```

## What the generator identity still owes

The generator identity names WHICH generator and WHICH rendered shape. It does
not name which SOURCE, and the honest reason is that the services cannot compute
that: reading their own source tree at expansion time is an ambient read the
plane forbids, and a build script that computed one before the expansion ran is
forbidden outright in this repository. So no self-digest is carried, rather than
one carried dishonestly.

The lawful shape of the missing piece is the declared-constant-plus-join pattern
this repository already uses for the toolchain pin, the workspace members, and
the band map: a declared source-digest constant in the services, verified by an
`xtask` check that hashes the services' source tree deterministically and refuses
when the two disagree. It is owed to `xtask` and not written, and it is not
written yet because it needs a ruling this boundary did not carry — the constant
would be a member of the tree it commits to, so the check needs a stated
fixed-point rule and a stated update road before it exists, or every ordinary
edit to this crate would fail the gate until somebody re-pasted a digest by hand.
Naming what it needs is the sequencing; carrying a fabricated digest would not be.

## What the services never do

They decide no meaning. The three body shapes are band 00's; the canonical cause
key grammar is band 00's; the selection order's content is the author's. A
tooling type may summarize, reference, plan, explain, or project an owner fact;
it may never create a second value that independently answers the owner's
semantic question — which is why an unanchored diagnostic says it is unanchored
rather than carrying a minted stand-in.
