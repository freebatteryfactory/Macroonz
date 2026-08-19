//! The planning half of the road: what the plan already decided about the
//! surface, read off the plan's own public surface — and the typed reading of
//! what this home is available for at all.
//!
//! Nothing here decides meaning and nothing here mints an identity. The planned
//! member's semantic key, its expected profile at its version, and its origin
//! trail are the PLAN's answers, read exactly; the address the surface stands on
//! is the entry account's one commitment; the rendering engine is the generator
//! the plan's context names; the contract the plan is bound to is the context's
//! own binding; and the port, the wire contract, and the facing are the kind
//! content's, read and not interpreted.
//!
//! # The shape is not here, and neither is the codec
//!
//! The plan's kind content names a PORT, a WIRE CONTRACT, and a DIRECTION. It
//! names no type, no road, no signature, no entry spelling — and no codec. So
//! [`RemoteSurfaceShape`](super::RemoteSurfaceShape) arrives from the CALLER,
//! pairing included, and this file reads only what the plan actually decided. The
//! codec that reads and writes a wire contract's bytes is its own projection over
//! its own plan; a generator that elected one here would be pairing somebody
//! else's surface with a reader nobody asked for.
//!
//! # The direction is read, and it decides the composition
//!
//! The facing is the one kind-content seat the rendering turns on: it is what the
//! facing table in `type_contract.rs` is asked, and therefore what decides which
//! of the pairing's two roads opens the rendered road and which closes it. This
//! file carries it forward so the surface can say which way it faces rather than
//! leaving a reader to infer it from the order of two calls.
//!
//! # What is available, and what is not
//!
//! [`surface_availability`] is the reading a caller takes BEFORE it holds a plan:
//! a kind whose target requirement is a bound host contract has no plan at all
//! under a target-free context, so the honest answer to "can I serve this?" is a
//! typed disposition rather than a refusal discovered three roads later. What
//! would open the road for a caller that holds no contract is
//! [`REMOTE_SURFACE_CONTRACT_MINT`](super::REMOTE_SURFACE_CONTRACT_MINT), and it
//! is carried in the answer rather than left in a README.

use super::{
    IntegrationTargetLanding, REMOTE_SURFACE_CONTRACT_MINT, RemoteSurfaceIssue, RemoteSurfacePlan,
    SurfaceAvailability,
};
use crate::plane::{RenderedRole, SoleRenderedUnit};
use crate::planning::{
    MemberDestination, ProjectionContext, ProjectionKind, ProjectionPlan, RemoteSurfaceProjection,
    TargetBinding,
};

/// Read one remote-surface plan into the statement of what its surface will be.
///
/// # Errors
///
/// Returns [`RemoteSurfaceIssue::RoleNotPlanned`] where the plan declares no
/// member under its kind's one rendered role — the membership is the quantifier,
/// so an unplanned role is an absence the plan itself states rather than a failure
/// to look hard enough.
///
/// Returns [`RemoteSurfaceIssue::DestinationNotIntegrationTarget`] where the
/// planned member is spliced at the declaration site: a remote surface lands in
/// its INTEGRATION target, which is a different file than the declaration the plan
/// was derived from, so the member is written as a standalone artifact under a
/// byte role and a member spliced beside the declaration is a different delivery.
///
/// Returns [`RemoteSurfaceIssue::TargetBindingFree`] where the plan's context
/// binds no host contract. That posture is foreclosed on this seam's own route —
/// [`ProjectionPlan::planned`](crate::planning::ProjectionPlan::planned) refuses a
/// target-free plan for a kind whose target requirement is a bound contract — and
/// the road exists so this reading has a truthful answer for the posture the TYPE
/// still admits rather than a fabricated one.
///
/// The three checks are DEPENDENT — there is no destination to read until a member
/// was found, and no surface to bind until the member lands where a surface lands
/// — so exactly one of them is ever established.
pub fn remote_surface_plan(
    plan: &ProjectionPlan<RemoteSurfaceProjection>,
) -> Result<RemoteSurfacePlan, RemoteSurfaceIssue> {
    let role = SoleRenderedUnit::Sole;
    let Some(member) = plan.membership().under(role) else {
        return Err(RemoteSurfaceIssue::RoleNotPlanned {
            role_slot: role.slot(),
        });
    };
    let byte_role = match member.output.destination {
        MemberDestination::AsArtifact { byte_role } => byte_role,
        MemberDestination::AtDeclarationSite => {
            return Err(RemoteSurfaceIssue::DestinationNotIntegrationTarget {
                role_slot: role.slot(),
            });
        }
    };
    let TargetBinding::HostContract(host_contract) = plan.context().target else {
        return Err(RemoteSurfaceIssue::TargetBindingFree {
            kind: RemoteSurfaceProjection::KIND_NAME,
        });
    };
    let content = plan.content();
    Ok(RemoteSurfacePlan {
        role,
        semantic_key: member.output.semantic_key,
        profile: member.output.expected_profile,
        profile_version: member.output.expected_profile_version,
        origin: member.output.origin.clone(),
        declaration: plan.account().commitment(),
        engine: plan.context().generator,
        host_contract,
        port: content.port,
        wire_contract: content.wire_contract,
        direction: content.direction,
        landing: IntegrationTargetLanding::in_integration_target(byte_role),
    })
}

/// What this home is available for under one shared context.
///
/// # Authority
///
/// **The reading is over the binding the CALLER holds, and the mint's standing is
/// a separate fact carried beside it.** A context that binds a contract makes the
/// whole road below reachable for whoever holds it; a context that binds none
/// makes a plan of this kind unmakeable, and the answer names what would open the
/// road rather than saying only that it is shut.
///
/// # Nonclaims
///
/// [`SurfaceAvailability::Bound`] says the caller holds an identity and nothing
/// about whether one can be obtained: a value held inside this workspace is not
/// evidence that the machine's mint exists, and
/// [`REMOTE_SURFACE_CONTRACT_MINT`](super::REMOTE_SURFACE_CONTRACT_MINT) is where
/// that question is answered. Neither arm claims anything about whether the
/// contract is current, admitted, or reachable — those are the machine's, on the
/// terms every owner identity reference in the plane states.
#[must_use]
pub fn surface_availability(context: &ProjectionContext) -> SurfaceAvailability {
    match context.target {
        TargetBinding::HostContract(contract) => SurfaceAvailability::Bound { contract },
        TargetBinding::TargetFree => SurfaceAvailability::NoHostContract {
            opening: REMOTE_SURFACE_CONTRACT_MINT,
        },
    }
}
