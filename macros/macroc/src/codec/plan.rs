//! The planning half of the road: what the plan already decided about the codec
//! surface, read off the plan's own public surface.
//!
//! Nothing here decides meaning and nothing here mints an identity. The planned
//! member's semantic key, its expected profile at its version, and its origin
//! trail are the PLAN's answers, read exactly; the address the surface stands on
//! is the entry account's one commitment; the rendering engine is the generator
//! the plan's context names; and the schema, the byte role, the direction, and
//! the assumed owner facts are the kind content's, read and not interpreted. Two
//! readings and no third: the account answers what this was planned over, the
//! membership answers what will be materialized, and this file joins them without
//! keeping a copy of either.
//!
//! # The shape is not here
//!
//! The plan's kind content names a SCHEMA, a BYTE ROLE, a DIRECTION, and the
//! owner facts the codec rests on. It does not name a type, a member, a wire
//! shape, a cardinality, or an assembly road — so
//! [`CodecShape`](super::CodecShape) arrives from the CALLER and this file reads
//! only what the plan actually decided. A generator that invented a member's wire
//! shape would be declaring how somebody else's value is written down and then
//! encoding it that way, which is the one thing these services never do.
//!
//! # The direction is read, and it decides the delivery
//!
//! The direction is the one kind-content seat the rendering DOES turn on: it is
//! what the road table in `type_contract.rs` is asked, and therefore what decides
//! whether the surface carries a reader at all. A plan covering only the encode
//! direction delivers no validator, and this file carries the direction forward
//! so the surface can say so rather than leaving a reader to infer it from an
//! absence.

use super::{CodecPlan, CodecSurfaceIssue};
use crate::plane::{RenderedRole, SoleRenderedUnit};
use crate::planning::{CodecProjection, MemberDestination, ProjectionPlan};

/// Read one codec plan into the statement of what its surface will be.
///
/// # Errors
///
/// Returns [`CodecSurfaceIssue::RoleNotPlanned`] where the plan declares no
/// member under its kind's one rendered role — the membership is the quantifier,
/// so an unplanned role is an absence the plan itself states rather than a
/// failure to look hard enough.
///
/// Returns [`CodecSurfaceIssue::DestinationNotDeclarationSite`] where the planned
/// member lands anywhere but the declaration site: both admitted placements are
/// expansion deliveries — spliced beside the owner's item, or wrapped in a
/// visibly published module — so a standalone artifact, deferred test cargo, and
/// deferred bench cargo are three other deliveries and each reaches this answer.
///
/// The two checks are DEPENDENT — there is no destination to read until a member
/// was found — so exactly one of them is ever established.
pub fn codec_plan(plan: &ProjectionPlan<CodecProjection>) -> Result<CodecPlan, CodecSurfaceIssue> {
    let role = SoleRenderedUnit::Sole;
    let Some(member) = plan.membership().under(role) else {
        return Err(CodecSurfaceIssue::RoleNotPlanned {
            role_slot: role.slot(),
        });
    };
    match member.output.destination {
        MemberDestination::AtDeclarationSite => {}
        // Every delivery this kind does not make reaches one answer, and the
        // arms are written out one by one rather than under a wildcard: a
        // delivery admitted later stops the compiler here until somebody says
        // whether a codec surface is ever written into it.
        MemberDestination::AsArtifact { .. }
        | MemberDestination::IntoTestCarrier
        | MemberDestination::IntoBenchCarrier => {
            return Err(CodecSurfaceIssue::DestinationNotDeclarationSite {
                role_slot: role.slot(),
            });
        }
    }
    let content = plan.content();
    Ok(CodecPlan {
        role,
        semantic_key: member.output.semantic_key,
        profile: member.output.expected_profile,
        profile_version: member.output.expected_profile_version,
        origin: member.output.origin.clone(),
        declaration: plan.account().commitment(),
        engine: plan.context().generator,
        schema: content.schema,
        byte_role: content.byte_role,
        direction: content.direction,
        assumptions: content.assumptions.clone(),
    })
}
