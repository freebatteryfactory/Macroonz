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
//! Rewrite-produced descriptors are admitted LAST. The trust order opens with
//! baseline qualification, then a witness rejection demonstrated under a
//! qualified adapter, then the mandatory no-mutation parity — and only with the
//! interpreted lane standing under all of that does this lane's material become
//! evidence rather than a candidate. [`admission`] is that reading, taken over
//! the interpreted lane's own availability so the order has ONE authority
//! rather than two that could disagree.
//!
//! # Why the interpreter gates this lane
//!
//! Mutation families expressed as rewrite descriptors join the plan once the
//! interpreter is the execution substrate that makes them cheap. Before that,
//! every rewrite is a source-level damage priced like a compiled mutation, and
//! the lane's whole reason for existing is that it is not.

use super::types::{
    InterpreterAvailability, RewriteAdmission, RewriteCandidate, RewriteRoster, RewriteWithheld,
    ScopeShape,
};
use crate::depot::operator_families::OPERATOR_FAMILIES;
use crate::depot::types::OperatorFamily;

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

/// Whether this lane's descriptors are admitted as evidence yet.
///
/// # Authority
///
/// Read over the interpreted lane's availability, which already folds the trust
/// order in its owner's sequence. A second reading of the pressure witness and
/// the parity here would be the same law standing in two places, and the weaker
/// copy would keep passing after the stronger one moved.
#[must_use]
pub fn admission(interpreter: &InterpreterAvailability<'_>) -> RewriteAdmission {
    match interpreter {
        InterpreterAvailability::Available { surface: _ } => RewriteAdmission::Admitted,
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
                .any(|descriptor| descriptor.family().slug() == family.slug)
        })
        .collect()
}
