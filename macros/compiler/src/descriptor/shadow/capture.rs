//! Reading one authored shadow declaration out of a typed token tree.
//!
//! # The authored grammar
//!
//! ```text
//! <helper>! { <name>, <name>, ... }
//! ```
//!
//! Each name is one bare word from the stated roster, the separator is a comma, and a trailing comma is lawful.
//! Order is preserved because it is emission order, and each name may be chosen once: the second choice of one name would emit the same items twice at one site.

use super::{SHADOW_ROSTER, ShadowCaptureError, ShadowRow, Shadows};
use crate::descriptor::{CaptureCause, Grammar};
use crate::token::{CapturedInput, SpanHandle};

/// Read one shadow payload out of the declaration's body.
///
/// # Errors
///
/// Returns [`ShadowCaptureError`] where a choice is not one bare name, where a name is outside the roster or chosen twice — each at its own token — and where the declaration chooses nothing at all.
pub fn chosen(body: &CapturedInput, grammar: Grammar) -> Result<Shadows, ShadowCaptureError> {
    let mut rows: Vec<ShadowRow> = Vec::new();
    let mut expecting_name = true;
    for tree in body.trees() {
        if expecting_name {
            let Some(word) = tree.word() else {
                return Err(refused(grammar, CaptureCause::ChoiceUnread, tree.span()));
            };
            let Some(row) = SHADOW_ROSTER.iter().find(|row| row.name() == word) else {
                return Err(refused(grammar, CaptureCause::NameUnshadowed, tree.span()));
            };
            if rows.iter().any(|held| held.name() == word) {
                return Err(refused(grammar, CaptureCause::ChoiceDoubled, tree.span()));
            }
            rows.push(*row);
            expecting_name = false;
        } else if tree.punct() == Some(',') {
            expecting_name = true;
        } else {
            return Err(refused(grammar, CaptureCause::ChoiceUnread, tree.span()));
        }
    }
    if rows.is_empty() {
        return Err(refused(
            grammar,
            CaptureCause::NothingChosen,
            SpanHandle::at(0),
        ));
    }
    Ok(Shadows::declared(rows))
}

/// One established grammar refusal at one token.
const fn refused(grammar: Grammar, cause: CaptureCause, at: SpanHandle) -> ShadowCaptureError {
    ShadowCaptureError::grammar_refused(grammar, cause, at)
}
