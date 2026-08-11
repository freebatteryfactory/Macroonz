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
    ExactIdentity, GeneratedUnitSubject, OriginNodeSubject, OutputBytesSubject, OwnerFactRef,
    PatternArgumentLimit, PatternArgumentSubject, PatternInstanceSubject, PatternSubject,
    TracedSubject,
};
use crate::planning::{
    InvalidationTrigger, OutputIdentity, PatternStampContent, PatternStampProjection,
    PlannedMembership, ProjectionContext, ProjectionPlan,
};
use crate::refusal::{BoundAxis, ProjectionPlanning};
use threadpak::types::{Bounded, ConstLimit};

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
    pub pattern: ExactIdentity<PatternSubject>,
    /// This instantiation of it.
    pub instance: ExactIdentity<PatternInstanceSubject>,
    /// The first typed argument: the guard type the caller named.
    pub guard_name: ExactIdentity<PatternArgumentSubject>,
    /// The second typed argument: the scope type the caller named. A string
    /// never becomes an argument here — the caller states a type.
    pub scope_type: ExactIdentity<PatternArgumentSubject>,
    /// The authored declaration the invocation sits in.
    pub authored_node: ExactIdentity<OriginNodeSubject>,
    /// The instantiated pattern as an origin node.
    pub instantiated_node: ExactIdentity<OriginNodeSubject>,
    /// The rendered guard as an origin node.
    pub rendered_node: ExactIdentity<OriginNodeSubject>,
    /// The generated unit the stamp materializes.
    pub stamped_unit: ExactIdentity<GeneratedUnitSubject>,
    /// That unit's canonical bytes.
    pub stamped_digest: ExactIdentity<OutputBytesSubject>,
    /// The subject the plan's decisions are recorded about.
    pub traced: ExactIdentity<TracedSubject>,
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
    let arguments =
        Bounded::admitted_const(vec![anchors.guard_name, anchors.scope_type]).map_err(|_| {
            ProjectionPlanning::bound_exceeded(
                BoundAxis::Declarations,
                PatternArgumentLimit::MAX,
                2,
            )
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
    let membership = PlannedMembership::from_output(OutputIdentity {
        unit: anchors.stamped_unit,
        digest: anchors.stamped_digest,
        origin: origin.clone(),
    });
    let invalidation = InvalidationTrigger::watched(
        InvalidationTrigger::SourceDeclarationChanged {
            watched: *anchors.context.sources.first(),
        },
        vec![
            InvalidationTrigger::GraphIdentityChanged {
                watched: anchors.context.graph,
            },
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

#[cfg(test)]
mod laws {
    use super::{ScopeGuardOwnerFacts, ScopeGuardStampAnchors, plan_scope_guard_stamp};
    use crate::origin_graph::{OriginRelation, TraceDecision};
    use crate::plane::{ExactIdentity, OwnerFactRef, ProfileVersion};
    use crate::planning::{ProjectionContext, TargetBinding};

    /// One owner fact, distinguished by its fact identity.
    fn owner_fact(fact: u8) -> OwnerFactRef {
        OwnerFactRef {
            home: ExactIdentity::decoded([100; 32]),
            fact: ExactIdentity::decoded([fact; 32]),
        }
    }

    /// The anchors one demo stamp is planned against.
    fn anchors() -> ScopeGuardStampAnchors {
        ScopeGuardStampAnchors {
            context: ProjectionContext {
                graph: ExactIdentity::decoded([101; 32]),
                profile: ExactIdentity::decoded([102; 32]),
                profile_version: ProfileVersion::declared(1),
                sources: ProjectionContext::one_source(ExactIdentity::decoded([103; 32])),
                generator: ExactIdentity::decoded([104; 32]),
                target: TargetBinding::TargetFree,
            },
            pattern: ExactIdentity::decoded([105; 32]),
            instance: ExactIdentity::decoded([106; 32]),
            guard_name: ExactIdentity::decoded([107; 32]),
            scope_type: ExactIdentity::decoded([108; 32]),
            authored_node: ExactIdentity::decoded([109; 32]),
            instantiated_node: ExactIdentity::decoded([110; 32]),
            rendered_node: ExactIdentity::decoded([111; 32]),
            stamped_unit: ExactIdentity::decoded([112; 32]),
            stamped_digest: ExactIdentity::decoded([113; 32]),
            traced: ExactIdentity::decoded([114; 32]),
            owner_facts: ScopeGuardOwnerFacts {
                class_c_carries_no_ordering: owner_fact(115),
                comparison_is_scope_guarded: owner_fact(116),
            },
        }
    }

    /// law: pattern-stamp.a-declarative-stamp-carries-a-complete-plan — the plan
    /// family carries a declarative stamp: one output, a trail that walks back
    /// through the pattern-instantiation edge to the authored declaration, two
    /// decisions in selection order each citing an identity-home fact, three
    /// watched identities, and the two typed arguments the caller stated.
    /// Owed reversal: a stamp planned without its instantiation edge, or with a
    /// string where a typed argument belongs, must break this law.
    #[test]
    fn a_declarative_stamp_carries_a_complete_plan() {
        let planned = plan_scope_guard_stamp(&anchors());
        assert!(planned.is_ok_and(|plan| {
            plan.membership().len() == 1
                && plan.origin().len() == 2
                && matches!(
                    plan.origin().first().relation,
                    OriginRelation::PatternInstantiation
                )
                && plan.trace().len() == 2
                && matches!(
                    plan.trace().first().decision,
                    TraceDecision::SelectedBecause(_)
                )
                && plan.invalidation().len() == 3
                && plan.content().arguments.len() == 2
                && plan.nonclaims().is_empty()
                && !plan.membership().first().origin.is_empty()
        }));
    }

    /// law: pattern-stamp.the-stamp-cites-the-identity-home-and-never-itself —
    /// both decisions cite owner facts of one home, and the two facts are
    /// distinct: a stamp that cited itself would be its own oracle.
    /// Owed reversal: collapsing the two facts into one citation must break this
    /// law.
    #[test]
    fn the_stamp_cites_the_identity_home_and_never_itself() {
        let facts = anchors().owner_facts;
        assert_eq!(
            facts.class_c_carries_no_ordering.home,
            facts.comparison_is_scope_guarded.home
        );
        assert_ne!(
            facts.class_c_carries_no_ordering.fact,
            facts.comparison_is_scope_guarded.fact
        );
        let planned = plan_scope_guard_stamp(&anchors());
        assert!(planned.is_ok_and(|plan| {
            matches!(
                plan.trace().first().decision,
                TraceDecision::SelectedBecause(cited) if cited == facts.class_c_carries_no_ordering
            )
        }));
    }
}
