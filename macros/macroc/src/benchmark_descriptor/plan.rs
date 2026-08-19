//! The planning half of the road: what the plan already decided about the bench
//! shell, read off the plan's own public surface.
//!
//! Nothing here decides meaning and nothing here mints an identity. The planned
//! member's semantic key, its expected profile at its version, and its origin
//! trail are the PLAN's answers, read exactly; the address the shell stands on is
//! the entry account's one commitment; the rendering engine is the generator the
//! plan's context names; and the measured unit, the work currency, and the claim
//! are the kind content's, read and not interpreted.
//!
//! # The rows are not here
//!
//! The plan's kind content names a MEASURED UNIT, a WORK CURRENCY, and a CLAIM.
//! It does not name an input-size axis, a correctness preflight, a planted-worse
//! falsifier, declared budgets, a contention posture, a work formula, or a
//! complexity-claim reference — so [`BenchTablePayload`](super::BenchTablePayload)
//! arrives from the caller. A generator that chose a row's sample count or
//! elected its falsifier would be setting the tolerance it is then measured
//! against.
//!
//! # A benchmark is evidence, never a specification
//!
//! What is read here is what one plan decided about one realization. Nothing in
//! it says what any other realization must do, and no seat of the statement
//! carries a measurement — measurements are the bench host's, taken by running.

use super::{BenchmarkPlan, BenchmarkPlanIssue};
use crate::plane::{RenderedRole, SoleRenderedUnit};
use crate::planning::{BenchmarkDescriptorProjection, MemberDestination, ProjectionPlan};

/// Read one benchmark-descriptor plan into the statement of what its shell will
/// be.
///
/// # Errors
///
/// Returns [`BenchmarkPlanIssue::RoleNotPlanned`] where the plan declares no
/// member under its kind's one rendered role — the membership is the quantifier,
/// so an unplanned role is an absence the plan itself states rather than a failure
/// to look hard enough.
///
/// Returns [`BenchmarkPlanIssue::DestinationNotDeclarationSite`] where the planned
/// member is written as a standalone artifact: the bench shell is emitted at the
/// declaration site as deferred tokens and invoked by the bench target, and a
/// member landing elsewhere is a different delivery.
///
/// The two checks are DEPENDENT — there is no destination to read until a member
/// was found — so exactly one of them is ever established.
pub fn benchmark_plan(
    plan: &ProjectionPlan<BenchmarkDescriptorProjection>,
) -> Result<BenchmarkPlan, BenchmarkPlanIssue> {
    let role = SoleRenderedUnit::Sole;
    let Some(member) = plan.membership().under(role) else {
        return Err(BenchmarkPlanIssue::RoleNotPlanned {
            role_slot: role.slot(),
        });
    };
    match member.output.destination {
        MemberDestination::AtDeclarationSite => {}
        MemberDestination::AsArtifact { .. } => {
            return Err(BenchmarkPlanIssue::DestinationNotDeclarationSite {
                role_slot: role.slot(),
            });
        }
    }
    let content = plan.content();
    Ok(BenchmarkPlan {
        role,
        semantic_key: member.output.semantic_key,
        profile: member.output.expected_profile,
        profile_version: member.output.expected_profile_version,
        origin: member.output.origin.clone(),
        declaration: plan.account().commitment(),
        engine: plan.context().generator,
        measured: content.measured,
        work_currency: content.work_currency,
        claim: content.claim,
    })
}
