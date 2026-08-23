//! The plan family's declarative trait implementations, and the constant
//! answers its closed rosters settle.
//!
//! The kind roster's own [`ProjectionKind`] implementations are written by the
//! `kinds!` declaration in `types.rs`, because a kind's contract is the
//! declaration rather than something added to it afterwards.
//! What stands here is the one roster a kind names rather than declares: the
//! rendered roles an implementation projection materializes, stated as a constant
//! roster, a constant slot, and a constant sentence — together with the three
//! facts that roster settles for every seat on it, each written as a constant
//! answer over the closed roster: which role is the other half of a role's PAIR,
//! which HALF of that pair a role is, and which DELIVERY a member under a role
//! is written into.
//! The destination roster's own constant answer stands here for the same reason:
//! which emission a destination reads to is settled once, over the closed
//! roster, so no reader decides it a second time.
//!
//! Constant answers and never derivations: a fifth role admitted later stops the
//! compiler in each `match` below until somebody says which role it pairs with,
//! which half of that pair it is, and which delivery it is written into — and a
//! fifth destination stops it until somebody says which emission carries it.
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
    /// Which delivery a member under this role is written into once it is
    /// rendered.
    ///
    /// Production implementations land at the declaration site. The generated
    /// mutation module rides the test carrier, where TestPak supplies the
    /// resolved directive and no mutation control enters the normal build.
    ///
    /// Stated as a constant answer over the closed roster rather than read off a
    /// planned member, so a plan that wrote an evaluation member at the
    /// declaration site — or a production member as a standalone artifact — is a
    /// plan the reading roads refuse against this answer instead of a delivery
    /// nobody recognized. The plan and the rendering both take this one answer,
    /// so the two cannot disagree about a delivery by disagreeing about a
    /// literal.
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
