//! Rendered units, the projection that carries them, and every local refusal the render boundary owns.
//!
//! The plan is the authority for a unit's declared facts, while the tree is the authority for its material.
//! These observations cross the public surface and require the render home to join those authorities without proving the complete rendering on closure's behalf.

use macroonz_compiler::render::{
    RENDERED_BYTE_LIMIT as MODULE_RENDERED_BYTE_LIMIT,
    RenderedProjection as ModuleRenderedProjection, RenderedUnit as ModuleRenderedUnit,
};
use macroonz_compiler::{
    CrateBinding, Destination, Diagnostic, Expansion, GENERATED_TOKEN_LIMIT, GeneratedToken,
    GeneratedTree, Kind, LineBody, MEMBERSHIP_LIMIT, NoQuestions, Observed, Phase, Producer,
    RENDERED_BYTE_LIMIT, Refused, RenderError, RenderedProjection,
    RenderedUnit as RootRenderedUnit, Request, Role, TextCapture,
};

/// The kind this lane renders under two declared seats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RenderKind;

impl Kind for RenderKind {
    const NAME: &'static str = "lane.render";
    type Content = &'static str;
    type Role = Seat;
    type Question = NoQuestions;
}

/// Two declared seats and one lawful role value deliberately absent from their roster.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Seat {
    /// The first declared unit.
    Head,
    /// The second declared unit.
    Tail,
    /// A role value no plan of this kind may declare.
    Foreign,
}

impl Role for Seat {
    const ALL: &'static [Self] = &[Self::Head, Self::Tail];

    fn name(self) -> &'static str {
        match self {
            Self::Head => "head",
            Self::Tail => "tail",
            Self::Foreign => "foreign",
        }
    }

    fn destination(self) -> Destination {
        Destination::DeclarationSite
    }
}

/// The door under which this lane's diagnostics are projected.
const DOOR: macroonz_compiler::Door = macroonz_compiler::Door::declared(
    "render-lane",
    "render-lane.grammar",
    "render_lane::expand",
    CrateBinding::declared("render_lane"),
    Producer {
        namespace: "render-lane",
        name: "render",
    },
);

/// One declaration captured through the callable text road.
const DECLARATION: &str = "struct Rendered;";

/// One generated tree spelling one word.
fn spelled(word: &str) -> Result<GeneratedTree, ()> {
    GeneratedTree::assembled(vec![GeneratedToken::word(word)]).map_err(|_refusal| ())
}

/// One word tree whose canonical encoding has exactly the requested magnitude.
fn canonical_word_tree(bytes: usize) -> Result<GeneratedTree, ()> {
    const WORD_FRAMING_BYTES: usize = 9;
    let spelling_bytes = bytes.checked_sub(WORD_FRAMING_BYTES).ok_or(())?;
    let tree = spelled(&"x".repeat(spelling_bytes))?;
    if tree.canonical_bytes().len() != bytes {
        return Err(());
    }
    Ok(tree)
}

/// The lawful expansion this lane uses as its plan and rendering fixture.
fn lawful() -> Result<Expansion<RenderKind>, ()> {
    let read = TextCapture::read(DECLARATION).map_err(|_refusal| ())?;
    Request::<RenderKind>::over(read.input().clone(), "render", &DOOR)
        .render(|_plan, out| {
            out.unit(
                Seat::Head,
                GeneratedTree::assembled(vec![GeneratedToken::word("head")])?,
            )?;
            out.unit(
                Seat::Tail,
                GeneratedTree::assembled(vec![GeneratedToken::word("tail")])?,
            )
        })
        .map_err(|_diagnostic| ())
}

/// Length-prefix one byte string exactly as the identity framing specifies.
fn framed(material: &[u8]) -> Vec<u8> {
    let mut bytes = u64::try_from(material.len())
        .unwrap_or(u64::MAX)
        .to_be_bytes()
        .to_vec();
    bytes.extend_from_slice(material);
    bytes
}

/// The independently written canonical encoding of one rendering refusal.
fn encoded_refusal(slot: u8, material: &[u8]) -> Vec<u8> {
    let mut bytes = vec![slot];
    bytes.extend_from_slice(&framed(material));
    bytes
}

/// The independently written material for a refusal that names one seat.
fn seat_material(role: &str) -> Vec<u8> {
    framed(role.as_bytes())
}

/// The independently written material for a refusal carrying one seat and two counts.
fn seat_and_counts(role: &str, bound: usize, observed: usize) -> Vec<u8> {
    let mut material = seat_material(role);
    material.extend_from_slice(&u64::try_from(bound).unwrap_or(u64::MAX).to_be_bytes());
    material.extend_from_slice(&u64::try_from(observed).unwrap_or(u64::MAX).to_be_bytes());
    material
}

/// The independently written material for a refusal carrying two counts.
fn counts(bound: usize, observed: usize) -> Vec<u8> {
    let mut material = u64::try_from(bound)
        .unwrap_or(u64::MAX)
        .to_be_bytes()
        .to_vec();
    material.extend_from_slice(&u64::try_from(observed).unwrap_or(u64::MAX).to_be_bytes());
    material
}

/// The root roster and the named module expose the same established public render vocabulary.
#[test]
fn root_and_module_render_paths_remain_public() -> Result<(), ()> {
    let bound = lawful()?;
    let root: &RenderedProjection<Seat> = bound.closure().rendered();
    let nested: &ModuleRenderedProjection<Seat> = root;
    let unit: &ModuleRenderedUnit<Seat> = nested.under(Seat::Head).ok_or(())?;
    let _: &RootRenderedUnit<Seat> = unit;
    assert_eq!(MODULE_RENDERED_BYTE_LIMIT, RENDERED_BYTE_LIMIT);
    assert_eq!(nested.count(), 2);
    Ok(())
}

/// A materialized unit takes every declared fact from its planned member and both byte names from its exact tree.
#[test]
fn one_unit_joins_one_planned_member_to_one_exact_tree() -> Result<(), ()> {
    let bound = lawful()?;
    let planned = bound.plan().membership().under(Seat::Head).ok_or(())?;
    let unit = bound.closure().rendered().under(Seat::Head).ok_or(())?;
    assert_eq!(unit.reconstructed(), planned.clone());
    assert_eq!(unit.semantic_key(), planned.output.semantic_key);
    assert_eq!(unit.profile(), planned.output.expected_profile);
    assert_eq!(unit.origin(), &planned.output.origin);
    assert_eq!(unit.address(), planned.output.address);
    assert_eq!(unit.destination(), planned.role.destination());
    assert_eq!(unit.bytes(), unit.tree().canonical_bytes());
    assert_eq!(
        unit.digest_under(planned.output.digest_contract),
        unit.digest()
    );

    let changed = RootRenderedUnit::materialized(planned, spelled("changed")?).map_err(|_| ())?;
    assert_eq!(changed.semantic_key(), unit.semantic_key());
    assert_ne!(changed.identity(), unit.identity());
    assert_ne!(changed.digest(), unit.digest());
    Ok(())
}

/// Rendering order remains observable while delivery readings walk declared seats and retain every unit under each seat.
#[test]
fn a_projection_preserves_rendering_order_and_reads_deliveries_in_roster_order() -> Result<(), ()> {
    let bound = lawful()?;
    let membership = bound.plan().membership();
    let head = membership.under(Seat::Head).ok_or(())?;
    let tail = membership.under(Seat::Tail).ok_or(())?;
    let projection = RenderedProjection::materialized(vec![
        RootRenderedUnit::materialized(tail, spelled("tail-first")?).map_err(|_| ())?,
        RootRenderedUnit::materialized(head, spelled("head-first")?).map_err(|_| ())?,
        RootRenderedUnit::materialized(head, spelled("head-second")?).map_err(|_| ())?,
    ])
    .map_err(|_| ())?;

    let rendered: Vec<String> = projection
        .units()
        .iter()
        .map(|unit| unit.tree().inspected())
        .collect();
    assert_eq!(rendered, ["tail-first ", "head-first ", "head-second "]);

    let under_head: Vec<String> = projection
        .units_under(Seat::Head)
        .map(|unit| unit.tree().inspected())
        .collect();
    assert_eq!(under_head, ["head-first ", "head-second "]);
    assert_eq!(projection.count_under(Seat::Head), 2);

    let delivered: Vec<String> = projection
        .units_to(Destination::DeclarationSite)
        .map(|unit| unit.tree().inspected())
        .collect();
    assert_eq!(delivered, ["head-first ", "head-second ", "tail-first "]);
    assert_eq!(projection.count_to(Destination::DeclarationSite), 3);
    Ok(())
}

/// A sink refuses a role absent from the plan before it can materialize a unit with no semantic key.
#[test]
fn a_sink_refuses_a_role_the_plan_did_not_declare() -> Result<(), ()> {
    let read = TextCapture::read(DECLARATION).map_err(|_refusal| ())?;
    let diagnostic: Diagnostic = Request::<RenderKind>::over(read.input().clone(), "render", &DOOR)
        .render(|_plan, out| {
            out.unit(
                Seat::Foreign,
                GeneratedTree::assembled(vec![GeneratedToken::word("foreign")])?,
            )
        })
        .err()
        .ok_or(())?;
    assert_eq!(diagnostic.phase(), Phase::Rendering);
    assert_eq!(diagnostic.observed(), Observed::ContractDisagreement);
    assert!(diagnostic.summary().contains("foreign"));
    assert!(diagnostic.related().carried().is_empty());
    Ok(())
}

/// Every declared render magnitude refuses at its own boundary and reports both counts without truncation.
#[test]
fn every_render_magnitude_refuses_at_its_own_boundary() -> Result<(), ()> {
    let bound = lawful()?;
    let planned = bound.plan().membership().under(Seat::Head).ok_or(())?;

    RootRenderedUnit::materialized(planned, canonical_word_tree(RENDERED_BYTE_LIMIT - 1)?)
        .map_err(|_| ())?;
    RootRenderedUnit::materialized(planned, canonical_word_tree(RENDERED_BYTE_LIMIT)?)
        .map_err(|_| ())?;

    let oversized_tree = canonical_word_tree(RENDERED_BYTE_LIMIT + 1)?;
    let observed_bytes = oversized_tree.canonical_bytes().len();
    let bytes_refusal = RootRenderedUnit::materialized(planned, oversized_tree)
        .err()
        .ok_or(())?;
    assert_eq!(
        bytes_refusal,
        RenderError::BytesUnbounded {
            role: "head",
            bound: RENDERED_BYTE_LIMIT,
            observed: observed_bytes,
        }
    );

    let mut units = Vec::new();
    for at in 0..=MEMBERSHIP_LIMIT {
        let spelling = format!("unit-{at}");
        units.push(RootRenderedUnit::materialized(planned, spelled(&spelling)?).map_err(|_| ())?);
    }
    let units_refusal = RenderedProjection::materialized(units).err().ok_or(())?;
    assert_eq!(
        units_refusal,
        RenderError::UnitsUnbounded {
            bound: MEMBERSHIP_LIMIT,
            observed: MEMBERSHIP_LIMIT + 1,
        }
    );

    let token_overflow =
        GeneratedTree::assembled(vec![GeneratedToken::word("x"); GENERATED_TOKEN_LIMIT + 1])
            .err()
            .ok_or(())?;
    assert_eq!(
        RenderError::from(token_overflow),
        RenderError::TokensUnbounded {
            bound: GENERATED_TOKEN_LIMIT,
            observed: GENERATED_TOKEN_LIMIT + 1,
        }
    );

    assert_eq!(
        RenderedProjection::<Seat>::materialized(Vec::new())
            .err()
            .ok_or(())?,
        RenderError::NothingRendered
    );
    Ok(())
}

/// The five refusal rows have stable discriminants and independently reproducible canonical bytes.
#[test]
fn rendering_refusal_bytes_are_complete_and_row_separated() {
    let rows = [
        RenderError::NothingRendered,
        RenderError::SeatUnplanned { role: "head" },
        RenderError::BytesUnbounded {
            role: "head",
            bound: 7,
            observed: 9,
        },
        RenderError::UnitsUnbounded {
            bound: 11,
            observed: 13,
        },
        RenderError::TokensUnbounded {
            bound: 17,
            observed: 19,
        },
    ];
    let expected = [
        encoded_refusal(1, &[]),
        encoded_refusal(2, &seat_material("head")),
        encoded_refusal(3, &seat_and_counts("head", 7, 9)),
        encoded_refusal(4, &counts(11, 13)),
        encoded_refusal(5, &counts(17, 19)),
    ];

    for (position, (row, bytes)) in rows.iter().zip(expected.iter()).enumerate() {
        assert_eq!(usize::from(row.slot()), position + 1);
        assert_eq!(&row.canonical_bytes(), bytes);
        assert_eq!(row.body(), LineBody::SingleCause);
        assert!(row.related().is_empty());
    }

    for (left_at, left) in rows.iter().enumerate() {
        for right in rows.iter().skip(left_at + 1) {
            assert_ne!(left.canonical_bytes(), right.canonical_bytes());
        }
    }
}
