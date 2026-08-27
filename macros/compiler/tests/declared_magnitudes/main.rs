//! The captured-input magnitudes, read from the constants that own them and driven from outside in both directions.
//!
//! Every producer of captured input walks under one nesting depth, one per-level token magnitude, one whole-tree token magnitude, and one capture-work budget.
//! Passing any of them refuses naming that bound before a partial tree exists, and the four causes stay distinct because repairing one tells a caller nothing about another.
//!
//! Each boundary input is derived from the constant that owns the bound, so this lane observes the road immediately below, at, and above the owner's own value without authoring a parallel policy number.
//!
//! # Producers
//!
//! The depth, per-level, and whole-tree magnitudes bite on the callable text route, so they are driven through it with no proc macro anywhere in the path.
//! The per-level magnitude is driven a second way at the checked capture builder, which is the seam a producer holding its own source positions meets it at, and the two roads reach one named bound.
//! The work budget is driven through the builder's discarded-observation seat, which is what a producer that skips trivia or backtracks uses without minting a token.
//!
//! # The planted coordinate
//!
//! The reversal is the coordinate the route replaces: a depth and an index, which name two tokens with one value.
//! This lane implements that coordinate itself and shows two distinct tokens colliding under it while their routes differ.

use core::convert::Infallible;
use macroonz_compiler::{
    CAPTURE_WORK_LIMIT, CAPTURED_TOKEN_LIMIT, CAPTURED_TREE_TOKEN_LIMIT, CaptureBound,
    CaptureBuildRefusal, CaptureBuilder, CapturedAtom, CapturedDelimiter, CapturedInput,
    CapturedTokenTree, CoordinateRole, TEXT_SOURCE_BYTE_LIMIT, TOKEN_PATH_DEPTH_LIMIT, TextCapture,
    TextReadCause, TextReadRefusal, TokenPath,
};

/// An ordinary declaration, small enough that no magnitude is anywhere near it.
///
/// The control that says the road admits ordinary material at all; every bound below carries its own near-magnitude control beside it.
const DECLARATION: &str = "fn greet(who: Greeting) -> Line { Line::over(who, \"hello\") }";

/// A text nesting one token inside `groups` parenthesized levels.
fn nested(groups: usize) -> String {
    format!("{}x{}", "(".repeat(groups), ")".repeat(groups))
}

/// A text of `groups` sibling groups, each carrying `per_level` word tokens.
///
/// The whole tree carries `groups * (per_level + 1)` tokens and no level carries more than the larger of the two counts, which is what lets a caller reach the whole-tree magnitude without going near the per-level one.
fn wide(groups: usize, per_level: usize) -> String {
    let mut text = String::new();
    for _ in 0..groups {
        text.push('(');
        for _ in 0..per_level {
            text.push_str("a ");
        }
        text.push_str(") ");
    }
    text
}

/// A text of `tokens` word tokens, all at the top level.
fn flat(tokens: usize) -> String {
    "a ".repeat(tokens)
}

/// Every route in one captured input, from the root inward, in reading order.
fn routes(input: &CapturedInput) -> Vec<Vec<u32>> {
    let mut found = Vec::new();
    for tree in input.trees() {
        collect_routes(tree, &mut found);
    }
    found
}

/// Every route at and below one captured token.
fn collect_routes(tree: &CapturedTokenTree, into: &mut Vec<Vec<u32>>) {
    into.push(tree.path().steps().to_vec());
    if let Some((_, inner)) = tree.group() {
        for held in inner {
            collect_routes(held, into);
        }
    }
}

/// The exact root, group, and nested-token shape authored by the builder specimen.
fn builder_specimen(
    input: &CapturedInput,
) -> Option<(&CapturedTokenTree, &CapturedTokenTree, &CapturedTokenTree)> {
    let [root, group] = input.trees() else {
        return None;
    };
    let Some((CapturedDelimiter::Bracket, [nested])) = group.group() else {
        return None;
    };
    Some((root, group, nested))
}

/// The planted coordinate: how deep a token sits, and its position inside its own group.
///
/// It is implemented here, by the lane, because the seam carries no such coordinate — and a law about a coordinate is only evidence if the coordinate can be built and seen to collide.
fn depth_and_index(route: &TokenPath) -> (usize, u32) {
    (route.depth(), route.steps().last().copied().unwrap_or(0))
}

/// One token per group, for the tokens the collision is exhibited over.
fn first_of_each_group(input: &CapturedInput) -> Vec<&CapturedTokenTree> {
    input
        .trees()
        .iter()
        .filter_map(|tree| tree.group())
        .filter_map(|(_, inner)| inner.first())
        .collect()
}

/// A route locates exactly one token, and two tokens never share one.
///
/// The control half: over a tree carrying siblings at several levels, every route the capture issued is distinct.
#[test]
fn every_route_in_one_capture_names_a_different_token() {
    let read = TextCapture::read("a (b c) (d (e))").map_err(|_| ());
    assert!(read.is_ok_and(|read| {
        let found = routes(read.input());
        let counted = found.len();
        let mut sorted = found;
        sorted.sort_unstable();
        sorted.dedup();
        counted > 0 && sorted.len() == counted
    }));
}

/// One builder operation owns the retained positions, paths, handles, and denominator together.
#[test]
fn the_builder_issues_every_capture_coordinate_from_one_walk()
-> Result<(), CaptureBuildRefusal<u64, Infallible>> {
    let mut builder = CaptureBuilder::declared();
    let level = builder.open();
    let level = level.atom(11u64, |_| {
        Ok::<_, Infallible>(CapturedAtom::Word(String::from("root")))
    })?;
    let level = level.group(22u64, CapturedDelimiter::Bracket, |_group, inner| {
        inner.atom(33u64, |_| {
            Ok::<_, Infallible>(CapturedAtom::Word(String::from("nested")))
        })
    })?;
    let input = level.finish();
    let specimen = builder_specimen(&input);
    assert!(specimen.is_some());

    assert_eq!(builder.positions(), &[11u64, 22u64, 33u64]);
    assert_eq!(input.issued(), 3);
    if let Some((root, group, nested)) = specimen {
        assert_eq!(root.span().index(), 0);
        assert_eq!(group.span().index(), 1);
        assert_eq!(nested.span().index(), 2);
        assert_eq!(root.path().steps(), &[0]);
        assert_eq!(group.path().steps(), &[1]);
        assert_eq!(nested.path().steps(), &[1, 0]);
    }
    Ok(())
}

/// A refused capture retains its failing position for diagnostics, then a fresh capture rolls back only the refused attempt.
#[test]
fn a_fresh_capture_after_refusal_preserves_prior_handles_without_ghost_positions()
-> Result<(), CaptureBuildRefusal<u64, &'static str>> {
    let mut builder = CaptureBuilder::declared();
    let first = builder
        .open()
        .atom(10u64, |_| Ok(CapturedAtom::Word(String::from("first"))))?
        .finish();
    assert_eq!(first.issued(), 1);

    assert!(matches!(
        builder.open().atom(20u64, |_| Err("unread")),
        Err(CaptureBuildRefusal::ProducerRefused { cause: "unread", path, at })
            if path.steps() == [0] && at.index() == 1
    ));
    assert_eq!(builder.positions(), &[10u64, 20u64]);

    let fresh = builder
        .open()
        .atom(30u64, |_| Ok(CapturedAtom::Word(String::from("fresh"))))?
        .finish();
    assert_eq!(builder.positions(), &[10u64, 30u64]);
    assert_eq!(fresh.issued(), 2);
    assert_eq!(
        fresh.trees().first().map(CapturedTokenTree::span),
        Some(macroonz_compiler::SpanHandle::at(1))
    );
    Ok(())
}

/// The planted coordinate names two different tokens with one value.
///
/// The first token of one group and the first token of its sibling sit at the same depth and the same position inside their own groups, so a depth-and-index reading points at whichever of them the reader guesses.
/// Under the route from the root they are two.
#[test]
fn the_planted_depth_and_index_coordinate_names_two_tokens_at_once() {
    let read = TextCapture::read("(b) (c)").map_err(|_| ());
    assert!(read.is_ok_and(|read| {
        let firsts = first_of_each_group(read.input());
        let (Some(left), Some(right)) = (firsts.first(), firsts.get(1)) else {
            return false;
        };
        let collided = depth_and_index(left.path()) == depth_and_index(right.path());
        let separate = left.path() != right.path();
        firsts.len() == 2 && collided && separate
    }));
}

/// Hostile caller text reaches typed refusals without an unchecked branch.
#[test]
fn malformed_text_shapes_refuse_at_their_established_bytes() {
    let cases = [
        (
            "(",
            TextReadRefusal {
                cause: TextReadCause::NotBalanced,
                at: 0,
            },
        ),
        (
            ")",
            TextReadRefusal {
                cause: TextReadCause::NotOpened,
                at: 0,
            },
        ),
        (
            "\"open",
            TextReadRefusal {
                cause: TextReadCause::NotTerminated,
                at: 0,
            },
        ),
        (
            "\"\\q\"",
            TextReadRefusal {
                cause: TextReadCause::NotEscapeFree,
                at: 0,
            },
        ),
    ];
    for (source, expected) in cases {
        assert_eq!(TextCapture::read(source), Err(expected));
    }
}

/// A trivia-only source reaches the source-byte magnitude even though it retains no token tree.
#[test]
fn source_bytes_are_bounded_independently_of_structural_tokens() {
    let lawful = " ".repeat(TEXT_SOURCE_BYTE_LIMIT);
    assert!(TextCapture::read(&lawful).is_ok_and(|read| read.input().is_empty()));

    let hostile = " ".repeat(TEXT_SOURCE_BYTE_LIMIT.saturating_add(1));
    assert_eq!(
        TextCapture::read(&hostile),
        Err(TextReadRefusal {
            cause: TextReadCause::SourceBytesUnbounded,
            at: u64::try_from(TEXT_SOURCE_BYTE_LIMIT).unwrap_or(u64::MAX),
        })
    );
}

/// Nesting to the declared depth reads, and one level deeper refuses naming the depth.
///
/// The refusal carries the byte it sits at, in the role the text route counts in, so a caller can point at the character rather than at the declaration.
#[test]
fn nesting_to_the_declared_depth_reads_and_one_deeper_refuses() {
    // A token inside this many groups carries a route of exactly the declared length: one step per group, and the token itself is the last step.
    let deepest = TOKEN_PATH_DEPTH_LIMIT.saturating_sub(1);
    let lawful = TextCapture::read(&nested(deepest)).map_err(|_| ());
    assert!(lawful.is_ok_and(|read| {
        routes(read.input())
            .iter()
            .any(|route| route.len() == TOKEN_PATH_DEPTH_LIMIT)
    }));

    let hostile = TextCapture::read(&nested(TOKEN_PATH_DEPTH_LIMIT));
    assert!(hostile.is_err_and(|refusal| {
        refusal.cause == TextReadCause::Unbounded(CaptureBound::Depth)
            && refusal.coordinate().role == CoordinateRole::Byte
    }));
}

/// A tree past the declared whole-tree magnitude refuses naming the tree, and a tree just under it reads.
///
/// Both texts are generated rather than written out, and both spread their tokens so that no single level approaches the per-level magnitude — which is what makes the whole-tree count the bound under judgement rather than a neighbour that happened to bite first.
/// Two controls answer two different doubts: the ordinary declaration reads, so the road admits ordinary material, and the near-magnitude text reads, so the refusal below is the tree magnitude's own rather than something this road does to every long text.
#[test]
fn a_tree_past_the_declared_token_magnitude_refuses() {
    let control = TextCapture::read(DECLARATION).map_err(|_| ());
    assert!(control.is_ok_and(|read| !read.input().is_empty()));

    let per_level = CAPTURED_TOKEN_LIMIT.saturating_sub(1);
    let group_width = per_level.saturating_add(1);
    let counted = |groups: usize| groups.saturating_mul(group_width);

    let offered_groups = CAPTURED_TREE_TOKEN_LIMIT.checked_div(group_width);
    assert!(offered_groups.is_some());
    let Some(admitted_groups) = offered_groups else {
        return;
    };
    assert!(
        per_level > 0
            && admitted_groups > 0
            && admitted_groups <= CAPTURED_TOKEN_LIMIT
            && counted(admitted_groups) <= CAPTURED_TREE_TOKEN_LIMIT,
        "the owner magnitudes admit no tree-bound control below the per-level magnitude"
    );
    let admitted = TextCapture::read(&wide(admitted_groups, per_level)).map_err(|_| ());
    assert!(admitted.is_ok_and(|read| read.input().len() == admitted_groups));

    let hostile_groups = admitted_groups.saturating_add(1);
    assert!(
        hostile_groups <= CAPTURED_TOKEN_LIMIT
            && counted(hostile_groups) > CAPTURED_TREE_TOKEN_LIMIT,
        "the owner magnitudes admit no tree-bound hostile below the per-level magnitude"
    );
    let refused = TextCapture::read(&wide(hostile_groups, per_level));
    assert!(
        refused.is_err_and(|refusal| refusal.cause == TextReadCause::Unbounded(CaptureBound::Tree))
    );
}

/// The builder charges producer work that does not become a captured token.
///
/// Kept tokens reach the tree magnitude through the text road above; this is the distinct discarded-work seat for trivia or backtracking.
#[test]
fn the_builder_counts_discarded_work_and_refuses_one_past_it()
-> Result<(), CaptureBuildRefusal<usize, Infallible>> {
    let mut spent_builder = CaptureBuilder::declared();
    let mut spent = spent_builder.open();
    for position in 0..CAPTURE_WORK_LIMIT {
        spent = spent.examined::<Infallible>(position)?;
    }
    assert!(matches!(
        spent.examined::<Infallible>(CAPTURE_WORK_LIMIT),
        Err(CaptureBuildRefusal::Unbounded {
            bound: CaptureBound::Work,
            at: CAPTURE_WORK_LIMIT,
        })
    ));
    Ok(())
}

/// One nesting level carrying more trees than the declared magnitude refuses naming the level, on the callable text route.
///
/// The route's span table stands under the whole-tree magnitude, so this lane first requires one value past the per-level owner constant to remain within the tree one.
/// The refusal then names the level, which is the magnitude the input actually overran.
#[test]
fn a_level_past_the_declared_magnitude_refuses_on_the_text_route() {
    let lawful = TextCapture::read(&flat(CAPTURED_TOKEN_LIMIT)).map_err(|_| ());
    assert!(lawful.is_ok_and(|read| read.input().len() == CAPTURED_TOKEN_LIMIT));

    let hostile_count = CAPTURED_TOKEN_LIMIT.saturating_add(1);
    assert!(hostile_count <= CAPTURED_TREE_TOKEN_LIMIT);
    let hostile = TextCapture::read(&flat(hostile_count));
    assert!(
        hostile
            .is_err_and(|refusal| refusal.cause == TextReadCause::Unbounded(CaptureBound::Level))
    );
}

/// One nesting level carrying more trees than the declared magnitude refuses naming the level, at the checked builder.
///
/// The same magnitude met at the other seam, with no text and no reader anywhere in the path: two roads, one named bound.
#[test]
fn a_level_past_the_declared_magnitude_refuses_at_the_constructor()
-> Result<(), CaptureBuildRefusal<usize, Infallible>> {
    let atom = || CapturedAtom::Word(String::from("a"));
    let mut lawful_builder = CaptureBuilder::declared();
    let mut lawful_level = lawful_builder.open();
    for position in 0..CAPTURED_TOKEN_LIMIT {
        lawful_level = lawful_level.atom(position, |_| Ok::<_, Infallible>(atom()))?;
    }
    let lawful = lawful_level.finish();
    assert_eq!(lawful.len(), CAPTURED_TOKEN_LIMIT);
    assert_eq!(lawful.issued(), CAPTURED_TOKEN_LIMIT);
    assert_eq!(lawful_builder.positions().len(), CAPTURED_TOKEN_LIMIT);

    let mut hostile_builder = CaptureBuilder::declared();
    let mut hostile_level = hostile_builder.open();
    for position in 0..CAPTURED_TOKEN_LIMIT {
        hostile_level = hostile_level.atom(position, |_| Ok::<_, Infallible>(atom()))?;
    }
    assert!(matches!(
        hostile_level.atom(CAPTURED_TOKEN_LIMIT, |_| Ok::<_, Infallible>(atom())),
        Err(CaptureBuildRefusal::Unbounded {
            bound: CaptureBound::Level,
            at: CAPTURED_TOKEN_LIMIT,
        })
    ));
    Ok(())
}
