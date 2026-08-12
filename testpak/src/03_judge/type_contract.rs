//! The judge seat's declarative surface: the closed table the mutation roster
//! is read through.
//!
//! # A mutation names the lane that owns it, and the lanes that do not
//!
//! **Not every lane catches every mutation, and pretending otherwise is the
//! defect this table exists to prevent.** A byte scan anchored on
//! `const SELECTION_ORDER` cannot tell a real constant from the same bytes
//! sitting inside a comment, and it has no opinion at all about what item the
//! constant is a member of. Recording "lane A catches this" for a mutation lane
//! A cannot see would turn the whole ledger into a green wall that measures
//! nothing.
//!
//! So [`ArtifactMutation::owned_by`] states, per mutation, which lane the
//! catching claim belongs to. Two of the three lanes read text and are seated in
//! this package — the byte scan in [`crate::judge::byte_profile`] and the
//! structural read in [`crate::judge::structural`] — and each is held, in
//! `tests/planted_defect.rs`, to exactly the mutations recorded against it and
//! to no others.
//!
//! The third lane needs a compiler, so its evidence is a compiled seat rather
//! than a reader: `tests/compiled_behaviour.rs` materializes the two mutations
//! recorded against it, hands them to `rustc`, and reads back a refusal to
//! compile and a disagreeing VALUE. That file also enumerates this roster's
//! compiled-behaviour rows, so a mutation recorded here without a compiled seat
//! fails rather than sitting in the ledger looking like coverage.
//!
//! Both tables are declarations rather than computations: a constant answer per
//! variant, stated rather than derived. The damage a mutation inflicts is
//! `mutation.rs`, and the roster itself is `types.rs`.

use super::types::{ArtifactMutation, LaneOwnership};

impl ArtifactMutation {
    /// Which lane's claim covers catching this mutation.
    ///
    /// Read this as a ledger of what is CLAIMED, not of what is comfortable. The
    /// nine that name [`LaneOwnership::Structural`] and the two that name
    /// [`LaneOwnership::CompiledBehaviour`] are not caught by the byte scan and
    /// are not recorded as though they were.
    ///
    /// Ownership is the seat of the CLAIM, not an exclusivity boast: the
    /// structural read happens to notice a permuted order too, and says nothing
    /// about it, because that verdict is stated over lane A's method and belongs
    /// to lane A's row.
    #[must_use]
    pub const fn owned_by(self) -> LaneOwnership {
        match self {
            // The first pair changes the exact spellings or the exact identities
            // the scan reads out of the anchored forms. The second pair changes
            // how MANY `CauseId` forms the artifact carries, which the scan's
            // magnitude check sees. Different reasons, one lane.
            Self::OrderPermuted
            | Self::IdentityRecycled
            | Self::PlannedOutputOmitted
            | Self::OutputDuplicated => LaneOwnership::ByteProfile,
            // What item is this, what does it target, which trait does it
            // realize, how is it written, does it exist at all under some `cfg`,
            // and is that constant a member of it once or twice — none of those
            // is a question about bytes.
            Self::ImplTargetAltered
            | Self::TraitPathWrong
            | Self::UnplannedOutputAdded
            | Self::DecoyInComment
            | Self::ImplMemberDuplicated
            | Self::ImplMemberUnexpected
            | Self::ConstructorPathAltered
            | Self::ImplPostureAltered
            | Self::MeaningBearingAttributeAdded => LaneOwnership::Structural,
            // A changed shape word and a malformed artifact are both caught
            // where the artifact is compiled and read back as values.
            Self::ShapeAltered | Self::MalformedRust => LaneOwnership::CompiledBehaviour,
        }
    }

    /// The mutation rendered for a person. A projection: nothing reads it back.
    #[must_use]
    pub const fn described(self) -> &'static str {
        match self {
            Self::OrderPermuted => "the textual selection order is reversed",
            Self::IdentityRecycled => "every cause is emitted under one identity",
            Self::PlannedOutputOmitted => "a planned output is deleted",
            Self::UnplannedOutputAdded => "an unplanned output is appended",
            Self::ImplTargetAltered => "the implementation targets a different type",
            Self::ShapeAltered => "the declared body shape is changed",
            Self::OutputDuplicated => "a planned output is emitted twice",
            Self::TraitPathWrong => "the trait path names a different contract",
            Self::DecoyInComment => "the anchored bytes are planted in a comment",
            Self::ImplMemberDuplicated => "one member constant is emitted twice",
            Self::ImplMemberUnexpected => "a member nobody planned joins the implementation",
            Self::ConstructorPathAltered => "a row is built through another constructor",
            Self::ImplPostureAltered => "the implementation is written under another posture",
            Self::MeaningBearingAttributeAdded => "an attribute that decides something is added",
            Self::MalformedRust => "the artifact stops being well-formed Rust",
        }
    }
}
