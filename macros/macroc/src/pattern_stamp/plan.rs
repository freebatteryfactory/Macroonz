//! The one road: read the anchors and state the account.
//!
//! Nothing here decides meaning. Every identity the plan carries arrives from
//! the caller and crosses unchanged; what this file writes down is the shape of
//! the account — the complete output set, the walk back to the authored
//! declaration, the decisions in selection order with the identity home's facts
//! cited, and the identities whose change makes the account stale.
//!
//! # What the watch set covers, exactly
//!
//! The shared half is not written here at all. It is
//! [`ProjectionContext::watch_set`](crate::planning::ProjectionContext::watch_set),
//! derived from the context's own seats — so the roster that used to stand at
//! this call site, and go stale against a context declared elsewhere, is gone.
//! One consequence lands immediately: the target binding is watched now. The
//! hand-written roster named the cause set, the graph, the profile and the
//! generator and stopped, so a plan bound to a host contract carried no trigger
//! for the contract it was bound to, while the roster's
//! [`InvalidationTrigger::TargetContractChanged`] seat sat unused. No context in
//! the tree is target-bound today, which is exactly why nobody met it.
//!
//! The watch set does NOT cover the anchors a caller supplies beside the context
//! — the authored pattern, this instantiation of it, the two typed arguments,
//! the origin nodes, the stamped unit, the traced subject, and the cited owner
//! facts. Those define the plan as surely as the context does, and a stamp
//! planned against different ones is a different account.
//!
//! That gap is now COUNTED rather than remembered. The road below destructures
//! [`ScopeGuardStampAnchors`] exhaustively, so every anchor is accounted for by
//! the compiler and an anchor added later stops the build until somebody decides
//! what it means for invalidation. Each binding says where its anchor reaches
//! the plan. The reason the remaining ones carry no trigger is not a judgment
//! that they do not matter: [`InvalidationTrigger`]'s roster declares no seat any
//! of them could be watched through, every seat it does declare is one
//! thirty-two-byte identity of a declared kind, and the set's magnitude IS that
//! roster's cardinality — so minting a seat per anchor would rebuild the
//! hand-maintained roster one level down and push the set past its own bound.
//! That is a declared-limit decision with its own controls, and it is not one
//! this file may take.
//!
//! The derivation home has no such gap: everything its plan is made of is
//! derived from the captured declaration, so watching the capture watches the
//! whole plan.

use super::ScopeGuardStampAnchors;
use crate::origin_graph::{
    DecisionTrace, OriginEdge, OriginRelation, OriginTrail, TraceDecision, TraceEntry,
};
use crate::plane::{AuthoringLimitProfile, PatternArgumentLimit, SoleRenderedUnit};
use crate::planning::{
    DigestContract, MemberDestination, PatternStampContent, PatternStampProjection, PlannedMember,
    PlannedMembership, PlannedOutput, ProjectionPlan,
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
/// the trace do. A stamp plan refuses rather than narrating a partial account.
pub fn plan_scope_guard_stamp(
    anchors: &ScopeGuardStampAnchors,
) -> Result<ProjectionPlan<PatternStampProjection>, ProjectionPlanning> {
    // Exhaustive, and that is the mechanism rather than a style. The watch set
    // used to be a roster written beside these anchors, so an anchor added later
    // joined the plan and joined nothing else, silently. Destructured, every
    // anchor is accounted for HERE: each binding below says where its anchor
    // reaches the plan, and one added tomorrow stops the build until somebody
    // says the same about it.
    let ScopeGuardStampAnchors {
        // The shared dependency keys. The only seat whose triggers are derived,
        // because it is the only seat the roster has kinds for.
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
    ProjectionPlan::<PatternStampProjection>::planned(
        context.clone(),
        PatternStampContent {
            pattern: *pattern,
            instance: *instance,
            arguments,
        },
        membership,
        context.watch_set()?,
        trace,
        origin,
        Bounded::empty(),
    )
}
