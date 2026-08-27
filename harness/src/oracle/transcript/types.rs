//! The independent-transcript derivation and verdict vocabulary.

#[path = "type_guard.rs"]
mod guard;

/// One member of a preimage, as this lane writes it.
///
/// The roster is the closed set of encoding decisions the lane makes on its own, from what a published specification states.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TranscriptMember {
    /// A length-prefixed byte string: eight big-endian length bytes, then the bytes themselves.
    Framed(Vec<u8>),
    /// One bare byte — the slot a specification assigns to a variant.
    Discriminant(u8),
    /// One 32-bit number, four big-endian bytes, unframed because its width is fixed.
    Fixed32(u32),
    /// One 64-bit number, eight big-endian bytes, unframed for the same reason.
    Fixed64(u64),
}

/// One preimage this lane composes for itself, member by member.
///
/// Every byte is written here from typed arguments a caller took off a published specification: not a framing, not a field order, not a spelling is imported from the producer whose identity is under judgement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TranscriptDerivation {
    members: Vec<TranscriptMember>,
}

/// One derivation context, spelled by this lane from a published grammar.
///
/// [`SpecifiedContext::spelled`] joins segments a caller writes out, and [`SpecifiedContext::under_version`] adds the `v<n>` segment a versioned profile grammar states.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecifiedContext(String);

/// Why one derivation context was refused.
#[must_use = "a refusal is the reason a context was not spelled"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextRefusal {
    /// No segments were offered, so the context names no domain.
    NoSegments,
    /// One segment is empty, which would spell a doubled separator and let two rosters name one context.
    EmptySegment {
        /// The segment's position in the assembled roster.
        at: usize,
    },
    /// One segment already contains `/`, so it is not one segment of this grammar.
    EmbeddedSeparator {
        /// The segment's position in the assembled roster.
        at: usize,
    },
}

/// The thirty-two bytes this lane derived from its own preimage.
///
/// The digest is BLAKE3's `derive_key`, and it is the one mechanism deliberately shared with the producer: the two sides differ in what they encode and in nothing else.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DerivedIdentity([u8; 32]);

/// One transcript disagreement: what this lane derived, and what the producer published.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TranscriptDisagreement {
    rederived: [u8; 32],
    published: [u8; 32],
}

/// What one independent re-derivation concluded.
#[must_use = "a verdict is what the re-derivation concluded about the published identity"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TranscriptVerdict {
    /// The specification, read independently, names the identity the producer published.
    Agrees,
    /// The two namings disagree, this way.
    Disagrees(TranscriptDisagreement),
}
