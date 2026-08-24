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
//! The per-level magnitude is driven a second way at the capture constructor, which is the seam a producer holding its own spans meets it at, and the two roads reach one named bound.
//! The work budget is reachable from neither producer this crate carries — both KEEP every token they examine, so the whole-tree magnitude bites first — and it is therefore driven at the seam that governs it, the walk itself.
//!
//! # The planted coordinate
//!
//! The reversal is the coordinate the route replaces: a depth and an index, which name two tokens with one value.
//! This lane implements that coordinate itself and shows two distinct tokens colliding under it while their routes differ.

use core::num::TryFromIntError;
use macroonz::{
    CAPTURE_WORK_LIMIT, CAPTURED_TOKEN_LIMIT, CAPTURED_TREE_TOKEN_LIMIT, CaptureBound, CaptureWalk,
    CapturedInput, CapturedPayload, CapturedTokenTree, CoordinateRole, SpanHandle,
    TOKEN_PATH_DEPTH_LIMIT, TextCapture, TextReadCause, TokenPath,
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

/// The walk counts to each declared magnitude and refuses one past it.
///
/// Two counters, charged separately: what a producer KEEPS is counted against the whole-tree magnitude, and what it LOOKS AT is spent against the budget.
/// Both directions for both, so the declared number of steps is admitted and the step after it refuses naming its own bound.
#[test]
fn the_walk_counts_to_each_declared_magnitude_and_refuses_past_it() {
    let mut kept = CaptureWalk::declared();
    for _ in 0..CAPTURED_TREE_TOKEN_LIMIT {
        assert!(kept.took().is_ok());
    }
    assert_eq!(kept.taken(), CAPTURED_TREE_TOKEN_LIMIT);
    assert_eq!(kept.took(), Err(CaptureBound::Tree));

    let mut spent = CaptureWalk::declared();
    for _ in 0..CAPTURE_WORK_LIMIT {
        assert!(spent.examined().is_ok());
    }
    assert_eq!(spent.remaining(), 0);
    assert_eq!(spent.examined(), Err(CaptureBound::Work));
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

/// One nesting level carrying more trees than the declared magnitude refuses naming the level, at the capture constructor.
///
/// The same magnitude met at the other seam, with no text and no reader anywhere in the path: two roads, one named bound.
#[test]
fn a_level_past_the_declared_magnitude_refuses_at_the_constructor() -> Result<(), TryFromIntError> {
    let tree = |position: usize| {
        Ok(CapturedTokenTree::captured(
            CapturedPayload::Word(String::from("a")),
            TokenPath::root(),
            SpanHandle::at(u32::try_from(position)?),
        ))
    };
    let admitted: Vec<CapturedTokenTree> = (0..CAPTURED_TOKEN_LIMIT)
        .map(tree)
        .collect::<Result<_, TryFromIntError>>()?;
    let admitted_count = u32::try_from(admitted.len())?;
    let lawful = CapturedInput::taken(admitted, admitted_count);
    assert!(lawful.is_ok_and(|input| input.len() == CAPTURED_TOKEN_LIMIT));

    let mut over: Vec<CapturedTokenTree> = (0..CAPTURED_TOKEN_LIMIT)
        .map(tree)
        .collect::<Result<_, TryFromIntError>>()?;
    over.push(tree(CAPTURED_TOKEN_LIMIT)?);
    let over_count = u32::try_from(over.len())?;
    assert_eq!(
        CapturedInput::taken(over, over_count).err(),
        Some(CaptureBound::Level)
    );
    Ok(())
}
