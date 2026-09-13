//! Codec member-order pressure composed through the existing codec declaration and renderer.

use super::{
    Alternative, CodecMutationError, DECLARED_ORDER_FAMILY, Declaration, FamilySlug, Surface,
};
use crate::codec::{CodecContent, CodecShape, codec_surface};
use crate::identity::encode_bytes;
use crate::kind::CanonicalContent;
use crate::token::GeneratedToken;

/// Complete a mutation declaration with unchanged codec source and each adjacent member swap.
///
/// The [mutation home](super) owns the carried source and evidence ceiling.
///
/// # Errors
///
/// Returns [`CodecMutationError`] for an unpressable roster or a refused codec, site, or rendering.
pub fn completed_from_codec_order(
    declaration: Declaration,
    content: &CodecContent,
) -> Result<Surface, CodecMutationError> {
    let members = content.shape.members().cloned().collect::<Vec<_>>();
    if members.len() < 2 {
        return Err(CodecMutationError::NoAdjacentMembers);
    }
    let family =
        FamilySlug::declared(DECLARED_ORDER_FAMILY).map_err(CodecMutationError::Declaration)?;
    let production = source_meaning(content)?;
    let mut alternatives = Vec::new();
    for left in 0..members.len().saturating_sub(1) {
        let mut swapped = members.clone();
        swapped.swap(left, left.saturating_add(1));
        let shape = CodecShape::declared(
            content.shape.owner().clone(),
            content.shape.refusal(),
            content.shape.assembly().clone(),
            swapped,
        )
        .map_err(CodecMutationError::Codec)?;
        let changed = CodecContent {
            shape,
            ..content.clone()
        };
        alternatives.push(
            Alternative::stated(
                family.clone(),
                operation(&changed),
                source_meaning(&changed)?,
            )
            .map_err(CodecMutationError::Declaration)?,
        );
    }
    declaration
        .completed(
            vec![
                GeneratedToken::alone('&'),
                GeneratedToken::joint('\''),
                GeneratedToken::word("static"),
                GeneratedToken::word("str"),
            ],
            production,
            operation(content),
            alternatives,
        )
        .map_err(CodecMutationError::Declaration)
}

fn operation(content: &CodecContent) -> Vec<u8> {
    let mut bytes = Vec::new();
    encode_bytes(b"codec-member-order/v1", &mut bytes);
    encode_bytes(&content.canonical_content_bytes(), &mut bytes);
    bytes
}

fn source_meaning(content: &CodecContent) -> Result<Vec<GeneratedToken>, CodecMutationError> {
    let source = codec_surface(content)
        .map_err(CodecMutationError::Tokens)?
        .inspected();
    Ok(vec![GeneratedToken::text(&source)])
}
