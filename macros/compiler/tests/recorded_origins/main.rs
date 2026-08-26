//! Origin walks and decision records, observed through the complete public compiler surface.
//!
//! The lane independently spells every canonical byte below rather than calling the identity home's framing helpers.
//! It therefore observes the origin home's public tables, walk and trace invariants, refusal priority, ordering, ceilings, and exact byte contract without becoming a second implementation of planning.

use macroonz_compiler::identity::{self, Identity, Transcript};
use macroonz_compiler::{
    DecisionTrace, Empty, NonEmptyError, Nonclaim, ORIGIN_EDGE_LIMIT, OriginEdge, OriginRelation,
    OriginTrail, Overflow, OwnerFact, TRACE_ENTRY_LIMIT, TraceDecision, TraceEntry, TrailError,
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
