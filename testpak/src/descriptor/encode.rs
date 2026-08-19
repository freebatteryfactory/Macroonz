//! The canonical bytes one generated-support schema declaration commits to.
//!
//! These bytes are the PREIMAGE of the generated-support schema identity, and
//! the identity is derived from them ([`GeneratedSupportSchema::identity`]).
//! The bytes are never "the id", and no reader is meant to parse meaning out of
//! them: they exist so that one declaration has exactly one byte string, and so
//! that a change to any member of that declaration moves the derived identity.
//!
//! # The specification
//!
//! The encoding is a function of the declaration and of nothing else — no
//! clock, no environment, no source text, no iteration order that is not the
//! declared one. It is stated completely here, because an independent reader
//! re-deriving this identity writes its own encoder from this paragraph and
//! imports nothing.
//!
//! Two primitives:
//!
//! - `u32be(n)` / `u64be(n)` — the integer in four or eight big-endian bytes.
//! - `bytes(x)` — `u64be(len(x))` followed by the bytes of `x`. Every
//!   variable-length member is written this way, so no two member sequences can
//!   be cut at a different boundary and produce one byte string.
//!
//! The declaration, in exactly this order, with no separators and no padding:
//!
//! | # | member | encoding |
//! | - | ------ | -------- |
//! | 1 | encoding version | `u32be` |
//! | 2 | descriptor member | member tag `1`, then its roster |
//! | 3 | mutation-point member | member tag `2`, then its roster |
//! | 4 | bench member | member tag `3`, then its roster |
//!
//! A roster is `u64be(field count)` followed by each field in declared order. A
//! field is `bytes(name)`, then its shape, then one byte for its cardinality
//! slot. A shape is one byte for its slot; the closed-choice shape additionally
//! writes `u64be(arm count)` followed by `bytes(arm)` for each arm in declared
//! order. The slots are the closed tables in `type_contract.rs`.
//!
//! Nothing is folded on the way in: every name and every arm spelling is
//! written at full length, so the derived identity is the only compression
//! anywhere in the derivation.

use super::types::{EncodeRefusal, FieldShape, GeneratedSupportSchema, SchemaField};

/// The version of this encoding itself.
///
/// It rides the preimage, so changing how the bytes are cut moves every derived
/// identity — a new encoding can never be mistaken for the old one over the
/// same declaration.
const ENCODING_VERSION: u32 = 1;

/// The tag the descriptor member is written under.
const DESCRIPTOR_MEMBER_TAG: u8 = 1;

/// The tag the mutation-point member is written under.
const MUTATION_POINT_MEMBER_TAG: u8 = 2;

/// The tag the bench member is written under.
const BENCH_MEMBER_TAG: u8 = 3;

/// The canonical bytes of one root schema declaration.
///
/// # Errors
///
/// Refuses a length that does not fit the sixty-four bit width the encoding
/// declares. The encoder states its widths rather than guessing at one; on
/// every target this crate is built for the case is unreachable.
pub fn encode_generated_support_schema(
    schema: &GeneratedSupportSchema,
) -> Result<Vec<u8>, EncodeRefusal> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&ENCODING_VERSION.to_be_bytes());
    push_member(&mut bytes, DESCRIPTOR_MEMBER_TAG, schema.descriptor().fields())?;
    push_member(&mut bytes, MUTATION_POINT_MEMBER_TAG, schema.mutation_point().fields())?;
    push_member(&mut bytes, BENCH_MEMBER_TAG, schema.bench().fields())?;
    Ok(bytes)
}

/// One member: its tag, then its roster.
fn push_member(out: &mut Vec<u8>, tag: u8, fields: &[SchemaField]) -> Result<(), EncodeRefusal> {
    out.push(tag);
    push_count(out, fields.len())?;
    for field in fields {
        push_text(out, field.name())?;
        push_shape(out, field.shape())?;
        out.push(field.cardinality().slot());
    }
    Ok(())
}

/// One shape: its slot, and the arm spellings the closed-choice shape carries.
fn push_shape(out: &mut Vec<u8>, shape: FieldShape) -> Result<(), EncodeRefusal> {
    out.push(shape.slot());
    match shape {
        FieldShape::ClosedChoice(arms) => {
            push_count(out, arms.len())?;
            for arm in arms {
                push_text(out, arm)?;
            }
        }
        FieldShape::NamespacedName
        | FieldShape::ContentAddress
        | FieldShape::Bytes
        | FieldShape::Count => {}
    }
    Ok(())
}

/// One length-prefixed text.
fn push_text(out: &mut Vec<u8>, text: &str) -> Result<(), EncodeRefusal> {
    push_count(out, text.len())?;
    out.extend_from_slice(text.as_bytes());
    Ok(())
}

/// One count, at the declared width.
fn push_count(out: &mut Vec<u8>, count: usize) -> Result<(), EncodeRefusal> {
    let declared = u64::try_from(count).map_err(|_| EncodeRefusal::LengthPastEncodingWidth)?;
    out.extend_from_slice(&declared.to_be_bytes());
    Ok(())
}
