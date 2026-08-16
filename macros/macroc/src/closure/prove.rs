//! The per-role pass.
//!
//! The roster is the quantifier.
//! Every role the kind declares is examined, in roster order, so "every
//! applicable role was checked" is a fact about the loop rather than a claim
//! about it.
//! A role that establishes an issue contributes no rebuilt member, which is why
//! a reconstruction is never a partial reading of a disagreement.
//!
//! Nothing here reaches a private field: the pass reads a rendered unit through
//! the same public answers any caller gets, and the rebuild it hands back is
//! the renderer's own answer in the shape a plan states it.
//! The roads that consume this pass live in `type_guard.rs`, because building a
//! closure and building the refusal body are both what must stay unreachable.

use super::{ClosureIssue, RenderedProjection};
use crate::plane::RenderedRole;
use crate::planning::{PlannedMember, PlannedMembership};

/// The per-role pass: every issue the two establish at a role, and the members
/// rebuilt at the roles where they agreed.
pub(super) fn examined<R: RenderedRole>(
    planned: &PlannedMembership<R>,
    rendered: &RenderedProjection<R>,
) -> (Vec<ClosureIssue<R>>, Vec<PlannedMember<R>>) {
    let mut issues: Vec<ClosureIssue<R>> = Vec::new();
    let mut rebuilt: Vec<PlannedMember<R>> = Vec::new();
    for role in R::ROLES.iter().copied() {
        // What the plan declared under the role is checked in its own right and
        // before anything is compared. A role is declared once, so a planned
        // count of two is a defect in the plan rather than a shape the check
        // accommodates — and `under` yields the first match, which would hide
        // it.
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
