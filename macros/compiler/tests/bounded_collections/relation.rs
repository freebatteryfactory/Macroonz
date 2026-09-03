//! Foreign-free keyed-roster rows and their duplicate-free relation promotion observed outside the compiler crate.

use core::error::Error;
use macroonz_compiler::relation::{
    KeyedRosterRelation as HomeKeyedRosterRelation, KeyedRosterRows as HomeKeyedRosterRows,
    KeyedRosterRowsError as HomeKeyedRosterRowsError,
    RepeatedRelationPair as HomeRepeatedRelationPair,
    RepeatedRelationPairs as HomeRepeatedRelationPairs,
};
use macroonz_compiler::{
    KeyedRoster, KeyedRosterError, KeyedRosterRelation, KeyedRosterRows, KeyedRosterRowsError,
    Overflow, RepeatedRelationPair, RepeatedRelationPairs,
};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, PartialEq, Eq)]
struct Node {
    key: u8,
}

#[derive(Debug, PartialEq, Eq)]
struct OtherNode {
    key: u8,
}

#[derive(Debug, PartialEq, Eq)]
struct Row<Payload> {
    left: u8,
    right: u8,
    payload: Payload,
}

type ReferencedRows<'rosters, Payload, const LEFT: usize, const RIGHT: usize, const ROWS: usize> =
    KeyedRosterRows<'rosters, Node, u8, OtherNode, u8, Row<Payload>, LEFT, RIGHT, ROWS>;

fn roster<const N: usize>(keys: &[u8]) -> Result<KeyedRoster<Node, u8, N>, String> {
    KeyedRoster::new(
        keys.iter().copied().map(|key| Node { key }).collect(),
        |node| node.key,
    )
    .map_err(|error| error.to_string())
}

fn other_roster<const N: usize>(keys: &[u8]) -> Result<KeyedRoster<OtherNode, u8, N>, String> {
    KeyedRoster::new(
        keys.iter().copied().map(|key| OtherNode { key }).collect(),
        |node| node.key,
    )
    .map_err(|error| error.to_string())
}

fn rows<'rosters, Payload, const LEFT: usize, const RIGHT: usize, const ROWS: usize>(
    left: &'rosters KeyedRoster<Node, u8, LEFT>,
    right: &'rosters KeyedRoster<OtherNode, u8, RIGHT>,
    offered: Vec<Row<Payload>>,
) -> Result<ReferencedRows<'rosters, Payload, LEFT, RIGHT, ROWS>, KeyedRosterRowsError<u8, u8, ROWS>>
{
    KeyedRosterRows::referenced(left, right, offered, |row| row.left, |row| row.right)
}

/// Row magnitude settles before either endpoint projection runs and preserves its concrete source.
#[test]
fn relation_row_magnitude_precedes_endpoint_work() -> Result<(), String> {
    let left = roster::<2>(&[0, 1])?;
    let right = other_roster::<2>(&[0, 1])?;
    let left_reads = AtomicUsize::new(0);
    let right_reads = AtomicUsize::new(0);
    let overflow = KeyedRosterRows::<Node, u8, OtherNode, u8, Row<()>, 2, 2, 1>::referenced(
        &left,
        &right,
        vec![
            Row {
                left: 0,
                right: 0,
                payload: (),
            },
            Row {
                left: 1,
                right: 1,
                payload: (),
            },
        ],
        |row| {
            left_reads.fetch_add(1, Ordering::SeqCst);
            row.left
        },
        |row| {
            right_reads.fetch_add(1, Ordering::SeqCst);
            row.right
        },
    );
    let overflow = overflow
        .err()
        .ok_or_else(|| "the overflowing row offering was admitted".to_owned())?;
    assert_eq!(overflow.to_string(), "2 items offered where at most 1 fit");
    assert!(overflow.source().is_some_and(<dyn Error>::is::<Overflow>));
    assert_eq!(left_reads.load(Ordering::SeqCst), 0);
    assert_eq!(right_reads.load(Ordering::SeqCst), 0);

    Ok(())
}

/// Every left reference settles before right projection work begins.
#[test]
fn left_reference_refusal_precedes_right_projection() -> Result<(), String> {
    let left = roster::<2>(&[0, 1])?;
    let right = other_roster::<2>(&[0, 1])?;
    let left_reads = AtomicUsize::new(0);
    let right_reads = AtomicUsize::new(0);
    let foreign_left = KeyedRosterRows::<Node, u8, OtherNode, u8, Row<()>, 2, 2, 2>::referenced(
        &left,
        &right,
        vec![
            Row {
                left: 9,
                right: 9,
                payload: (),
            },
            Row {
                left: 8,
                right: 8,
                payload: (),
            },
        ],
        |row| {
            left_reads.fetch_add(1, Ordering::SeqCst);
            row.left
        },
        |row| {
            right_reads.fetch_add(1, Ordering::SeqCst);
            row.right
        },
    );
    let foreign = match foreign_left {
        Err(KeyedRosterRowsError::ForeignLeft(foreign)) => foreign,
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Err("foreign left references were admitted".to_owned()),
    };
    assert_eq!(
        foreign
            .iter()
            .map(|issue| (*issue.key(), issue.offered_position()))
            .collect::<Vec<_>>(),
        vec![(9, 0), (8, 1)]
    );
    assert_eq!(left_reads.load(Ordering::SeqCst), 2);
    assert_eq!(right_reads.load(Ordering::SeqCst), 0);
    let foreign_left_error = KeyedRosterRowsError::<u8, u8, 2>::ForeignLeft(foreign);
    assert_eq!(
        foreign_left_error.to_string(),
        "2 relation rows reference keys outside the left roster"
    );
    assert!(foreign_left_error.source().is_none());

    Ok(())
}

/// Right-reference refusals retain complete magnitude grammar after every left reference is lawful.
#[test]
fn right_reference_refusal_preserves_its_complete_magnitude() -> Result<(), String> {
    let left = roster::<2>(&[0, 1])?;
    let right = other_roster::<2>(&[0, 1])?;
    let foreign_right = KeyedRosterRows::<Node, u8, OtherNode, u8, Row<()>, 2, 2, 2>::referenced(
        &left,
        &right,
        vec![
            Row {
                left: 0,
                right: 9,
                payload: (),
            },
            Row {
                left: 1,
                right: 8,
                payload: (),
            },
        ],
        |row| row.left,
        |row| row.right,
    );
    let foreign_right = foreign_right
        .err()
        .ok_or_else(|| "foreign right references were admitted".to_owned())?;
    assert_eq!(
        foreign_right.to_string(),
        "2 relation rows reference keys outside the right roster"
    );
    assert!(foreign_right.source().is_none());

    let one_foreign_right =
        KeyedRosterRows::<Node, u8, OtherNode, u8, Row<()>, 2, 2, 1>::referenced(
            &left,
            &right,
            vec![Row {
                left: 0,
                right: 9,
                payload: (),
            }],
            |row| row.left,
            |row| row.right,
        )
        .err()
        .ok_or_else(|| "one foreign right reference was admitted".to_owned())?;
    assert_eq!(
        one_foreign_right.to_string(),
        "one relation row references a key outside the right roster"
    );
    Ok(())
}

/// Authored order remains readable while the canonical position order ignores a distinct set's declaration permutation.
#[test]
fn authored_and_canonical_relation_orders_remain_separate() -> Result<(), String> {
    let left = roster::<2>(&[0, 1])?;
    let right = other_roster::<2>(&[0, 1])?;
    let first = rows::<_, 2, 2, 4>(
        &left,
        &right,
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
    )
    .map_err(|error| error.to_string())?
    .distinct()
    .map_err(|error| error.to_string())?;
    let second = rows::<_, 2, 2, 4>(
        &left,
        &right,
        vec![
            Row {
                left: 0,
                right: 0,
                payload: 0,
            },
            Row {
                left: 1,
                right: 0,
                payload: 10,
            },
            Row {
                left: 0,
                right: 1,
                payload: 1,
            },
            Row {
                left: 1,
                right: 1,
                payload: 11,
            },
        ],
    )
    .map_err(|error| error.to_string())?
    .distinct()
    .map_err(|error| error.to_string())?;

    let authored = first
        .rows()
        .indexed()
        .map(
            |(_index, left_key, _left_member, right_key, _right_member, row)| {
                (*left_key, *right_key, row.payload)
            },
        )
        .collect::<Vec<_>>();
    let canonical_first = canonical_payloads(first.rows())?;
    let canonical_second = canonical_payloads(second.rows())?;
    assert_eq!(authored, vec![(1, 1, 11), (0, 1, 1), (1, 0, 10), (0, 0, 0)]);
    assert_eq!(
        canonical_first,
        vec![(0, 0, 0), (0, 1, 1), (1, 0, 10), (1, 1, 11)]
    );
    assert_eq!(canonical_first, canonical_second);
    Ok(())
}

fn canonical_payloads<const LEFT: usize, const RIGHT: usize, const ROWS: usize>(
    relation: &KeyedRosterRows<'_, Node, u8, OtherNode, u8, Row<u8>, LEFT, RIGHT, ROWS>,
) -> Result<Vec<(u8, u8, u8)>, String> {
    (0..relation.count())
        .map(|position| {
            relation
                .canonical_at(position)
                .map(|(left, _left_member, right, _right_member, row)| (*left, *right, row.payload))
                .ok_or_else(|| "a canonical row position escaped its checked relation".to_owned())
        })
        .collect()
}

fn assert_repeated_pair_debug(pairs: &RepeatedRelationPairs<5>) {
    assert_eq!(
        pairs
            .iter()
            .map(|pair| format!("{pair:?}"))
            .collect::<Vec<_>>(),
        vec![
            "RepeatedRelationPair { left_position: 0, right_position: 1, first: 0, repeated: NonEmpty { head: 2, tail: [] } }",
            "RepeatedRelationPair { left_position: 1, right_position: 0, first: 1, repeated: NonEmpty { head: 3, tail: [4] } }",
        ]
    );
}

/// Repetition remains representable until distinct relation posture is requested, then every duplicated pair is reported once.
#[test]
fn distinct_relation_promotion_reports_every_repeated_pair() -> Result<(), String> {
    let left = roster::<2>(&[0, 1])?;
    let right = other_roster::<2>(&[0, 1])?;
    let repeated = rows::<_, 2, 2, 5>(
        &left,
        &right,
        vec![
            Row {
                left: 0,
                right: 1,
                payload: 0_u8,
            },
            Row {
                left: 1,
                right: 0,
                payload: 1_u8,
            },
            Row {
                left: 0,
                right: 1,
                payload: 2_u8,
            },
            Row {
                left: 1,
                right: 0,
                payload: 3_u8,
            },
            Row {
                left: 1,
                right: 0,
                payload: 4_u8,
            },
        ],
    )
    .map_err(|error| error.to_string())?;
    assert_eq!(
        repeated
            .payloads_for(&1, &0)
            .map(|row| row.payload)
            .collect::<Vec<_>>(),
        vec![1_u8, 3_u8, 4_u8]
    );
    assert_eq!(repeated.payloads_for(&1, &1).count(), 0);
    let refusal = repeated
        .distinct()
        .err()
        .ok_or_else(|| "the repeated relation was promoted as duplicate-free".to_owned())?;
    assert_eq!(refusal.count(), 2);
    assert_eq!(
        refusal.to_string(),
        "2 relation endpoint pairs occur more than once"
    );
    assert_eq!(
        refusal
            .iter()
            .map(|pair| (
                pair.left_position(),
                pair.right_position(),
                pair.first_position(),
                pair.repeated_positions()
                    .iter()
                    .copied()
                    .collect::<Vec<_>>()
            ))
            .collect::<Vec<_>>(),
        vec![(0, 1, 0, vec![2]), (1, 0, 1, vec![3, 4])]
    );
    assert_repeated_pair_debug(&refusal);

    let singular = rows::<_, 2, 2, 2>(
        &left,
        &right,
        vec![
            Row {
                left: 0,
                right: 0,
                payload: (),
            },
            Row {
                left: 0,
                right: 0,
                payload: (),
            },
        ],
    )
    .map_err(|error| error.to_string())?
    .distinct()
    .err()
    .ok_or_else(|| "one repeated pair was promoted as distinct".to_owned())?;
    assert_eq!(
        singular.to_string(),
        "one relation endpoint pair occurs more than once"
    );
    assert!(singular.source().is_none());
    Ok(())
}

/// Empty, same-roster, cross-roster, optional-payload, and exact-payload rows share one reference mechanism without `Clone` bounds.
#[test]
fn one_reference_mechanism_carries_each_required_relation_shape() -> Result<(), String> {
    let same = roster::<2>(&[0, 1])?;
    let empty = KeyedRosterRows::<Node, u8, Node, u8, Row<()>, 2, 2, 0>::referenced(
        &same,
        &same,
        Vec::new(),
        |row| row.left,
        |row| row.right,
    )
    .map_err(|error| error.to_string())?;
    assert!(empty.is_empty());
    assert!(empty.distinct().is_ok());

    let self_edge = KeyedRosterRows::<Node, u8, Node, u8, Row<Option<fn()>>, 2, 2, 1>::referenced(
        &same,
        &same,
        vec![Row {
            left: 1,
            right: 1,
            payload: None,
        }],
        |row| row.left,
        |row| row.right,
    )
    .map_err(|error| error.to_string())?;
    assert_eq!(self_edge.count(), 1);
    assert!(!self_edge.is_empty());

    let other = other_roster::<2>(&[4, 5])?;
    let effect_path: fn() = exact_effect;
    let exact = rows::<_, 2, 2, 1>(
        &same,
        &other,
        vec![Row {
            left: 0,
            right: 5,
            payload: effect_path,
        }],
    )
    .map_err(|error| error.to_string())?
    .distinct()
    .map_err(|error| error.to_string())?;
    let retained_effect = exact
        .rows()
        .at(0)
        .map(|(_left, _left_member, _right, _right_member, row)| row.payload)
        .ok_or_else(|| "the exact relation row was absent".to_owned())?;
    retained_effect();
    Ok(())
}

fn exact_effect() {}

/// Every small finite offering reaches left-foreign, right-foreign, repeated, or distinct standing under the public precedence.
#[test]
fn relation_rows_match_an_exhaustive_finite_oracle() -> Result<(), String> {
    for length in 0_usize..=3 {
        let population = (0..length).fold(1_usize, |count, _| count.saturating_mul(6));
        for encoded in 0..population {
            verify_finite_relation(encoded, length)?;
        }
    }
    Ok(())
}

fn verify_finite_relation(mut encoded: usize, length: usize) -> Result<(), String> {
    let left = roster::<2>(&[0, 1])?;
    let right = other_roster::<2>(&[0, 1])?;
    let mut offered = Vec::with_capacity(length);
    for _ in 0..length {
        let digit = encoded.checked_rem(6).unwrap_or(0);
        encoded = encoded.checked_div(6).unwrap_or(0);
        let (left_key, right_key) = match digit {
            0 => (0, 0),
            1 => (0, 1),
            2 => (1, 0),
            3 => (1, 1),
            4 => (2, 0),
            _ => (0, 2),
        };
        offered.push(Row {
            left: left_key,
            right: right_key,
            payload: (),
        });
    }
    let expected_left = offered.iter().any(|row| row.left >= 2);
    let expected_right = !expected_left && offered.iter().any(|row| row.right >= 2);
    let expected_repeated = !expected_left && !expected_right && has_repeated_pair(&offered);
    match rows::<_, 2, 2, 3>(&left, &right, offered) {
        Err(KeyedRosterRowsError::ForeignLeft(_)) if expected_left => Ok(()),
        Err(KeyedRosterRowsError::ForeignRight(_)) if expected_right => Ok(()),
        Err(error) => Err(format!(
            "finite relation reached the wrong refusal: {error}"
        )),
        Ok(referenced) => {
            let observed_repeated = referenced.distinct().is_err();
            if observed_repeated == expected_repeated {
                Ok(())
            } else {
                Err("finite relation reached the wrong repetition standing".to_owned())
            }
        }
    }
}

fn has_repeated_pair<Payload>(rows: &[Row<Payload>]) -> bool {
    rows.iter().enumerate().any(|(index, row)| {
        rows.iter()
            .skip(index.saturating_add(1))
            .any(|other| other.left == row.left && other.right == row.right)
    })
}

/// Root and home paths expose one relation vocabulary and its exact refusal contracts.
#[test]
fn relation_paths_and_refusal_sentences_are_stable() -> Result<(), String> {
    let root_rows: Option<KeyedRosterRows<'static, u8, u8, u8, u8, u8, 1, 1, 1>> = None;
    let home_rows: Option<HomeKeyedRosterRows<'static, u8, u8, u8, u8, u8, 1, 1, 1>> = root_rows;
    let root_relation: Option<KeyedRosterRelation<'static, u8, u8, u8, u8, u8, 1, 1, 1>> = None;
    let home_relation: Option<HomeKeyedRosterRelation<'static, u8, u8, u8, u8, u8, 1, 1, 1>> =
        root_relation;
    let left = roster::<1>(&[0])?;
    let right = other_roster::<1>(&[0])?;
    let root_error = rows::<_, 1, 1, 1>(
        &left,
        &right,
        vec![Row {
            left: 1,
            right: 0,
            payload: (),
        }],
    )
    .err()
    .ok_or_else(|| "the foreign left reference was admitted".to_owned())?;
    let root_error_type: Option<KeyedRosterRowsError<u8, u8, 1>> = None;
    let home_error: Option<HomeKeyedRosterRowsError<u8, u8, 1>> = root_error_type;
    let home_pair: Option<HomeRepeatedRelationPair<1>> = Option::<RepeatedRelationPair<1>>::None;
    let home_pairs: Option<HomeRepeatedRelationPairs<1>> = Option::<RepeatedRelationPairs<1>>::None;

    assert_eq!(home_rows, None);
    assert_eq!(home_relation, None);
    assert_eq!(home_error, None);
    assert_eq!(home_pair, None);
    assert_eq!(home_pairs, None);
    assert_eq!(
        root_error.to_string(),
        "one relation row references a key outside the left roster"
    );
    Ok(())
}

/// Keep the ordinary roster error in this lane's public denominator.
#[test]
fn roster_error_type_remains_the_relation_input_refusal() {
    let refusal: Option<KeyedRosterError<u8, 1>> = None;
    assert_eq!(refusal, None);
}
