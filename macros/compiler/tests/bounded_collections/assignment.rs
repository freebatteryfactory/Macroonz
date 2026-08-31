//! Exact keyed-roster assignment observed from outside: one payload per denominator member, one unique caller-declared payload seat, and no inferred semantic relation.

use core::error::Error;
use macroonz_compiler::bounded::{
    ForeignRosterReference as HomeForeignRosterReference,
    KeyedRosterAssignment as HomeKeyedRosterAssignment,
    KeyedRosterAssignmentError as HomeKeyedRosterAssignmentError,
    UnassignedRosterMember as HomeUnassignedRosterMember,
};
use macroonz_compiler::{
    Empty, ForeignRosterReference, KeyedRoster, KeyedRosterAssignment, KeyedRosterAssignmentError,
    KeyedRosterError, Overflow, UnassignedRosterMember,
};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, PartialEq, Eq)]
struct Denominator {
    name: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Payload {
    reference: String,
    seat: String,
    value: u8,
}

fn denominator<const N: usize>(
    names: &[&str],
) -> Result<KeyedRoster<Denominator, String, N>, KeyedRosterError<String, N>> {
    let members = names
        .iter()
        .map(|name| Denominator {
            name: (*name).to_owned(),
        })
        .collect();
    KeyedRoster::new(members, |member| member.name.clone())
}

fn payload(reference: &str, seat: &str, value: u8) -> Payload {
    Payload {
        reference: reference.to_owned(),
        seat: seat.to_owned(),
        value,
    }
}

fn assign<const N: usize>(
    denominator: KeyedRoster<Denominator, String, N>,
    payloads: Vec<Payload>,
) -> Result<
    KeyedRosterAssignment<Denominator, String, Payload, String, N>,
    KeyedRosterAssignmentError<String, String, N>,
> {
    KeyedRosterAssignment::complete(
        denominator,
        payloads,
        |offered| offered.reference.clone(),
        |offered| offered.seat.clone(),
    )
}

fn refused_assignment<const N: usize>(
    names: &[&str],
    payloads: Vec<Payload>,
) -> Result<KeyedRosterAssignmentError<String, String, N>, String> {
    assign(
        denominator::<N>(names).map_err(|error| error.to_string())?,
        payloads,
    )
    .err()
    .ok_or_else(|| "the structurally invalid assignment was admitted".to_owned())
}

/// Empty and overflowing offerings refuse before either caller projection runs.
#[test]
fn assignment_settles_magnitude_before_key_work() -> Result<(), String> {
    let references = AtomicUsize::new(0);
    let seats = AtomicUsize::new(0);
    let empty = KeyedRosterAssignment::complete(
        denominator::<2>(&["a", "b"]).map_err(|error| error.to_string())?,
        Vec::<Payload>::new(),
        |offered| {
            references.fetch_add(1, Ordering::SeqCst);
            offered.reference.clone()
        },
        |offered| {
            seats.fetch_add(1, Ordering::SeqCst);
            offered.seat.clone()
        },
    );
    let overflow = KeyedRosterAssignment::complete(
        denominator::<2>(&["a", "b"]).map_err(|error| error.to_string())?,
        vec![
            payload("a", "sa", 1),
            payload("b", "sb", 2),
            payload("a", "sc", 3),
        ],
        |offered| {
            references.fetch_add(1, Ordering::SeqCst);
            offered.reference.clone()
        },
        |offered| {
            seats.fetch_add(1, Ordering::SeqCst);
            offered.seat.clone()
        },
    );

    assert!(matches!(
        empty,
        Err(KeyedRosterAssignmentError::Empty(Empty))
    ));
    assert!(matches!(
        overflow,
        Err(KeyedRosterAssignmentError::Overflow(Overflow {
            capacity: 2,
            offered: 3
        }))
    ));
    assert_eq!(references.load(Ordering::SeqCst), 0);
    assert_eq!(seats.load(Ordering::SeqCst), 0);
    Ok(())
}

/// Every foreign reference is retained in offered order, and seat projection never runs past that refusal.
#[test]
fn assignment_reports_foreign_references_before_other_key_work() -> Result<(), String> {
    let seats = AtomicUsize::new(0);
    let result = KeyedRosterAssignment::complete(
        denominator::<4>(&["a", "b", "c"]).map_err(|error| error.to_string())?,
        vec![
            payload("foreign-b", "same", 1),
            payload("a", "same", 2),
            payload("foreign-a", "other", 3),
        ],
        |offered| offered.reference.clone(),
        |offered| {
            seats.fetch_add(1, Ordering::SeqCst);
            offered.seat.clone()
        },
    );
    let foreign = match result {
        Err(KeyedRosterAssignmentError::ForeignReferences(foreign)) => foreign,
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Err("the foreign assignment was admitted".to_owned()),
    };
    let observed = foreign
        .iter()
        .map(|reference| (reference.key().as_str(), reference.offered_position()))
        .collect::<Vec<_>>();

    assert_eq!(observed, vec![("foreign-b", 0), ("foreign-a", 2)]);
    assert_eq!(seats.load(Ordering::SeqCst), 0);
    Ok(())
}

/// First-position and later duplicate references are grouped once per denominator key before any seat work.
#[test]
fn assignment_reports_every_duplicate_reference_once() -> Result<(), String> {
    let seats = AtomicUsize::new(0);
    let result = KeyedRosterAssignment::complete(
        denominator::<6>(&["a", "b", "c", "d", "e"]).map_err(|error| error.to_string())?,
        vec![
            payload("a", "s0", 0),
            payload("b", "s1", 1),
            payload("a", "s2", 2),
            payload("b", "s3", 3),
            payload("b", "s4", 4),
        ],
        |offered| offered.reference.clone(),
        |offered| {
            seats.fetch_add(1, Ordering::SeqCst);
            offered.seat.clone()
        },
    );
    let duplicates = match result {
        Err(KeyedRosterAssignmentError::DuplicateReferences(duplicates)) => duplicates,
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Err("the duplicate references were admitted".to_owned()),
    };
    let observed = duplicates
        .iter()
        .map(|duplicate| {
            (
                duplicate.key().as_str(),
                duplicate.first_position(),
                duplicate
                    .repeated_positions()
                    .iter()
                    .copied()
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(observed, vec![("a", 0, vec![2]), ("b", 1, vec![3, 4])]);
    assert_eq!(seats.load(Ordering::SeqCst), 0);
    Ok(())
}

/// Every reused payload seat is grouped once after the denominator references have become lawful and unique.
#[test]
fn assignment_reports_every_reused_payload_seat_once() -> Result<(), String> {
    let result = assign(
        denominator::<5>(&["a", "b", "c", "d", "e"]).map_err(|error| error.to_string())?,
        vec![
            payload("a", "left", 0),
            payload("b", "left", 1),
            payload("c", "right", 2),
            payload("d", "right", 3),
            payload("e", "right", 4),
        ],
    );
    let duplicates = match result {
        Err(KeyedRosterAssignmentError::ReusedPayloadSeats(duplicates)) => duplicates,
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Err("the reused payload seats were admitted".to_owned()),
    };
    let observed = duplicates
        .iter()
        .map(|duplicate| {
            (
                duplicate.key().as_str(),
                duplicate.first_position(),
                duplicate
                    .repeated_positions()
                    .iter()
                    .copied()
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        observed,
        vec![("left", 0, vec![1]), ("right", 2, vec![3, 4])]
    );
    Ok(())
}

/// Missing members are reported in denominator order after every offered reference and seat has proved lawful and unique.
#[test]
fn assignment_reports_missing_members_in_denominator_order() -> Result<(), String> {
    let result = assign(
        denominator::<4>(&["a", "b", "c", "d"]).map_err(|error| error.to_string())?,
        vec![payload("c", "seat-c", 3), payload("a", "seat-a", 1)],
    );
    let missing = match result {
        Err(KeyedRosterAssignmentError::MissingMembers(missing)) => missing,
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Err("the incomplete assignment was admitted".to_owned()),
    };

    assert_eq!(
        missing
            .iter()
            .map(|member| (member.key().as_str(), member.denominator_position()))
            .collect::<Vec<_>>(),
        vec![("b", 1), ("d", 3)]
    );
    Ok(())
}

/// A lawful assignment reorders equal-valued payloads into denominator order, retains each seat, and supports checked borrowed-key reads.
#[test]
fn assignment_aligns_payloads_and_exposes_one_read_only_road() -> Result<(), String> {
    let references = AtomicUsize::new(0);
    let seats = AtomicUsize::new(0);
    let assignment = KeyedRosterAssignment::complete(
        denominator::<3>(&["alpha", "beta", "gamma"]).map_err(|error| error.to_string())?,
        vec![
            payload("gamma", "seat-gamma", 7),
            payload("alpha", "seat-alpha", 7),
            payload("beta", "seat-beta", 7),
        ],
        |offered| {
            references.fetch_add(1, Ordering::SeqCst);
            offered.reference.clone()
        },
        |offered| {
            seats.fetch_add(1, Ordering::SeqCst);
            offered.seat.clone()
        },
    )
    .map_err(|error| error.to_string())?;
    let observed = assignment
        .indexed()
        .map(|(index, key, member, seat, held)| {
            (
                index,
                key.as_str(),
                member.name.as_str(),
                seat.as_str(),
                held.value,
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        observed,
        vec![
            (0, "alpha", "alpha", "seat-alpha", 7),
            (1, "beta", "beta", "seat-beta", 7),
            (2, "gamma", "gamma", "seat-gamma", 7),
        ]
    );
    assert_eq!(assignment.count(), 3);
    assert_eq!(assignment.first().0, "alpha");
    assert_eq!(assignment.at(3), None);
    assert_eq!(
        assignment.get("beta").map(|(member, seat, held)| (
            member.name.as_str(),
            seat.as_str(),
            held.value
        )),
        Some(("beta", "seat-beta", 7))
    );
    assert_eq!(assignment.get("foreign"), None);
    assert_eq!(assignment.denominator().first_key(), "alpha");
    assert_eq!(assignment.payloads().first_key(), "seat-alpha");
    assert_eq!(references.load(Ordering::SeqCst), 3);
    assert_eq!(seats.load(Ordering::SeqCst), 3);
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct FiniteDenominator {
    key: u8,
}

#[derive(Debug, PartialEq, Eq)]
struct FinitePayload {
    reference: u8,
    seat: u8,
}

fn finite_digits(mut encoded: usize, length: usize, radix: usize) -> Vec<u8> {
    let mut digits = Vec::with_capacity(length);
    for _ in 0..length {
        let digit = encoded.checked_rem(radix).unwrap_or(0);
        digits.push(u8::try_from(digit).unwrap_or(u8::MAX));
        encoded = encoded.checked_div(radix).unwrap_or(0);
    }
    digits
}

fn has_duplicate(values: &[u8]) -> bool {
    values.iter().enumerate().any(|(index, value)| {
        values
            .iter()
            .skip(index.saturating_add(1))
            .any(|other| other == value)
    })
}

/// Every small finite reference-and-seat offering reaches the refusal class dictated by the public precedence or the one complete assignment.
#[test]
fn assignment_matches_an_exhaustive_finite_precedence_oracle() -> Result<(), String> {
    for length in 1_usize..=3 {
        let reference_count = (0..length).fold(1_usize, |count, _| count.saturating_mul(4));
        let seat_count = (0..length).fold(1_usize, |count, _| count.saturating_mul(3));
        for reference_encoding in 0..reference_count {
            let references = finite_digits(reference_encoding, length, 4);
            for seat_encoding in 0..seat_count {
                let seats = finite_digits(seat_encoding, length, 3);
                verify_finite_case(&references, &seats)?;
            }
        }
    }
    Ok(())
}

fn verify_finite_case(references: &[u8], seats: &[u8]) -> Result<(), String> {
    let denominator = KeyedRoster::<FiniteDenominator, u8, 3>::new(
        (0_u8..3).map(|key| FiniteDenominator { key }).collect(),
        |member| member.key,
    )
    .map_err(|error| error.to_string())?;
    let payloads = references
        .iter()
        .copied()
        .zip(seats.iter().copied())
        .map(|(reference, seat)| FinitePayload { reference, seat })
        .collect();
    let result = KeyedRosterAssignment::complete(
        denominator,
        payloads,
        |offered| offered.reference,
        |offered| offered.seat,
    );
    let expected = if references.iter().any(|reference| *reference >= 3) {
        matches!(
            result,
            Err(KeyedRosterAssignmentError::ForeignReferences(_))
        )
    } else if has_duplicate(references) {
        matches!(
            result,
            Err(KeyedRosterAssignmentError::DuplicateReferences(_))
        )
    } else if has_duplicate(seats) {
        matches!(
            result,
            Err(KeyedRosterAssignmentError::ReusedPayloadSeats(_))
        )
    } else if references.len() < 3 {
        matches!(result, Err(KeyedRosterAssignmentError::MissingMembers(_)))
    } else {
        result.is_ok()
    };
    if expected {
        Ok(())
    } else {
        Err(format!(
            "finite assignment reached the wrong standing: references={references:?}, seats={seats:?}"
        ))
    }
}

/// Home and root paths name one assignment vocabulary, including every coordinate-bearing refusal.
#[test]
fn assignment_home_and_root_paths_name_one_vocabulary() {
    let assignment: Option<HomeKeyedRosterAssignment<u8, u8, u8, u8, 1>> = None;
    let refusal: Option<HomeKeyedRosterAssignmentError<u8, u8, 1>> = None;
    let foreign: Option<HomeForeignRosterReference<u8>> = None;
    let missing: Option<HomeUnassignedRosterMember<u8>> = None;

    assert_eq!(
        assignment,
        Option::<KeyedRosterAssignment<u8, u8, u8, u8, 1>>::None
    );
    assert_eq!(
        refusal,
        Option::<KeyedRosterAssignmentError<u8, u8, 1>>::None
    );
    assert_eq!(foreign, Option::<ForeignRosterReference<u8>>::None);
    assert_eq!(missing, Option::<UnassignedRosterMember<u8>>::None);
}

/// Assignment refusals preserve magnitude sources while structural disagreements remain typed terminal causes.
#[test]
fn assignment_refusal_trait_contracts_are_concrete() {
    let empty = KeyedRosterAssignmentError::<u8, u8, 1>::Empty(Empty);
    let overflow = KeyedRosterAssignmentError::<u8, u8, 1>::Overflow(Overflow {
        capacity: 1,
        offered: 2,
    });

    assert_eq!(
        empty.to_string(),
        "no item offered where at least one is required"
    );
    assert_eq!(overflow.to_string(), "2 items offered where at most 1 fit");
    assert!(empty.source().is_some_and(<dyn Error>::is::<Empty>));
    assert!(overflow.source().is_some_and(<dyn Error>::is::<Overflow>));
}

/// Foreign-reference sentences distinguish one foreign payload from several without discarding the typed coordinates.
#[test]
fn assignment_foreign_reference_sentences_name_the_exact_magnitude() -> Result<(), String> {
    let one = refused_assignment::<2>(&["a", "b"], vec![payload("foreign", "s0", 0)])?;
    let many = refused_assignment::<2>(
        &["a", "b"],
        vec![payload("foreign-a", "s0", 0), payload("foreign-b", "s1", 1)],
    )?;

    assert_eq!(
        one.to_string(),
        "one offered payload references a key outside the denominator"
    );
    assert_eq!(
        many.to_string(),
        "2 offered payloads reference keys outside the denominator"
    );
    Ok(())
}

/// Duplicate-reference sentences distinguish one duplicated denominator key from several.
#[test]
fn assignment_duplicate_reference_sentences_name_the_exact_magnitude() -> Result<(), String> {
    let one = refused_assignment::<2>(
        &["a", "b"],
        vec![payload("a", "s0", 0), payload("a", "s1", 1)],
    )?;
    let many = refused_assignment::<4>(
        &["a", "b"],
        vec![
            payload("a", "s0", 0),
            payload("a", "s1", 1),
            payload("b", "s2", 2),
            payload("b", "s3", 3),
        ],
    )?;

    assert_eq!(
        one.to_string(),
        "one denominator key is referenced by more than one offered payload"
    );
    assert_eq!(
        many.to_string(),
        "2 denominator keys are referenced by more than one offered payload"
    );
    Ok(())
}

/// Payload-seat sentences distinguish one reused caller key from several.
#[test]
fn assignment_reused_payload_seat_sentences_name_the_exact_magnitude() -> Result<(), String> {
    let one = refused_assignment::<2>(
        &["a", "b"],
        vec![payload("a", "same", 0), payload("b", "same", 1)],
    )?;
    let many = refused_assignment::<4>(
        &["a", "b", "c", "d"],
        vec![
            payload("a", "left", 0),
            payload("b", "left", 1),
            payload("c", "right", 2),
            payload("d", "right", 3),
        ],
    )?;

    assert_eq!(
        one.to_string(),
        "one caller-declared payload-seat key is used more than once"
    );
    assert_eq!(
        many.to_string(),
        "2 caller-declared payload-seat keys are used more than once"
    );
    Ok(())
}

/// Missing-member sentences distinguish one unassigned denominator member from several.
#[test]
fn assignment_missing_member_sentences_name_the_exact_magnitude() -> Result<(), String> {
    let one = refused_assignment::<2>(&["a", "b"], vec![payload("a", "s0", 0)])?;
    let many = refused_assignment::<3>(&["a", "b", "c"], vec![payload("a", "s0", 0)])?;

    assert_eq!(
        one.to_string(),
        "one denominator member has no offered payload"
    );
    assert_eq!(
        many.to_string(),
        "2 denominator members have no offered payload"
    );
    Ok(())
}
