//! Planning a declarative stamp: the pre-magic story of the machine's
//! `scope_guard_version!` pattern.
//!
//! # Why a declarative stamp still owes a plan
//!
//! The stamp itself is a `macro_rules!` in the machine's identity home — the
//! home that owns the Class-C shape stamps it, and nothing here writes a byte of
//! it. What the services owe is the account: which authored pattern was
//! instantiated, with which typed arguments, on whose declared facts, producing
//! which complete output set, and what would make that account stale.
//!
//! That account is a [`ProjectionPlan<PatternStampProjection>`] like any other,
//! and building one here is the proof that the plan family carries a
//! *declarative* stamp and not only a derive: the pattern kind was already in
//! the sealed roster, and this module shows the roster meant it.
//!
//! # The services mint nothing
//!
//! Every identity a stamp plan carries names something the machine owns — the
//! closed graph, the profile, the declaration that caused it, the authored
//! pattern, the instantiation, the typed arguments, the generated unit. The
//! caller supplies them as [`ScopeGuardStampAnchors`]; this module reads them
//! and adapts none. Nothing here observes the stamp's expansion, and nothing
//! here decides what the stamp means.

use crate::origin_graph::{
    DecisionTrace, OriginEdge, OriginRelation, OriginTrail, TraceDecision, TraceEntry,
};
use crate::plane::{
    GeneratedUnitSubject, OriginNodeSubject, OwnerFactRef, OwnerIdentityRef, PatternArgumentLimit,
    PatternArgumentSubject, PatternInstanceSubject, PatternSubject, ProjectionIdentity,
    SoleRenderedUnit, TracedSubject,
};
use crate::planning::{
    DigestContract, InvalidationTrigger, MemberDestination, PatternStampContent,
    PatternStampProjection, PlannedMember, PlannedMembership, PlannedOutput, ProjectionContext,
    ProjectionPlan,
};
use crate::refusal::{BoundAxis, ProjectionPlanning};
use threadpak::types::{AdmittedLimit, Bounded, ConstLimit};

/// The owner facts one scope-guard stamp cites.
///
/// Both belong to the machine's identity home. The stamp writes nothing they do
/// not already declare, and the plan's trace says so by naming them rather than
/// by asserting that a rule was followed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeGuardOwnerFacts {
    /// The identity home's fact that a Class-C position carries no ordering
    /// operator of its own.
    pub class_c_carries_no_ordering: OwnerFactRef,
    /// The identity home's fact that comparison is total within one scope and
    /// refuses across scopes.
    pub comparison_is_scope_guarded: OwnerFactRef,
}

/// The exact identities one scope-guard stamp is planned against.
///
/// There is no constructor and no default: every seat is required, because a
/// stamp plan that could omit its pattern, its instantiation, or its arguments
/// would be an account that sometimes says less than it knows.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScopeGuardStampAnchors {
    /// The shared plan context: closed graph, profile and version, cause set,
    /// generator version, and target binding.
    pub context: ProjectionContext,
    /// The authored pattern — the machine's scope-guard version pattern.
    pub pattern: OwnerIdentityRef<PatternSubject>,
    /// This instantiation of it.
    pub instance: OwnerIdentityRef<PatternInstanceSubject>,
    /// The first typed argument: the guard type the caller named.
    pub guard_name: OwnerIdentityRef<PatternArgumentSubject>,
    /// The second typed argument: the scope type the caller named. A string
    /// never becomes an argument here — the caller states a type.
    pub scope_type: OwnerIdentityRef<PatternArgumentSubject>,
    /// The authored declaration the invocation sits in.
    pub authored_node: ProjectionIdentity<OriginNodeSubject>,
    /// The instantiated pattern as an origin node.
    pub instantiated_node: ProjectionIdentity<OriginNodeSubject>,
    /// The rendered guard as an origin node.
    pub rendered_node: ProjectionIdentity<OriginNodeSubject>,
    /// The generated unit the stamp materializes.
    pub stamped_unit: ProjectionIdentity<GeneratedUnitSubject>,
    /// The subject the plan's decisions are recorded about.
    pub traced: ProjectionIdentity<TracedSubject>,
    /// The owner facts the stamp rests on.
    pub owner_facts: ScopeGuardOwnerFacts,
}

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
        &AdmittedLimit::under_ceiling(),
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
