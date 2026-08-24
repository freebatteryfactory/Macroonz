//! The structural-rewrite lane: mutation families expressed as descriptor rows, planned as candidates for the harness to audit.
//!
//! A rewrite descriptor is data — a pattern, the shape it rewrites to, and the operator family the pair realizes.
//! Nothing here compiles, executes, or interprets either side, and nothing here invokes a rewriter: this file states which damages the lane would ask for, and under which scope.
//!
//! # Why the interpreter gates this lane
//!
//! Rewrite families are cheap only once the interpreter is the execution substrate under them; before that every rewrite is a source-level damage priced like a compiled mutation, which is the thing this lane exists not to be.
//! So [`admission`] reads the interpreted lane's availability, and an admitted descriptor is still an audit candidate rather than evidence until an actual execution earns a later claim.

use super::types::{
    ArtifactMutation, InterpreterAvailability, RewriteAdmission, RewriteCandidate, RewriteRoster,
    RewriteWithheld, ScopeShape,
};
use crate::depot::operator_families::OPERATOR_FAMILIES;
use crate::depot::types::OperatorFamily;

impl ArtifactMutation {
    /// The damage rendered for a person.
    ///
    /// A projection, and no decision anywhere consults it.
    #[must_use]
    pub const fn described(self) -> &'static str {
        match self {
            Self::OrderPermuted => "the emitted members are written in reverse of declared order",
            Self::IdentityRecycled => {
                "every emitted member is written under the first member's key"
            }
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

/// Plan one roster's descriptors as audit candidates under one scope.
///
/// A pure function of its arguments, and every candidate carries [`RewriteTrust::AuditPending`](super::RewriteTrust::AuditPending), because a planned rewrite is something the harness audits and never something it has established.
#[must_use]
pub fn planned(roster: &RewriteRoster, scope: &ScopeShape) -> Vec<RewriteCandidate> {
    roster
        .descriptors()
        .iter()
        .map(|descriptor| RewriteCandidate::planned(*descriptor, scope.clone()))
        .collect()
}

/// Whether this lane's descriptors may enter the interpreted audit road.
///
/// Read over the interpreted lane's availability, which already folds the trust order in its owner's sequence.
/// Availability retains one exact active selection, so a point-free surface cannot reach the admitted arm.
#[must_use]
pub fn admission<Input, Meaning>(
    interpreter: &InterpreterAvailability<'_, '_, '_, '_, '_, '_, Input, Meaning>,
) -> RewriteAdmission {
    match interpreter {
        InterpreterAvailability::Available(_) => RewriteAdmission::Admitted,
        InterpreterAvailability::NoConformingSurface => {
            RewriteAdmission::Withheld(RewriteWithheld::InterpreterUnavailable)
        }
        InterpreterAvailability::TrustNotOpened { missing } => {
            RewriteAdmission::Withheld(RewriteWithheld::TrustNotOpened(*missing))
        }
    }
}

/// The bank's operator families no descriptor in this roster realizes.
///
/// A planning reading over the fact bank, computed rather than hand-counted: a family no descriptor realizes is pressure this roster does not apply.
/// It states an absence and nothing about whether that absence matters.
#[must_use]
pub fn unrealized_families(roster: &RewriteRoster) -> Vec<OperatorFamily> {
    OPERATOR_FAMILIES
        .iter()
        .copied()
        .filter(|family| {
            !roster
                .descriptors()
                .iter()
                .any(|descriptor| descriptor.family().slug() == family.slug())
        })
        .collect()
}
