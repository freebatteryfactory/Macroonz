//! The four declared magnitudes a captured input stands under, driven from
//! outside the services, both directions each.
//!
//! # What is under judgement
//!
//! Every producer of captured input walks under one nesting depth, one per-level
//! token magnitude, one whole-tree token magnitude, and one capture-work budget,
//! and exceeding any of them refuses NAMING THAT BOUND before a partial tree
//! exists. Four magnitudes and four causes, because they are four different
//! facts about a declared input and repairing one of them tells a caller nothing
//! about the other three.
//!
//! Each magnitude is stated HERE, as a number this plane wrote down, and the
//! assertions are between that number and what the road does. Nothing below asks
//! the services what their bounds are — where a declared constant is read at all
//! it is read to assert that the two statements agree, which is the comparison,
//! not the evidence.
//!
//! # Which producer each bound is reached from, and why
//!
//! The depth and whole-tree magnitudes bite on the callable text route, so they
//! are driven through it — the same road `compile_refusal_text` takes, with no
//! proc-macro anywhere in the path.
//!
//! The per-level magnitude and the work budget are not reachable from either of
//! today's producers, and saying so is the honest state rather than a gap. The
//! text route issues one span offset per token into a table bounded at the
//! per-level magnitude, so a text carrying more tokens than one level admits
//! overruns the whole-tree count first; and both of today's producers KEEP every
//! token they examine, so the tree magnitude bites before the walk's budget can.
//! The budget is what bounds a producer that reads material it discards. Both
//! are therefore driven at the seam that governs them — the walk itself, and the
//! capture constructor — which is where a future producer would meet them.
//!
//! # The planted mutant
//!
//! The reversal this file owes is the coordinate that used to stand where the
//! route stands: a depth and an index, which named two tokens with one value.
//! The plane implements that coordinate itself, below, and shows two distinct
//! tokens colliding under it while their routes differ.

use threadpak::types::ConstLimit;
use threadpak_macroc::plane::{CapturedTokenLimit, CapturedTreeTokenLimit, TokenPathDepthLimit};
use threadpak_macroc::{
    CaptureBound, CaptureWalk, CapturedInput, CapturedPayload, CapturedTokenTree, SpanHandle,
    TextCapture, TextCompileRefusal, TextReadCause, TokenPath, compile_refusal_text,
};

/// The deepest a declared input may nest, stated here. A route to a token
/// carries one step per level, so a token sitting inside this many groups is the
/// deepest lawful token.
const DECLARED_NESTING_DEPTH: usize = 32;

/// The most token trees one nesting level may carry, stated here.
const DECLARED_LEVEL_TOKENS: usize = 4096;

/// The most tokens one whole captured tree may carry, stated here.
const DECLARED_TREE_TOKENS: u32 = 16_384;

/// The capture-work budget one walk may spend, stated here, in units of one
/// examined token.
const DECLARED_WORK_BUDGET: u32 = 65_536;

/// The lawful declaration, small enough that no magnitude is anywhere near it.
/// It is the control every hostile below is measured against.
const DECLARATION: &str = "#[refusal(family = \"testpak.demo\", shape = single_cause, \
    order(NotCanonical = \"not-canonical\", NotAdmitted = \"not-admitted\", \
    Unbounded = \"unbounded\"))] enum DemoFamily { NotAdmitted, Unbounded, NotCanonical, }";

/// A text nesting one token inside `groups` parenthesized levels.
fn nested(groups: usize) -> String {
    format!("{}x{}", "(".repeat(groups), ")".repeat(groups))
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

/// The coordinate that used to stand where the route stands: how deep the token
/// sits, and its position inside its own group.
///
/// This is the planted mutant. It is implemented here, by the judge, because the
/// services no longer carry it — and a law about a killed coordinate is only
/// evidence if the coordinate can be restored and seen to collide.
fn saturating_coordinate(route: &TokenPath) -> (usize, u32) {
    (route.depth(), route.steps().copied().last().unwrap_or(0))
}

/// The declared magnitudes this plane states and the ones the services declare
/// are the same numbers.
///
/// The comparison is the point, not the evidence: every hostile below drives the
/// road against the number written down HERE, and this test is what would fail
/// first if the services moved a bound without anyone restating it.
#[test]
fn the_stated_magnitudes_and_the_declared_ones_agree() {
    assert_eq!(TokenPathDepthLimit::MAX, DECLARED_NESTING_DEPTH);
    assert_eq!(CapturedTokenLimit::MAX, DECLARED_LEVEL_TOKENS);
    assert_eq!(
        u32::try_from(CapturedTreeTokenLimit::MAX).unwrap_or(u32::MAX),
        DECLARED_TREE_TOKENS
    );
    assert_eq!(CaptureWalk::DECLARED_WORK, DECLARED_WORK_BUDGET);
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

/// The killed coordinate, restored: it names two different tokens with one
/// value.
///
/// The first token of one group and the first token of its sibling sit at the
/// same depth and the same position inside their own groups. Under the pair that
/// used to stand here they are one coordinate, so a diagnostic, an origin
/// mapping, or an inspection reading it was pointing at whichever of them the
/// reader guessed. Under the route from the root they are two.
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
    let deepest = DECLARED_NESTING_DEPTH.saturating_sub(1);
    let lawful = TextCapture::read(&nested(deepest)).map_err(|_| ());
    assert!(lawful.is_ok_and(|read| {
        routes(read.input())
            .iter()
            .any(|route| route.len() == DECLARED_NESTING_DEPTH)
    }));

    let hostile = TextCapture::read(&nested(DECLARED_NESTING_DEPTH));
    assert!(hostile.is_err_and(|refusal| matches!(
        refusal.cause,
        TextReadCause::Unbounded(CaptureBound::DepthUnbounded)
    )));

    let compiled =
        compile_refusal_text(&nested(DECLARED_NESTING_DEPTH)).map(|(_, closed)| closed.identity());
    assert!(compiled.is_err_and(|refusal| match refusal {
        TextCompileRefusal::NotReadable(read) => matches!(
            read.cause,
            TextReadCause::Unbounded(CaptureBound::DepthUnbounded)
        ),
        TextCompileRefusal::Refused(_) => false,
    }));
}

/// A tree past the declared whole-tree magnitude refuses, naming the tree, and
/// the lawful declaration reads.
///
/// The hostile is generated rather than written out: it nests its tokens so that
/// no single level approaches the per-level magnitude, which is what makes the
/// whole-tree count the bound that bites. The control is the lawful declaration
/// itself — the text route's span table is bounded at the per-level magnitude,
/// so no text anywhere near the tree magnitude reads successfully, and pretending
/// one does would be a control that never held.
#[test]
fn a_tree_past_the_declared_token_magnitude_refuses() {
    let control = TextCapture::read(DECLARATION).map_err(|_| ());
    assert!(control.is_ok_and(|read| !read.input().is_empty()));

    let level = 4_000usize;
    let groups = 5usize;
    let mut hostile = String::new();
    for _ in 0..groups {
        hostile.push('(');
        for _ in 0..level {
            hostile.push_str("a ");
        }
        hostile.push_str(") ");
    }
    let counted = u32::try_from(groups.saturating_mul(level.saturating_add(1))).unwrap_or(u32::MAX);
    assert!(
        counted > DECLARED_TREE_TOKENS,
        "the hostile text does not reach the declared tree magnitude"
    );

    let refused = TextCapture::read(&hostile);
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
    for _ in 0..DECLARED_TREE_TOKENS {
        assert!(kept.took().is_ok());
    }
    assert_eq!(kept.taken(), DECLARED_TREE_TOKENS);
    assert_eq!(kept.took(), Err(CaptureBound::TreeUnbounded));

    let mut spent = CaptureWalk::declared();
    for _ in 0..DECLARED_WORK_BUDGET {
        assert!(spent.examined().is_ok());
    }
    assert_eq!(spent.remaining(), 0);
    assert_eq!(spent.examined(), Err(CaptureBound::WorkUnbounded));
}

/// One nesting level carrying more trees than the declared magnitude refuses,
/// naming the level.
///
/// Driven at the capture constructor rather than through a text, because the
/// text route's span table is bounded at this same magnitude and would overrun
/// its whole-tree count first. This is the seam a producer holding its own spans
/// — a compiler shell, or a future language frontend — meets the bound at.
#[test]
fn a_level_past_the_declared_magnitude_refuses() {
    let tree = |position: u32| {
        CapturedTokenTree::captured(
            CapturedPayload::Word(String::from("a")),
            TokenPath::root(),
            SpanHandle::at(position),
        )
    };
    let admitted: Vec<CapturedTokenTree> = (0..DECLARED_LEVEL_TOKENS)
        .map(|position| tree(u32::try_from(position).unwrap_or(u32::MAX)))
        .collect();
    let counted = admitted.len();
    let lawful = CapturedInput::taken(admitted, u32::try_from(counted).unwrap_or(u32::MAX));
    assert!(lawful.is_ok_and(|input| input.len() == DECLARED_LEVEL_TOKENS));

    let mut over: Vec<CapturedTokenTree> = (0..DECLARED_LEVEL_TOKENS)
        .map(|position| tree(u32::try_from(position).unwrap_or(u32::MAX)))
        .collect();
    over.push(tree(
        u32::try_from(DECLARED_LEVEL_TOKENS).unwrap_or(u32::MAX),
    ));
    let counted = u32::try_from(over.len()).unwrap_or(u32::MAX);
    assert_eq!(
        CapturedInput::taken(over, counted).err(),
        Some(CaptureBound::LevelUnbounded)
    );
}
