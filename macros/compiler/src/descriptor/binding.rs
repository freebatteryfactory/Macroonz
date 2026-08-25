//! Reading one direct adapter binding from authored path tokens.
//!
//! A direct projection compiles where it is declared, so its physical dependency path arrives in that declaration rather than through a carrier matcher.
//! This reading turns the authored `crate_alias::module` shape into the same informed path value every direct renderer consumes.

use super::{CaptureCause, CaptureIssue, DeclarationError, DirectBinding};
use crate::token::{CapturedTokenTree, SpanHandle, rendered_name};

/// Read one non-empty dependency path, with every segment and separator consumed.
pub(crate) fn direct_binding(
    trees: &[&CapturedTokenTree],
) -> Result<DirectBinding, (CaptureIssue, SpanHandle)> {
    let Some(first) = trees.first() else {
        return Err((
            CaptureIssue::Grammar {
                cause: CaptureCause::PathUnread,
            },
            SpanHandle::at(0),
        ));
    };
    let mut segments = Vec::new();
    let mut remaining = trees.iter().copied().peekable();
    while let Some(tree) = remaining.next() {
        let Some(word) = tree.word() else {
            return Err((
                CaptureIssue::Grammar {
                    cause: CaptureCause::PathUnread,
                },
                tree.span(),
            ));
        };
        if !rendered_name(word) {
            return Err((
                CaptureIssue::Vocabulary {
                    refusal: DeclarationError::NotAnIdentifier,
                },
                tree.span(),
            ));
        }
        segments.push(word.to_owned());
        let Some(first_colon) = remaining.next() else {
            break;
        };
        let Some(second_colon) = remaining.next() else {
            return Err((
                CaptureIssue::Grammar {
                    cause: CaptureCause::PathUnread,
                },
                first_colon.span(),
            ));
        };
        if first_colon.punct() != Some(':') || second_colon.punct() != Some(':') {
            return Err((
                CaptureIssue::Grammar {
                    cause: CaptureCause::PathUnread,
                },
                first_colon.span(),
            ));
        }
        if remaining.peek().is_none() {
            return Err((
                CaptureIssue::Grammar {
                    cause: CaptureCause::PathUnread,
                },
                second_colon.span(),
            ));
        }
    }
    DirectBinding::declared(segments).map_err(|refusal| {
        (
            CaptureIssue::Vocabulary { refusal },
            trees.last().map_or(first.span(), |tree| tree.span()),
        )
    })
}
