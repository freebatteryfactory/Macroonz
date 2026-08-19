//! The planning half of the road: read the anchors and state the account, and
//! read a plan back into the statement of what its published artifact will be.
//!
//! Nothing here decides meaning. Every identity the plan carries arrives from
//! the caller and crosses unchanged; what the first road writes down is the
//! shape of the account — the complete output set, the walk back to the authored
//! declaration, the decisions in selection order with the identity home's facts
//! cited, and the identities whose change makes the account stale. What the
//! second road writes down is what the plan already decided, read off the plan's
//! own public surface and never re-derived.
//!
//! # The entry account
//!
//! A stamp is planned while an expansion is holding token material, so the ONE
//! address its content walked in with is the capture the plane derived for it and
//! the dependency set is empty — which is a STATED fact about content that stands
//! on nothing, not a set somebody forgot to supply. The account is built here,
//! once, from the anchor the caller supplied, and it is MOVED into the plan: the
//! plan's own answer to "what were you planned over" is then the value its
//! identity, its watch set, and its origin edges were all read off, and no second
//! account of content dependencies forms anywhere.
//!
//! # The watch set
//!
//! The shared half is not written here at all. It is
//! [`ProjectionContext::watch_set`](crate::planning::ProjectionContext::watch_set),
//! derived from the context's own seats and from the entry account beside them,
//! which is why a plan bound to a host contract carries a trigger for the
//! contract it is bound to and a plan over one capture carries a trigger for that
//! capture.
//!
//! The watch set does NOT cover the anchors a caller supplies beside the context
//! and the account — the authored pattern, this instantiation of it, the two
//! typed arguments, the origin nodes, the stamped unit, the traced subject, and
//! the cited owner facts. Those define the plan as surely as the context does,
//! and a stamp planned against different ones is a different account.
//!
//! That gap is COUNTED rather than remembered. The road below destructures
//! [`ScopeGuardStampAnchors`] exhaustively, so every anchor is accounted for by
//! the compiler and an anchor added later stops the build until somebody decides
//! what it means for invalidation. Each binding says where its anchor reaches
//! the plan. The reason the remaining ones carry no trigger is not a judgment
//! that they do not matter:
//! [`InvalidationTrigger`](crate::planning::InvalidationTrigger)'s roster
//! declares no seat any of them could be watched through, every seat it does
//! declare is one thirty-two-byte identity of a declared kind, and the set's
//! magnitude IS that roster's cardinality — so minting a seat per anchor would
//! rebuild a hand-maintained roster one level down and push the set past its
//! own bound.
//! That is a declared-limit decision with its own controls, and it is not one
//! this file may take.
//!
//! The derivation home has no such gap: everything its plan is made of is
//! derived from the captured declaration, so watching the capture watches the
//! whole plan.
//!
//! # Two roads, two vocabularies
//!
//! Planning a stamp and reading one back refuse in two different families, and
//! they are not folded into one. A magnitude a stamp plan could not fit inside is
//! a fact about the PLANNING, and a plan that declares its member at the
//! declaration site is a fact about the DELIVERY — a caller told only "the stamp
//! failed" would go looking in the wrong place.

use super::{ScopeGuardStampAnchors, StampedUnitPlan, StampedUnitPlanIssue};
use crate::origin_graph::{
    DecisionTrace, OriginEdge, OriginRelation, OriginTrail, TraceDecision, TraceEntry,
};
use crate::plane::{AuthoringLimitProfile, PatternArgumentLimit, RenderedRole, SoleRenderedUnit};
use crate::planning::{
    DigestContract, MemberDestination, OwnerContentAccount, PatternStampContent,
    PatternStampProjection, PlanDecisions, PlannedMember, PlannedMembership, PlannedOutput,
    ProjectionPlan,
};
use crate::refusal::{BoundAxis, ProjectionPlanning};
use threadpak::types::{AdmittedLimit, Bounded, ConstLimit};

/// Plan one scope-guard stamp.
///
/// The plan states the complete output set — one generated unit, because one
/// invocation stamps one guard — the origin trail that walks back through the
/// [`OriginRelation::PatternInstantiation`] edge to the authored declaration,
/// the decisions in selection order with the identity home's facts cited, and
/// the identities whose change makes the account stale.
///
/// # Errors
///
/// Returns the planning family naming [`BoundAxis::Declarations`] when the
/// supplied typed arguments outgrow the declared magnitude, and
/// [`BoundAxis::OriginEdges`] or [`BoundAxis::TraceEntries`] when the trail or
/// the trace do. It returns the same family naming
/// [`ProjectionPlanningIssue::CauseSetUnwatchable`](crate::refusal::ProjectionPlanningIssue::CauseSetUnwatchable)
/// where the entry account names more commitments than the trigger roster can
/// watch. A stamp plan refuses rather than narrating a partial account.
pub fn plan_scope_guard_stamp(
    anchors: &ScopeGuardStampAnchors,
) -> Result<ProjectionPlan<PatternStampProjection>, ProjectionPlanning> {
    // Exhaustive, and that is the mechanism rather than a style. Each binding
    // below says where its anchor reaches the plan, and an anchor added later
    // stops the build until somebody says the same about it.
    let ScopeGuardStampAnchors {
        // The ONE address the content walked in with. It reaches the plan
        // through the entry account below, which is what the intent identity,
        // the cause trigger, the explanation's causing-declaration answer, and
        // the origin node are all read off.
        content,
        // The shared dependency keys. Together with the account, the only seats
        // whose triggers are derived, because they are the only seats the roster
        // has kinds for.
        context,
        // Reaches the plan through the kind's own content, which the plan
        // transcript deliberately does not commit to — the boundary
        // `PlanDerivation` states. No trigger seat.
        pattern,
        instance,
        // The two typed arguments, likewise inside the kind's content. No
        // trigger seat.
        guard_name,
        scope_type,
        // Reach the plan through the origin trail, and through the member's own
        // copy of it. No trigger seat.
        authored_node,
        instantiated_node,
        rendered_node,
        // Reaches the plan through the membership: it is the member's semantic
        // key and the anchor its digest contract binds to. No trigger seat.
        stamped_unit,
        // Reaches the plan through the decision trace, as the subject both
        // decisions are recorded about. No trigger seat.
        traced,
        // Reach the plan through the decision trace, as the facts the two
        // decisions cite. No trigger seat.
        owner_facts,
    } = anchors;

    // The one account, built once and moved into the plan below. The watch
    // derivation reads it rather than a copy, so there is a single holder of
    // what this content is and what it stands on.
    let account = OwnerContentAccount::<PatternStampProjection>::captured(*content);

    let arguments = Bounded::admitted_const(
        vec![*guard_name, *scope_type],
        &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
    )
    .map_err(|_| {
        ProjectionPlanning::bound_exceeded(BoundAxis::Declarations, PatternArgumentLimit::MAX, 2)
    })?;
    let origin = OriginTrail::drawn(
        OriginEdge {
            from: *authored_node,
            relation: OriginRelation::PatternInstantiation,
            to: *instantiated_node,
        },
        vec![OriginEdge {
            from: *instantiated_node,
            relation: OriginRelation::Rendering,
            to: *rendered_node,
        }],
    )?;
    let trace = DecisionTrace::recorded(
        TraceEntry {
            subject: *traced,
            decision: TraceDecision::SelectedBecause(owner_facts.class_c_carries_no_ordering),
        },
        vec![TraceEntry {
            subject: *traced,
            decision: TraceDecision::SelectedBecause(owner_facts.comparison_is_scope_guarded),
        }],
    )?;
    let membership = PlannedMembership::from_member(PlannedMember {
        role: SoleRenderedUnit::Sole,
        output: PlannedOutput {
            semantic_key: *stamped_unit,
            destination: MemberDestination::AtDeclarationSite,
            origin: origin.clone(),
            expected_profile: context.profile,
            expected_profile_version: context.profile_version,
            digest_contract: DigestContract::over(*stamped_unit),
        },
    });
    let invalidation = context.watch_set(&account)?;
    ProjectionPlan::<PatternStampProjection>::planned(
        account,
        context.clone(),
        PatternStampContent {
            pattern: *pattern,
            instance: *instance,
            arguments,
        },
        PlanDecisions {
            membership,
            invalidation,
            trace,
            origin,
            nonclaims: Bounded::empty(),
        },
    )
}

/// Read one pattern-stamp plan into the statement of what its published artifact
/// will be.
///
/// # Errors
///
/// Returns [`StampedUnitPlanIssue::RoleNotPlanned`] where the plan declares no
/// member under its kind's one rendered role — the membership is the quantifier,
/// so an unplanned role is an absence the plan itself states rather than a
/// failure to look hard enough.
///
/// Returns [`StampedUnitPlanIssue::DestinationNotArtifact`] where the planned
/// member is spliced into the declaration it came from: a published stamp is a
/// standalone artifact written under a byte role, staged and landed by the
/// publication operation and committed by a human, and a member that lands at the
/// declaration site is the delivery that needs no publication road at all.
///
/// The two checks are DEPENDENT — there is no destination to read until a member
/// was found — so exactly one of them is ever established.
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
        MemberDestination::AtDeclarationSite => {
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
