//! The closure home's declarative surface: the refusal family this home
//! declares, the closed table its issue roster is read through, and the proof
//! contract a complete explanation is answered over.
//!
//! All three are declarations rather than computations.
//! Nothing here decides anything — the deciding is `prove.rs`, and the proving
//! is `type_guard.rs`.

use super::ClosureIssue;
use crate::explanation_protocol::{ClosureProofSeal, ProvedClosure};
use crate::plane::{ClosureId, RenderedRole};
use macroonz::{FamilyShape, RefusalFamily};

use super::{ProjectionClosure, ProjectionClosureRefusal};

impl<R: RenderedRole> RefusalFamily for ProjectionClosureRefusal<R> {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
}

/// The proof a complete explanation is answered over is THIS proof, and there is
/// no other.
///
/// The explanation-protocol home declares the contract because it is declared
/// earlier than this one — its terminal seat is what this home's binding
/// consumes — and a home cannot name a type declared after it without the module
/// order carrying a backward edge. The contract is sealed, so this
/// implementation is the only one there can be: a view answered "over a closure"
/// was answered over a value somebody proved.
///
/// It hands back the proof's own NAME and nothing else. What was proved, what it
/// partitioned, and what it delivers are this home's surface, and a view reads
/// none of them.
impl<R: RenderedRole> ProvedClosure for ProjectionClosure<R> {
    const SEAL: ClosureProofSeal = ClosureProofSeal::admitted();
    type Rendered = R;

    /// The inherent road, named explicitly: a closure's identity is stated once,
    /// on the closure, and this contract is a reading of it rather than a second
    /// answer.
    fn identity(&self) -> ClosureId {
        ProjectionClosure::<R>::identity(self)
    }
}

impl<R: RenderedRole> ClosureIssue<R> {
    /// The issue kind's position in the declared roster, written ahead of the
    /// issue's own material so two kinds never encode alike.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::MemberMissing { .. } => 0,
            Self::MemberUnplanned { .. } => 1,
            Self::MemberDuplicated { .. } => 2,
            Self::OriginOrphan { .. } => 3,
            Self::DigestMismatch { .. } => 4,
            Self::SemanticKeyMismatch { .. } => 5,
            Self::MaterializationMismatch { .. } => 6,
            Self::MemberPlannedTwice { .. } => 7,
            Self::MembershipDisagreement { .. } => 8,
            Self::ReconstructionEmpty => 9,
            Self::ReconstructionUndeclarable { .. } => 10,
            Self::JoinedTreeUnbounded { .. } => 11,
            Self::ArtifactAddressDoubled { .. } => 12,
        }
    }

    /// The role this issue was established at, where it is about one.
    #[must_use]
    pub const fn role(&self) -> Option<R> {
        match self {
            Self::MemberMissing { role }
            | Self::MemberUnplanned { role }
            | Self::MemberDuplicated { role, .. }
            | Self::OriginOrphan { role }
            | Self::DigestMismatch { role }
            | Self::SemanticKeyMismatch { role }
            | Self::MaterializationMismatch { role }
            | Self::MemberPlannedTwice { role, .. }
            | Self::MembershipDisagreement { role }
            | Self::ArtifactAddressDoubled { role, .. } => Some(*role),
            Self::ReconstructionEmpty
            | Self::ReconstructionUndeclarable { .. }
            | Self::JoinedTreeUnbounded { .. } => None,
        }
    }

    /// How the two disagreed, rendered for a person.
    /// A projection of the typed value: nothing reads it back.
    #[must_use]
    pub const fn described(&self) -> &'static str {
        match self {
            Self::MemberMissing { .. } => "a planned role nothing materialized",
            Self::MemberUnplanned { .. } => "a rendered role nothing planned",
            Self::MemberDuplicated { .. } => "a role rendered more than once",
            Self::OriginOrphan { .. } => "a rendered unit whose origin is not the planned one",
            Self::DigestMismatch { .. } => "a digest that is not the digest of the bytes rendered",
            Self::SemanticKeyMismatch { .. } => "the planned role, answering to another key",
            Self::MaterializationMismatch { .. } => {
                "a destination or profile the plan did not name"
            }
            Self::MemberPlannedTwice { .. } => "a role the plan itself declared twice",
            Self::MembershipDisagreement { .. } => {
                "the rebuilt membership and the planned one are not the same set under this role"
            }
            Self::ReconstructionEmpty => "the rebuild produced no member at all",
            Self::ReconstructionUndeclarable { .. } => {
                "the rebuild will not declare as a complete output set"
            }
            Self::JoinedTreeUnbounded { .. } => {
                "one emission's joined token tree outgrows its declared magnitude"
            }
            Self::ArtifactAddressDoubled { .. } => "two published units stand at one address",
        }
    }
}
