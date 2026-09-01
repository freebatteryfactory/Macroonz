//! Completing a captured mutation declaration from the enum it sits on.
//!
//! The operation reads only variant names and their authored order, then states the unchanged order and one adjacent transposition per neighboring pair.

use super::{
    ALTERNATIVE_LIMIT, Alternative, DECLARED_ORDER_FAMILY, Declaration, FamilySlug,
    MutationCaptureError, Surface,
};
use crate::bounded::Overflow;
use crate::descriptor::{CaptureCause, DeclarationError, Grammar, Name, Seat};
use crate::identity::{encode_bytes, encode_length};
use crate::token::{
    CapturedDelimiter, CapturedTokenTree, GeneratedDelimiter, GeneratedToken, SpanHandle,
};

/// The version byte the declared-order operation encoding opens with.
const ORDER_OPERATION_VERSION: u32 = 1;

/// The label the declared-order operation encoding carries after its version.
const ORDER_OPERATION_LABEL: &[u8] = b"declared-variant-order";

/// Complete one captured declaration with the site material this door computes from the item the helper sits on.
///
/// # What the door reads, and what it refuses to
///
/// The item must be an `enum`, and its variant list — in the order the author wrote it — is the declared order the site presses.
/// The reading is structural: variant NAMES and their order, never fields, discriminants, or what any variant means.
///
/// The unchanged operation is the authored order itself.
/// Each alternative is one adjacent transposition of it, under the [`DECLARED_ORDER_FAMILY`] operator family, so what the pressure asks is exactly: would any witness notice if two neighbors of this declared order traded places?
///
/// The order type is `[&'static str; N]` and every value of it is the spellings themselves, so the rendered module is pure data and resolves in a test target that has never seen the declaring crate's module tree.
///
/// # Errors
///
/// Returns [`MutationCaptureError`] where the item states no enum body ([`CaptureCause::ItemUnread`]), where the order has fewer than two members to transpose ([`CaptureCause::OrderUnpressable`]), and where the values it states are not a lawful site — each at the token it was established at.
pub fn completed(
    declaration: Declaration,
    item: &[&CapturedTokenTree],
    grammar: Grammar,
) -> Result<Surface, MutationCaptureError> {
    let (at, order) = declared_order(grammar, item)?;
    completed_from_order(declaration, &order, at, grammar)
}

/// Complete one captured declaration from an already informed authored order.
///
/// This is the composition road for a caller that already proved the exact member roster and therefore must not reparse a second copy of the enum item.
/// It applies the same adjacent-transposition mechanics and refusal contract as [`completed`].
///
/// # Errors
///
/// Returns [`MutationCaptureError`] where the order has fewer than two members, exceeds the alternative bound, or cannot be rendered within its declared token magnitudes.
pub fn completed_from_order(
    declaration: Declaration,
    order: &[String],
    at: SpanHandle,
    grammar: Grammar,
) -> Result<Surface, MutationCaptureError> {
    if order.len() < 2 {
        return Err(refused(grammar, CaptureCause::OrderUnpressable, at));
    }
    let offered_alternatives = order.len().saturating_sub(1);
    if offered_alternatives > ALTERNATIVE_LIMIT {
        return Err(carried(
            grammar,
            DeclarationError::unbounded(Seat::Alternative, ALTERNATIVE_LIMIT, offered_alternatives),
            at,
        ));
    }
    let family = declaration.policy().family().clone();
    let unchanged = order_operation(&family, order);
    let production = spelling_array(order).map_err(|overflow| overflown(grammar, overflow, at))?;
    let order_type =
        order_type(order.len()).map_err(|overflow| overflown(grammar, overflow, at))?;

    let mut alternatives: Vec<Alternative> = Vec::new();
    for left in 0..order.len().saturating_sub(1) {
        let mut transposed = order.to_vec();
        transposed.swap(left, left.saturating_add(1));
        let operation = order_operation(&family, &transposed);
        let meaning =
            spelling_array(&transposed).map_err(|overflow| overflown(grammar, overflow, at))?;
        let slug = FamilySlug::declared(DECLARED_ORDER_FAMILY)
            .map_err(|refusal| carried(grammar, refusal, at))?;
        alternatives.push(
            Alternative::stated(slug, operation, meaning)
                .map_err(|refusal| carried(grammar, refusal, at))?,
        );
    }
    declaration
        .completed(order_type, production, unchanged, alternatives)
        .map_err(|refusal| carried(grammar, refusal, at))
}

/// One vocabulary refusal carried whole, at the token the value was read from.
const fn carried(
    grammar: Grammar,
    refusal: DeclarationError,
    at: SpanHandle,
) -> MutationCaptureError {
    MutationCaptureError::vocabulary_refused(grammar, refusal, at)
}

/// One established grammar refusal at one token.
const fn refused(grammar: Grammar, cause: CaptureCause, at: SpanHandle) -> MutationCaptureError {
    MutationCaptureError::grammar_refused(grammar, cause, at)
}

/// One token-magnitude overflow, carried as the vocabulary refusal about the site's alternatives.
fn overflown(grammar: Grammar, overflow: Overflow, at: SpanHandle) -> MutationCaptureError {
    carried(
        grammar,
        DeclarationError::unbounded(Seat::Alternative, overflow.capacity, overflow.offered),
        at,
    )
}

/// The declared order the item states: the enum's variant names, in authored order, and the token the body sits at.
fn declared_order(
    grammar: Grammar,
    item: &[&CapturedTokenTree],
) -> Result<(SpanHandle, Vec<String>), MutationCaptureError> {
    let fallback = item
        .first()
        .map_or_else(|| SpanHandle::at(0), |tree| tree.span());
    let opened = item
        .iter()
        .position(|tree| tree.word() == Some("enum"))
        .ok_or_else(|| refused(grammar, CaptureCause::ItemUnread, fallback))?;
    let body = item
        .iter()
        .skip(opened)
        .rev()
        .find_map(|tree| match tree.group() {
            Some((CapturedDelimiter::Brace, inner)) => Some((tree.span(), inner)),
            Some(_) | None => None,
        })
        .ok_or_else(|| refused(grammar, CaptureCause::ItemUnread, fallback))?;
    let (at, inner) = body;
    let mut order: Vec<String> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in inner {
        if tree.punct() == Some(',') {
            if group.is_empty() {
                return Err(refused(
                    grammar,
                    CaptureCause::SeparatorDangling,
                    tree.span(),
                ));
            }
            order.push(variant(grammar, &group, at)?.to_owned());
            group.clear();
        } else {
            group.push(tree);
        }
    }
    if !group.is_empty() {
        order.push(variant(grammar, &group, at)?.to_owned());
    }
    if order.len() < 2 {
        return Err(refused(grammar, CaptureCause::OrderUnpressable, at));
    }
    Ok((at, order))
}

/// One variant's identifier name, read past whatever attributes stand before it.
fn variant<'trees>(
    grammar: Grammar,
    group: &[&'trees CapturedTokenTree],
    at: SpanHandle,
) -> Result<&'trees str, MutationCaptureError> {
    let mut trees = group.iter();
    while let Some(tree) = trees.next() {
        if tree.punct() == Some('#') {
            let _attribute_body = trees.next();
            continue;
        }
        return tree
            .word()
            .or_else(|| tree.raw_identifier())
            .ok_or_else(|| refused(grammar, CaptureCause::ItemUnread, tree.span()));
    }
    Err(refused(grammar, CaptureCause::ItemUnread, at))
}

/// The semantic bytes one order is identified by: a version, a label, the evaluation family, and the members in sequence.
fn order_operation(family: &Name, order: &[String]) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&ORDER_OPERATION_VERSION.to_be_bytes());
    encode_bytes(ORDER_OPERATION_LABEL, &mut bytes);
    encode_bytes(family.namespace().as_bytes(), &mut bytes);
    encode_bytes(family.stem().as_bytes(), &mut bytes);
    encode_length(order.len(), &mut bytes);
    for spelling in order {
        encode_bytes(spelling.as_bytes(), &mut bytes);
    }
    bytes
}

/// The type every rendered order value inhabits: an array of the declared width over static text.
fn order_type(count: usize) -> Result<Vec<GeneratedToken>, Overflow> {
    Ok(vec![GeneratedToken::group(
        GeneratedDelimiter::Bracket,
        vec![
            GeneratedToken::alone('&'),
            GeneratedToken::joint('\''),
            GeneratedToken::word("static"),
            GeneratedToken::word("str"),
            GeneratedToken::alone(';'),
            GeneratedToken::number(u64::try_from(count).unwrap_or(u64::MAX)),
        ],
    )?])
}

/// One order as the array literal that spells it.
fn spelling_array(order: &[String]) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut members: Vec<GeneratedToken> = Vec::new();
    for spelling in order {
        members.push(GeneratedToken::text(spelling));
        members.push(GeneratedToken::alone(','));
    }
    Ok(vec![GeneratedToken::group(
        GeneratedDelimiter::Bracket,
        members,
    )?])
}
