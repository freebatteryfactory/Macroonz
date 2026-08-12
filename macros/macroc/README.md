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
  tooling-red: owed-to-testpak — a controlled mutant deleting each seat in turn
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
