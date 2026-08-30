//! Informed keyed values projected through the paved and raw token roads from outside the compiler crate.
//!
//! These claims fix ordering, exact byte parity, public paths, and the generated-token ceiling without making the projector an oracle over what one row means.

use macroonz_compiler::token::{
    keyed_assignment_slice as home_assignment_slice, keyed_roster_slice as home_roster_slice,
};
use macroonz_compiler::{
    CrateBinding, GENERATED_TOKEN_LIMIT, GeneratedDelimiter, GeneratedToken, GeneratedTree,
    KeyedRoster, KeyedRosterAssignment, Kind, NoQuestions, Overflow, Producer, Request, SoleRole,
    TextCapture, comma_many, group, keyed_assignment_slice, keyed_roster_slice,
};

const MEMBER_LIMIT: usize = 4;

/// The single planned output that receives one paved projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProjectionKind;

impl Kind for ProjectionKind {
    const NAME: &'static str = "structural-token-projection";
    type Content = ();
    type Role = SoleRole;
    type Question = NoQuestions;
}

/// The callable compiler door used by the complete-road crossing.
const PROJECTION_DOOR: macroonz_compiler::Door = macroonz_compiler::Door::declared(
    "structural-token-projection",
    "structural-token-projection.grammar",
    "structural_token_projection::expand",
    CrateBinding::declared("structural_token_projection"),
    Producer {
        namespace: "structural-token-projection",
        name: "project",
    },
);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Member {
    key: String,
    variant: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Payload {
    member: String,
    seat: String,
    value: u64,
}

fn members(
    order: &[(&str, &'static str)],
) -> Result<KeyedRoster<Member, String, MEMBER_LIMIT>, ()> {
    let offered = order
        .iter()
        .map(|(key, variant)| Member {
            key: (*key).to_owned(),
            variant,
        })
        .collect();
    KeyedRoster::new(offered, |member| member.key.clone()).map_err(|_refusal| ())
}

fn payloads(
    denominator: KeyedRoster<Member, String, MEMBER_LIMIT>,
    offered: &[(&str, &str, u64)],
) -> Result<KeyedRosterAssignment<Member, String, Payload, String, MEMBER_LIMIT>, ()> {
    let offered = offered
        .iter()
        .map(|(member, seat, value)| Payload {
            member: (*member).to_owned(),
            seat: (*seat).to_owned(),
            value: *value,
        })
        .collect();
    KeyedRosterAssignment::complete(
        denominator,
        offered,
        |payload| payload.member.clone(),
        |payload| payload.seat.clone(),
    )
    .map_err(|_refusal| ())
}

fn member_row(index: usize, key: &str, member: &Member) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::number(u64::try_from(index).unwrap_or(u64::MAX)),
        GeneratedToken::text(key),
        GeneratedToken::word(member.variant),
    ]
}

fn payload_row(
    index: usize,
    key: &str,
    member: &Member,
    seat: &str,
    payload: &Payload,
) -> Result<Vec<GeneratedToken>, Overflow> {
    Ok(vec![group(
        GeneratedDelimiter::Parenthesis,
        comma_many(vec![
            vec![GeneratedToken::number(
                u64::try_from(index).unwrap_or(u64::MAX),
            )],
            vec![GeneratedToken::text(key)],
            vec![GeneratedToken::word(member.variant)],
            vec![GeneratedToken::text(seat)],
            vec![GeneratedToken::number(payload.value)],
        ]),
    )?])
}

fn raw_roster_slice(
    roster: &KeyedRoster<Member, String, MEMBER_LIMIT>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let rows = roster
        .indexed()
        .map(|(index, key, member)| Ok(member_row(index, key, member)))
        .collect::<Result<Vec<_>, _>>()?;
    borrowed_slice(rows)
}

fn raw_assignment_slice(
    assignment: &KeyedRosterAssignment<Member, String, Payload, String, MEMBER_LIMIT>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let rows = assignment
        .indexed()
        .map(|(index, key, member, seat, payload)| payload_row(index, key, member, seat, payload))
        .collect::<Result<Vec<_>, _>>()?;
    borrowed_slice(rows)
}

fn borrowed_slice(rows: Vec<Vec<GeneratedToken>>) -> Result<Vec<GeneratedToken>, Overflow> {
    Ok(vec![
        GeneratedToken::alone('&'),
        group(GeneratedDelimiter::Bracket, comma_many(rows))?,
    ])
}

fn tree(tokens: Vec<GeneratedToken>) -> Result<GeneratedTree, ()> {
    GeneratedTree::assembled(tokens).map_err(|_refusal| ())
}

/// Claim: the crate root and token home expose one keyed projection behavior.
/// Subject: one two-member keyed roster and one exact assignment over it.
/// Population: both public paths for both new operations.
/// Hostile control: each path runs independently and their canonical bytes must agree.
/// Evidence ceiling: this fixes public path parity, not a facade-crate reexport.
#[test]
fn root_and_token_home_paths_project_one_surface() -> Result<(), ()> {
    let roster = members(&[("first", "First"), ("second", "Second")])?;
    let roster_root = keyed_roster_slice(&roster, |index, key, member| {
        Ok(member_row(index, key, member))
    })
    .map_err(|_refusal| ())?;
    let roster_home = home_roster_slice(&roster, |index, key, member| {
        Ok(member_row(index, key, member))
    })
    .map_err(|_refusal| ())?;
    assert_eq!(
        tree(roster_root)?.canonical_bytes(),
        tree(roster_home)?.canonical_bytes()
    );

    let assignment = payloads(
        roster,
        &[("second", "second-seat", 2), ("first", "first-seat", 1)],
    )?;
    let assignment_root =
        keyed_assignment_slice(&assignment, |index, key, member, seat, payload| {
            payload_row(index, key, member, seat, payload)
        })
        .map_err(|_refusal| ())?;
    let assignment_home =
        home_assignment_slice(&assignment, |index, key, member, seat, payload| {
            payload_row(index, key, member, seat, payload)
        })
        .map_err(|_refusal| ())?;
    assert_eq!(
        tree(assignment_root)?.canonical_bytes(),
        tree(assignment_home)?.canonical_bytes()
    );
    Ok(())
}

/// Claim: the paved keyed-roster road is byte-identical to a raw caller-owned renderer.
/// Subject: one ordered caller-keyed roster.
/// Population: every retained member, key, and index.
/// Hostile control: reversing the denominator must move the generated bytes rather than being silently sorted.
/// Evidence ceiling: this proves one conventional borrowed-slice projection, not a general enum or dispatch renderer.
#[test]
fn keyed_roster_projection_matches_raw_tokens_and_preserves_order() -> Result<(), ()> {
    let declared = members(&[("first", "First"), ("second", "Second")])?;
    let moved = members(&[("second", "Second"), ("first", "First")])?;
    let paved = keyed_roster_slice(&declared, |index, key, member| {
        Ok(member_row(index, key, member))
    })
    .map_err(|_refusal| ())?;
    let raw = raw_roster_slice(&declared).map_err(|_refusal| ())?;
    let moved = keyed_roster_slice(&moved, |index, key, member| {
        Ok(member_row(index, key, member))
    })
    .map_err(|_refusal| ())?;
    let paved = tree(paved)?;
    assert_eq!(paved.canonical_bytes(), tree(raw)?.canonical_bytes());
    assert_ne!(paved.canonical_bytes(), tree(moved)?.canonical_bytes());
    assert_eq!(
        paved.inspected(),
        "& [ 0 \"first\" First , 1 \"second\" Second ] "
    );
    Ok(())
}

/// Claim: exact assignment projection follows denominator order and ignores offered payload order.
/// Subject: two exact assignments carrying the same keyed rows in opposite offered order.
/// Population: every denominator member, retained key, payload seat, payload, and index.
/// Hostile control: a raw renderer independently walks the informed assignment, and both offered orders must reach its bytes.
/// Evidence ceiling: this establishes structural alignment, not the meaning of either payload.
#[test]
fn assignment_projection_matches_raw_tokens_and_denominator_order() -> Result<(), ()> {
    let first = payloads(
        members(&[("first", "First"), ("second", "Second")])?,
        &[("first", "first-seat", 1), ("second", "second-seat", 2)],
    )?;
    let reversed = payloads(
        members(&[("first", "First"), ("second", "Second")])?,
        &[("second", "second-seat", 2), ("first", "first-seat", 1)],
    )?;
    let projected = keyed_assignment_slice(&first, |index, key, member, seat, payload| {
        payload_row(index, key, member, seat, payload)
    })
    .map_err(|_refusal| ())?;
    let reversed = keyed_assignment_slice(&reversed, |index, key, member, seat, payload| {
        payload_row(index, key, member, seat, payload)
    })
    .map_err(|_refusal| ())?;
    let raw = raw_assignment_slice(&first).map_err(|_refusal| ())?;
    let projected = tree(projected)?;
    assert_eq!(projected.canonical_bytes(), tree(raw)?.canonical_bytes());
    assert_eq!(
        projected.canonical_bytes(),
        tree(reversed)?.canonical_bytes()
    );
    Ok(())
}

/// Claim: the paved road invokes one caller row at most once and in retained order.
/// Subject: a three-member keyed roster.
/// Population: every callback coordinate supplied by the operation.
/// Hostile control: the callback records all coordinates independently of the returned tokens.
/// Evidence ceiling: this fixes one successful traversal and does not claim callback execution after a refusal.
#[test]
fn row_projection_runs_once_in_retained_order() -> Result<(), ()> {
    let roster = members(&[("first", "First"), ("second", "Second"), ("third", "Third")])?;
    let mut observed = Vec::new();
    let projected = keyed_roster_slice(&roster, |index, key, member| {
        observed.push((index, key.clone(), member.variant));
        Ok(vec![GeneratedToken::word(member.variant)])
    })
    .map_err(|_refusal| ())?;
    assert!(!projected.is_empty());
    assert_eq!(
        observed,
        vec![
            (0, String::from("first"), "First"),
            (1, String::from("second"), "Second"),
            (2, String::from("third"), "Third"),
        ]
    );
    Ok(())
}

/// Claim: a caller-owned row refusal is returned exactly and stops later row projection.
/// Subject: a three-member keyed roster whose second row refuses.
/// Population: every callback reached before the first refusal.
/// Hostile control: the third coordinate would be observable if traversal continued.
/// Evidence ceiling: prior callbacks can have caller-owned effects; this claims no rollback.
#[test]
fn row_refusal_propagates_exactly_and_stops_projection() -> Result<(), ()> {
    let roster = members(&[("first", "First"), ("second", "Second"), ("third", "Third")])?;
    let expected = Overflow {
        capacity: 7,
        offered: 9,
    };
    let mut observed = Vec::new();
    let refusal = keyed_roster_slice(&roster, |index, key, _member| {
        observed.push((index, key.clone()));
        if index == 1 {
            return Err(expected);
        }
        Ok(vec![GeneratedToken::word("row")])
    });
    assert_eq!(refusal, Err(expected));
    assert_eq!(
        observed,
        vec![(0, String::from("first")), (1, String::from("second")),]
    );
    Ok(())
}

/// Claim: the paved projection is ordinary renderer material inside the existing complete compiler road.
/// Subject: one informed roster rendered under a request's sole planned output.
/// Population: request, plan, output sink, closure, and retained rendered unit.
/// Hostile control: the caller supplies a raw identifier row that the projector must not reinterpret.
/// Evidence ceiling: this proves compiler-road composition, not that the projected source type-checks in a downstream crate.
#[test]
fn paved_projection_enters_the_existing_planned_output_road() -> Result<(), ()> {
    let roster = members(&[("raw", "IgnoredByCallerRow")])?;
    let projected = keyed_roster_slice(&roster, |_index, _key, _member| {
        Ok(vec![GeneratedToken::raw_identifier("type")])
    })
    .map_err(|_refusal| ())?;
    let projected = tree(projected)?;
    assert_eq!(projected.inspected(), "& [ r#type ] ");
    let expected = projected.canonical_bytes();

    let captured = TextCapture::read("struct Projection;").map_err(|_refusal| ())?;
    let expansion = Request::<ProjectionKind>::over(captured.input().clone(), (), &PROJECTION_DOOR)
        .render(|_plan, output| output.unit(SoleRole::Sole, projected))
        .map_err(|_diagnostic| ())?;
    let rendered = expansion
        .closure()
        .rendered()
        .under(SoleRole::Sole)
        .ok_or(())?;
    assert_eq!(rendered.tree().canonical_bytes(), expected);
    Ok(())
}

/// Claim: a borrowed-slice projection refuses whole when its generated group exceeds the token ceiling.
/// Subject: one lawful keyed member whose caller-owned row is wider than the generation magnitude.
/// Population: the completed bracket group built by the paved roster operation.
/// Hostile control: the row is admitted structurally and only its generated width is hostile.
/// Evidence ceiling: this fixes group overflow and does not claim a rendered-byte ceiling crossing.
#[test]
fn oversized_projected_group_refuses_without_a_partial_slice() -> Result<(), ()> {
    let roster = members(&[("only", "Only")])?;
    let refusal = keyed_roster_slice(&roster, |_index, _key, _member| {
        Ok(vec![
            GeneratedToken::word("wide");
            GENERATED_TOKEN_LIMIT + 1
        ])
    });
    assert_eq!(
        refusal,
        Err(Overflow {
            capacity: GENERATED_TOKEN_LIMIT,
            offered: GENERATED_TOKEN_LIMIT + 1,
        })
    );
    Ok(())
}
