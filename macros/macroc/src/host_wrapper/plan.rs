//! The planning half of the road: what the plan already decided about the
//! wrapper, read off the plan's own public surface — and the typed reading of
//! what this home is available for at all.
//!
//! Nothing here decides meaning and nothing here mints an identity. The planned
//! member's semantic key, its expected profile at its version, and its origin
//! trail are the PLAN's answers, read exactly; the address the wrapper stands on
//! is the entry account's one commitment; the rendering engine is the generator
//! the plan's context names; the contract the plan is bound to is the context's
//! own binding; and the contract, the components, and the capability that
//! selected them are the kind content's, read and not interpreted.
//!
//! # The shape is not here
//!
//! The plan's kind content names a CONTRACT, the COMPONENTS composed, and the
//! declared CAPABILITY that selected them. It names no type, no road, no
//! signature and no entry spelling — so [`WrapperShape`](super::WrapperShape)
//! arrives from the CALLER and this file reads only what the plan actually
//! decided. A generator that decided which road a host answers admission on
//! would be declaring somebody else's calling convention and then calling it.
//!
//! # The components ARE read, and they decide what is composed
//!
//! The component roster is the one kind-content seat the composition turns on:
//! it is the quantifier the declared stages are checked against, in both
//! directions, and the plane's own roster order is what the rendering walks. It
//! is read by IDENTITY and never by spelling, which is why the composition law
//! stands whole while the plane's roster declares no stable name at all.
//!
//! # The binding is read twice, and the two readings are not folded
//!
//! A plan of this kind carries a host contract in TWO places: the context's
//! target binding, which
//! [`ProjectionPlan::planned`](crate::planning::ProjectionPlan::planned) refused
//! a target-free plan over, and the kind content's own contract seat. Nothing in
//! the plane requires them to agree, and this file elects neither: both travel,
//! named for which reading they came from, so a caller that cares can compare
//! them and a caller that does not is never handed one standing for the other.
//!
//! # What is available, and what is not
//!
//! [`wrapper_availability`] is the reading a caller takes BEFORE it holds a plan:
//! a kind whose target requirement is a bound host contract has no plan at all
//! under a target-free context, so the honest answer to "can I wrap this?" is a
//! typed disposition rather than a refusal discovered three roads later. What
//! would open the road for a caller that holds no contract is
//! [`HOST_WRAPPER_CONTRACT_MINT`](super::HOST_WRAPPER_CONTRACT_MINT), and it is
//! carried in the answer rather than left in a README.

use super::{
    HOST_WRAPPER_CONTRACT_MINT, HostTargetLanding, HostWrapperPlan, WrapperAvailability,
    WrapperSurfaceIssue,
};
use crate::plane::{RenderedRole, SoleRenderedUnit};
use crate::planning::{
    HostWrapperProjection, MemberDestination, ProjectionContext, ProjectionKind, ProjectionPlan,
    TargetBinding,
};

/// Read one host-wrapper plan into the statement of what its wrapper will be.
///
/// # Errors
///
/// Returns [`WrapperSurfaceIssue::RoleNotPlanned`] where the plan declares no
/// member under its kind's one rendered role — the membership is the quantifier,
/// so an unplanned role is an absence the plan itself states rather than a
/// failure to look hard enough.
///
/// Returns [`WrapperSurfaceIssue::DestinationNotHostTarget`] where the planned
/// member is spliced at the declaration site: a wrapper lands in the HOST's own
/// target, which is a different file than the declaration the plan was derived
/// from, so the member is written as a standalone artifact under a byte role and
/// a member spliced beside the declaration is a different delivery.
///
/// Returns [`WrapperSurfaceIssue::TargetBindingFree`] where the plan's context
/// binds no host contract. That posture is foreclosed on this seam's own route —
/// [`ProjectionPlan::planned`](crate::planning::ProjectionPlan::planned) refuses
/// a target-free plan for a kind whose target requirement is a bound contract —
/// and the road exists so this reading has a truthful answer for the posture the
/// TYPE still admits rather than a fabricated one.
///
/// The three checks are DEPENDENT — there is no destination to read until a
/// member was found, and no wrapper to bind until the member lands where a
/// wrapper lands — so exactly one of them is ever established.
pub fn host_wrapper_plan(
    plan: &ProjectionPlan<HostWrapperProjection>,
) -> Result<HostWrapperPlan, WrapperSurfaceIssue> {
    let role = SoleRenderedUnit::Sole;
    let Some(member) = plan.membership().under(role) else {
        return Err(WrapperSurfaceIssue::RoleNotPlanned {
            role_slot: role.slot(),
        });
    };
    let byte_role = match member.output.destination {
        MemberDestination::AsArtifact { byte_role } => byte_role,
        MemberDestination::AtDeclarationSite => {
            return Err(WrapperSurfaceIssue::DestinationNotHostTarget {
                role_slot: role.slot(),
            });
        }
    };
    let TargetBinding::HostContract(host_contract) = plan.context().target else {
        return Err(WrapperSurfaceIssue::TargetBindingFree {
            kind: HostWrapperProjection::KIND_NAME,
        });
    };
    let content = plan.content();
    Ok(HostWrapperPlan {
        role,
        semantic_key: member.output.semantic_key,
        profile: member.output.expected_profile,
        profile_version: member.output.expected_profile_version,
        origin: member.output.origin.clone(),
        declaration: plan.account().commitment(),
        engine: plan.context().generator,
        host_contract,
        content_contract: content.host_contract,
        components: content.components.clone(),
        capability_basis: content.capability_basis,
        landing: HostTargetLanding::in_host_target(byte_role),
    })
}

/// What this home is available for under one shared context.
///
/// # Authority
///
/// **The reading is over the binding the CALLER holds, and the mint's standing
/// is a separate fact carried beside it.** A context that binds a contract makes
/// the whole road below reachable for whoever holds it; a context that binds none
/// makes a plan of this kind unmakeable, and the answer names what would open the
/// road rather than saying only that it is shut.
///
/// # Nonclaims
///
/// [`WrapperAvailability::Bound`] says the caller holds an identity and nothing
/// about whether one can be obtained: a value held inside this workspace is not
/// evidence that the machine's mint exists, and
/// [`HOST_WRAPPER_CONTRACT_MINT`](super::HOST_WRAPPER_CONTRACT_MINT) is where
/// that question is answered. Neither arm claims anything about whether the
/// contract is current, admitted, or reachable — those are the machine's, on the
/// terms every owner identity reference in the plane states.
#[must_use]
pub fn wrapper_availability(context: &ProjectionContext) -> WrapperAvailability {
    match context.target {
        TargetBinding::HostContract(contract) => WrapperAvailability::Bound { contract },
        TargetBinding::TargetFree => WrapperAvailability::NoHostContract {
            opening: HOST_WRAPPER_CONTRACT_MINT,
        },
    }
}
