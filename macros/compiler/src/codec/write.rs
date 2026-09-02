//! The generated road that writes one declared shape's canonical bytes.

use super::spell::{
    CARRIED_BINDING, INTO_BINDING, MATERIAL_BINDING, NESTED_BINDING, appended, associated,
    borrowed_self_member, byte_sink, byte_slice, empty_vector, framed_length, generics, qualified,
    road_spelling, self_member, statement, type_path,
};
use super::type_contract::rendering_contract;
use super::types::WriteRoad;
use super::{Cardinality, CodecMember, CodecShape};
use crate::bounded::Overflow;
use crate::token::{
    GeneratedDelimiter, GeneratedToken, absolute_path, bound_local, call, documentation, group,
    method_call,
};

/// The sentence the rendered encode road documents itself with.
const ENCODE_SENTENCE: &str = "Append this value's canonical bytes. Every variable-length member \
     is written length-prefixed at the framing width, in the order the shape declares its members, \
     so two values this shape considers different never encode identically.";

/// Which borrowed material one framed write carries.
#[derive(Clone, Copy)]
enum FramedMaterial {
    /// Bytes are already the material the frame carries.
    Bytes,
    /// Text contributes its UTF-8 bytes to the frame.
    Text,
}

/// One member's write, over the subject its cardinality handed it.
///
/// The subject always stands for a reference to one occurrence, so the five wire roads never learn how many of the member there were.
fn write_member(
    member: &CodecMember,
    subject: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let contract = rendering_contract(member.shape());
    let road = road_spelling(contract.bill.encode_road);
    match contract.write {
        WriteRoad::Count => write_count(subject, road),
        WriteRoad::Bytes => write_framed(member, subject, FramedMaterial::Bytes, road),
        WriteRoad::Text => write_framed(member, subject, FramedMaterial::Text, road),
        WriteRoad::ClosedChoice => write_slot(subject, road),
        WriteRoad::Nested => write_nested(subject, road),
    }
}

/// A count, at the framing width.
fn write_count(subject: Vec<GeneratedToken>, road: &str) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut dereferenced = vec![GeneratedToken::alone('*')];
    dereferenced.extend(subject);
    let mut widening = vec![GeneratedToken::word("u64")];
    widening.extend(associated(road));
    let widened = call(widening, dereferenced)?;
    let bytes = method_call(widened, "to_be_bytes", Vec::new())?;
    let mut borrowed = vec![GeneratedToken::alone('&')];
    borrowed.extend(bytes);
    appended(borrowed)
}

/// Framed bytes, or framed text read as its UTF-8 bytes.
fn write_framed(
    member: &CodecMember,
    subject: Vec<GeneratedToken>,
    posture: FramedMaterial,
    road_name: &str,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let target = match posture {
        FramedMaterial::Bytes => byte_slice()?,
        FramedMaterial::Text => vec![GeneratedToken::word("str")],
    };
    let mut contract = absolute_path(&["core", "convert", "AsRef"]);
    contract.extend(generics(target));
    let road = qualified(type_path(member.held_as()), contract, road_name);
    let borrowed = call(road, subject)?;
    let expression = match posture {
        FramedMaterial::Bytes => borrowed,
        FramedMaterial::Text => method_call(borrowed, "as_bytes", Vec::new())?,
    };
    let mut tokens = bound_local(MATERIAL_BINDING, expression);
    let material = vec![GeneratedToken::word(MATERIAL_BINDING)];
    tokens.extend(appended(framed_length(material.clone())?)?);
    tokens.extend(appended(material)?);
    Ok(tokens)
}

/// One arm of a closed roster, as its own declared position.
fn write_slot(subject: Vec<GeneratedToken>, road: &str) -> Result<Vec<GeneratedToken>, Overflow> {
    let slot = method_call(subject, road, Vec::new())?;
    let pushed = method_call(vec![GeneratedToken::word(INTO_BINDING)], "push", slot)?;
    Ok(statement(pushed))
}

/// A nested value, written by its own codec and then framed at its own length.
fn write_nested(subject: Vec<GeneratedToken>, road: &str) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = super::spell::bound_mutable(NESTED_BINDING, empty_vector()?);
    let sink = vec![
        GeneratedToken::alone('&'),
        GeneratedToken::word("mut"),
        GeneratedToken::word(NESTED_BINDING),
    ];
    tokens.extend(statement(method_call(subject, road, sink)?));
    let nested = vec![GeneratedToken::word(NESTED_BINDING)];
    tokens.extend(appended(framed_length(nested)?)?);
    tokens.extend(appended(vec![
        GeneratedToken::alone('&'),
        GeneratedToken::word(NESTED_BINDING),
    ])?);
    Ok(tokens)
}

/// One member's complete contribution to the encode road, under its declared cardinality.
fn encode_member(member: &CodecMember) -> Result<Vec<GeneratedToken>, Overflow> {
    match member.cardinality() {
        Cardinality::Required => {
            let subject = borrowed_self_member(member.spelling())?;
            let written = write_member(member, subject)?;
            Ok(vec![group(GeneratedDelimiter::Brace, written)?])
        }
        Cardinality::Optional => encode_optional(member),
        Cardinality::Repeated => encode_repeated(member),
    }
}

/// An optional member: its presence byte, then its value where there is one.
///
/// The presence byte is `u8::from(…)` over the member's own answer rather than a numeric literal, and the decode road reads it back through the very same road.
fn encode_optional(member: &CodecMember) -> Result<Vec<GeneratedToken>, Overflow> {
    let asked = method_call(self_member(member.spelling()), "is_some", Vec::new())?;
    let mut presence = vec![GeneratedToken::word("u8")];
    presence.extend(associated("from"));
    let presence = call(presence, asked)?;
    let pushed = method_call(vec![GeneratedToken::word(INTO_BINDING)], "push", presence)?;
    let mut tokens = statement(pushed);
    let written = write_member(member, vec![GeneratedToken::word(CARRIED_BINDING)])?;
    tokens.push(GeneratedToken::word("if"));
    tokens.push(GeneratedToken::word("let"));
    tokens.extend(absolute_path(&["core", "option", "Option", "Some"]));
    tokens.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word(CARRIED_BINDING)],
    )?);
    tokens.push(GeneratedToken::alone('='));
    tokens.push(GeneratedToken::alone('&'));
    tokens.extend(self_member(member.spelling()));
    tokens.push(group(GeneratedDelimiter::Brace, written)?);
    Ok(tokens)
}

/// A repeated member: its framed count, then each occurrence in order.
fn encode_repeated(member: &CodecMember) -> Result<Vec<GeneratedToken>, Overflow> {
    let counted = framed_length(self_member(member.spelling()))?;
    let mut tokens = appended(counted)?;
    let written = write_member(member, vec![GeneratedToken::word(CARRIED_BINDING)])?;
    tokens.push(GeneratedToken::word("for"));
    tokens.push(GeneratedToken::word(CARRIED_BINDING));
    tokens.push(GeneratedToken::word("in"));
    tokens.push(GeneratedToken::alone('&'));
    tokens.extend(self_member(member.spelling()));
    tokens.push(group(GeneratedDelimiter::Brace, written)?);
    Ok(tokens)
}

/// The encode road: one member at a time, in the order the shape declares them.
pub(super) fn encode_road(shape: &CodecShape) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut body: Vec<GeneratedToken> = Vec::new();
    for member in shape.members() {
        body.extend(encode_member(member)?);
    }
    let mut parameters = vec![
        GeneratedToken::alone('&'),
        GeneratedToken::word("self"),
        GeneratedToken::alone(','),
        GeneratedToken::word(INTO_BINDING),
        GeneratedToken::alone(':'),
        GeneratedToken::alone('&'),
        GeneratedToken::word("mut"),
    ];
    parameters.extend(byte_sink());
    let mut tokens = documentation(ENCODE_SENTENCE)?;
    tokens.push(GeneratedToken::word("pub"));
    tokens.push(GeneratedToken::word("fn"));
    tokens.push(GeneratedToken::word(super::ENCODE_ROAD));
    tokens.push(group(GeneratedDelimiter::Parenthesis, parameters)?);
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}
