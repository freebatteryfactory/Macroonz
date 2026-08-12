//! The one road: read the anchors and state the account.
//!
//! Nothing here decides meaning. Every identity the plan carries arrives from
//! the caller and crosses unchanged; what this file writes down is the shape of
//! the account — the complete output set, the walk back to the authored
//! declaration, the decisions in selection order with the identity home's facts
//! cited, and the identities whose change makes the account stale.

use super::ScopeGuardStampAnchors;
use crate::origin_graph::{
    DecisionTrace, OriginEdge, OriginRelation, OriginTrail, TraceDecision, TraceEntry,
};
use crate::plane::{AuthoringLimitProfile, PatternArgumentLimit, SoleRenderedUnit};
use crate::planning::{
    DigestContract, InvalidationTrigger, MemberDestination, PatternStampContent,
    PatternStampProjection, PlannedMember, PlannedMembership, PlannedOutput, ProjectionPlan,
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
    let arguments = Bounded::admitted_const(
        vec![anchors.guard_name, anchors.scope_type],
        &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
    )
    .map_err(|_| {
        ProjectionPlanning::bound_exceeded(BoundAxis::Declarations, PatternArgumentLimit::MAX, 2)
    })?;
    let origin = OriginTrail::drawn(
        OriginEdge {
            from: anchors.authored_node,
            relation: OriginRelation::PatternInstantiation,
            to: anchors.instantiated_node,
        },
        vec![OriginEdge {
            from: anchors.instantiated_node,
            relation: OriginRelation::Rendering,
            to: anchors.rendered_node,
        }],
    )?;
    let trace = DecisionTrace::recorded(
        TraceEntry {
            subject: anchors.traced,
            decision: TraceDecision::SelectedBecause(
                anchors.owner_facts.class_c_carries_no_ordering,
            ),
        },
        vec![TraceEntry {
            subject: anchors.traced,
            decision: TraceDecision::SelectedBecause(
                anchors.owner_facts.comparison_is_scope_guarded,
            ),
        }],
    )?;
    let membership = PlannedMembership::from_member(PlannedMember {
        role: SoleRenderedUnit::Sole,
        output: PlannedOutput {
            semantic_key: anchors.stamped_unit,
            destination: MemberDestination::AtDeclarationSite,
            origin: origin.clone(),
            expected_profile: anchors.context.profile,
            expected_profile_version: anchors.context.profile_version,
            digest_contract: DigestContract::over(anchors.stamped_unit),
        },
    });
    let invalidation = InvalidationTrigger::watched(
        anchors.context.cause_trigger(),
        vec![
            anchors.context.graph_trigger(),
            InvalidationTrigger::GeneratorVersionChanged {
                watched: anchors.context.generator,
            },
        ],
    )?;
    ProjectionPlan::<PatternStampProjection>::planned(
        anchors.context.clone(),
        PatternStampContent {
            pattern: anchors.pattern,
            instance: anchors.instance,
            arguments,
        },
        membership,
        invalidation,
        trace,
        origin,
        Bounded::empty(),
    )
}
