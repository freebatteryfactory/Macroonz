//! The token half of the road: the surface road one declared port speaks a wire
//! contract over, and the primitives it is written from.
//!
//! # Tokens, not text
//!
//! Every path is spelled as segments, every brace is a group, and no function
//! here composes Rust source. The Rust a person reads is
//! [`GeneratedTree::inspected`](crate::token::GeneratedTree::inspected), a
//! projection of what is emitted rather than the thing itself.
//!
//! # Three calls, and the facing decides only their order
//!
//! A surface OPENS with one of the pairing's two roads, CALLS the port's own
//! road, and CLOSES with the pairing's other road. Both facings ride both pairing
//! roads and both call the port between them; what an inbound surface and an
//! outbound surface disagree about is which end of the wire they stand at, and
//! that disagreement is exactly which road runs first. The table that answers it
//! is `facing` in `type_contract.rs`, and this file asks rather than decides.
//!
//! # The codec is the caller's, and so is the port
//!
//! The plan names a wire contract and a port DECLARATION; the type that realizes
//! the port and the codec whose roads carry the bytes are the caller's, arriving
//! on the shape. This file writes the calls and the integration target's own
//! compiler answers whether the roads fit — which is exactly where a missing or
//! mis-shaped road on somebody else's type belongs. The bill each pairing road is
//! called under is stated once as `PAIRING_CONTRACT` in `type_contract.rs`.
//!
//! # No numeric literal is written anywhere here
//!
//! Nothing this home renders needs a count: a surface is three named calls, and
//! three is the length of a token run rather than a value written into one.

use super::super::type_contract::facing;
use super::{
    PairedCodecRoad, RemoteSurfaceIssue, RemoteSurfaceShape, SurfacePathRooting, SurfaceTypePath,
};
use crate::plane::GeneratedTokenLimit;
use crate::planning::SurfaceDirection;
use crate::token::{GeneratedDelimiter, GeneratedToken, GeneratedTree};
use threadpak::types::ConstLimit;

// ---------------------------------------------------------------------------
// The spellings this home writes at the address it renders into.
// ---------------------------------------------------------------------------

/// The surface's one parameter: the value the opening pairing road is handed.
pub const ENTRY_PARAMETER: &str = "carried";

/// The binding the opening pairing road's answer stands under.
pub const OPENED_BINDING: &str = "opened";

/// The binding the port's own road's answer stands under.
pub const SERVED_BINDING: &str = "served";

/// The binding the closing pairing road's answer stands under — the surface's own
/// answer.
pub const CLOSED_BINDING: &str = "closed";

/// The sentence the rendered surface carries as its own documentation.
///
/// Every public item this home renders carries one, because a lint wall that
/// denies an undocumented public item is the wall an integration target is most
/// likely to be standing behind.
pub const SURFACE_SENTENCE: &str =
    "The remote surface this port speaks its wire contract over: one pairing road opens it, the \
     port's own road answers, and the pairing's other road closes it.";

// ---------------------------------------------------------------------------
// The token primitives.
// ---------------------------------------------------------------------------

/// The issue a tree that outgrew the declared token magnitude amounts to.
#[must_use]
pub fn unbounded() -> RemoteSurfaceIssue {
    RemoteSurfaceIssue::SurfaceTreeUnbounded {
        bound: u64::try_from(GeneratedTokenLimit::MAX).unwrap_or(u64::MAX),
    }
}

/// One delimited group, with a tree past the declared magnitude refused in this
/// home's own vocabulary.
///
/// # Errors
///
/// Returns [`RemoteSurfaceIssue::SurfaceTreeUnbounded`] where the group carries
/// more tokens than the declared magnitude admits.
pub fn group(
    delimiter: GeneratedDelimiter,
    tokens: Vec<GeneratedToken>,
) -> Result<GeneratedToken, RemoteSurfaceIssue> {
    GeneratedToken::group(delimiter, tokens).map_err(|_| unbounded())
}

/// One path a caller declared, spelled from the rooting it stated.
#[must_use]
pub fn type_path(path: &SurfaceTypePath) -> Vec<GeneratedToken> {
    let segments: Vec<&str> = path.segments().map(String::as_str).collect();
    match path.rooting() {
        SurfacePathRooting::CrateAbsolute => GeneratedToken::absolute_path(&segments),
        SurfacePathRooting::InScope => in_scope_path(&segments),
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
/// Returns [`RemoteSurfaceIssue::SurfaceTreeUnbounded`] where the attribute
/// outgrows the declared token magnitude.
pub fn attribute(body: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, RemoteSurfaceIssue> {
    Ok(vec![
        GeneratedToken::alone('#'),
        group(GeneratedDelimiter::Bracket, body)?,
    ])
}

/// One doc attribute, as the tokens that spell it.
///
/// # Errors
///
/// Returns [`RemoteSurfaceIssue::SurfaceTreeUnbounded`] where the attribute
/// outgrows the declared token magnitude.
pub fn doc_attribute(sentence: &str) -> Result<Vec<GeneratedToken>, RemoteSurfaceIssue> {
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

/// The language's own result type over what the road answers with and the shape's
/// refusal.
#[must_use]
pub fn result_type(answers: &SurfaceTypePath, refusal: &SurfaceTypePath) -> Vec<GeneratedToken> {
    let mut tokens = language_path(&["core", "result", "Result"]);
    tokens.push(GeneratedToken::alone('<'));
    tokens.extend(type_path(answers));
    tokens.push(GeneratedToken::alone(','));
    tokens.extend(type_path(refusal));
    tokens.push(GeneratedToken::alone('>'));
    tokens
}

/// The surface's answer: the closing binding, wrapped in the language's own
/// success arm.
///
/// # Errors
///
/// Returns [`RemoteSurfaceIssue::SurfaceTreeUnbounded`] where the expression
/// outgrows the declared token magnitude.
pub fn answered(binding: &str) -> Result<Vec<GeneratedToken>, RemoteSurfaceIssue> {
    let mut tokens = language_path(&["core", "result", "Result"]);
    tokens.extend(associated("Ok"));
    tokens.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word(binding)],
    )?);
    Ok(tokens)
}

/// One checked call: `<Subject>::<road>(<argument>)?`.
///
/// Qualified by the subject's own path rather than called on the value, so the
/// road named is the road its owner declared and never an inherent method that
/// happened to share a spelling with something on the carried type.
///
/// # Errors
///
/// Returns [`RemoteSurfaceIssue::SurfaceTreeUnbounded`] where the call outgrows
/// the declared token magnitude.
pub fn checked_call(
    subject: &SurfaceTypePath,
    road: &str,
    argument: &str,
) -> Result<Vec<GeneratedToken>, RemoteSurfaceIssue> {
    let mut tokens = type_path(subject);
    tokens.extend(associated(road));
    tokens.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word(argument)],
    )?);
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// One of the pairing's roads, called on the binding handed to it.
///
/// # Errors
///
/// Returns [`RemoteSurfaceIssue::SurfaceTreeUnbounded`] where the call outgrows
/// the declared token magnitude.
pub fn pairing_call(
    shape: &RemoteSurfaceShape,
    road: PairedCodecRoad,
    argument: &str,
) -> Result<Vec<GeneratedToken>, RemoteSurfaceIssue> {
    let pairing = shape.pairing();
    checked_call(pairing.codec(), pairing.road(road), argument)
}

// ---------------------------------------------------------------------------
// The surface road.
// ---------------------------------------------------------------------------

/// The surface's signature and body, as the item that spells them.
///
/// # Errors
///
/// Returns [`RemoteSurfaceIssue::SurfaceTreeUnbounded`] where the item outgrows
/// the declared token magnitude.
pub fn surface_entry(
    shape: &RemoteSurfaceShape,
    body: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, RemoteSurfaceIssue> {
    let signature = shape.signature();
    let mut parameters = vec![
        GeneratedToken::word(ENTRY_PARAMETER),
        GeneratedToken::alone(':'),
    ];
    parameters.extend(type_path(signature.accepts()));
    let mut tokens = doc_attribute(SURFACE_SENTENCE)?;
    tokens.push(GeneratedToken::word("pub"));
    tokens.push(GeneratedToken::word("fn"));
    tokens.push(GeneratedToken::word(shape.entry()));
    tokens.push(group(GeneratedDelimiter::Parenthesis, parameters)?);
    tokens.push(GeneratedToken::joint('-'));
    tokens.push(GeneratedToken::alone('>'));
    tokens.extend(result_type(signature.answers(), signature.refusal()));
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// The whole surface road under one declared facing: the opening pairing call,
/// the port's own call, the closing pairing call, and the answer.
///
/// The facing decides only which pairing road opens and which closes, and it is
/// read from the table rather than branched on here — so a third direction
/// admitted to the plane stops the compiler at the table instead of falling
/// through a rendering that guessed.
///
/// # Errors
///
/// Returns [`RemoteSurfaceIssue::SurfaceTreeUnbounded`] where the surface outgrows
/// the declared token magnitude.
pub fn surface_road(
    shape: &RemoteSurfaceShape,
    direction: SurfaceDirection,
) -> Result<GeneratedTree, RemoteSurfaceIssue> {
    let faces = facing(direction);
    let mut body = bound(
        OPENED_BINDING,
        pairing_call(shape, faces.opens_with, ENTRY_PARAMETER)?,
    );
    body.extend(bound(
        SERVED_BINDING,
        checked_call(shape.port(), shape.call(), OPENED_BINDING)?,
    ));
    body.extend(bound(
        CLOSED_BINDING,
        pairing_call(shape, faces.closes_with, SERVED_BINDING)?,
    ));
    body.extend(answered(CLOSED_BINDING)?);
    GeneratedTree::assembled(surface_entry(shape, body)?).map_err(|_| unbounded())
}
