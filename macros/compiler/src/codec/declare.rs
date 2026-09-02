//! The rendered decode refusal and the conversion a checked assembly earns.

use super::spell::{CARRIED_BINDING, MEMBER_SEAT, associated, generics, type_path};
use super::{AssemblyPosture, CodecShape, DecodeRefusal};
use crate::bounded::Overflow;
use crate::token::{
    GeneratedDelimiter, GeneratedToken, absolute_path, associated_function, attribute,
    documentation, function_signature, group, implementation, typed_parameter,
};

/// The sentence the rendered decode refusal documents itself with.
const REFUSAL_SENTENCE: &str = "Why one decode of this shape's canonical bytes refused. Holding \
     one is the whole of what went wrong: the arm says which read established it and, where the \
     read was about one member, which member it was standing at.";

/// `#[derive(Debug, Clone, PartialEq, Eq)]`.
///
/// Four and no more: a refusal is shown in a report, cloned into one, and compared against an expectation, and nothing about a decode refusal needs ordering or hashing.
/// `Copy` is absent because the assembly arm may carry a refusal the owner declared, and this home does not decide whether that one copies.
fn derive_attribute() -> Result<Vec<GeneratedToken>, Overflow> {
    let named = group(
        GeneratedDelimiter::Parenthesis,
        vec![
            GeneratedToken::word("Debug"),
            GeneratedToken::alone(','),
            GeneratedToken::word("Clone"),
            GeneratedToken::alone(','),
            GeneratedToken::word("PartialEq"),
            GeneratedToken::alone(','),
            GeneratedToken::word("Eq"),
        ],
    )?;
    attribute(vec![GeneratedToken::word("derive"), named])
}

/// One variant of the rendered refusal: its sentence, its spelling, and the payload it carries.
fn variant(
    arm: DecodeRefusal,
    payload: Option<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = documentation(arm.sentence())?;
    tokens.push(GeneratedToken::word(arm.name()));
    if let Some(seat) = payload {
        tokens.push(seat);
    }
    tokens.push(GeneratedToken::alone(','));
    Ok(tokens)
}

/// `{ member: &'static str, }` — the seat a member-bearing arm names its member through.
fn member_seat() -> Result<GeneratedToken, Overflow> {
    let mut tokens = documentation("The declared member at which decoding refused.")?;
    tokens.extend([
        GeneratedToken::word(MEMBER_SEAT),
        GeneratedToken::alone(':'),
        GeneratedToken::alone('&'),
        GeneratedToken::joint('\''),
        GeneratedToken::word("static"),
        GeneratedToken::word("str"),
        GeneratedToken::alone(','),
    ]);
    group(GeneratedDelimiter::Brace, tokens)
}

/// The decode refusal one shape's surface declares.
///
/// Every member-bearing arm, then the whole-material arm, then the assembly arm a checked assembly earns — and only that posture earns it, so a total assembly renders a refusal with nothing on it that cannot happen.
pub(super) fn refusal_declaration(shape: &CodecShape) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut variants: Vec<GeneratedToken> = Vec::new();
    for arm in DecodeRefusal::ALL.iter().copied() {
        if arm.carries_member() {
            variants.extend(variant(arm, Some(member_seat()?))?);
        }
    }
    variants.extend(variant(DecodeRefusal::TrailingBytes, None)?);
    if let AssemblyPosture::Checked { refusal } = shape.assembly().posture() {
        let carried = group(GeneratedDelimiter::Parenthesis, type_path(refusal))?;
        variants.extend(variant(DecodeRefusal::NotAssembled, Some(carried))?);
    }
    let mut tokens = documentation(REFUSAL_SENTENCE)?;
    tokens.extend(derive_attribute()?);
    tokens.push(GeneratedToken::word("pub"));
    tokens.push(GeneratedToken::word("enum"));
    tokens.push(GeneratedToken::word(shape.refusal()));
    tokens.push(group(GeneratedDelimiter::Brace, variants)?);
    Ok(tokens)
}

/// The conversion a checked assembly earns: the owner's own refusal into this surface's.
///
/// Rendered rather than billed, so a checked assembly costs the address nothing.
pub(super) fn refusal_conversion(shape: &CodecShape) -> Result<Vec<GeneratedToken>, Overflow> {
    let AssemblyPosture::Checked { refusal } = shape.assembly().posture() else {
        return Ok(Vec::new());
    };
    let carried = type_path(refusal);
    let mut body = vec![GeneratedToken::word("Self")];
    body.extend(associated(DecodeRefusal::NotAssembled.name()));
    body.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word(CARRIED_BINDING)],
    )?);
    let signature = function_signature(
        Vec::new(),
        GeneratedToken::word("from"),
        vec![typed_parameter(
            vec![GeneratedToken::word(CARRIED_BINDING)],
            carried.clone(),
        )],
        Vec::new(),
        Some(vec![GeneratedToken::word("Self")]),
        Vec::new(),
    )?;
    let road = associated_function(signature, Some(body))?;
    let mut trait_path = absolute_path(&["core", "convert", "From"]);
    trait_path.extend(generics(carried));
    implementation(
        Vec::new(),
        Vec::new(),
        Some(trait_path),
        vec![GeneratedToken::word(shape.refusal())],
        Vec::new(),
        road,
    )
}
