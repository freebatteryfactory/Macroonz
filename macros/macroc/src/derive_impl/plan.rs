//! The planning half of the road: what the plan already decided about the two
//! surfaces, read off the plan's own public surface.
//!
//! Nothing here decides meaning and nothing here mints an identity for a member.
//! Each surface's semantic key, the expected profile at its version, and the
//! origin trail are the PLAN's answers, read exactly; the address the two
//! surfaces stand on is the entry account's one commitment; and the rendering
//! engine is the generator the plan's context names. Two readings and no third:
//! the account answers what this was planned over, the membership answers what
//! will be materialized, and this file joins them without keeping a copy of
//! either.
//!
//! # Both surfaces are planned members, and this file reads the pair
//!
//! One implementation meaning is delivered as two surfaces, and the plan
//! declares BOTH: the production unit under its role, and the evaluation copy
//! under that role's twin ([`RenderedImplementation::twin`]). The output
//! firewall is that the declared set is the whole set, so a copy standing
//! outside the membership would be material emitted past it — which is exactly
//! why the roster carries the evaluation roles and why the generator's schema
//! version moved when it did.
//!
//! So there is no identity CONTRACT stated here and no second rule for the
//! copy's name. The copy has a planned semantic key of its own and a digest
//! contract of its own, on exactly the terms the production member has them, and
//! the rendering derives the copy's identity over its own canonical bytes
//! anchored on its own key — the derivation any planned member's rendered unit
//! is identified by.
//!
//! # Either half names the pair
//!
//! [`surface_plan`] is total in the role it is handed. A production role names
//! its evaluation twin and an evaluation role names its production original, so
//! a caller holding one half cannot compose the two surfaces backwards, and
//! there is no precondition a reader has to keep.

use super::types::{ImplementationSurfaceIssue, SurfacePlan};
use crate::plane::RenderedRole;
use crate::planning::{
    DeriveImplProjection, PlannedMember, ProjectionPlan, RenderedImplementation,
};

/// Read one planned PAIR into the statement of what its two surfaces will be.
///
/// The role names one half and the roster names the other: a production role is
/// taken as given, an evaluation role is turned into its production original
/// ([`RenderedImplementation::is_evaluation_copy`]), and the twin of whichever
/// one that is names the copy. The road is therefore total in the role it is
/// handed, and the pair it reads is the same pair either way.
///
/// # Errors
///
/// Returns [`ImplementationSurfaceIssue::RoleNotPlanned`] where the plan
/// declares no member under one half of the pair — the membership is the
/// quantifier, so an unplanned role is an absence the plan itself states rather
/// than a failure to look hard enough. Both halves are looked for, because both
/// are planned members and a delivery missing either is half a delivery.
///
/// Returns [`ImplementationSurfaceIssue::DestinationNotRoleDeclared`] where a
/// planned member lands somewhere other than the landing its ROLE declares
/// ([`RenderedImplementation::destination`]): where a member of this kind lands
/// is the roster's own constant answer, and the two halves of a pair are
/// answered differently — the production implementation at the declaration site,
/// the evaluation copy in the test carrier — so a plan that wrote either of them
/// into the other's delivery is refused against that answer rather than against
/// a literal repeated here.
///
/// The checks are DEPENDENT per half — there is no destination to read until a
/// member was found — so at most one of them is established per role, and the
/// production half is read before the evaluation half.
pub fn surface_plan(
    plan: &ProjectionPlan<DeriveImplProjection>,
    role: RenderedImplementation,
) -> Result<SurfacePlan, ImplementationSurfaceIssue> {
    let production_role = if role.is_evaluation_copy() {
        role.twin()
    } else {
        role
    };
    let evaluation_role = production_role.twin();
    let production = planned_member(plan, production_role)?;
    let evaluation = planned_member(plan, evaluation_role)?;
    Ok(SurfacePlan {
        role: production_role,
        production_key: production.output.semantic_key,
        evaluation_role,
        evaluation_key: evaluation.output.semantic_key,
        profile: production.output.expected_profile,
        profile_version: production.output.expected_profile_version,
        origin: production.output.origin.clone(),
        declaration: plan.account().commitment(),
        engine: plan.context().generator,
    })
}

/// The member one role plans, where the plan declares one and lands it where the
/// role says it lands.
///
/// One road for both halves of the pair, so the two are read under one rule: a
/// second reading written per half is a second rule that agrees until one of
/// them is edited.
fn planned_member(
    plan: &ProjectionPlan<DeriveImplProjection>,
    role: RenderedImplementation,
) -> Result<&PlannedMember<RenderedImplementation>, ImplementationSurfaceIssue> {
    let Some(member) = plan.membership().under(role) else {
        return Err(ImplementationSurfaceIssue::RoleNotPlanned {
            role_slot: role.slot(),
        });
    };
    if member.output.destination == role.destination() {
        Ok(member)
    } else {
        Err(ImplementationSurfaceIssue::DestinationNotRoleDeclared {
            role_slot: role.slot(),
        })
    }
}
