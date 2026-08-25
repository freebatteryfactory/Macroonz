//! Converting one compiler token stream into a captured declaration, issuing one span handle per token.

use super::types::{CaptureError, Spans};
use crate::token::{
    CaptureBuildRefusal, CaptureLevel, CapturedAtom, CapturedDelimiter, CapturedInput,
    LiteralReadCause, capture_literal,
};
use proc_macro::{Delimiter, Span, TokenStream, TokenTree};

/// Capture one declared input, issuing one handle per token into the table beside it.
///
/// The table is the caller's, because a refusal names a handle in it: one consumed by the road that refused could not resolve the very handle the refusal carries.
/// The walk's budget is this call's, so a caller capturing two streams into one table stands each of them under the declared magnitudes on its own.
///
/// # Errors
///
/// Returns [`CaptureError::Unbounded`] where the read runs past one of the declared magnitudes, and [`CaptureError::Unread`] where a literal is written in a form this grammar does not read.
pub fn capture(stream: TokenStream, spans: &mut Spans) -> Result<CapturedInput, CaptureError> {
    let level = spans.builder().open();
    let level = capture_stream(stream, level).map_err(capture_refusal)?;
    Ok(level.finish())
}

/// Convert one token stream through the builder level that owns its paths and handles.
fn capture_stream(
    stream: TokenStream,
    mut level: CaptureLevel<'_, Span>,
) -> Result<CaptureLevel<'_, Span>, CaptureBuildRefusal<Span, LiteralReadCause>> {
    for tree in stream {
        let position = tree.span();
        level = match tree {
            TokenTree::Ident(word) => level.atom(position, |_| {
                Ok::<_, LiteralReadCause>(CapturedAtom::Word(word.to_string()))
            })?,
            TokenTree::Punct(punct) => level.atom(position, |_| {
                Ok::<_, LiteralReadCause>(CapturedAtom::Punct(punct.as_char()))
            })?,
            TokenTree::Literal(literal) => {
                level.atom(position, |_| capture_literal(&literal.to_string()))?
            }
            TokenTree::Group(group) => level.group(
                position,
                captured_delimiter(group.delimiter()),
                |_span, inner| capture_stream(group.stream(), inner),
            )?,
        };
    }
    Ok(level)
}

/// Lower the checked builder's two refusal seats into this host's capture refusal.
const fn capture_refusal(refusal: CaptureBuildRefusal<Span, LiteralReadCause>) -> CaptureError {
    match refusal {
        CaptureBuildRefusal::Unbounded { bound, at: _ } => CaptureError::Unbounded { bound },
        CaptureBuildRefusal::ProducerRefused { cause, at } => CaptureError::Unread { cause, at },
    }
}

/// The captured delimiter one compiler delimiter names.
///
/// A group the compiler wrote no delimiter around is a real group and captures as one; flattening it away would join two fragments into a declaration nobody wrote.
const fn captured_delimiter(delimiter: Delimiter) -> CapturedDelimiter {
    match delimiter {
        Delimiter::Parenthesis => CapturedDelimiter::Parenthesis,
        Delimiter::Brace => CapturedDelimiter::Brace,
        Delimiter::Bracket => CapturedDelimiter::Bracket,
        Delimiter::None => CapturedDelimiter::Bare,
    }
}
