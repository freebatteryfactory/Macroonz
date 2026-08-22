# plane — the metaprogramming plane's shared carriers

The two identity families, the preimage-family roster and the profile version each one carries, owner-fact references, bounded human projections, the magnitude stamp every limit family in the services is stamped by, and the plane's own magnitude rows.

Every limit family states its magnitude once, in one row, and carries it at both widths its readers hold it in: a collection's and a counter's.

## One owner per question

**The plane owns the MECHANISM. Each semantic home owns its own ROWS.**

The mechanism is `limits!`:

the declaration stamp that turns one magnitude row into a capacity authority, a compile-time ladder, and the two integer widths its readers hold it at.

That is the same work for every row in the services, so it is written once, here, and a home that spelled it out again would be carrying a second copy of one mechanism rather than a second opinion about anything.

A row is the meaning:

which capacity it governs, what number it states, and why that number and not another. That is the question a semantic home owns, so the row is declared in that home, beside the capacity it bounds — invoked as `crate::plane::limits! { … }` in the home's own `types.rs`.

The stamp is a published in-crate road and not a private one; a home reaching it is using the plane's mechanism, not borrowing the plane's authority.

A magnitude belongs on the rows in THIS home when more than one home asks its question, and belongs in a semantic home when only that home asks it.

Every row still seated here answers a question at least two homes ask; the rows that answered one home's question have been moved to that home, carrying their numbers and their reasons unchanged.

Moving a row is a decision about which home owns the question — never an edit, never a renumbering, and never something a home does to a neighbour's row.

A magnitude index over the WHOLE crate, if one is ever wanted, is a generated projection over the home rows: `limits! { roster NAME; … }` emits one from the same rows it stamps, so an index is those rows read a second way and never a second owner of any of them.

**A hand-kept central table would be exactly the second owner this law exists to refuse.**

## The two identity families

**[`OwnerIdentityRef`] is a read-only lens on an identity the MACHINE minted.**

The machine's identity home mints; the services never do. A lens arrives through [`OwnerIdentityRef::of_commitment`], which reads a machine commitment's published bytes and adapts nothing: identity, schema, authority, bounds, and meaning cross unchanged, which is exactly what a projection is allowed to do. Holding one says that the compiler refers to this owner identity, and says nothing else.

**[`ProjectionIdentity`] is an identity the COMPILER PLANE owns.** Plans, origin nodes, rendered units, generated units, closures, and bundles are the plane's own material:

the machine has no opinion about them and mints nothing for them, so the plane names them itself. Every one is derived deterministically from a COMPLETE [`ProjectionTranscript`], under a versioned, domain-separated profile its own preimage family declares, and the record of that derivation is a separate inspectable value, [`ProjectionProvenance`], carried once where the derivation happened rather than inside every identity.

The two families are different types over different subject markers and neither converts to the other. A plane identity is never accepted by the machine as a mint, and an owner lens is never derived by the plane.

## One version per grammar

**Each canonical preimage grammar is a [`PreimageFamily`], and each family carries its own version.** A version is what renames: it rides the derive-key context and the transcript, so moving it renames every identity derived under it.

One position shared by every grammar therefore renames the whole tree whenever any one grammar widens — a rendering-shape change renames the intent identity of a door whose meaning did not move, and the equivalence that door's builder compares stops answering the question it was asked.

So the position is split. A family's version moves when THAT family's preimage grammar moves and at no other time, and each family's `Versions` prose states which of the retired single position's five moves would have moved it and which would not.

The derive-key context is `<stem>/<family>/v<version>/<subject>/<role>`, with the family segment ahead of the version, so position one of one family and position one of another are two key spaces rather than one reached twice. The stem is [`IDENTITY_PROFILE_STEM`] and is shared;

the family segment is a row of a closed roster, where a per-family stem would be a literal a declaration could repeat.

**The family is read off the role, never passed.** A mint site names the role it derives for, [`ProjectionRole::family`] answers which grammar that role stands in, and [`PreimageFamily::profile`] answers which version — so no call site carries a second opinion about which ladder a preimage belongs to.

## The generator is provenance

**[`MACROC_GENERATOR`] names which generator produced a value, and no preimage names it.** It is recorded on [`ProjectionProvenance`], read back, and compared for staleness by [`ProjectionProvenance::under_current_shape`] — and it is written into no transcript, so the same exact rendered bytes stay the same artifact across the producers that emitted them.

Where a generator's shape genuinely changes what something IS, the change reaches identity through the seat that states it:

a plan whose rendered-role roster grew declares a different membership, and a membership is a plan's own transcript member.

## Human text

[`HumanProjection`] carries bytes a caller may show a person.

Nothing in the plane reads it back, matches on it, or decides from it — every decision cites an [`OwnerFactRef`] or a typed value instead.

## The seats

`types.rs` declares — the subject roster, the preimage-family roster and the profile constant each family's version lives on, the magnitude stamp, and the plane's own magnitude rows — and its own child `type_guard.rs` holds every road that reaches a private field.

`type_contract.rs` states the one declarative roster and the two constant answers the closed rosters settle — which profile a family versions under, and which family a role stands in — `encode.rs` owns the single length framing every canonical encoding in the services is written through, and `transcript.rs` is the transcript specification as code.
