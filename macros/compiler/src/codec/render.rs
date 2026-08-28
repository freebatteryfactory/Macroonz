//! The token orchestrator for one codec surface.
//!
//! Refusal declaration, writing, reading, placement, and Rust-token spelling are separate operations beside this one, all under the codec home's one semantic owner.

use super::declare::{refusal_conversion, refusal_declaration};
use super::place::published_module;
use super::read::decode_road;
use super::spell::type_path;
use super::write::encode_road;
use super::{CodecContent, CodecPlacement, CodecProjection};
use crate::bounded::Overflow;
use crate::kind::SoleRole;
use crate::plan::Plan;
use crate::render::{Output, RenderError};
use crate::token::{GeneratedDelimiter, GeneratedToken, GeneratedTree, group};

/// Render the one unit a codec request produces.
///
/// Naming the seat is the whole call: everything else the unit answers to is that seat's planned member, read by the sink.
///
/// # Errors
///
/// Returns [`RenderError::SeatUnplanned`] where the plan declares no member under the kind's one seat, [`RenderError::BytesUnbounded`] where the surface passes the rendered-byte magnitude, and [`RenderError::TokensUnbounded`] where a level of it passes the per-level one.
pub fn render_codec(
    plan: &Plan<CodecProjection>,
    out: &mut Output<'_, CodecProjection>,
) -> Result<(), RenderError> {
    let tree = codec_surface(plan.content())?;
    out.unit(SoleRole::Sole, tree)
}

/// The whole surface: the refusal the decode road answers with, the conversion a checked assembly earns, the roads the direction covers, and the placement carrying them.
///
/// The refusal and the conversion are rendered only where the direction covers the decode road, so an encode-only surface declares nothing that cannot happen — and carries no reader, which is what an encode-only direction means.
///
/// # Errors
///
/// Returns [`Overflow`] where a level of the surface passes the declared per-level token magnitude.
pub fn codec_surface(content: &CodecContent) -> Result<GeneratedTree, Overflow> {
    let shape = &content.shape;
    let reads = content.direction.reads();
    let mut tokens: Vec<GeneratedToken> = Vec::new();
    if reads {
        tokens.extend(refusal_declaration(shape)?);
        tokens.extend(refusal_conversion(shape)?);
    }
    let mut inherent: Vec<GeneratedToken> = Vec::new();
    if content.direction.writes() {
        inherent.extend(encode_road(shape)?);
    }
    if reads {
        inherent.extend(decode_road(shape)?);
    }
    tokens.push(GeneratedToken::word("impl"));
    tokens.extend(type_path(shape.owner()));
    tokens.push(group(GeneratedDelimiter::Brace, inherent)?);
    let placed = match &content.placement {
        CodecPlacement::AtDeclarationSite => tokens,
        CodecPlacement::PublishedModule { spelling } => {
            published_module(spelling.spelling(), tokens)?
        }
    };
    GeneratedTree::assembled(placed)
}
