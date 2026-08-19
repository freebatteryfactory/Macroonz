//! The planning half of the road: what the plan already decided about the
//! shell, read off the plan's own public surface.
//!
//! Nothing here decides meaning and nothing here mints an identity. The planned
//! member's semantic key, its expected profile at its version, and its origin
//! trail are the PLAN's answers, read exactly; the address the shell stands on is
//! the entry account's one commitment; the rendering engine is the generator the
//! plan's context names; and the obligation and the challenge method are the
//! kind content's, read and not interpreted. Two readings and no third: the
//! account answers what this was planned over, the membership answers what will
//! be materialized, and this file joins them without keeping a copy of either.
//!
//! # Two roads, two vocabularies
//!
//! Reading a plan and rendering a shell refuse in two different families, and
//! they are not folded into one. A plan that declares no member under its kind's
//! one role is a fact about the PLAN, and a rendering that cannot spell a literal
//! the gate demands is a fact about the SEAM — a caller told only "the shell
//! failed" would go looking in the wrong place, and the two are not
//! co-establishable anyway, because there is nothing to render until the plan has
//! been read.
//!
//! # The rows are not here
//!
//! The plan's kind content names an obligation and a challenge method; it does
//! not name a claim, a suite, roles, tags, a subject route, a check reference, a
//! population, or a callable. Those are the harness's declarations, they arrive
//! from the CALLER as [`TrialTablePayload`](super::TrialTablePayload), and a
//! generator that invented them would be producing its own facts and then
//! proving them.

use super::{DescriptorPlan, DescriptorPlanIssue};
use crate::plane::{RenderedRole, SoleRenderedUnit};
use crate::planning::{MemberDestination, ProjectionPlan, TestDescriptorProjection};

/// Read one test-descriptor plan into the statement of what its shell will be.
///
/// # Errors
///
/// Returns [`DescriptorPlanIssue::RoleNotPlanned`] where the plan declares no
/// member under its kind's one rendered role — the membership is the quantifier,
/// so an unplanned role is an absence the plan itself states rather than a
/// failure to look hard enough.
///
/// Returns [`DescriptorPlanIssue::DestinationNotDeclarationSite`] where the
/// planned member is written as a standalone artifact: the generated support
/// shell is emitted at the declaration site as deferred tokens and invoked by the
/// consumption target, and a member landing elsewhere is a different delivery.
///
/// The two checks are DEPENDENT — there is no destination to read until a member
/// was found — so exactly one of them is ever established.
pub fn descriptor_plan(
    plan: &ProjectionPlan<TestDescriptorProjection>,
) -> Result<DescriptorPlan, DescriptorPlanIssue> {
    let role = SoleRenderedUnit::Sole;
    let Some(member) = plan.membership().under(role) else {
        return Err(DescriptorPlanIssue::RoleNotPlanned {
            role_slot: role.slot(),
        });
    };
    match member.output.destination {
        MemberDestination::AtDeclarationSite => {}
        MemberDestination::AsArtifact { .. } => {
            return Err(DescriptorPlanIssue::DestinationNotDeclarationSite {
                role_slot: role.slot(),
            });
        }
    }
    let content = plan.content();
    Ok(DescriptorPlan {
        role,
        semantic_key: member.output.semantic_key,
        profile: member.output.expected_profile,
        profile_version: member.output.expected_profile_version,
        origin: member.output.origin.clone(),
        declaration: plan.account().commitment(),
        engine: plan.context().generator,
        obligation: content.obligation,
        challenge: content.challenge,
    })
}
