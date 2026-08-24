//! Converting one compiler token stream into a captured declaration, issuing one span handle per token.

use super::types::{CaptureError, Spans};
use crate::token::{
    CaptureBound, CaptureWalk, CapturedDelimiter, CapturedInput, CapturedPayload,
    CapturedTokenTree, TokenPath, capture_literal,
};
use proc_macro::{Delimiter, TokenStream, TokenTree};

/// Capture one declared input, issuing one handle per token into the table beside it.
///
/// The table is the caller's, because a refusal names a handle in it: one consumed by the road that refused could not resolve the very handle the refusal carries.
/// The walk's budget is this call's, so a caller capturing two streams into one table stands each of them under the declared magnitudes on its own.
///
/// # Errors
///
/// Returns [`CaptureError::Unbounded`] where the read runs past one of the declared magnitudes, and [`CaptureError::Unread`] where a literal is written in a form this grammar does not read.
pub fn capture(stream: TokenStream, spans: &mut Spans) -> Result<CapturedInput, CaptureError> {
    let mut walk = CaptureWalk::declared();
    let trees = capture_stream(stream, &TokenPath::root(), &mut walk, spans)?;
    let issued = spans.issued()?;
    CapturedInput::taken(trees, issued).map_err(CaptureError::from)
}

/// Convert one token stream, at the route it sits at.
///
/// Each token's own route is that route stepped by the token's position, so this host and the compiler's own text reader build routes the same way and spend the same declared walk.
fn capture_stream(
    stream: TokenStream,
    path: &TokenPath,
    walk: &mut CaptureWalk,
    spans: &mut Spans,
) -> Result<Vec<CapturedTokenTree>, CaptureError> {
    let mut captured = Vec::new();
    for (position, tree) in stream.into_iter().enumerate() {
        walk.examined()?;
        walk.took()?;
        let index = u32::try_from(position).map_err(|_| CaptureBound::Level)?;
        let stepped = path.stepped(index)?;
        captured.push(capture_tree(&tree, stepped, walk, spans)?);
    }
    Ok(captured)
}

/// Convert one token tree, issuing its handle first so handle order matches reading order.
fn capture_tree(
    tree: &TokenTree,
    path: TokenPath,
    walk: &mut CaptureWalk,
    spans: &mut Spans,
) -> Result<CapturedTokenTree, CaptureError> {
    let span = spans.issue(tree.span())?;
    let payload = match tree {
        TokenTree::Ident(word) => CapturedPayload::Word(word.to_string()),
        TokenTree::Punct(punct) => CapturedPayload::Punct(punct.as_char()),
        TokenTree::Literal(literal) => capture_literal(&literal.to_string())
            .map_err(|cause| CaptureError::Unread { cause, at: span })?,
        TokenTree::Group(group) => {
            let inner = capture_stream(group.stream(), &path, walk, spans)?;
            let delimiter = captured_delimiter(group.delimiter());
            return CapturedTokenTree::group_of(delimiter, inner, path, span)
                .map_err(CaptureError::from);
        }
    };
    Ok(CapturedTokenTree::captured(payload, path, span))
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
