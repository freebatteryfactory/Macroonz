//! The one line every refusal is projected through, and the seats the diagnostic carrying it fills.
//!
//! `<prefix>: <class>: <first established issue>[<body>][<site>]`, and there is no second composition anywhere in the crate.
//! A user of three derives built on this compiler reads three diagnostics shaped one way, so the grammar is asked here exactly rather than approximately.
//!
//! # Reversals
//!
//! A grammar that composed anything would satisfy a lane that only checked for a non-empty sentence.
//! So each clause is required to appear where it belongs and to stay absent where it does not: a whole-declaration refusal adds no position, a handle the producer's table does not reach says so rather than rendering a number, and two refusals that classify alike stay two refusals.

use macroonz::{
    Bounded, Capping, CoordinateRole, CrateBinding, Diagnostic, Door, ExplanationError,
    ExplanationIssue, Family, Line, LineBody, LineSite, Observed, Phase, Placement, Producer,
    RELATED_ISSUE_LIMIT, REPAIR_LIMIT, RefusalClass, Refused, RenderError, Repair, SiteCoordinate,
    SourceCoordinate, SpanHandle, TextCapture, UniversalQuestion, composed,
};

/// The one value that says who is asking.
const DOOR: Door = Door::declared(
    "lane",
    "lane.line.grammar",
    "lane::line",
    CrateBinding::declared("demo"),
    Producer {
        namespace: "lane",
        name: "line",
    },
);

/// One refusal that enumerates a remainder wider than a related set carries, declared by this lane under its own family.
///
/// The compiler's own homes cap their issue bodies at or under the related magnitude, so with the primary cause outside the set none of them can reach the truncated posture any more.
/// An adopter's refusal is under no such cap, and this is one: the first cause plus a remainder exactly as wide as the whole related magnitude.
struct Overflowing;

impl Refused for Overflowing {
    const PHASE: Phase = Phase::Rendering;
    const FAMILY: Family = Family::declared("lane/overflowing");

    fn class(&self) -> RefusalClass {
        RefusalClass::RenderingNotClosed
    }

    fn first(&self) -> String {
        "the first of many established issues".to_owned()
    }

    fn observed(&self) -> Observed {
        Observed::ContractDisagreement
    }

    fn body(&self) -> LineBody {
        LineBody::Body {
            further: RELATED_ISSUE_LIMIT,
            capping: Capping::Complete,
        }
    }

    fn related(&self) -> Vec<Vec<u8>> {
        (0..RELATED_ISSUE_LIMIT)
            .map(|at| at.to_be_bytes().to_vec())
            .collect()
    }

    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        Bounded::empty()
    }
}

/// One refusal of the rendering family, for the lanes that need a second family.
const NOTHING_RENDERED: RenderError = RenderError::NothingRendered;

/// One refusal of the explanation family, on the same terms.
fn unanswered() -> ExplanationError {
    ExplanationError::of(ExplanationIssue::UniversalUnanswered {
        question: UniversalQuestion::WhatAreYou,
    })
}

/// Every line opens with the door's prefix and the class's own sentence, and a single-cause line says nothing more.
///
/// A single-cause refusal enumerates nothing, so a line reporting "and 0 further issues, complete" would answer a question never asked of it.
#[test]
fn every_line_opens_with_the_doors_prefix_and_its_own_class() {
    let line = Line {
        class: RefusalClass::PlanNotStated,
        first: "the account names a capture nobody declared",
        body: LineBody::SingleCause,
    };
    assert_eq!(
        composed(&DOOR, &line, LineSite::WholeDeclaration),
        "lane: planning refused: the account names a capture nobody declared"
    );
}

/// A line says where the refusal sits in the role the producer counts its positions in.
///
/// The role travels with the position, so a byte offset never reads as a token ordinal and an ordinal never reads as a byte.
#[test]
fn a_line_says_where_the_refusal_sits_in_the_role_the_producer_counts_in() {
    let line = Line {
        class: RefusalClass::DeclarationNotRead,
        first: "a delimited group was never closed",
        body: LineBody::SingleCause,
    };
    let byte = composed(
        &DOOR,
        &line,
        LineSite::At(SiteCoordinate::Resolved(SourceCoordinate {
            role: CoordinateRole::Byte,
            position: 7,
        })),
    );
    assert!(byte.ends_with(" (at byte 7)"));

    let ordinal = composed(
        &DOOR,
        &line,
        LineSite::At(SiteCoordinate::Resolved(SourceCoordinate {
            role: CoordinateRole::SemanticOrigin,
            position: 7,
        })),
    );
    assert!(ordinal.ends_with(" (at semantic-origin position 7)"));
}

/// A body line counts the issues it is a summary of, and says separately whether the body kept them all.
///
/// The remainder is not lost: every issue has its own identity in the related set, and the typed body is the value the caller of the refusing step holds.
#[test]
fn a_body_line_counts_the_issues_it_is_a_summary_of() {
    let complete = Line {
        class: RefusalClass::RenderingNotClosed,
        first: "the plan declares a member at sole and nothing rendered one",
        body: LineBody::Body {
            further: 2,
            capping: Capping::Complete,
        },
    };
    assert!(
        composed(&DOOR, &complete, LineSite::WholeDeclaration)
            .ends_with("(and 2 further established issues)")
    );

    let capped = Line {
        class: RefusalClass::RenderingNotClosed,
        first: "the plan declares a member at sole and nothing rendered one",
        body: LineBody::Body {
            further: 2,
            capping: Capping::Truncated { omitted: 5 },
        },
    };
    let shown = composed(&DOOR, &capped, LineSite::WholeDeclaration);
    assert!(shown.contains("(and 2 further established issues)"));
    assert!(shown.contains("5 of them do not fit the declared issue bound"));
}

/// A refusal about the declaration as a whole adds no position to its line, and its site names no token and no coordinate.
///
/// A whole-declaration refusal is a STATED posture rather than a site somebody forgot to supply, and a position inside the declaration would send a reader to an arbitrary spot the refusal is not about.
/// The machine seat says the same thing as the prose: no manufactured handle-zero, no resolved position-zero — a reader that asks WHERE is told there is no where narrower than the whole.
#[test]
fn a_whole_declaration_refusal_adds_no_position() {
    let refused = Diagnostic::refused(&NOTHING_RENDERED, &DOOR, &Placement::WholeDeclaration);
    assert!(!refused.summary().contains("(at "));
    assert_eq!(refused.phase(), Phase::Rendering);
    assert_eq!(refused.site().token(), None);
    assert_eq!(refused.site().coordinate(), None);
}

/// A refusal at one token names where the producer's own table put it.
///
/// The handle is the load-bearing half — the producer resolves it to the exact compiler span — and the coordinate beside it is whatever that producer's table answered.
#[test]
fn a_refusal_at_one_token_names_where_the_producer_put_it() -> Result<(), ()> {
    let read = TextCapture::read("a b c").map_err(|_| ())?;
    let refused = Diagnostic::refused(
        &NOTHING_RENDERED,
        &DOOR,
        &Placement::AtToken {
            token: SpanHandle::at(1),
            spans: read.spans(),
        },
    );
    assert_eq!(refused.site().token(), Some(SpanHandle::at(1)));
    assert_eq!(
        refused.site().coordinate(),
        Some(SiteCoordinate::Resolved(SourceCoordinate {
            role: CoordinateRole::Byte,
            position: 2,
        }))
    );
    assert!(refused.summary().ends_with(" (at byte 2)"));
    Ok(())
}

/// A handle the producer's table does not reach says so, rather than being filled with a stand-in.
///
/// A coordinate written where a table did not reach would read exactly like a coordinate the table resolved, and the reader has no third value to compare it against.
#[test]
fn a_handle_the_producers_table_does_not_reach_says_so() -> Result<(), ()> {
    let read = TextCapture::read("a b c").map_err(|_| ())?;
    let refused = Diagnostic::refused(
        &NOTHING_RENDERED,
        &DOOR,
        &Placement::AtToken {
            token: SpanHandle::at(9),
            spans: read.spans(),
        },
    );
    assert!(matches!(
        refused.site().coordinate(),
        Some(SiteCoordinate::NotReached(_))
    ));
    assert!(refused.summary().contains("does not reach handle 9"));
    Ok(())
}

/// Two families observing one classification are still two refusals.
///
/// A rendering that produced nothing and an explanation that left a question unanswered both observe an absent seat — and they are different absences, of different things, repaired differently.
/// The phase and the line say so; the related sets do NOT, and that is the point of the last two asserts: each refusal is a single cause, a single cause enumerates nothing, and neither primary rides its own related set dressed as an enumeration.
#[test]
fn two_families_observing_one_classification_are_still_two_refusals() {
    let rendering = Diagnostic::refused(&NOTHING_RENDERED, &DOOR, &Placement::WholeDeclaration);
    let explanation = Diagnostic::refused(&unanswered(), &DOOR, &Placement::WholeDeclaration);
    assert_eq!(rendering.observed(), Observed::SeatAbsent);
    assert_eq!(explanation.observed(), Observed::SeatAbsent);
    assert_ne!(rendering.phase(), explanation.phase());
    assert_ne!(rendering.summary(), explanation.summary());
    assert!(rendering.related().carried().is_empty());
    assert!(explanation.related().carried().is_empty());
}

/// A related set capped at its declared bound is written into the line.
///
/// The typed capping beside the set is not something a compiler shows, so a reader handed only the body's identity would otherwise take the coarser commitment for the full one.
#[test]
fn a_capped_related_set_is_written_into_the_line() {
    let refused = Diagnostic::refused(&Overflowing, &DOOR, &Placement::WholeDeclaration);
    assert_eq!(
        refused.related().capping(),
        Capping::Truncated {
            omitted: RELATED_ISSUE_LIMIT,
        }
    );
    assert!(refused.summary().contains(&format!(
        "(and {RELATED_ISSUE_LIMIT} further established issues)"
    )));
    assert!(
        refused
            .summary()
            .contains("the related set was capped at the declared issue bound")
    );
    assert_eq!(refused.related().carried().len(), 1);
}

/// Every diagnostic carries the door's own grammar and the road that reaches the observation again.
///
/// The reproduction route is what makes the callable road a road rather than a promise: this whole lane reaches it without a proc macro anywhere in the path.
#[test]
fn every_diagnostic_carries_the_doors_grammar_and_its_reproduction_route() {
    let refused = Diagnostic::refused(&NOTHING_RENDERED, &DOOR, &Placement::WholeDeclaration);
    assert_eq!(refused.expected(), DOOR.grammar());
    assert_eq!(refused.route().entry(), DOOR.entry());
    assert!(refused.repairs().is_empty());
}
