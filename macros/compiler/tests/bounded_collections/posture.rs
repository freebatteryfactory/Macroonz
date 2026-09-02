//! Caller-selected structural postures and pure relation questions observed through public roads.

use super::build_refuses;
use core::error::Error;
use macroonz_compiler::relation::StructuralRequirement as HomeStructuralRequirement;
use macroonz_compiler::{
    AbsencePosture, CanonicalContent, CompletenessPosture, CompletenessStanding, CyclePosture,
    CycleStanding, DensityPosture, DensityStanding, EmptyPosture, KeyedRoster, KeyedRosterRows,
    MembershipPosture, OccupancyStanding, ReachabilityError, RepetitionPosture, RepetitionStanding,
    RosterRelationStanding, RowOrder, SelfRelationPosture, SelfRelationStanding,
    StructuralRequirement, encode_bytes,
};

#[derive(Debug, PartialEq, Eq)]
struct Node {
    key: u8,
}

#[derive(Debug, PartialEq, Eq)]
struct Row {
    left: u8,
    right: u8,
    payload: u8,
}

#[derive(Debug, PartialEq, Eq)]
enum CallerAnswer {
    OvenCold,
    OvenHot,
}

type SameRows<'roster, const MEMBERS: usize, const ROWS: usize> =
    KeyedRosterRows<'roster, Node, u8, Node, u8, Row, MEMBERS, MEMBERS, ROWS>;

fn roster<const N: usize>(keys: &[u8]) -> Result<KeyedRoster<Node, u8, N>, String> {
    KeyedRoster::new(
        keys.iter().copied().map(|key| Node { key }).collect(),
        |node| node.key,
    )
    .map_err(|error| error.to_string())
}

fn rows<'roster, const MEMBERS: usize, const ROWS: usize>(
    left: &'roster KeyedRoster<Node, u8, MEMBERS>,
    right: &'roster KeyedRoster<Node, u8, MEMBERS>,
    offered: Vec<Row>,
) -> Result<SameRows<'roster, MEMBERS, ROWS>, String> {
    KeyedRosterRows::referenced(left, right, offered, |row| row.left, |row| row.right)
        .map_err(|error| error.to_string())
}

fn required<Answer>(
    requirement: Option<StructuralRequirement<Answer>>,
) -> Result<StructuralRequirement<Answer>, String> {
    requirement.ok_or_else(|| "the restrictive posture stated no requirement".to_owned())
}

fn names<Value: Copy>(values: &[Value], name: impl Fn(Value) -> &'static str) -> Vec<&'static str> {
    values.iter().copied().map(name).collect()
}

/// Permissive and restrictive postures consume the same informed value without mutating or re-informing it.
#[test]
fn opposite_postures_settle_over_one_informed_relation() -> Result<(), String> {
    let roster = roster::<2>(&[0, 1])?;
    let relation = rows::<2, 2>(
        &roster,
        &roster,
        vec![
            Row {
                left: 0,
                right: 0,
                payload: 10,
            },
            Row {
                left: 0,
                right: 0,
                payload: 11,
            },
        ],
    )?;

    assert_eq!(relation.occupancy_standing(), OccupancyStanding::Populated);
    assert_eq!(relation.repetition_standing(), RepetitionStanding::Repeated);
    assert_eq!(
        relation.self_relation_standing(),
        Ok(SelfRelationStanding::Present)
    );
    assert_eq!(relation.cycle_standing(), Ok(CycleStanding::Cyclic));
    assert_eq!(relation.left_completeness(), CompletenessStanding::Partial);
    assert_eq!(relation.right_completeness(), CompletenessStanding::Partial);
    assert_eq!(relation.density_standing(), DensityStanding::Sparse);

    assert_eq!(EmptyPosture::Allowed.requirement(), None);
    assert_eq!(RepetitionPosture::Allowed.requirement(), None);
    assert_eq!(SelfRelationPosture::Allowed.requirement(), None);
    assert_eq!(CyclePosture::Allowed.requirement(), None);
    assert_eq!(CompletenessPosture::Partial.requirement(), None);
    assert_eq!(DensityPosture::Sparse.requirement(), None);

    assert_eq!(
        required(EmptyPosture::Refusal.requirement())?.settle(relation.occupancy_standing()),
        Ok(OccupancyStanding::Populated)
    );
    let repetition = required(RepetitionPosture::Refusal.requirement())?
        .settle(relation.repetition_standing())
        .err()
        .ok_or_else(|| "the repeated relation satisfied a distinct requirement".to_owned())?;
    assert_eq!(repetition.required(), &RepetitionStanding::Distinct);
    assert_eq!(repetition.observed(), &RepetitionStanding::Repeated);
    assert_eq!(
        repetition.to_string(),
        "the computed structural answer differs from the caller-required answer"
    );
    assert!(repetition.source().is_none());
    assert!(
        required(SelfRelationPosture::Refusal.requirement())?
            .settle(
                relation
                    .self_relation_standing()
                    .map_err(|error| error.to_string())?
            )
            .is_err()
    );
    assert!(
        required(CyclePosture::Refusal.requirement())?
            .settle(
                relation
                    .cycle_standing()
                    .map_err(|error| error.to_string())?
            )
            .is_err()
    );
    assert!(
        required(CompletenessPosture::Total.requirement())?
            .settle(relation.left_completeness())
            .is_err()
    );
    assert!(
        required(DensityPosture::Dense.requirement())?
            .settle(relation.density_standing())
            .is_err()
    );

    Ok(())
}

/// Empty and populated relations retain exact vacuity rather than inheriting a hidden nonempty law.
#[test]
fn empty_standing_is_computed_before_the_caller_selects_its_law() -> Result<(), String> {
    let roster = roster::<2>(&[0, 1])?;
    let empty = rows::<2, 0>(&roster, &roster, Vec::new())?;

    assert_eq!(empty.occupancy_standing(), OccupancyStanding::Empty);
    assert_eq!(empty.repetition_standing(), RepetitionStanding::Distinct);
    assert_eq!(empty.left_completeness(), CompletenessStanding::Partial);
    assert_eq!(empty.right_completeness(), CompletenessStanding::Partial);
    assert_eq!(empty.density_standing(), DensityStanding::Sparse);
    assert_eq!(
        empty.self_relation_standing(),
        Ok(SelfRelationStanding::Absent)
    );
    assert_eq!(empty.cycle_standing(), Ok(CycleStanding::Acyclic));
    assert!(
        required(EmptyPosture::Refusal.requirement())?
            .settle(empty.occupancy_standing())
            .is_err()
    );
    assert_eq!(EmptyPosture::Allowed.requirement(), None);

    Ok(())
}

/// Authored and canonical order, complete coverage, and semantic-only posture names stay distinct.
#[test]
fn order_completeness_density_membership_and_absence_have_exact_readings() -> Result<(), String> {
    let roster = roster::<2>(&[0, 1])?;
    let relation = rows::<2, 4>(
        &roster,
        &roster,
        vec![
            Row {
                left: 1,
                right: 1,
                payload: 11,
            },
            Row {
                left: 0,
                right: 1,
                payload: 1,
            },
            Row {
                left: 1,
                right: 0,
                payload: 10,
            },
            Row {
                left: 0,
                right: 0,
                payload: 0,
            },
        ],
    )?;

    let authored = relation
        .at_in(RowOrder::Authored, 0)
        .map(|(_left, _left_member, _right, _right_member, row)| row.payload);
    let canonical = relation
        .at_in(RowOrder::Canonical, 0)
        .map(|(_left, _left_member, _right, _right_member, row)| row.payload);
    assert_eq!(authored, Some(11));
    assert_eq!(canonical, Some(0));
    assert_eq!(relation.left_completeness(), CompletenessStanding::Complete);
    assert_eq!(
        relation.right_completeness(),
        CompletenessStanding::Complete
    );
    assert_eq!(relation.density_standing(), DensityStanding::Dense);
    assert_eq!(
        required(CompletenessPosture::Total.requirement())?.settle(relation.left_completeness()),
        Ok(CompletenessStanding::Complete)
    );
    assert_eq!(
        required(DensityPosture::Dense.requirement())?.settle(relation.density_standing()),
        Ok(DensityStanding::Dense)
    );
    assert_eq!(RowOrder::ALL, &[RowOrder::Authored, RowOrder::Canonical]);
    assert_eq!(MembershipPosture::Open.name(), "open");
    assert_eq!(MembershipPosture::Closed.name(), "closed");
    assert_eq!(AbsencePosture::Allowed.name(), "allowed");
    assert_eq!(AbsencePosture::Refusal.name(), "refused");

    Ok(())
}

fn chain(graph: &KeyedRoster<Node, u8, 3>) -> Result<SameRows<'_, 3, 2>, String> {
    rows::<3, 2>(
        graph,
        graph,
        vec![
            Row {
                left: 0,
                right: 1,
                payload: 1,
            },
            Row {
                left: 1,
                right: 2,
                payload: 2,
            },
        ],
    )
}

/// Reachability is roster-ordered and a foreign root refuses at its exact site.
#[test]
fn same_roster_reachability_preserves_its_exact_subject() -> Result<(), String> {
    let graph = roster::<3>(&[0, 1, 2])?;
    let acyclic = chain(&graph)?;
    assert_eq!(
        acyclic.roster_relation_standing(),
        RosterRelationStanding::Same
    );
    assert_eq!(acyclic.cycle_standing(), Ok(CycleStanding::Acyclic));
    let all = acyclic
        .reachability_from(0)
        .map_err(|error| error.to_string())?;
    assert_eq!(all.reachable_positions().collect::<Vec<_>>(), vec![0, 1, 2]);
    assert_eq!(
        all.unreachable_positions().collect::<Vec<_>>(),
        Vec::<usize>::new()
    );
    assert_eq!(all.standing(), CompletenessStanding::Complete);
    let tail = acyclic
        .reachability_from(2)
        .map_err(|error| error.to_string())?;
    assert_eq!(tail.reachable_positions().collect::<Vec<_>>(), vec![2]);
    assert_eq!(tail.unreachable_positions().collect::<Vec<_>>(), vec![0, 1]);
    assert_eq!(tail.standing(), CompletenessStanding::Partial);

    let foreign_root = acyclic
        .reachability_from(9)
        .err()
        .ok_or_else(|| "a foreign root entered reachability".to_owned())?;
    assert!(matches!(
        foreign_root,
        ReachabilityError::RootOutsideRoster { root: 9 }
    ));
    assert_eq!(
        foreign_root.to_string(),
        "the reachability root is outside the shared roster"
    );
    assert!(foreign_root.source().is_none());

    Ok(())
}

/// Adding the return edge changes the same roster's exact cycle answer.
#[test]
fn same_roster_cycle_question_distinguishes_opposite_answers() -> Result<(), String> {
    let graph = roster::<3>(&[0, 1, 2])?;
    assert_eq!(chain(&graph)?.cycle_standing(), Ok(CycleStanding::Acyclic));
    let cyclic = rows::<3, 3>(
        &graph,
        &graph,
        vec![
            Row {
                left: 0,
                right: 1,
                payload: 1,
            },
            Row {
                left: 1,
                right: 2,
                payload: 2,
            },
            Row {
                left: 2,
                right: 0,
                payload: 0,
            },
        ],
    )?;
    assert_eq!(cyclic.cycle_standing(), Ok(CycleStanding::Cyclic));

    Ok(())
}

/// Same-type rosters remain cross-roster subjects unless both sides borrow one instance.
#[test]
fn graph_questions_refuse_two_distinct_roster_instances() -> Result<(), String> {
    let graph = roster::<3>(&[0, 1, 2])?;
    let other = roster::<3>(&[0, 1, 2])?;
    let cross = rows::<3, 1>(
        &graph,
        &other,
        vec![Row {
            left: 0,
            right: 1,
            payload: 1,
        }],
    )?;
    assert_eq!(
        cross.roster_relation_standing(),
        RosterRelationStanding::Cross
    );
    let cycle_refusal = cross.cycle_standing().err().ok_or_else(|| {
        "a cross-roster relation answered a same-roster cycle question".to_owned()
    })?;
    assert_eq!(
        cycle_refusal.to_string(),
        "the structural question requires both relation sides to borrow one roster instance"
    );
    let reachability_refusal = cross
        .reachability_from(0)
        .err()
        .ok_or_else(|| "a cross-roster relation answered reachability".to_owned())?;
    assert!(matches!(
        reachability_refusal,
        ReachabilityError::DifferentRosters(_)
    ));
    assert!(reachability_refusal.source().is_some());

    Ok(())
}

/// A one-member empty relation reaches its root vacuously but does not invent an edge or a cycle.
#[test]
fn one_member_empty_graph_has_exact_vacuity() -> Result<(), String> {
    let roster = roster::<1>(&[7])?;
    let empty = rows::<1, 0>(&roster, &roster, Vec::new())?;
    let reachability = empty
        .reachability_from(7)
        .map_err(|error| error.to_string())?;

    assert_eq!(
        reachability.reachable_positions().collect::<Vec<_>>(),
        vec![0]
    );
    assert_eq!(
        reachability.unreachable_positions().collect::<Vec<_>>(),
        Vec::<usize>::new()
    );
    assert_eq!(reachability.standing(), CompletenessStanding::Complete);
    assert_eq!(
        empty.self_relation_standing(),
        Ok(SelfRelationStanding::Absent)
    );
    assert_eq!(empty.cycle_standing(), Ok(CycleStanding::Acyclic));
    assert_eq!(empty.density_standing(), DensityStanding::Sparse);

    let populated = rows::<1, 1>(
        &roster,
        &roster,
        vec![Row {
            left: 7,
            right: 7,
            payload: 1,
        }],
    )?;
    assert_eq!(
        populated.left_completeness(),
        CompletenessStanding::Complete
    );
    assert_eq!(
        populated.right_completeness(),
        CompletenessStanding::Complete
    );
    assert_eq!(populated.density_standing(), DensityStanding::Dense);

    Ok(())
}

/// Every small same-roster relation agrees with an independently calculated structural oracle.
#[test]
fn structural_questions_match_an_exhaustive_two_member_oracle() -> Result<(), String> {
    for length in 0_usize..=3 {
        let population = (0..length).fold(1_usize, |count, _| count.saturating_mul(4));
        for encoded in 0..population {
            verify_structural_oracle(encoded, length)?;
        }
    }
    Ok(())
}

fn verify_structural_oracle(mut encoded: usize, length: usize) -> Result<(), String> {
    let roster = roster::<2>(&[0, 1])?;
    let mut offered = Vec::with_capacity(length);
    for position in 0..length {
        let pair = encoded.checked_rem(4).unwrap_or(0);
        encoded = encoded.checked_div(4).unwrap_or(0);
        offered.push(Row {
            left: pair.checked_div(2).unwrap_or(0).try_into().unwrap_or(0),
            right: pair.checked_rem(2).unwrap_or(0).try_into().unwrap_or(0),
            payload: position.try_into().unwrap_or(u8::MAX),
        });
    }
    let relation = rows::<2, 3>(&roster, &roster, offered)?;
    let pairs = relation
        .indexed()
        .map(|(_position, left, _left_member, right, _right_member, _row)| (*left, *right))
        .collect::<Vec<_>>();
    assert_eq!(
        relation.occupancy_standing(),
        if pairs.is_empty() {
            OccupancyStanding::Empty
        } else {
            OccupancyStanding::Populated
        }
    );
    assert_eq!(relation.repetition_standing(), repetition_of(&pairs));
    assert_eq!(relation.left_completeness(), left_completeness_of(&pairs));
    assert_eq!(relation.right_completeness(), right_completeness_of(&pairs));
    assert_eq!(relation.density_standing(), density_of(&pairs));
    assert_eq!(
        relation.self_relation_standing(),
        Ok(self_relation_of(&pairs))
    );
    assert_eq!(relation.cycle_standing(), Ok(cycle_of(&pairs)));
    assert_eq!(
        relation
            .reachability_from(0)
            .map_err(|error| error.to_string())?
            .reachable_positions()
            .collect::<Vec<_>>(),
        reachability_of(&pairs, 0)
    );
    assert_eq!(
        relation
            .reachability_from(1)
            .map_err(|error| error.to_string())?
            .reachable_positions()
            .collect::<Vec<_>>(),
        reachability_of(&pairs, 1)
    );
    Ok(())
}

fn repetition_of(pairs: &[(u8, u8)]) -> RepetitionStanding {
    if pairs.iter().enumerate().any(|(position, pair)| {
        pairs
            .iter()
            .skip(position.saturating_add(1))
            .any(|later| later == pair)
    }) {
        RepetitionStanding::Repeated
    } else {
        RepetitionStanding::Distinct
    }
}

fn left_completeness_of(pairs: &[(u8, u8)]) -> CompletenessStanding {
    if [0_u8, 1]
        .iter()
        .all(|member| pairs.iter().any(|pair| pair.0 == *member))
    {
        CompletenessStanding::Complete
    } else {
        CompletenessStanding::Partial
    }
}

fn right_completeness_of(pairs: &[(u8, u8)]) -> CompletenessStanding {
    if [0_u8, 1]
        .iter()
        .all(|member| pairs.iter().any(|pair| pair.1 == *member))
    {
        CompletenessStanding::Complete
    } else {
        CompletenessStanding::Partial
    }
}

fn density_of(pairs: &[(u8, u8)]) -> DensityStanding {
    if [(0_u8, 0_u8), (0, 1), (1, 0), (1, 1)]
        .iter()
        .all(|pair| pairs.contains(pair))
    {
        DensityStanding::Dense
    } else {
        DensityStanding::Sparse
    }
}

fn self_relation_of(pairs: &[(u8, u8)]) -> SelfRelationStanding {
    if pairs.iter().any(|pair| pair.0 == pair.1) {
        SelfRelationStanding::Present
    } else {
        SelfRelationStanding::Absent
    }
}

fn cycle_of(pairs: &[(u8, u8)]) -> CycleStanding {
    let self_cycle = pairs.iter().any(|pair| pair.0 == pair.1);
    let two_member_cycle = pairs.contains(&(0, 1)) && pairs.contains(&(1, 0));
    if self_cycle || two_member_cycle {
        CycleStanding::Cyclic
    } else {
        CycleStanding::Acyclic
    }
}

fn reachability_of(pairs: &[(u8, u8)], root: u8) -> Vec<usize> {
    let other = 1_u8.saturating_sub(root);
    let reaches_other = pairs.contains(&(root, other));
    [0_u8, 1]
        .into_iter()
        .filter(|member| *member == root || (*member == other && reaches_other))
        .map(usize::from)
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SelectedPostures {
    order: RowOrder,
    empty: EmptyPosture,
    cycle: Option<CyclePosture>,
}

impl CanonicalContent for SelectedPostures {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.order.name().as_bytes(), into);
        encode_bytes(self.empty.name().as_bytes(), into);
        match self.cycle {
            Some(cycle) => encode_bytes(cycle.name().as_bytes(), into),
            None => encode_bytes(&[], into),
        }
    }
}

/// Selected posture changes move caller-owned canonical content, while merely asking an unselected question does not.
#[test]
fn semantic_holder_controls_posture_identity_movement() -> Result<(), String> {
    let roster = roster::<1>(&[0])?;
    let relation = rows::<1, 0>(&roster, &roster, Vec::new())?;
    let allowed = SelectedPostures {
        order: RowOrder::Canonical,
        empty: EmptyPosture::Allowed,
        cycle: None,
    };
    let before = allowed.canonical_content_bytes();
    assert_eq!(relation.cycle_standing(), Ok(CycleStanding::Acyclic));
    assert_eq!(before, allowed.canonical_content_bytes());

    let refused = SelectedPostures {
        order: RowOrder::Canonical,
        empty: EmptyPosture::Refusal,
        cycle: None,
    };
    assert_ne!(before, refused.canonical_content_bytes());
    let selected_cycle = SelectedPostures {
        order: RowOrder::Canonical,
        empty: EmptyPosture::Allowed,
        cycle: Some(CyclePosture::Refusal),
    };
    assert_ne!(before, selected_cycle.canonical_content_bytes());

    Ok(())
}

/// Root and home paths expose one requirement type without a parallel posture API.
#[test]
fn posture_paths_share_one_public_type() {
    let root = StructuralRequirement::stated(OccupancyStanding::Populated);
    let home: HomeStructuralRequirement<OccupancyStanding> = root;
    assert_eq!(home.required(), &OccupancyStanding::Populated);
}

/// Every public posture name is structural, and a caller-domain answer never enters the owned mismatch sentence.
#[test]
fn public_posture_vocabulary_and_diagnostics_remain_domain_free() -> Result<(), String> {
    assert_eq!(
        names(RowOrder::ALL, RowOrder::name),
        vec!["authored", "canonical"]
    );
    assert_eq!(
        names(RepetitionPosture::ALL, RepetitionPosture::name),
        vec!["allowed", "refused"]
    );
    assert_eq!(
        names(EmptyPosture::ALL, EmptyPosture::name),
        vec!["allowed", "refused"]
    );
    assert_eq!(
        names(MembershipPosture::ALL, MembershipPosture::name),
        vec!["open", "closed"]
    );
    assert_eq!(
        names(CompletenessPosture::ALL, CompletenessPosture::name),
        vec!["partial", "total"]
    );
    assert_eq!(
        names(DensityPosture::ALL, DensityPosture::name),
        vec!["sparse", "dense"]
    );
    assert_eq!(
        names(AbsencePosture::ALL, AbsencePosture::name),
        vec!["allowed", "refused"]
    );
    assert_eq!(
        names(SelfRelationPosture::ALL, SelfRelationPosture::name),
        vec!["allowed", "refused"]
    );
    assert_eq!(
        names(CyclePosture::ALL, CyclePosture::name),
        vec!["allowed", "refused"]
    );
    assert_eq!(
        names(OccupancyStanding::ALL, OccupancyStanding::name),
        vec!["empty", "populated"]
    );
    assert_eq!(
        names(RepetitionStanding::ALL, RepetitionStanding::name),
        vec!["distinct", "repeated"]
    );
    assert_eq!(
        names(CompletenessStanding::ALL, CompletenessStanding::name),
        vec!["partial", "complete"]
    );
    assert_eq!(
        names(DensityStanding::ALL, DensityStanding::name),
        vec!["sparse", "dense"]
    );
    assert_eq!(
        names(RosterRelationStanding::ALL, RosterRelationStanding::name),
        vec!["same", "cross"]
    );
    assert_eq!(
        names(SelfRelationStanding::ALL, SelfRelationStanding::name),
        vec!["absent", "present"]
    );
    assert_eq!(
        names(CycleStanding::ALL, CycleStanding::name),
        vec!["acyclic", "cyclic"]
    );

    let mismatch = StructuralRequirement::stated(CallerAnswer::OvenCold)
        .settle(CallerAnswer::OvenHot)
        .err()
        .ok_or_else(|| "two caller-domain answers agreed".to_owned())?;
    assert_eq!(
        mismatch.to_string(),
        "the computed structural answer differs from the caller-required answer"
    );
    assert!(!mismatch.to_string().contains("Oven"));

    Ok(())
}

/// A caller cannot forge a reachability partition around the question that establishes it.
#[test]
fn reachability_partition_fields_remain_private() -> Result<(), String> {
    build_refuses(
        include_str!("build-fail/a-reachability-partition-cannot-be-forged.rs"),
        "fields `reachable` and `unreachable` of struct `Reachability` are private",
    )
}
