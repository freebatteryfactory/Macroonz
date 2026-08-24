//! The constant answers this home's issue roster settles, and the contracts a closure refusal stands under.
//!
//! Each table is total, so an issue admitted later stops the compiler in every one of them until somebody says what that row's position, sentence, class, and classification are.
//! Nothing here decides anything: the pass that establishes an issue is `prove.rs`, and the proving is `type_guard.rs`.

use super::{ClosureError, ClosureIssue};
use crate::bounded::{Bounded, Capping};
use crate::diagnostic::{
    CLOSURE_FAMILY, Family, LineBody, Observed, Phase, REPAIR_LIMIT, RefusalClass, Refused, Repair,
};
use crate::kind::Role;
use core::fmt;

impl<R: Role> ClosureIssue<R> {
    /// This row's position in the declared roster, written ahead of the issue's own material.
    ///
    /// Appended and never renumbered: the byte stands inside every identity derived over a refusal that carries it.
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
            Self::ArtifactAddressAbsent { .. } => 13,
        }
    }

    /// The seat this issue was established at, where it is about one.
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
            | Self::ArtifactAddressDoubled { role, .. }
            | Self::ArtifactAddressAbsent { role } => Some(*role),
            Self::ReconstructionEmpty
            | Self::ReconstructionUndeclarable { .. }
            | Self::JoinedTreeUnbounded { .. } => None,
        }
    }

    /// How what this issue observed differs from the contract that was expected.
    #[must_use]
    pub const fn observed(&self) -> Observed {
        match self {
            Self::MemberMissing { .. }
            | Self::ReconstructionEmpty
            | Self::ArtifactAddressAbsent { .. } => Observed::SeatAbsent,
            Self::MemberUnplanned { .. }
            | Self::MemberDuplicated { .. }
            | Self::MemberPlannedTwice { .. }
            | Self::MembershipDisagreement { .. }
            | Self::ArtifactAddressDoubled { .. } => Observed::ContractDisagreement,
            Self::OriginOrphan { .. } => Observed::OriginAbsent,
            Self::DigestMismatch { .. } | Self::SemanticKeyMismatch { .. } => {
                Observed::IdentityDisagreement
            }
            Self::MaterializationMismatch { .. } => Observed::ProfileDisagreement,
            Self::ReconstructionUndeclarable { .. } | Self::JoinedTreeUnbounded { .. } => {
                Observed::BoundExceeded
            }
        }
    }

    /// Which class of refusal a line opening with this issue is about.
    ///
    /// Two rows are magnitudes rather than disagreements: what they report is a rendering that would have passed a declared bound, and the seats it filled are not in question.
    #[must_use]
    pub const fn class(&self) -> RefusalClass {
        match self {
            Self::ReconstructionUndeclarable { .. } | Self::JoinedTreeUnbounded { .. } => {
                RefusalClass::MagnitudeNotHeld
            }
            Self::MemberMissing { .. }
            | Self::MemberUnplanned { .. }
            | Self::MemberDuplicated { .. }
            | Self::OriginOrphan { .. }
            | Self::DigestMismatch { .. }
            | Self::SemanticKeyMismatch { .. }
            | Self::MaterializationMismatch { .. }
            | Self::MemberPlannedTwice { .. }
            | Self::MembershipDisagreement { .. }
            | Self::ReconstructionEmpty
            | Self::ArtifactAddressDoubled { .. }
            | Self::ArtifactAddressAbsent { .. } => RefusalClass::RenderingNotClosed,
        }
    }
}

impl<R: Role> fmt::Display for ClosureIssue<R> {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MemberMissing { role } => {
                let seat = role.name();
                write!(
                    into,
                    "the plan declares a member at {seat} and nothing rendered one"
                )
            }
            Self::MemberUnplanned { role } => {
                let seat = role.name();
                write!(
                    into,
                    "a unit was rendered at {seat} and the plan declares none"
                )
            }
            Self::MemberDuplicated { role, observed } => {
                let seat = role.name();
                write!(into, "{observed} units were rendered at {seat}")
            }
            Self::OriginOrphan { role } => {
                let seat = role.name();
                write!(
                    into,
                    "the unit at {seat} walks back to an origin the plan did not declare"
                )
            }
            Self::DigestMismatch { role } => {
                let seat = role.name();
                write!(
                    into,
                    "the digest at {seat} is not the digest of the bytes that unit rendered"
                )
            }
            Self::SemanticKeyMismatch { role } => {
                let seat = role.name();
                write!(
                    into,
                    "the unit at {seat} answers to a semantic key the plan declared elsewhere"
                )
            }
            Self::MaterializationMismatch { role } => {
                let seat = role.name();
                write!(
                    into,
                    "the unit at {seat} names a profile or an address the plan did not declare"
                )
            }
            Self::MemberPlannedTwice { role, observed } => {
                let seat = role.name();
                write!(
                    into,
                    "the plan itself declares {observed} members at {seat}"
                )
            }
            Self::MembershipDisagreement { role } => {
                let seat = role.name();
                write!(
                    into,
                    "the rebuilt membership and the planned one are not the same set at {seat}"
                )
            }
            Self::ReconstructionEmpty => into.write_str("the rebuild produced no member at all"),
            Self::ReconstructionUndeclarable { observed } => write!(
                into,
                "the {observed} rebuilt members will not declare as a complete output set"
            ),
            Self::JoinedTreeUnbounded { destination } => {
                let delivery = destination.name();
                write!(
                    into,
                    "the tokens joined for {delivery} outgrow the declared magnitude"
                )
            }
            Self::ArtifactAddressDoubled { role, address } => {
                let seat = role.name();
                let subject = address.subject;
                write!(
                    into,
                    "the artifact at {seat} stands at an address under {subject} already taken"
                )
            }
            Self::ArtifactAddressAbsent { role } => {
                let seat = role.name();
                write!(
                    into,
                    "the unit at {seat} is delivered to an address and the plan names none"
                )
            }
        }
    }
}

impl<R: Role> fmt::Display for ClosureError<R> {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(into, "{}", self.first_issue())?;
        let further = self.issues().count().saturating_sub(1);
        if further > 0 {
            write!(into, ", and {further} further issues")?;
        }
        if let Capping::Truncated { omitted } = self.capping() {
            write!(into, ", {omitted} of them not carried")?;
        }
        Ok(())
    }
}

impl<R: Role> core::error::Error for ClosureError<R> {}

impl<R: Role> Refused for ClosureError<R> {
    const PHASE: Phase = Phase::Closure;
    const FAMILY: Family = CLOSURE_FAMILY;

    fn class(&self) -> RefusalClass {
        self.first_issue().class()
    }

    fn first(&self) -> String {
        self.first_issue().to_string()
    }

    fn observed(&self) -> Observed {
        self.first_issue().observed()
    }

    fn body(&self) -> LineBody {
        let further = self.issues().count().saturating_sub(1);
        let capping = self.capping();
        if further == 0 && capping == Capping::Complete {
            LineBody::SingleCause
        } else {
            LineBody::Body { further, capping }
        }
    }

    fn related(&self) -> Vec<Vec<u8>> {
        self.issues()
            .iter()
            .map(ClosureIssue::canonical_bytes)
            .collect()
    }

    /// This home declares no repair of its own.
    ///
    /// Every issue above is about what the caller's own plan declared or what the caller's own renderer produced, so the repair is one of those two declarations; a sentence composed here would be this compiler citing a fact nobody declared.
    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        Bounded::empty()
    }
}
