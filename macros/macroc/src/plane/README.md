# plane — the metaprogramming plane's shared carriers

The two identity families, owner-fact references, profile versions, bounded human
projections, and the plane's declared limit families.

## Two identity families, and neither can stand in for the other

**[`OwnerIdentityRef`] is a read-only lens on an identity the MACHINE minted.**
The machine's identity home mints; the services never do. A lens arrives through
[`OwnerIdentityRef::of_commitment`], which reads a machine commitment's published
bytes and adapts nothing: identity, schema, authority, bounds, and meaning cross
unchanged, which is exactly what a projection is allowed to do. Holding one means
only "the compiler refers exactly to this owner identity" — nothing about
admission, authority, freshness, or equivalence. There is no public raw-byte road
at all.

**[`ProjectionIdentity`] is an identity the COMPILER PLANE owns.** Plans, origin
nodes, rendered units, generated units, closures, and bundles are the plane's own
material: the machine has no opinion about them and mints nothing for them, so
the plane names them itself. Every one is derived deterministically from a
COMPLETE [`ProjectionTranscript`] under the versioned, domain-separated profile
[`PROJECTION_IDENTITY_PROFILE`], and the derivation record — which subject, which
role, which profile version, which transcript members — is a separate inspectable
value, [`ProjectionProvenance`], carried once where the derivation happened
rather than inside every identity.

The two families are different types over different subject markers and neither
converts to the other. A plane identity is never accepted by the machine as a
mint, and an owner lens is never derived by the plane.

## Human text is never load-bearing

[`HumanProjection`] carries bytes a caller may show a person. Nothing in the
plane reads it back, matches on it, or decides from it — every decision cites an
[`OwnerFactRef`] or a typed value instead.

## The seats

`types.rs` declares; its own child `type_guard.rs` holds every road that reaches
a private field, which is what makes the walls above structural.
`type_contract.rs` states the one declarative roster. `encode.rs` owns the single
length framing every canonical encoding in the services is written through, and
`transcript.rs` is the transcript specification as code.

This home's qualification obligations live in the crate README's tooling-obligation
blocks.
