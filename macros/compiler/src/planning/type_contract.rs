//! The plan family's declarative trait implementations, and the constant
//! answers its closed rosters settle.
//!
//! The kind roster's own [`ProjectionKind`] implementations are written by the
//! `kinds!` declaration in `types.rs`, because a kind's contract is the
//! declaration rather than something added to it afterwards.
//! What stands here is the one roster a kind names rather than declares: the three rendered roles an implementation projection may materialize, each stated with a constant slot and a constant sentence.
//! The same roster gives every role one constant delivery answer.
//! The destination roster gives every destination one constant emission answer, so no reader decides either mapping a second time.
//!
//! Constant answers and never derivations: a fourth role admitted later stops the compiler in each `match` below until its slot, description, and delivery are stated, and a fifth destination stops it until its emission is stated.
//!
//! [`ProjectionKind`]: super::ProjectionKind

use super::{EmissionPartition, MemberDestination, RenderedImplementation};
use crate::plane::{RenderedRole, RenderedRoleSeal};

impl RenderedRole for RenderedImplementation {
    const SEAL: RenderedRoleSeal = RenderedRoleSeal::admitted();
    const ROLES: &'static [Self] = &[
        Self::RenderedFamilyImpl,
        Self::RenderedCauseOrderImpl,
        Self::RenderedMutationEvaluation,
    ];

    fn slot(self) -> u32 {
        match self {
            Self::RenderedFamilyImpl => 0,
            Self::RenderedCauseOrderImpl => 1,
            Self::RenderedMutationEvaluation => 2,
        }
    }

    fn described(self) -> &'static str {
        match self {
            Self::RenderedFamilyImpl => "the family contract's production implementation",
            Self::RenderedCauseOrderImpl => "the typed cause order's production implementation",
            Self::RenderedMutationEvaluation => {
                "the generated mutation discovery and evaluation module"
            }
        }
    }
}

impl RenderedImplementation {
    /// Which delivery a member under this role is written into once it is rendered.
    ///
    /// Production implementations land at the declaration site.
    /// The generated mutation module rides the test carrier, where `TestPak` supplies the resolved directive and no mutation control enters the normal build.
    ///
    /// This is a constant answer over the closed roster, so a plan that writes a mutation member at the declaration site or a production member as a standalone artifact refuses against this owner instead of inventing another delivery.
    /// The plan and rendering consume this same answer and cannot disagree through duplicated literals.
    #[must_use]
    pub const fn destination(self) -> MemberDestination {
        match self {
            Self::RenderedFamilyImpl | Self::RenderedCauseOrderImpl => {
                MemberDestination::AtDeclarationSite
            }
            Self::RenderedMutationEvaluation => MemberDestination::IntoTestCarrier,
        }
    }
}

impl MemberDestination {
    /// The emission a member written to this destination belongs to.
    ///
    /// Total, and the ONE road from a destination to a partition: the join reads
    /// it, the closure's proof reads it, and a consumption target routing cargo
    /// reads it, so three readers asking which delivery a member belongs to get
    /// one answer rather than three matches that agree until one is edited.
    ///
    /// The artifact arm's byte role does not take part. Which artifact a member
    /// is written as is that member's ADDRESS, and the address is what separates
    /// two artifacts from each other inside the one publication emission; it is
    /// not what separates a publication from a build.
    ///
    /// # Bounds
    ///
    /// It is a total function and never an injection: every destination reads to
    /// exactly one partition, and a partition may be reached by more than one
    /// destination the moment a second destination is admitted into it. Nothing
    /// here reads back — a partition names no destination, because a partition
    /// is what several members share and a destination is what one member
    /// declared.
    #[must_use]
    pub const fn partition(self) -> EmissionPartition {
        match self {
            Self::AtDeclarationSite => EmissionPartition::DeclarationSite,
            Self::IntoTestCarrier => EmissionPartition::TestCarrier,
            Self::IntoBenchCarrier => EmissionPartition::BenchCarrier,
            Self::AsArtifact { .. } => EmissionPartition::PublicationArtifact,
        }
    }
}
