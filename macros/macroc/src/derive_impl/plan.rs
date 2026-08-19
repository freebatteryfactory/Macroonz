//! The planning half of the road: what the plan already decided about the two
//! surfaces, read off the plan's own public surface.
//!
//! Nothing here decides meaning and nothing here mints an identity for a member.
//! The production surface's semantic key, its expected profile at its version,
//! and its origin trail are the PLAN's answers, read exactly; the address the
//! two surfaces stand on is the entry account's one commitment; and the
//! rendering engine is the generator the plan's context names. Two readings and
//! no third: the account answers what this was planned over, the membership
//! answers what will be materialized, and this file joins them without keeping a
//! copy of either.
//!
//! # The evaluation copy has no planned identity, and none is invented here
//!
//! The evaluation copy is not a member of the plan's declared membership — the
//! output firewall is exactly that the declared set is the whole set — so
//! nothing planned an identity for it. What this file states is the CONTRACT
//! that identity must satisfy: the role it will stand under, and the production
//! member it will be anchored to. The identity itself is derived at rendering
//! time over the copy's own canonical bytes, because an identity derived here
//! would be a fact about bytes nobody has produced.

use super::types::{EvaluationIdentityContract, ImplementationSurfaceIssue};
use crate::origin_graph::OriginTrail;
use crate::plane::{
    GeneratedUnitSubject, GeneratorVersionSubject, ProfileVersion, ProjectionIdentity,
    ProjectionProfileSubject, RenderedRole,
};
use crate::planning::{
    CauseAnchoring, DeriveImplProjection, MemberDestination, ProjectionPlan, RenderedImplementation,
};

/// What one planned member's two surfaces will be, stated before either is
/// rendered.
///
/// Every seat is public and required, because a statement that could omit its
/// engine, its declaration, or its identity contract would be an account that
/// sometimes says less than it knows. There is no private field here and this
/// home's invariant nucleus holds nothing of it.
///
/// # Nonclaims
///
/// Holding one claims that these are the facts the plan carries under this role,
/// and nothing about whether anything was rendered. The seats are copies of the
/// plan's answers taken at one moment; the plan remains the value they were read
/// from.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SurfacePlan {
    /// The rendered role both surfaces stand for.
    pub role: RenderedImplementation,
    /// The production member's semantic key, exactly as the plan declared it.
    pub production_key: ProjectionIdentity<GeneratedUnitSubject>,
    /// What the evaluation copy's eventual identity must satisfy.
    pub identity_contract: EvaluationIdentityContract,
    /// The profile the plan expects to render the production member.
    pub profile: ProjectionIdentity<ProjectionProfileSubject>,
    /// That profile's version.
    pub profile_version: ProfileVersion,
    /// The production member's origin trail, walked back to authored material.
    pub origin: OriginTrail,
    /// The ONE address the entry account walked in the door carrying.
    pub declaration: CauseAnchoring,
    /// The rendering engine both surfaces are written by.
    pub engine: ProjectionIdentity<GeneratorVersionSubject>,
}

/// Read one planned member into the statement of what its two surfaces will be.
///
/// # Errors
///
/// Returns [`ImplementationSurfaceIssue::RoleNotPlanned`] where the plan
/// declares no member under this role — the membership is the quantifier, so an
/// unplanned role is an absence the plan itself states rather than a failure to
/// look hard enough.
///
/// Returns [`ImplementationSurfaceIssue::DestinationNotDeclarationSite`] where
/// the planned member is written as a standalone artifact: the
/// derive-implementation projection's production surface lands at the
/// declaration site, and a member landing elsewhere is a different delivery.
///
/// The two checks are DEPENDENT — there is no destination to read until a member
/// was found — so exactly one of them is ever established.
pub fn surface_plan(
    plan: &ProjectionPlan<DeriveImplProjection>,
    role: RenderedImplementation,
) -> Result<SurfacePlan, ImplementationSurfaceIssue> {
    let Some(member) = plan.membership().under(role) else {
        return Err(ImplementationSurfaceIssue::RoleNotPlanned {
            role_slot: role.slot(),
        });
    };
    match member.output.destination {
        MemberDestination::AtDeclarationSite => {}
        MemberDestination::AsArtifact { .. } => {
            return Err(ImplementationSurfaceIssue::DestinationNotDeclarationSite {
                role_slot: role.slot(),
            });
        }
    }
    let production_key = member.output.semantic_key;
    Ok(SurfacePlan {
        role,
        production_key,
        identity_contract: EvaluationIdentityContract::over(production_key),
        profile: member.output.expected_profile,
        profile_version: member.output.expected_profile_version,
        origin: member.output.origin.clone(),
        declaration: plan.account().commitment(),
        engine: plan.context().generator,
    })
}
