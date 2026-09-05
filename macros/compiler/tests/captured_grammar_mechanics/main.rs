//! Generic captured-token grammar mechanics observed from outside their owner.
//!
//! The specimens repeat only the mechanical shapes found across four historical subject parsers.
//! Their nouns and semantic laws stay in this external lane, while cursor movement, punctuation adjacency, groups, exact spans, and bounded trailing-separated rows come from the compiler.

use core::{cell::Cell, convert::Infallible};
use macroonz_compiler::{
    CaptureBuildRefusal, CaptureBuilder, CaptureCursor, CaptureExpectation, CaptureReadIssue,
    CaptureReadRefusal, CapturedAtom, CapturedDelimiter, CapturedInput, CapturedSpacing,
    SpanHandle, TextCapture, TextLexicalCause, TextReadCause, TextReadRefusal,
};

#[derive(Debug)]
enum TestError {
    Text,
    Grammar,
}

impl From<TextReadRefusal> for TestError {
    fn from(_refusal: TextReadRefusal) -> Self {
        Self::Text
    }
}

impl From<CaptureReadRefusal> for TestError {
    fn from(_refusal: CaptureReadRefusal) -> Self {
        Self::Grammar
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ShapeMember {
    name: String,
    held_as: String,
    shape: String,
    cardinality: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EvolutionEdge {
    from: String,
    to: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GuardedTransition {
    from: String,
    event: String,
    to: String,
    seat: String,
    added: String,
}

fn parse_shape(capture: &CapturedInput) -> Result<(String, Vec<ShapeMember>), CaptureReadRefusal> {
    let mut root = capture.cursor();
    root.word("record")?;
    let (_, record) = root.identifier()?;
    let members = root
        .group(CapturedDelimiter::Brace)?
        .trailing_separated::<ShapeMember, 8>(',', |row| {
            let (_, name) = row.identifier()?;
            row.punctuation(':', CapturedSpacing::Alone)?;
            let (_, held_as) = row.identifier()?;
            row.fat_arrow()?;
            let (_, shape) = row.identifier()?;
            let (_, cardinality) = row.identifier()?;
            Ok(ShapeMember {
                name: name.to_owned(),
                held_as: held_as.to_owned(),
                shape: shape.to_owned(),
                cardinality: cardinality.to_owned(),
            })
        })?;
    root.finish()?;
    Ok((record.to_owned(), members.as_slice().to_vec()))
}

fn parse_surface(capture: &CapturedInput) -> Result<(Vec<String>, String), CaptureReadRefusal> {
    let mut root = capture.cursor();
    root.word("contract")?;
    root.punctuation('=', CapturedSpacing::Alone)?;
    let (_, first) = root.identifier()?;
    root.punctuation(':', CapturedSpacing::Joint)?;
    root.punctuation(':', CapturedSpacing::Alone)?;
    let (_, second) = root.identifier()?;
    root.punctuation(',', CapturedSpacing::Alone)?;
    root.word("member")?;
    root.punctuation('=', CapturedSpacing::Alone)?;
    let (_, member) = root.identifier()?;
    root.finish()?;
    Ok((vec![first.to_owned(), second.to_owned()], member.to_owned()))
}

fn parse_evolution(capture: &CapturedInput) -> Result<Vec<EvolutionEdge>, CaptureReadRefusal> {
    let mut root = capture.cursor();
    root.word("edges")?;
    let edges = root
        .group(CapturedDelimiter::Brace)?
        .trailing_separated::<EvolutionEdge, 16>(',', |row| {
            let (_, from) = row.identifier()?;
            row.thin_arrow()?;
            let (_, to) = row.identifier()?;
            Ok(EvolutionEdge {
                from: from.to_owned(),
                to: to.to_owned(),
            })
        })?;
    root.finish()?;
    Ok(edges.as_slice().to_vec())
}

fn parse_guarded(capture: &CapturedInput) -> Result<Vec<GuardedTransition>, CaptureReadRefusal> {
    let mut root = capture.cursor();
    root.word("effects")?;
    let effects = root
        .group(CapturedDelimiter::Brace)?
        .trailing_separated::<GuardedTransition, 16>(',', |row| {
            let (_, from) = row.identifier()?;
            row.punctuation('+', CapturedSpacing::Alone)?;
            let (_, event) = row.identifier()?;
            row.thin_arrow()?;
            let (_, to) = row.identifier()?;
            row.punctuation('=', CapturedSpacing::Alone)?;
            let (_, seat) = row.identifier()?;
            root_payload(row).map(|added| GuardedTransition {
                from: from.to_owned(),
                event: event.to_owned(),
                to: to.to_owned(),
                seat: seat.to_owned(),
                added,
            })
        })?;
    root.finish()?;
    Ok(effects.as_slice().to_vec())
}

fn root_payload(cursor: &mut CaptureCursor<'_>) -> Result<String, CaptureReadRefusal> {
    cursor.word("add")?;
    let mut arguments = cursor.group(CapturedDelimiter::Parenthesis)?;
    let (_, amount) = arguments.number()?;
    arguments.finish()?;
    Ok(amount.to_owned())
}

fn parse_arrow_row(capture: &CapturedInput) -> Result<(String, String), CaptureReadRefusal> {
    let mut cursor = capture.cursor();
    let (_, from) = cursor.identifier()?;
    cursor.thin_arrow()?;
    let (_, to) = cursor.identifier()?;
    cursor.punctuation(',', CapturedSpacing::Alone)?;
    cursor.finish()?;
    Ok((from.to_owned(), to.to_owned()))
}

fn manual_arrow_capture() -> Result<CapturedInput, CaptureBuildRefusal<u64, Infallible>> {
    let mut builder = CaptureBuilder::declared();
    let level = builder.open();
    let level = level.atom(20, |_| {
        Ok(CapturedAtom::RawIdentifier(String::from("type")))
    })?;
    let level = level.atom(21, |_| Ok(CapturedAtom::JointPunct('-')))?;
    let level = level.atom(22, |_| Ok(CapturedAtom::Punct('>')))?;
    let level = level.atom(23, |_| Ok(CapturedAtom::Word(String::from("Target"))))?;
    let level = level.atom(24, |_| Ok(CapturedAtom::Punct(',')))?;
    Ok(level.finish())
}

fn bare_group_capture() -> Result<CapturedInput, CaptureBuildRefusal<u64, Infallible>> {
    let mut builder = CaptureBuilder::declared();
    let level = builder.open();
    let level = level.group(50, CapturedDelimiter::Bare, |_span, inner| {
        inner.atom(51, |_| Ok(CapturedAtom::Word(String::from("Held"))))
    })?;
    Ok(level.finish())
}

#[test]
fn four_historical_parser_shapes_share_one_mechanical_reader() -> Result<(), TestError> {
    let shape = TextCapture::read(
        "record Packet { count: u64 => count required, label: String => text optional, }",
    )?;
    assert_eq!(
        parse_shape(shape.input())?,
        (
            String::from("Packet"),
            vec![
                ShapeMember {
                    name: String::from("count"),
                    held_as: String::from("u64"),
                    shape: String::from("count"),
                    cardinality: String::from("required"),
                },
                ShapeMember {
                    name: String::from("label"),
                    held_as: String::from("String"),
                    shape: String::from("text"),
                    cardinality: String::from("optional"),
                },
            ],
        )
    );

    let surface = TextCapture::read("contract = crate::Contract, member = VALUE")?;
    assert_eq!(
        parse_surface(surface.input())?,
        (
            vec![String::from("crate"), String::from("Contract")],
            String::from("VALUE"),
        )
    );

    let evolution = TextCapture::read("edges { V1 -> V2, V2 -> V3, }")?;
    assert_eq!(
        parse_evolution(evolution.input())?,
        vec![
            EvolutionEdge {
                from: String::from("V1"),
                to: String::from("V2"),
            },
            EvolutionEdge {
                from: String::from("V2"),
                to: String::from("V3"),
            },
        ]
    );

    let guarded = TextCapture::read("effects { Idle + Arm -> Armed = ArmEffect add(3), }")?;
    assert_eq!(
        parse_guarded(guarded.input())?,
        vec![GuardedTransition {
            from: String::from("Idle"),
            event: String::from("Arm"),
            to: String::from("Armed"),
            seat: String::from("ArmEffect"),
            added: String::from("3"),
        }]
    );
    Ok(())
}

#[test]
fn text_and_independent_builder_inputs_share_parsing_and_identity() -> Result<(), ()> {
    let from_text = TextCapture::read("r#type -> Target,").map_err(|_| ())?;
    let from_builder = manual_arrow_capture().map_err(|_| ())?;
    let text_bytes = from_text.input().canonical_bytes();
    let builder_bytes = from_builder.canonical_bytes();
    assert_eq!(text_bytes, builder_bytes);
    assert_eq!(
        parse_arrow_row(from_text.input()).map_err(|_| ())?,
        parse_arrow_row(&from_builder).map_err(|_| ())?
    );
    assert_eq!(from_text.input().canonical_bytes(), text_bytes);
    assert_eq!(from_builder.canonical_bytes(), builder_bytes);
    Ok(())
}

#[test]
fn raw_identifiers_lifetimes_and_nested_groups_remain_visible() -> Result<(), TestError> {
    let source = TextCapture::read("r#type 'a (Inner)")?;
    let mut root = source.input().cursor();
    let (raw, spelling) = root.identifier()?;
    assert_eq!(raw.raw_identifier(), Some("type"));
    assert_eq!(spelling, "type");
    root.punctuation('\'', CapturedSpacing::Joint)?;
    let (lifetime, lifetime_name) = root.identifier()?;
    assert_eq!(lifetime.word(), Some("a"));
    assert_eq!(lifetime_name, "a");
    let mut nested = root.group(CapturedDelimiter::Parenthesis)?;
    let (_, inner) = nested.identifier()?;
    assert_eq!(inner, "Inner");
    nested.finish()?;
    root.finish()?;
    Ok(())
}

/// Raw lifetimes retain their form and apply the raw-identifier exclusion roster.
#[test]
fn raw_lifetimes_keep_their_distinct_name_law() -> Result<(), TestError> {
    let raw = TextCapture::read("'r#kind")?;
    let mut raw_cursor = raw.input().cursor();
    raw_cursor.punctuation('\'', CapturedSpacing::Joint)?;
    let (raw_name, raw_spelling) = raw_cursor.identifier()?;
    assert_eq!(raw_name.raw_identifier(), Some("kind"));
    assert_eq!(raw_spelling, "kind");
    raw_cursor.finish()?;

    assert_eq!(
        TextCapture::read("'r#self"),
        Err(TextReadRefusal {
            cause: TextReadCause::Lexical(TextLexicalCause::InvalidIdentifier),
            at: 0,
        })
    );
    Ok(())
}

#[test]
fn an_invisible_group_remains_a_real_group_boundary() -> Result<(), ()> {
    let capture = bare_group_capture().map_err(|_| ())?;
    let mut root = capture.cursor();
    let mut bare = root.group(CapturedDelimiter::Bare).map_err(|_| ())?;
    let (_, held) = bare.identifier().map_err(|_| ())?;
    assert_eq!(held, "Held");
    bare.finish().map_err(|_| ())?;
    root.finish().map_err(|_| ())
}

#[test]
fn an_uninterpreted_token_can_cross_without_losing_its_span_or_payload() -> Result<(), ()> {
    let capture = TextCapture::read("@").map_err(|_| ())?;
    let mut cursor = capture.input().cursor();
    let token = cursor.token().map_err(|_| ())?;
    assert_eq!(token.punct(), Some('@'));
    assert_eq!(token.span(), SpanHandle::at(0));
    cursor.finish().map_err(|_| ())
}

#[test]
fn missing_unexpected_and_remaining_tokens_name_exact_available_spans() -> Result<(), ()> {
    let empty = TextCapture::read("").map_err(|_| ())?;
    let missing = empty.input().cursor().word("record").err().ok_or(())?;
    assert_eq!(
        missing.issue(),
        &CaptureReadIssue::Missing(CaptureExpectation::Word(String::from("record")))
    );
    assert_eq!(missing.token(), None);

    let wrong = TextCapture::read("wrong").map_err(|_| ())?;
    let unexpected = wrong.input().cursor().word("record").err().ok_or(())?;
    assert_eq!(
        unexpected.issue(),
        &CaptureReadIssue::Unexpected(CaptureExpectation::Word(String::from("record")))
    );
    assert_eq!(unexpected.token(), Some(SpanHandle::at(0)));

    let trailing = TextCapture::read("first second").map_err(|_| ())?;
    let mut cursor = trailing.input().cursor();
    cursor.word("first").map_err(|_| ())?;
    let remaining = cursor.finish().err().ok_or(())?;
    assert_eq!(remaining.issue(), &CaptureReadIssue::InputRemaining);
    assert_eq!(remaining.token(), Some(SpanHandle::at(1)));
    Ok(())
}

#[test]
fn punctuation_group_and_number_disagreements_refuse_at_the_token() -> Result<(), ()> {
    let punctuation = TextCapture::read("=>").map_err(|_| ())?;
    let mut cursor = punctuation.input().cursor();
    let spacing = cursor
        .punctuation('=', CapturedSpacing::Alone)
        .err()
        .ok_or(())?;
    assert_eq!(
        spacing.issue(),
        &CaptureReadIssue::Unexpected(CaptureExpectation::Punctuation {
            mark: '=',
            spacing: CapturedSpacing::Alone,
        })
    );
    assert_eq!(spacing.token(), Some(SpanHandle::at(0)));

    let parenthesized = TextCapture::read("(A)").map_err(|_| ())?;
    let group = parenthesized
        .input()
        .cursor()
        .group(CapturedDelimiter::Brace)
        .err()
        .ok_or(())?;
    assert_eq!(group.token(), Some(SpanHandle::at(0)));

    let word = TextCapture::read("A").map_err(|_| ())?;
    let number = word.input().cursor().number().err().ok_or(())?;
    assert_eq!(number.token(), Some(SpanHandle::at(0)));

    let raw = TextCapture::read("r#type").map_err(|_| ())?;
    let ordinary_word = raw.input().cursor().word("type").err().ok_or(())?;
    assert_eq!(ordinary_word.token(), Some(SpanHandle::at(0)));
    Ok(())
}

#[test]
fn mechanical_expectations_and_refusals_render_exactly() -> Result<(), ()> {
    let expectations = [
        (
            CaptureExpectation::Token,
            String::from("one captured token"),
        ),
        (
            CaptureExpectation::Word(String::from("record")),
            String::from("the ordinary word `record`"),
        ),
        (
            CaptureExpectation::Identifier,
            String::from("one ordinary or raw identifier"),
        ),
        (
            CaptureExpectation::Number,
            String::from("one numeric literal"),
        ),
        (
            CaptureExpectation::Punctuation {
                mark: '-',
                spacing: CapturedSpacing::Joint,
            },
            String::from("the punctuation `-` joined to what follows"),
        ),
        (
            CaptureExpectation::Punctuation {
                mark: ',',
                spacing: CapturedSpacing::Alone,
            },
            String::from("the punctuation `,` standing alone"),
        ),
        (
            CaptureExpectation::Group(CapturedDelimiter::Parenthesis),
            String::from("one parenthesized token group"),
        ),
        (
            CaptureExpectation::Group(CapturedDelimiter::Brace),
            String::from("one braced token group"),
        ),
        (
            CaptureExpectation::Group(CapturedDelimiter::Bracket),
            String::from("one bracketed token group"),
        ),
        (
            CaptureExpectation::Group(CapturedDelimiter::Bare),
            String::from("one invisibly grouped token group"),
        ),
    ];
    for (expectation, rendered) in expectations {
        assert_eq!(expectation.to_string(), rendered);
    }

    let issues = [
        (
            CaptureReadIssue::Missing(CaptureExpectation::Token),
            String::from("the captured sequence ended before one captured token"),
        ),
        (
            CaptureReadIssue::Unexpected(CaptureExpectation::Identifier),
            String::from("the next captured token is not one ordinary or raw identifier"),
        ),
        (
            CaptureReadIssue::InputRemaining,
            String::from("the captured sequence carries an unconsumed token"),
        ),
        (
            CaptureReadIssue::SequenceUnbounded { limit: 4 },
            String::from(
                "the captured sequence carries more members than its declared magnitude of 4",
            ),
        ),
        (
            CaptureReadIssue::SequenceMemberDidNotAdvance,
            String::from("the separated-sequence member reader returned without consuming a token"),
        ),
    ];
    for (issue, rendered) in issues {
        assert_eq!(issue.to_string(), rendered);
    }

    let empty = TextCapture::read("").map_err(|_| ())?;
    let boundary = empty.input().cursor().token().err().ok_or(())?;
    assert_eq!(
        boundary.to_string(),
        "the captured sequence ended before one captured token at the declaration boundary"
    );

    let wrong = TextCapture::read("wrong").map_err(|_| ())?;
    let token = wrong.input().cursor().word("record").err().ok_or(())?;
    assert_eq!(
        token.to_string(),
        "the next captured token is not the ordinary word `record` at captured span 0"
    );
    Ok(())
}

#[test]
fn group_end_is_the_truthful_site_for_a_missing_inner_token() -> Result<(), ()> {
    let capture = TextCapture::read("{}").map_err(|_| ())?;
    let mut root = capture.input().cursor();
    let mut group = root.group(CapturedDelimiter::Brace).map_err(|_| ())?;
    let missing = group.identifier().err().ok_or(())?;
    assert_eq!(
        missing.issue(),
        &CaptureReadIssue::Missing(CaptureExpectation::Identifier)
    );
    assert_eq!(missing.token(), Some(SpanHandle::at(0)));
    Ok(())
}

#[test]
fn trailing_separated_rows_refuse_missing_progress_separator_and_capacity() -> Result<(), ()> {
    let nonadvancing = TextCapture::read("{ A, }").map_err(|_| ())?;
    let mut nonadvancing_root = nonadvancing.input().cursor();
    let nonadvancing_group = nonadvancing_root
        .group(CapturedDelimiter::Brace)
        .map_err(|_| ())?;
    let nonadvancing_refusal = nonadvancing_group
        .trailing_separated::<(), 2>(',', |_row| Ok(()))
        .err()
        .ok_or(())?;
    assert_eq!(
        nonadvancing_refusal.issue(),
        &CaptureReadIssue::SequenceMemberDidNotAdvance
    );
    assert_eq!(nonadvancing_refusal.token(), Some(SpanHandle::at(1)));

    let missing_separator = TextCapture::read("{ A }").map_err(|_| ())?;
    let mut missing_separator_root = missing_separator.input().cursor();
    let missing_separator_group = missing_separator_root
        .group(CapturedDelimiter::Brace)
        .map_err(|_| ())?;
    let missing_separator_refusal = missing_separator_group
        .trailing_separated::<String, 2>(',', |row| {
            let (_, name) = row.identifier()?;
            Ok(name.to_owned())
        })
        .err()
        .ok_or(())?;
    assert_eq!(
        missing_separator_refusal.issue(),
        &CaptureReadIssue::Missing(CaptureExpectation::Punctuation {
            mark: ',',
            spacing: CapturedSpacing::Alone,
        })
    );
    assert_eq!(missing_separator_refusal.token(), Some(SpanHandle::at(0)));

    let overbound = TextCapture::read("{ A, B, }").map_err(|_| ())?;
    let mut overbound_root = overbound.input().cursor();
    let overbound_group = overbound_root
        .group(CapturedDelimiter::Brace)
        .map_err(|_| ())?;
    let reads = Cell::new(0_usize);
    let overbound_refusal = overbound_group
        .trailing_separated::<String, 1>(',', |row| {
            reads.set(reads.get().saturating_add(1));
            let (_, name) = row.identifier()?;
            Ok(name.to_owned())
        })
        .err()
        .ok_or(())?;
    assert_eq!(
        overbound_refusal.issue(),
        &CaptureReadIssue::SequenceUnbounded { limit: 1 }
    );
    assert_eq!(overbound_refusal.token(), Some(SpanHandle::at(3)));
    assert_eq!(reads.get(), 1);
    Ok(())
}
