//! The captured-input bounds, read from their owners and driven from outside the services in both directions.
//!
//! # Bounds
//!
//! Every producer of captured input walks under one nesting depth, one per-level
//! token magnitude, one whole-tree token magnitude, and one capture-work budget,
//! and exceeding any of them refuses naming that bound before a partial tree
//! exists. The causes remain distinct because repairing one bound tells a caller nothing about another.
//!
//! Each boundary input is derived from the constant that owns the bound.
//! The lane observes the road immediately below, at, and above that owner value without authoring a parallel policy number.
//!
//! # Producers
//!
//! The depth, per-level, and whole-tree bounds bite on the callable text route, so they are driven through it, the same road `compile_refusal_text` takes, with no proc-macro anywhere in the path. The text route issues one span offset per token it
//! keeps into a table bounded at the WHOLE-TREE magnitude, which is what the
//! table counts, so a level that overruns its own magnitude reaches that bound
//! and names it rather than tripping a count it never approached.
//!
//! The per-level magnitude is driven a second way as well, at the capture
//! constructor: that is the seam a producer holding its own spans — a compiler
//! shell, or a future language frontend — meets it at, and the two roads reach
//! the same named bound.
//!
//! The work budget is reachable from neither producer the services carry, and
//! saying so is the honest state rather than a gap: both producers KEEP every
//! token they examine, so the whole-tree magnitude bites before the walk's
//! budget can. The budget is what bounds a producer that reads material it
//! discards, so it is driven at the seam that governs it — the walk itself.
//!
//! # The planted mutant
//!
//! The planted reversal is the coordinate the route replaces: a depth and an
//! index, which name two tokens with one value. The plane implements that
//! coordinate itself, below, and shows two distinct tokens colliding under it
//! while their routes differ.

use std::num::TryFromIntError;
use threadpak::types::ConstLimit;
use threadpak_macroc::plane::CapturedTokenLimit;
use threadpak_macroc::token::{CapturedTreeTokenLimit, TokenPathDepthLimit};
use threadpak_macroc::{
    CaptureBound, CaptureWalk, CapturedInput, CapturedPayload, CapturedTokenTree, SpanHandle,
    TextCapture, TextCompileRefusal, TextReadCause, TokenPath, compile_refusal_text,
};

/// The lawful declaration, small enough that no magnitude is anywhere near it.
/// It is the control that says the road admits an ordinary declaration at all;
/// the bounds below carry their own near-magnitude controls beside it.
const DECLARATION: &str = "#[refusal(family = \"testpak.demo\", shape = single_cause, \
    order(NotCanonical = \"not-canonical\", NotAdmitted = \"not-admitted\", \
    Unbounded = \"unbounded\"))] enum DemoFamily { NotAdmitted, Unbounded, NotCanonical, }";

/// A text nesting one token inside `groups` parenthesized levels.
fn nested(groups: usize) -> String {
    format!("{}x{}", "(".repeat(groups), ")".repeat(groups))
}

/// A text of `groups` sibling groups, each carrying `per_level` word tokens.
///
/// The whole tree carries `groups * (per_level + 1)` tokens — one for each group
/// and one for each word inside it — and no level carries more than the larger
/// of `groups` and `per_level`, which is what lets a caller reach the whole-tree
/// magnitude without going anywhere near the per-level one.
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
    into.push(tree.path().steps().copied().collect());
    if let Some((_, inner)) = tree.group() {
        for nested in inner.iter() {
            collect_routes(nested, into);
        }
    }
}

/// The planted coordinate: how deep the token sits, and its position inside its
/// own group.
///
/// It is implemented here, by the judge, because the services carry no such
/// coordinate — and a law about a coordinate is only evidence if the coordinate
/// can be built and seen to collide.
fn saturating_coordinate(route: &TokenPath) -> (usize, u32) {
    (route.depth(), route.steps().copied().last().unwrap_or(0))
}

/// A route locates exactly one token, and two tokens never share one.
///
/// The control half: over a tree carrying siblings at several levels, every
/// route the capture issued is distinct.
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
/// The first token of one group and the first token of its sibling sit at the
/// same depth and the same position inside their own groups. Under a depth and
/// an index they are one coordinate, so a diagnostic, an origin mapping, or an
/// inspection reading it points at whichever of them the reader guesses. Under
/// the route from the root they are two.
#[test]
fn the_killed_depth_and_index_coordinate_names_two_tokens_at_once() {
    let read = TextCapture::read("(b) (c)").map_err(|_| ());
    assert!(read.is_ok_and(|read| {
        let firsts: Vec<&CapturedTokenTree> = read
            .input()
            .trees()
            .filter_map(|tree| tree.group())
            .filter_map(|(_, inner)| inner.iter().next())
            .collect();
        let (Some(left), Some(right)) = (firsts.first(), firsts.get(1)) else {
            return false;
        };
        let collided = saturating_coordinate(left.path()) == saturating_coordinate(right.path());
        let separate = left.path() != right.path();
        firsts.len() == 2 && collided && separate
    }));
}

/// Nesting to the declared depth reads; one level deeper refuses, naming the
/// depth.
///
/// The refusal travels the callable road too: a declaration nested past the
/// magnitude reaches `compile_refusal_text` as a text that could not be read at
/// all, carrying the bound it overran rather than a single "unbounded" word.
#[test]
fn nesting_to_the_declared_depth_reads_and_one_deeper_refuses() {
    // A token inside this many groups carries a route of exactly the declared
    // length: one step per group, and the token itself is the last step.
    let deepest = TokenPathDepthLimit::MAX.saturating_sub(1);
    let lawful = TextCapture::read(&nested(deepest)).map_err(|_| ());
    assert!(lawful.is_ok_and(|read| {
        routes(read.input())
            .iter()
            .any(|route| route.len() == TokenPathDepthLimit::MAX)
    }));

    let hostile = TextCapture::read(&nested(TokenPathDepthLimit::MAX));
    assert!(hostile.is_err_and(|refusal| matches!(
        refusal.cause,
        TextReadCause::Unbounded(CaptureBound::DepthUnbounded)
    )));

    let compiled = compile_refusal_text(&nested(TokenPathDepthLimit::MAX))
        .map(|(_, closed)| closed.identity());
    assert!(compiled.is_err_and(|refusal| match refusal {
        TextCompileRefusal::NotReadable(read) => matches!(
            read.cause,
            TextReadCause::Unbounded(CaptureBound::DepthUnbounded)
        ),
        TextCompileRefusal::Refused(_) => false,
    }));
}

/// A tree past the declared whole-tree magnitude refuses, naming the tree, and
/// a tree just under it reads.
///
/// Both texts are generated rather than written out, and both nest their tokens
/// so that no single level approaches the per-level magnitude — which is what
/// makes the whole-tree count the bound under judgement here rather than a
/// neighbour that happened to bite first.
///
/// Two controls, and each answers a different doubt. The lawful declaration
/// reads, so the road admits an ordinary declaration at all. The near-magnitude
/// text reads, so the refusal below is the tree magnitude's own and not
/// something this road does to every long text.
#[test]
fn a_tree_past_the_declared_token_magnitude_refuses() {
    let control = TextCapture::read(DECLARATION).map_err(|_| ());
    assert!(control.is_ok_and(|read| !read.input().is_empty()));

    let per_level = CapturedTokenLimit::MAX.saturating_sub(1);
    let group_width = per_level.saturating_add(1);
    let counted = |groups: usize| groups.saturating_mul(group_width);

    let admitted_groups = CapturedTreeTokenLimit::MAX.checked_div(group_width);
    assert!(admitted_groups.is_some());
    let Some(admitted_groups) = admitted_groups else {
        return;
    };
    assert!(
        per_level > 0
            && admitted_groups > 0
            && admitted_groups <= CapturedTokenLimit::MAX
            && counted(admitted_groups) <= CapturedTreeTokenLimit::MAX,
        "the owner bounds do not admit a tree-bound control below the per-level bound"
    );
    let admitted = TextCapture::read(&wide(admitted_groups, per_level)).map_err(|_| ());
    assert!(admitted.is_ok_and(|read| read.input().len() == admitted_groups));

    let hostile_groups = admitted_groups.saturating_add(1);
    assert!(
        hostile_groups <= CapturedTokenLimit::MAX
            && counted(hostile_groups) > CapturedTreeTokenLimit::MAX,
        "the owner bounds do not admit a tree-bound hostile below the per-level bound"
    );
    let refused = TextCapture::read(&wide(hostile_groups, per_level));
    assert!(refused.is_err_and(|refusal| matches!(
        refusal.cause,
        TextReadCause::Unbounded(CaptureBound::TreeUnbounded)
    )));
}

/// The walk counts to each declared magnitude and refuses one past it.
///
/// Two counters, charged separately: what a producer KEEPS is counted against
/// the whole-tree magnitude, and what it LOOKS AT is spent against the budget.
/// Both directions for both: the declared number of steps is admitted, and the
/// step after it refuses naming its own bound.
#[test]
fn the_walk_counts_to_each_declared_magnitude_and_refuses_past_it() {
    let mut kept = CaptureWalk::declared();
    for _ in 0..CapturedTreeTokenLimit::MAX {
        assert!(kept.took().is_ok());
    }
    assert_eq!(
        Some(kept.taken()),
        u32::try_from(CapturedTreeTokenLimit::MAX).ok()
    );
    assert_eq!(kept.took(), Err(CaptureBound::TreeUnbounded));

    let mut spent = CaptureWalk::declared();
    for _ in 0..CaptureWalk::DECLARED_WORK {
        assert!(spent.examined().is_ok());
    }
    assert_eq!(spent.remaining(), 0);
    assert_eq!(spent.examined(), Err(CaptureBound::WorkUnbounded));
}

/// One nesting level carrying more trees than the declared magnitude refuses,
/// naming the level, on the callable text route.
///
/// The route's span table stands under the whole-tree bound, so this lane first requires one value past the per-level owner bound to remain within the tree owner bound.
/// The refusal then names the level, which is the bound the input actually overran.
#[test]
fn a_level_past_the_declared_magnitude_refuses_on_the_text_route() {
    let lawful = TextCapture::read(&flat(CapturedTokenLimit::MAX)).map_err(|_| ());
    assert!(lawful.is_ok_and(|read| read.input().len() == CapturedTokenLimit::MAX));

    let hostile_count = CapturedTokenLimit::MAX.saturating_add(1);
    assert!(hostile_count <= CapturedTreeTokenLimit::MAX);
    let hostile = TextCapture::read(&flat(hostile_count));
    assert!(hostile.is_err_and(|refusal| matches!(
        refusal.cause,
        TextReadCause::Unbounded(CaptureBound::LevelUnbounded)
    )));
}

/// One nesting level carrying more trees than the declared magnitude refuses,
/// naming the level, at the capture constructor.
///
/// The same bound met at the other seam: this is where a producer holding its
/// own spans — a compiler shell, or a future language frontend — reaches it,
/// with no text and no reader anywhere in the path. Two roads, one named bound.
#[test]
fn a_level_past_the_declared_magnitude_refuses_at_the_constructor() -> Result<(), TryFromIntError> {
    let tree = |position: usize| {
        Ok(CapturedTokenTree::captured(
            CapturedPayload::Word(String::from("a")),
            TokenPath::root(),
            SpanHandle::at(u32::try_from(position)?),
        ))
    };
    let admitted: Vec<CapturedTokenTree> = (0..CapturedTokenLimit::MAX)
        .map(tree)
        .collect::<Result<_, TryFromIntError>>()?;
    let admitted_count = admitted.len();
    let lawful = CapturedInput::taken(admitted, u32::try_from(admitted_count)?);
    assert!(lawful.is_ok_and(|input| input.len() == CapturedTokenLimit::MAX));

    let mut over: Vec<CapturedTokenTree> = (0..CapturedTokenLimit::MAX)
        .map(tree)
        .collect::<Result<_, TryFromIntError>>()?;
    over.push(tree(CapturedTokenLimit::MAX)?);
    let over_count = u32::try_from(over.len())?;
    assert_eq!(
        CapturedInput::taken(over, over_count).err(),
        Some(CaptureBound::LevelUnbounded)
    );
    Ok(())
}
