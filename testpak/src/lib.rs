//! `threadpak-testpak`: the qualification plane's judges.
//!
//! # What a judge here may read, and what it may not
//!
//! A judge reads an ARTIFACT — the rendered source text a service produced —
//! and compares it against a declared order the caller states independently. It
//! never asks the service under judgement what the answer was, because a
//! comparison between a value and itself proves that the value equals itself.
//!
//! The readers below are deliberately dumb: they find one declared construct in
//! the text and report what they found. A cleverer reader would start agreeing
//! with the renderer about what the text means, which is the failure mode this
//! whole package exists to avoid.
//!
//! # Verdicts are claim-specific
//!
//! [`RenderVerdict`] says what happened to one rendering under one comparison.
//! It is not a grade, not a score, and not a statement about a service.

/// What a judge found when it read one rendered projection against an
/// independently declared order.
///
/// Three answers, and none of them is silence: a reading that found nothing to
/// compare says so rather than passing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderVerdict {
    /// The rendering states exactly the declared order, spelling for spelling
    /// and identity for identity.
    Conforms,
    /// The rendering and the declared order disagree.
    Deviates,
    /// No projection could be read out of the rendering at all.
    Unreadable,
}

/// The `SELECTION_ORDER` opening this reader looks for.
const SELECTION_ORDER_OPENING: &str = "const SELECTION_ORDER: &'static [&'static str] = &[";

/// The `CauseId` opening this reader looks for.
const CAUSE_IDENTITY_OPENING: &str = "CauseId::declared(\"";

/// The textual selection order one rendering states, or `None` where the
/// rendering states none.
#[must_use]
pub fn selection_order_in(rendered: &str) -> Option<Vec<String>> {
    let start = rendered
        .find(SELECTION_ORDER_OPENING)?
        .checked_add(SELECTION_ORDER_OPENING.len())?;
    let tail = rendered.get(start..)?;
    let end = tail.find(']')?;
    let inner = tail.get(..end)?;
    Some(inner.split(',').filter_map(unquoted).collect())
}

/// The stable cause identities one rendering states, in the order it states
/// them.
#[must_use]
pub fn cause_identities_in(rendered: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    let mut rest = rendered;
    while let Some(at) = rest.find(CAUSE_IDENTITY_OPENING) {
        let Some(after) = at
            .checked_add(CAUSE_IDENTITY_OPENING.len())
            .and_then(|from| rest.get(from..))
        else {
            break;
        };
        let Some(end) = after.find('"') else {
            break;
        };
        let Some(identity) = after.get(..end) else {
            break;
        };
        found.push(identity.to_owned());
        rest = after.get(end..).unwrap_or_default();
    }
    found
}

/// Judge one rendering against an independently declared order.
///
/// The caller states the spellings and the identities it expects. Both must
/// agree, in the same positions, for the rendering to conform: a rendering that
/// keeps the spellings and recycles an identity is as wrong as one that permutes
/// the spellings, and this judge catches either.
#[must_use]
pub fn judge_declared_order(
    rendered: &str,
    declared_spellings: &[&str],
    declared_identities: &[&str],
) -> RenderVerdict {
    let Some(spellings) = selection_order_in(rendered) else {
        return RenderVerdict::Unreadable;
    };
    let identities = cause_identities_in(rendered);
    let same_magnitude = spellings.len() == declared_spellings.len()
        && identities.len() == declared_identities.len();
    let same_spellings = spellings
        .iter()
        .zip(declared_spellings.iter())
        .all(|(read, declared)| read == declared);
    let same_identities = identities
        .iter()
        .zip(declared_identities.iter())
        .all(|(read, declared)| read == declared);
    if same_magnitude && same_spellings && same_identities {
        RenderVerdict::Conforms
    } else {
        RenderVerdict::Deviates
    }
}

/// One quoted item of a rendered string list, without its quotes.
fn unquoted(item: &str) -> Option<String> {
    let trimmed = item.trim();
    let inner = trimmed.strip_prefix('"')?.strip_suffix('"')?;
    Some(inner.to_owned())
}
