//! Reading a retained pattern-stamp plan into its published-artifact statement.

use super::{StampedUnitPlan, StampedUnitPlanIssue};
use crate::plane::{RenderedRole, SoleRenderedUnit};
use crate::planning::{MemberDestination, PatternStampProjection, ProjectionPlan};

/// Read one pattern-stamp plan into the statement of what its published artifact will be.
///
/// # Errors
///
/// Returns [`StampedUnitPlanIssue::RoleNotPlanned`] when the plan declares no member under its kind's rendered role.
///
/// Returns [`StampedUnitPlanIssue::DestinationNotArtifact`] when the planned member does not land as a standalone artifact.
pub fn stamped_unit_plan(
    plan: &ProjectionPlan<PatternStampProjection>,
) -> Result<StampedUnitPlan, StampedUnitPlanIssue> {
    let role = SoleRenderedUnit::Sole;
    let Some(member) = plan.membership().under(role) else {
        return Err(StampedUnitPlanIssue::RoleNotPlanned {
            role_slot: role.slot(),
        });
    };
    let byte_role = match member.output.destination {
        MemberDestination::AsArtifact { byte_role } => byte_role,
        MemberDestination::AtDeclarationSite
        | MemberDestination::IntoTestCarrier
        | MemberDestination::IntoBenchCarrier => {
            return Err(StampedUnitPlanIssue::DestinationNotArtifact {
                role_slot: role.slot(),
            });
        }
    };
    let content = plan.content();
    Ok(StampedUnitPlan {
        role,
        semantic_key: member.output.semantic_key,
        byte_role,
        profile: member.output.expected_profile,
        profile_version: member.output.expected_profile_version,
        origin: member.output.origin.clone(),
        digest_contract: member.output.digest_contract,
        declaration: plan.account().commitment(),
        engine: plan.context().generator,
        pattern: content.pattern,
        instance: content.instance,
    })
}
