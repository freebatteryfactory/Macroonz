//! The placement operation that wraps a codec surface in one visibly published module.

use crate::bounded::Overflow;
use crate::token::{GeneratedDelimiter, GeneratedToken, documentation, group};

/// The one import a published module's head writes.
///
/// A wrapped surface names the owner's type and every member's type in the scope the module sits in rather than in its own, so the head brings that scope with it.
/// One import and no more: a module that reached further would be deciding what else a caller's generated module can see.
const MODULE_PRELUDE_ROOT: &str = "super";

/// The sentence a published module documents itself with.
const MODULE_SENTENCE: &str = "The canonical encode and decode roads for one declared shape, \
     published here rather than spliced beside the declaration. Its head imports the scope the \
     module sits in, which is where the shape's own names live.";

/// One visibly published module carrying a rendered surface.
///
/// Its head writes the one import a wrapped surface needs, because the shape's own names live in the scope the module sits in.
pub(super) fn published_module(
    spelling: &str,
    surface: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut body = vec![
        GeneratedToken::word("use"),
        GeneratedToken::word(MODULE_PRELUDE_ROOT),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::alone('*'),
        GeneratedToken::alone(';'),
    ];
    body.extend(surface);
    let mut tokens = documentation(MODULE_SENTENCE)?;
    tokens.push(GeneratedToken::word("pub"));
    tokens.push(GeneratedToken::word("mod"));
    tokens.push(GeneratedToken::word(spelling));
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}
