//! The token half of the road: the wrapper shell one bound host contract's
//! demand composes to, and the primitives it is written from.
//!
//! # Tokens, not text
//!
//! Every path is spelled as segments, every brace is a group, and no function
//! here composes Rust source. The Rust a person reads is
//! [`GeneratedTree::inspected`](crate::token::GeneratedTree::inspected), a
//! projection of what is emitted rather than the thing itself.
//!
//! # The plane's roster is the order, and the shape's list is not
//!
//! A wrapper composes its components in the order the plane declares them
//! ([`WRAPPER_COMPONENTS`](crate::planning::WRAPPER_COMPONENTS)), not in the
//! order a caller happened to list its stages and not in the order a plan
//! happened to list its selection. [`composition_order`] is that walk, and it is
//! the only place this home decides what comes before what: a wrapper that
//! decoded before it admitted would be a different wrapper, so the order is
//! taken from the one roster that declares it rather than from either list that
//! merely names members.
//!
//! # One value, threaded, and the host's compiler answers the joins
//!
//! Every stage is one checked call on a road the host declared: it takes what the
//! stage before it handed back and hands the next one its answer, and the last
//! stage's answer is the wrapper's. The bill is uniform and stated once — a road
//! takes the carried type and answers with the carried type or the shape's
//! refusal — so this home writes the call and the HOST TARGET's own compiler
//! answers whether the road actually fits, which is exactly where a missing or
//! mis-shaped road on somebody else's type belongs.
//!
//! The local each stage's answer is bound under is this home's fact and is
//! stated once, as the stage contract in `type_contract.rs`. The roads are the
//! caller's and become method names rather than bindings, so nothing a caller
//! spells can shadow a local this file declares.
//!
//! # No numeric literal is written anywhere here
//!
//! Nothing this home renders needs a count: a wrapper is a run of named calls,
//! and the number of them is the length of a token run rather than a value
//! written into one.

use super::type_contract::stage;
use super::{WrapperPathRooting, WrapperShape, WrapperSurfaceIssue, WrapperTypePath};
use crate::plane::{GeneratedTokenLimit, WrapperComponentLimit};
use crate::planning::{WRAPPER_COMPONENTS, WrapperComponent};
use crate::token::{GeneratedDelimiter, GeneratedToken, GeneratedTree};
use threadpak::types::{ConstLimit, NonEmptyBounded};

// ---------------------------------------------------------------------------
// The spellings this home writes at the address it renders into.
// ---------------------------------------------------------------------------

/// The wrapper's one parameter: the value the first stage is handed.
///
/// It is also the value the wrapper hands back where no component is composed at
/// all — a road that composed nothing would return what it was given, which is
/// exactly why a shape declaring no stage is refused before a rendering exists.
pub const ENTRY_PARAMETER: &str = "carried";

/// The sentence the rendered wrapper carries as its own documentation.
///
/// Every public item this home renders carries one, because a lint wall that
/// denies an undocumented public item is the wall a host target is most likely to
/// be standing behind.
pub const WRAPPER_SENTENCE: &str = "The wrapper this host contract's declared capability composed, in the component roster's \
     own order. Each stage calls one road the host declared and hands the next its answer.";

// ---------------------------------------------------------------------------
// The token primitives.
// ---------------------------------------------------------------------------

/// The issue a tree that outgrew the declared token magnitude amounts to.
#[must_use]
pub fn unbounded() -> WrapperSurfaceIssue {
    WrapperSurfaceIssue::WrapperTreeUnbounded {
        bound: u64::try_from(GeneratedTokenLimit::MAX).unwrap_or(u64::MAX),
    }
}

/// One delimited group, with a tree past the declared magnitude refused in this
/// home's own vocabulary.
///
/// # Errors
///
/// Returns [`WrapperSurfaceIssue::WrapperTreeUnbounded`] where the group carries
/// more tokens than the declared magnitude admits.
pub fn group(
    delimiter: GeneratedDelimiter,
    tokens: Vec<GeneratedToken>,
) -> Result<GeneratedToken, WrapperSurfaceIssue> {
    GeneratedToken::group(delimiter, tokens).map_err(|_| unbounded())
}

/// One path a caller declared, spelled from the rooting it stated.
#[must_use]
pub fn type_path(path: &WrapperTypePath) -> Vec<GeneratedToken> {
    let segments: Vec<&str> = path.segments().map(String::as_str).collect();
    match path.rooting() {
        WrapperPathRooting::CrateAbsolute => GeneratedToken::absolute_path(&segments),
        WrapperPathRooting::InScope => in_scope_path(&segments),
    }
}

/// One path resolved in the scope the artifact lands in: the first segment as a
/// plain word, and every later one behind a separator.
fn in_scope_path(segments: &[&str]) -> Vec<GeneratedToken> {
    let mut tokens: Vec<GeneratedToken> = Vec::new();
    for segment in segments {
        if tokens.is_empty() {
            tokens.push(GeneratedToken::word(segment));
            continue;
        }
        tokens.push(GeneratedToken::joint(':'));
        tokens.push(GeneratedToken::alone(':'));
        tokens.push(GeneratedToken::word(segment));
    }
    tokens
}

/// One path rooted at the language's own crates, spelled absolutely.
#[must_use]
pub fn language_path(segments: &[&str]) -> Vec<GeneratedToken> {
    GeneratedToken::absolute_path(segments)
}

/// One separator followed by a word — the road from a path to an associated item
/// on it.
#[must_use]
pub fn associated(spelling: &str) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word(spelling),
    ]
}

/// One attribute over the body a caller spelled.
///
/// # Errors
///
/// Returns [`WrapperSurfaceIssue::WrapperTreeUnbounded`] where the attribute
/// outgrows the declared token magnitude.
pub fn attribute(body: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, WrapperSurfaceIssue> {
    Ok(vec![
        GeneratedToken::alone('#'),
        group(GeneratedDelimiter::Bracket, body)?,
    ])
}

/// One doc attribute, as the tokens that spell it.
///
/// # Errors
///
/// Returns [`WrapperSurfaceIssue::WrapperTreeUnbounded`] where the attribute
/// outgrows the declared token magnitude.
pub fn doc_attribute(sentence: &str) -> Result<Vec<GeneratedToken>, WrapperSurfaceIssue> {
    attribute(vec![
        GeneratedToken::word("doc"),
        GeneratedToken::alone('='),
        GeneratedToken::text(sentence),
    ])
}

/// One statement: the tokens a caller spelled, closed with a semicolon.
#[must_use]
pub fn statement(mut tokens: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    tokens.push(GeneratedToken::alone(';'));
    tokens
}

/// One `let <binding> = <expression>;` statement.
#[must_use]
pub fn bound(binding: &str, expression: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = vec![
        GeneratedToken::word("let"),
        GeneratedToken::word(binding),
        GeneratedToken::alone('='),
    ];
    tokens.extend(expression);
    statement(tokens)
}

/// The language's own result type over the carried type and the shape's refusal.
#[must_use]
pub fn result_type(carried: &WrapperTypePath, refusal: &WrapperTypePath) -> Vec<GeneratedToken> {
    let mut tokens = language_path(&["core", "result", "Result"]);
    tokens.push(GeneratedToken::alone('<'));
    tokens.extend(type_path(carried));
    tokens.push(GeneratedToken::alone(','));
    tokens.extend(type_path(refusal));
    tokens.push(GeneratedToken::alone('>'));
    tokens
}

/// The wrapper's answer: the last stage's binding, wrapped in the language's own
/// success arm.
///
/// # Errors
///
/// Returns [`WrapperSurfaceIssue::WrapperTreeUnbounded`] where the expression
/// outgrows the declared token magnitude.
pub fn answered(binding: &str) -> Result<Vec<GeneratedToken>, WrapperSurfaceIssue> {
    let mut tokens = language_path(&["core", "result", "Result"]);
    tokens.extend(associated("Ok"));
    tokens.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word(binding)],
    )?);
    Ok(tokens)
}

/// One stage's checked call: `<Host>::<road>(<carried>)?`.
///
/// Qualified by the host's own path rather than called on the value, so the road
/// the stage names is the road the host declared and never an inherent method
/// that happened to share a spelling with something on the carried type.
///
/// # Errors
///
/// Returns [`WrapperSurfaceIssue::WrapperTreeUnbounded`] where the call outgrows
/// the declared token magnitude.
pub fn checked_call(
    host: &WrapperTypePath,
    road: &str,
    argument: &str,
) -> Result<Vec<GeneratedToken>, WrapperSurfaceIssue> {
    let mut tokens = type_path(host);
    tokens.extend(associated(road));
    tokens.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word(argument)],
    )?);
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

// ---------------------------------------------------------------------------
// The composition, in the plane's own order.
// ---------------------------------------------------------------------------

/// The components one plan composes, in the PLANE's declared roster order.
///
/// # Authority
///
/// **The roster is the order, and neither list a caller holds is.** A plan states
/// a selection and a shape states stages; both are membership statements whose
/// own order says nothing, so the walk is over
/// [`WRAPPER_COMPONENTS`](crate::planning::WRAPPER_COMPONENTS) with the selection
/// asked once per component. Two plans that named the same components in
/// different orders therefore render one wrapper, and a plan that named a
/// component twice renders it once.
///
/// # Ordering
///
/// The result's order IS meaning: it is the order the rendered wrapper calls the
/// host's roads in.
#[must_use]
pub fn composition_order(
    selected: &NonEmptyBounded<WrapperComponent, WrapperComponentLimit>,
) -> Vec<WrapperComponent> {
    WRAPPER_COMPONENTS
        .into_iter()
        .filter(|component| selected.iter().any(|named| named == component))
        .collect()
}

/// The wrapper's signature and body, as the item that spells them.
///
/// # Errors
///
/// Returns [`WrapperSurfaceIssue::WrapperTreeUnbounded`] where the item outgrows
/// the declared token magnitude.
pub fn wrapper_entry(
    shape: &WrapperShape,
    body: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, WrapperSurfaceIssue> {
    let mut parameters = vec![
        GeneratedToken::word(ENTRY_PARAMETER),
        GeneratedToken::alone(':'),
    ];
    parameters.extend(type_path(shape.carried()));
    let mut tokens = doc_attribute(WRAPPER_SENTENCE)?;
    tokens.push(GeneratedToken::word("pub"));
    tokens.push(GeneratedToken::word("fn"));
    tokens.push(GeneratedToken::word(shape.entry()));
    tokens.push(group(GeneratedDelimiter::Parenthesis, parameters)?);
    tokens.push(GeneratedToken::joint('-'));
    tokens.push(GeneratedToken::alone('>'));
    tokens.extend(result_type(shape.carried(), shape.refusal()));
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// The whole wrapper shell: one checked call per composed component, in the
/// order given, each handing the next its answer.
///
/// The order is the caller's to supply and [`composition_order`] is what produces
/// it, so this road never re-decides what comes before what — it writes the walk
/// it was handed.
///
/// # Errors
///
/// Returns [`WrapperSurfaceIssue::SelectedComponentNotStaged`] where a component
/// in the order has no stage in the shape. The composition pass establishes that
/// disagreement first wherever a surface is composed, so this is the honest
/// answer for a caller that reached the rendering directly rather than a case the
/// pass leaves open.
///
/// Returns [`WrapperSurfaceIssue::WrapperTreeUnbounded`] where the wrapper
/// outgrows the declared token magnitude.
pub fn wrapper_shell(
    shape: &WrapperShape,
    order: &[WrapperComponent],
) -> Result<GeneratedTree, WrapperSurfaceIssue> {
    let mut body: Vec<GeneratedToken> = Vec::new();
    let mut carried = ENTRY_PARAMETER;
    for component in order {
        let Some(staged) = shape.staged(*component) else {
            return Err(WrapperSurfaceIssue::SelectedComponentNotStaged {
                component: *component,
            });
        };
        let binding = stage(*component).carried_as;
        body.extend(bound(
            binding,
            checked_call(shape.host(), staged.road(), carried)?,
        ));
        carried = binding;
    }
    body.extend(answered(carried)?);
    GeneratedTree::assembled(wrapper_entry(shape, body)?).map_err(|_| unbounded())
}
