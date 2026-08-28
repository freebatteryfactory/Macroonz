# identity

Thirty-two bytes that say which thing this is, and a transcript that says why.

An identity here is never a handle, a counter, or a name somebody picked.
It is a BLAKE3 derivation over a complete declared preimage under an owner-qualified key space.
The same transcript derives the same bytes on every run and host, while a change to an identity-bearing fact moves the result.

## Complete declared material

Each mint commits to the whole material its semantic owner says distinguishes the subject.
Framing preserves member boundaries, anchoring names the fact an identity stands under, and the role and position state the seat the material occupies.
The public [`Transcript`](super::Transcript) contract owns the exact preimage grammar and [`Transcript::encoded`](super::Transcript::encoded) is its implementation.

Nothing folds an anchor or its material before derivation.
The final identity is the only compression in the road.

## Subjects belong to their owner

[`Subject`](super::Subject) is open so an adopter declares its own key spaces under its own stem.
The [`subjects!`](crate::subjects) macro declares a roster and refuses names that would not remain distinct inside that owner's context.

```rust
macroonz_compiler::subjects! {
    stem = "my-crate/identity";
    /// One obligation a trial row challenges.
    Obligation = "obligation",
}

use macroonz_compiler::Subject;

assert_eq!(Obligation::STEM, "my-crate/identity");
assert_eq!(Obligation::NAME, "obligation");
```

Two subjects with the same name under different stems occupy unrelated key spaces.
The compiler's own subjects live under [`MACROONZ_STEM`](super::MACROONZ_STEM), while an adopter's subjects remain in the adopter's vocabulary.

A projection kind is qualified by the producer namespace and name on its request door before the kind's declared name is derived.
A projection-content commitment then joins that kind identity, the content's canonical bytes, and the captured declaration it stands under.
Changing any of those facts moves the binding before a plan exists.

## One version per grammar

A [`Profile`](super::Profile) names one preimage grammar and carries that grammar's [`Version`](super::Version).
Changing a version renames identities derived under that profile and does not rename another profile's identities.
This keeps independent grammars from moving together merely because they share the identity machinery.

A version moves when the preimage grammar moves and a holder must distinguish the old grammar from the new one.
If no identity is minted or held under a declared grammar, changing an unconsumed position does not by itself preserve or create a compatibility claim.

## Provenance is not identity

[`Provenance`](super::Provenance) records which generator produced a value.
The value's transcript states what the value is, so a generator-shape change does not rename an unchanged semantic value.
When generator output changes an identity-bearing fact, that fact reaches the appropriate owning transcript instead.

## Caller-owned citations

[`OwnerIdentity`](super::OwnerIdentity) lets a caller cite identity bytes it minted under its own authority.
This home does not reinterpret or validate those bytes.

[`OwnerFact`](super::OwnerFact) cites a fact by its declaring home and name.
Selections, omissions, and non-applicability use that citation rather than a bare boolean with no semantic owner.

[`HumanProjection`](super::HumanProjection) carries bounded bytes intended for a person to read.
It is display material, never an input to identity or judgment.

## Boundary

This home owns identity subjects, profiles, transcripts, derivation, citations, and the byte framing they share.
Each semantic owner remains responsible for the completeness and ordering of the material it hands to a transcript.
This home neither mints identities on a consumer's behalf nor reads identity bytes back as policy.
