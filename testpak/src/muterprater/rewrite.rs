//! The structural-rewrite lane: mutation families expressed as descriptor rows,
//! planned as candidates for the harness to audit.
//!
//! A rewrite descriptor is DATA — a pattern, the shape it rewrites to, and the
//! operator family the pair realizes. Nothing here compiles, executes, or
//! interprets either side of a pair, and nothing here invokes a rewriter: this
//! file states which damages the lane would ask for and under which scope.
//!
//! # Trust-last, by the staging
//!
//! Rewrite-produced descriptors enter the interpreted audit road LAST. The trust order opens with baseline qualification, then a witness rejection demonstrated under a qualified adapter, then mandatory no-mutation parity; a mutable surface must also provide an executable point. [`admission`] reads that execution availability once. The descriptor remains an audit candidate, not evidence, until an actual execution earns a later evidence claim.
//!
//! # Why the interpreter gates this lane
//!
//! Mutation families expressed as rewrite descriptors join the plan once the
//! interpreter is the execution substrate that makes them cheap. Before that,
//! every rewrite is a source-level damage priced like a compiled mutation, and
//! the lane's whole reason for existing is that it is not.

use super::types::{
    ArtifactMutation, InterpreterAvailability, PointCatalogPosture, RewriteAdmission,
    RewriteCandidate, RewriteRoster, RewriteWithheld, ScopeShape,
};
use crate::depot::operator_families::OPERATOR_FAMILIES;
use crate::depot::types::OperatorFamily;

impl ArtifactMutation {
    /// The damage rendered for a person.
    ///
    /// A projection: a plan and a survivor explanation name a row through it, and no decision anywhere consults it.
    #[must_use]
    pub const fn described(self) -> &'static str {
        match self {
            Self::OrderPermuted => "the textual selection order is reversed",
            Self::IdentityRecycled => "every cause is emitted under one local key",
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
/// # Authority
///
/// A pure function of its arguments. Every candidate it produces carries
/// [`RewriteTrust::AuditPending`](super::RewriteTrust::AuditPending), because
/// that is the only posture this lane's material has: a planned rewrite is
/// something the harness audits, never something it has established.
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
/// # Authority
///
/// Read over the interpreted lane's availability, which already folds the trust order in its owner's sequence. A trusted point-free surface still withholds this road because it offers no active execution; an admitted descriptor remains [`super::RewriteTrust::AuditPending`], not evidence.
#[must_use]
pub fn admission<Input, Meaning>(
    interpreter: &InterpreterAvailability<'_, '_, '_, '_, '_, Input, Meaning>,
) -> RewriteAdmission {
    match interpreter {
        InterpreterAvailability::Available(trust) => match trust.surface().catalog_posture() {
            PointCatalogPosture::NoAdmittedPoints => {
                RewriteAdmission::Withheld(RewriteWithheld::NoAdmittedPoint)
            }
            PointCatalogPosture::Mutable => RewriteAdmission::Admitted,
        },
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
/// # Authority
///
/// A planning reading over the fact bank, computed rather than hand-counted: the
/// bank declares which damages the harness is willing to inflict, and a family
/// no descriptor realizes is pressure this roster does not apply. It states an
/// absence and nothing about whether that absence matters — which families are
/// worth realizing is the planner's decision and the owner's ruling.
#[must_use]
pub fn unrealized_families(roster: &RewriteRoster) -> Vec<OperatorFamily> {
    OPERATOR_FAMILIES
        .into_iter()
        .filter(|family| {
            !roster
                .descriptors()
                .iter()
                .any(|descriptor| descriptor.family().slug() == family.slug())
        })
        .collect()
}
