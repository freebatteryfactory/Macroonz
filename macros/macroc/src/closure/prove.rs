//! The per-role pass, and the refusal an established issue list amounts to.
//!
//! The roster is the quantifier. Every role the kind declares is examined, in
//! roster order, so "every applicable role was checked" is a fact about the loop
//! rather than a claim about it. A role that establishes an issue contributes no
//! rebuilt member, which is why a reconstruction is never a partial reading of a
//! disagreement.
//!
//! Nothing here reaches a private field: the pass reads a rendered unit through
//! the same public answers any caller gets, and the rebuild it hands back is the
//! renderer's own answer in the shape a plan states it. The proof that consumes
//! this pass lives in `type_guard.rs`, because building a closure is what must
//! stay unreachable.

use super::{ClosureIssue, ProjectionClosureRefusal, RenderedProjection};
use crate::plane::{AuthoringLimitProfile, RenderedRole};
use crate::planning::{PlannedMember, PlannedMembership};
use threadpak::refusal::{AdmittedPrefix, StopBound};
use threadpak::types::PositiveLimit;

/// The per-role pass: every issue the two establish at a role, and the members
/// rebuilt at the roles where they agreed.
///
/// The roster is the quantifier. Every role the kind declares is examined, in
/// roster order, and a role that establishes an issue contributes no rebuilt
/// member — so a rebuild is never a partial reading of a disagreement.
pub(super) fn examined<R: RenderedRole>(
    planned: &PlannedMembership<R>,
    rendered: &RenderedProjection<R>,
) -> (Vec<ClosureIssue<R>>, Vec<PlannedMember<R>>) {
    let mut issues: Vec<ClosureIssue<R>> = Vec::new();
    let mut rebuilt: Vec<PlannedMember<R>> = Vec::new();
    for role in R::ROLES.iter().copied() {
        // What the PLAN declared under the role is checked in its own right, and
        // before anything is compared. Today every role a plan declares is
        // declared exactly once, so a planned count of two is a defect in the
        // plan rather than a shape the check has to accommodate — and reading
        // the plan's own count through `under`, which yields the first match,
        // would have hidden it.
        let planned_count = planned.count_under(role);
        if planned_count > 1 {
            issues.push(ClosureIssue::MemberPlannedTwice {
                role,
                observed: u32::try_from(planned_count).unwrap_or(u32::MAX),
            });
            continue;
        }
        let rendered_count = rendered.count_under(role);
        if rendered_count > 1 {
            issues.push(ClosureIssue::MemberDuplicated {
                role,
                observed: u32::try_from(rendered_count).unwrap_or(u32::MAX),
            });
            continue;
        }
        match (planned.under(role), rendered.under(role)) {
            (Some(_), None) => issues.push(ClosureIssue::MemberMissing { role }),
            (None, Some(_)) => issues.push(ClosureIssue::MemberUnplanned { role }),
            (None, None) => {}
            (Some(member), Some(unit)) => {
                let reconstruction = unit.reconstructed();
                if reconstruction.output.semantic_key != member.output.semantic_key {
                    issues.push(ClosureIssue::SemanticKeyMismatch { role });
                } else if reconstruction.output.origin != member.output.origin {
                    issues.push(ClosureIssue::OriginOrphan { role });
                } else if unit.digest_under(member.output.digest_contract) != unit.digest() {
                    issues.push(ClosureIssue::DigestMismatch { role });
                } else if reconstruction.output.destination != member.output.destination
                    || reconstruction.output.expected_profile != member.output.expected_profile
                    || reconstruction.output.expected_profile_version
                        != member.output.expected_profile_version
                {
                    issues.push(ClosureIssue::MaterializationMismatch { role });
                } else {
                    rebuilt.push(reconstruction);
                }
            }
        }
    }
    (issues, rebuilt)
}

/// The refusal one established issue list amounts to, or nothing where the list
/// is empty.
///
/// One road for every pass in [`ProjectionClosure::proved`], so no pass can
/// establish issues and then walk on past them.
///
/// [`ProjectionClosure::proved`]: super::ProjectionClosure::proved
pub(super) fn refused<R: RenderedRole>(
    issues: Vec<ClosureIssue<R>>,
) -> Option<ProjectionClosureRefusal<R>> {
    let mut established = issues.into_iter();
    let first = established.next()?;
    Some(ProjectionClosureRefusal::established(
        first,
        established.collect(),
    ))
}

impl<R: RenderedRole> ProjectionClosureRefusal<R> {
    /// The body a closure check refuses with.
    ///
    /// The per-role pass above walks the kind's whole roster before a body
    /// exists, so the posture here is about the REPORT rather than the pass.
    /// Where every established issue fits the declared bound the body carries
    /// all of them; where it does not, the body carries what the bound holds and
    /// names how many established issues stand outside it — never a silent drop.
    pub(super) fn established(first: ClosureIssue<R>, rest: Vec<ClosureIssue<R>>) -> Self {
        Self {
            report: AdmittedPrefix::examined_completely(
                first,
                rest,
                &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
                StopBound::DeclaredIssueBound,
            ),
        }
    }
}
