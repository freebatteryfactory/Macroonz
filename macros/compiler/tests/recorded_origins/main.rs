//! Origin walks and decision records, observed through the complete public compiler surface.
//!
//! The lane independently spells every canonical byte below rather than calling the identity home's framing helpers.
//! It therefore observes the origin home's public tables, walk and trace invariants, refusal priority, ordering, ceilings, and exact byte contract without becoming a second implementation of planning.

use core::error::Error;
use macroonz_compiler::identity::{self, Identity, Transcript};
use macroonz_compiler::origin::{
    DecisionTrace as ModuleDecisionTrace, Nonclaim as ModuleNonclaim,
    ORIGIN_EDGE_LIMIT as MODULE_ORIGIN_EDGE_LIMIT, OriginEdge as ModuleOriginEdge,
    OriginRelation as ModuleOriginRelation, OriginTrail as ModuleOriginTrail,
    TRACE_ENTRY_LIMIT as MODULE_TRACE_ENTRY_LIMIT, TraceDecision as ModuleTraceDecision,
    TraceEntry as ModuleTraceEntry, TrailError as ModuleTrailError,
};
use macroonz_compiler::{
    BoundAxis, CrateBinding, DecisionTrace, Destination, Door, Empty, GeneratedToken,
    GeneratedTree, Kind, NoQuestions, NonEmptyError, Nonclaim, ORIGIN_EDGE_LIMIT, Observed,
    OriginEdge, OriginRelation, OriginTrail, Overflow, OwnerFact, PlanError, PlanIssue, Producer,
    Refused, Request, Role, SELECTION_FACT, TRACE_ENTRY_LIMIT, TextCapture, TraceDecision,
    TraceEntry, TrailError,
};

/// The kind used to observe the live request crossing into origin values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GeneratedOrigin;

impl Kind for GeneratedOrigin {
    const NAME: &'static str = "lane.generated-origin";
    type Content = &'static str;
    type Role = Seat;
    type Question = NoQuestions;
}

/// The one generated seat in the crossing fixture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Seat {
    /// The unit emitted at the declaration site.
    Unit,
}

impl Role for Seat {
    const ALL: &'static [Self] = &[Self::Unit];

    fn name(self) -> &'static str {
        "unit"
    }

    fn destination(self) -> Destination {
        Destination::DeclarationSite
    }
}

/// The door used by the live request crossing.
const DOOR: Door = Door::declared(
    "origin-lane",
    "origin-lane.grammar",
    "origin_lane::expand",
    CrateBinding::declared("origin_lane"),
    Producer {
        namespace: "origin-lane",
        name: "recorded-origins",
    },
);

/// The first caller fact recorded after the request home's selection fact.
const FIRST_FACT: OwnerFact = OwnerFact {
    home: "origin-lane",
    name: "first-declared-fact",
};

/// The second caller fact recorded after the first.
const SECOND_FACT: OwnerFact = OwnerFact {
    home: "origin-lane",
    name: "second-declared-fact",
};

/// One length in the canonical eight-byte big-endian spelling.
fn length(width: usize, into: &mut Vec<u8>) {
    into.extend_from_slice(&u64::try_from(width).unwrap_or(u64::MAX).to_be_bytes());
}

/// One independently framed byte string.
fn framed(material: &[u8], into: &mut Vec<u8>) {
    length(material.len(), into);
    into.extend_from_slice(material);
}

/// One origin node over declared lane material.
fn node(material: &[u8]) -> Identity<identity::OriginNode> {
    Identity::derived(Transcript::rooted(identity::Role::OriginNode, material, 0))
}

/// One traced subject over declared lane material.
fn subject(material: &[u8]) -> Identity<identity::Traced> {
    Identity::derived(Transcript::rooted(identity::Role::Plan, material, 0))
}

/// One generated-unit identity over declared lane material.
fn unit(material: &[u8]) -> Identity<identity::GeneratedUnit> {
    Identity::derived(Transcript::rooted(
        identity::Role::GeneratedUnit,
        material,
        0,
    ))
}

/// The independently spelled citation bytes of one owner fact.
fn fact_bytes(fact: OwnerFact) -> Vec<u8> {
    let mut bytes = Vec::new();
    framed(fact.home.as_bytes(), &mut bytes);
    framed(fact.name.as_bytes(), &mut bytes);
    bytes
}

/// Appends one edge under the origin home's published byte grammar.
fn edge_bytes(edge: OriginEdge, into: &mut Vec<u8>) {
    framed(edge.from.as_bytes(), into);
    into.push(edge.relation.slot());
    framed(edge.to.as_bytes(), into);
}

/// Appends one decision under the origin home's published byte grammar.
fn decision_bytes(decision: TraceDecision, into: &mut Vec<u8>) {
    into.push(decision.slot());
    match decision {
        TraceDecision::SelectedBecause(fact) | TraceDecision::OmittedBecause(fact) => {
            framed(&fact_bytes(fact), into);
        }
        TraceDecision::NotRun => framed(&[], into),
    }
}

/// Appends one trace entry under the origin home's published byte grammar.
fn entry_bytes(entry: TraceEntry, into: &mut Vec<u8>) {
    framed(entry.subject.as_bytes(), into);
    decision_bytes(entry.decision, into);
}

/// The root roster and the named module expose the same established public origin vocabulary.
#[test]
fn root_and_module_origin_paths_remain_public() {
    let one = node(b"one");
    let two = node(b"two");
    let relation: ModuleOriginRelation = OriginRelation::ExplicitLink;
    let edge: ModuleOriginEdge = OriginEdge {
        from: one,
        relation,
        to: two,
    };
    let trail: ModuleOriginTrail = OriginTrail::from_edge(edge);
    let decision: ModuleTraceDecision = TraceDecision::NotRun;
    let entry: ModuleTraceEntry = TraceEntry {
        subject: subject(b"public-path"),
        decision,
    };
    let trace: ModuleDecisionTrace = DecisionTrace::from_entry(entry);
    let nonclaim: ModuleNonclaim = Nonclaim {
        unclaimed: Identity::derived(Transcript::rooted(identity::Role::Plan, b"public-path", 0)),
        because: FIRST_FACT,
    };
    let refusal: ModuleTrailError = TrailError::Discontinuous { at: 1 };
    assert_eq!(trail.first(), &edge);
    assert_eq!(trace.first(), &entry);
    assert_eq!(decision, TraceDecision::NotRun);
    assert_eq!(nonclaim.because, FIRST_FACT);
    assert_eq!(refusal, TrailError::Discontinuous { at: 1 });
    assert_eq!(MODULE_ORIGIN_EDGE_LIMIT, ORIGIN_EDGE_LIMIT);
    assert_eq!(MODULE_TRACE_ENTRY_LIMIT, TRACE_ENTRY_LIMIT);
}

/// A complete request mints the plan trail, member trail, and decision order that the public road promises.
#[test]
fn a_request_carries_authored_derivation_and_decision_order_into_its_plan() -> Result<(), ()> {
    let read = TextCapture::read("struct Generated;").map_err(|_| ())?;
    let expansion = Request::<GeneratedOrigin>::over(read.input().clone(), "generated", &DOOR)
        .assuming(vec![FIRST_FACT, SECOND_FACT])
        .render(|_plan, out| {
            out.unit(
                Seat::Unit,
                GeneratedTree::assembled(vec![GeneratedToken::word("generated")])?,
            )
        })
        .map_err(|_| ())?;
    let plan = expansion.plan();
    let member = plan.membership().under(Seat::Unit).ok_or(())?;
    let plan_edge = plan.origin().first();
    let member_edge = member.output.origin.first();
    assert_eq!(plan.origin().edges().count(), 1);
    assert_eq!(member.output.origin.edges().count(), 1);
    assert_eq!(plan_edge.from, plan.account().origin_node());
    assert_eq!(member_edge.from, plan.account().origin_node());
    assert_eq!(plan_edge.to, member_edge.to);
    assert_eq!(plan_edge.relation, OriginRelation::AuthoredDeclaration);
    assert_eq!(member_edge.relation, OriginRelation::SemanticDerivation);

    let decisions: Vec<TraceDecision> = plan
        .trace()
        .entries()
        .iter()
        .map(|entry| entry.decision)
        .collect();
    assert_eq!(
        decisions,
        [
            TraceDecision::SelectedBecause(SELECTION_FACT),
            TraceDecision::SelectedBecause(FIRST_FACT),
            TraceDecision::SelectedBecause(SECOND_FACT),
        ]
    );
    Ok(())
}

/// Every trail refusal keeps its existing planning issue and diagnostic classification.
#[test]
fn trail_refusals_keep_their_planning_and_diagnostic_meaning() {
    let generated = unit(b"generated");
    let orphan = PlanError::over_trail(generated, TrailError::Empty(Empty));
    assert_eq!(
        orphan.first_issue(),
        &PlanIssue::OrphanGeneratedNode { node: generated }
    );
    assert_eq!(orphan.observed(), Observed::OriginAbsent);

    let discontinuous = PlanError::over_trail(generated, TrailError::Discontinuous { at: 7 });
    assert_eq!(
        discontinuous.first_issue(),
        &PlanIssue::TrailDiscontinuous { at: 7 }
    );
    assert_eq!(discontinuous.observed(), Observed::OriginAbsent);

    let overflow = Overflow {
        capacity: ORIGIN_EDGE_LIMIT,
        offered: ORIGIN_EDGE_LIMIT.saturating_add(1),
    };
    let unbounded = PlanError::over_trail(generated, TrailError::Overflow(overflow));
    assert_eq!(
        unbounded.first_issue(),
        &PlanIssue::BoundExceeded {
            axis: BoundAxis::OriginEdges,
            bound: u64::try_from(overflow.capacity).unwrap_or(u64::MAX),
            observed: u64::try_from(overflow.offered).unwrap_or(u64::MAX),
        }
    );
    assert_eq!(unbounded.observed(), Observed::BoundExceeded);
}

/// Trail errors retain their own sentence and the bounded refusal that caused one where a cause exists.
#[test]
fn trail_error_contracts_preserve_the_concrete_cause() {
    let discontinuous = TrailError::Discontinuous { at: 7 };
    let empty = TrailError::from(NonEmptyError::Empty(Empty));
    let overflow = TrailError::from(NonEmptyError::Overflow(Overflow {
        capacity: 2,
        offered: 3,
    }));
    assert_eq!(
        discontinuous.to_string(),
        "the edge at position 7 does not start where the edge before it ended"
    );
    assert!(discontinuous.source().is_none());
    assert!(empty.source().is_some_and(<dyn Error>::is::<Empty>));
    assert!(overflow.source().is_some_and(<dyn Error>::is::<Overflow>));
}

/// Every relation has one declared name, one append-only slot, and one exact edge spelling.
#[test]
fn the_relation_roster_and_edge_bytes_are_complete() {
    let expected = [
        (OriginRelation::AuthoredDeclaration, "authored-declaration"),
        (
            OriginRelation::PatternInstantiation,
            "pattern-instantiation",
        ),
        (OriginRelation::SemanticDerivation, "semantic-derivation"),
        (OriginRelation::ExplicitLink, "explicit-link"),
        (OriginRelation::Normalization, "normalization"),
        (OriginRelation::ProfileSelection, "profile-selection"),
        (OriginRelation::ProjectionSelection, "projection-selection"),
        (OriginRelation::Rendering, "rendering"),
        (OriginRelation::TestDerivation, "test-derivation"),
        (OriginRelation::BenchmarkDerivation, "benchmark-derivation"),
        (
            OriginRelation::DiagnosticDerivation,
            "diagnostic-derivation",
        ),
    ];
    assert_eq!(OriginRelation::ALL.len(), expected.len());
    let from = node(b"authored");
    let to = node(b"generated");
    for (position, (declared, (relation, name))) in OriginRelation::ALL
        .iter()
        .copied()
        .zip(expected)
        .enumerate()
    {
        assert_eq!(declared, relation);
        assert_eq!(relation.name(), name);
        assert_eq!(usize::from(relation.slot()), position);
        let edge = OriginEdge { from, relation, to };
        let mut observed = Vec::new();
        edge.encode_into(&mut observed);
        let mut specified = Vec::new();
        edge_bytes(edge, &mut specified);
        assert_eq!(observed, specified);
    }
}

/// A trail is one non-empty bounded walk, with discontinuity settled before magnitude.
#[test]
fn a_trail_admits_exactly_one_joined_walk() -> Result<(), ()> {
    let authored = node(b"authored");
    let selected = node(b"selected");
    let rendered = node(b"rendered");
    let first = OriginEdge {
        from: authored,
        relation: OriginRelation::ProfileSelection,
        to: selected,
    };
    let second = OriginEdge {
        from: selected,
        relation: OriginRelation::Rendering,
        to: rendered,
    };
    let walked: OriginTrail = OriginTrail::drawn(vec![first, second]).map_err(|_| ())?;
    assert_eq!(walked.first(), &first);
    assert!(walked.edges().iter().copied().eq([first, second]));

    assert_eq!(
        OriginTrail::drawn(Vec::new()).err().ok_or(())?,
        TrailError::Empty(Empty)
    );
    let broken = OriginEdge {
        from: authored,
        relation: OriginRelation::Rendering,
        to: rendered,
    };
    assert_eq!(
        OriginTrail::drawn(vec![first, broken]).err().ok_or(())?,
        TrailError::Discontinuous { at: 1 }
    );

    let looped = OriginEdge {
        from: authored,
        relation: OriginRelation::ExplicitLink,
        to: authored,
    };
    let offered = ORIGIN_EDGE_LIMIT.saturating_add(1);
    assert_eq!(
        OriginTrail::drawn(vec![looped; offered]).err().ok_or(())?,
        TrailError::Overflow(Overflow {
            capacity: ORIGIN_EDGE_LIMIT,
            offered,
        })
    );
    let mut over_and_broken = vec![looped; offered];
    *over_and_broken.get_mut(1).ok_or(())? = OriginEdge {
        from: selected,
        relation: OriginRelation::Rendering,
        to: rendered,
    };
    assert_eq!(
        OriginTrail::drawn(over_and_broken).err().ok_or(())?,
        TrailError::Discontinuous { at: 1 }
    );
    Ok(())
}

/// Trail bytes carry the member count and exact walk order.
#[test]
fn trail_bytes_preserve_the_walk_order() -> Result<(), ()> {
    let first_node = node(b"first");
    let second_node = node(b"second");
    let outbound = OriginEdge {
        from: first_node,
        relation: OriginRelation::SemanticDerivation,
        to: second_node,
    };
    let returning = OriginEdge {
        from: second_node,
        relation: OriginRelation::ExplicitLink,
        to: first_node,
    };
    let forward = OriginTrail::drawn(vec![outbound, returning]).map_err(|_| ())?;
    let reverse = OriginTrail::drawn(vec![returning, outbound]).map_err(|_| ())?;
    let mut observed = Vec::new();
    forward.encode_into(&mut observed);
    let mut specified = Vec::new();
    length(2, &mut specified);
    edge_bytes(outbound, &mut specified);
    edge_bytes(returning, &mut specified);
    assert_eq!(observed, specified);
    let mut reversed = Vec::new();
    reverse.encode_into(&mut reversed);
    assert_ne!(observed, reversed);
    Ok(())
}

/// A decision trace preserves every decision, its discriminant, its citation posture, and its order.
#[test]
fn decision_trace_bytes_preserve_decisions_and_order() -> Result<(), ()> {
    let selected_fact = OwnerFact {
        home: "lane",
        name: "selected",
    };
    let omitted_fact = OwnerFact {
        home: "lane",
        name: "omitted",
    };
    let traced = subject(b"request");
    let selected = TraceEntry {
        subject: traced,
        decision: TraceDecision::SelectedBecause(selected_fact),
    };
    let omitted = TraceEntry {
        subject: traced,
        decision: TraceDecision::OmittedBecause(omitted_fact),
    };
    let not_run = TraceEntry {
        subject: traced,
        decision: TraceDecision::NotRun,
    };
    assert_eq!(selected.decision.slot(), 0);
    assert_eq!(omitted.decision.slot(), 1);
    assert_eq!(not_run.decision.slot(), 2);

    let trace = DecisionTrace::recorded(vec![selected, omitted, not_run]).map_err(|_| ())?;
    assert_eq!(trace.first(), &selected);
    assert!(
        trace
            .entries()
            .iter()
            .copied()
            .eq([selected, omitted, not_run])
    );
    let mut observed = Vec::new();
    trace.encode_into(&mut observed);
    let mut specified = Vec::new();
    length(3, &mut specified);
    for entry in [selected, omitted, not_run] {
        entry_bytes(entry, &mut specified);
    }
    assert_eq!(observed, specified);

    let reversed = DecisionTrace::recorded(vec![not_run, omitted, selected]).map_err(|_| ())?;
    let mut reversed_bytes = Vec::new();
    reversed.encode_into(&mut reversed_bytes);
    assert_ne!(observed, reversed_bytes);
    Ok(())
}

/// A decision trace refuses both absence and a count past its declared ceiling.
#[test]
fn a_decision_trace_is_non_empty_and_bounded() -> Result<(), ()> {
    assert_eq!(
        DecisionTrace::recorded(Vec::new()).err().ok_or(())?,
        NonEmptyError::Empty(Empty)
    );
    let entry = TraceEntry {
        subject: subject(b"bounded"),
        decision: TraceDecision::NotRun,
    };
    let offered = TRACE_ENTRY_LIMIT.saturating_add(1);
    assert_eq!(
        DecisionTrace::recorded(vec![entry; offered])
            .err()
            .ok_or(())?,
        NonEmptyError::Overflow(Overflow {
            capacity: TRACE_ENTRY_LIMIT,
            offered,
        })
    );
    Ok(())
}

/// A nonclaim carries the unclaimed subject and the complete cited fact in that order.
#[test]
fn nonclaim_bytes_carry_the_subject_and_its_reason() {
    let because = OwnerFact {
        home: "lane",
        name: "outside-scope",
    };
    let nonclaim = Nonclaim {
        unclaimed: Identity::derived(Transcript::rooted(identity::Role::Plan, b"unclaimed", 0)),
        because,
    };
    let mut observed = Vec::new();
    nonclaim.encode_into(&mut observed);
    let mut specified = Vec::new();
    framed(nonclaim.unclaimed.as_bytes(), &mut specified);
    framed(&fact_bytes(because), &mut specified);
    assert_eq!(observed, specified);
}
