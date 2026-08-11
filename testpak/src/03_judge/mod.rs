//! Seat 03 — judge: the readers that state a verdict over one rendered
//! artifact.
//!
//! # What a judge here may read, and what it may not
//!
//! A judge reads an ARTIFACT — the rendered source text a service produced —
//! and compares it against a declared order the caller states independently. It
//! never asks the service under judgement what the answer was, because a
//! comparison between a value and itself proves that the value equals itself.
//!
//! # Two lanes, and a verdict belongs to exactly one of them
//!
//! **The fast lane is a string scan, and it is deliberately dumb.** The readers
//! below find one declared construct in the text and report what they found.
//! They read text `rustc` never touched, which is precisely their value: they
//! catch a renderer that emits the wrong bytes before those bytes are ever
//! offered to a compiler, and they cost a string search. A cleverer reader
//! would start agreeing with the renderer about what the text means, so the
//! dumbness is the design and not a stage on the way to something better.
//!
//! **The authoritative lane compiles the artifact and reads its trait constants
//! as values.** There, `rustc` is the independent decoder: it parses the
//! rendered source by its own rules, with no anchor of ours anywhere in the
//! path, and hands back typed values rather than substrings. That lane is the
//! consumer-fixture parity tests at `xtask/fixtures/macro-consumer`, which
//! apply the shell's derive in a crate owning neither participant and compare
//! the derived `SHAPE`, `SELECTION_ORDER`, and `DECLARED_ORDER` against a
//! hand-written twin, value for value.
//!
//! Neither lane subsumes the other and neither is a weaker version of the
//! other. **A verdict is method-specific**, exactly as the machine's evidence
//! law requires: "the permuted rendering was rejected by the string scan over
//! these two declared orders" and "the derived implementation equals its
//! hand-written twin under compilation" are two claims, each true of its own
//! method and neither standing in for the other. Reporting a fast-lane verdict
//! as if it came from the authoritative lane — or the reverse — is the
//! collapse the whole plane exists to refuse.

pub mod types;

pub use types::RenderVerdict;

/// The `SELECTION_ORDER` opening this reader looks for.
///
/// The anchor is an exact spelling on purpose. Where a lawful artifact stops
/// matching it, the reader reports [`RenderVerdict::Unreadable`] and the anchor
/// is re-stated here, deliberately — never loosened to match whatever arrived.
const SELECTION_ORDER_OPENING: &str = "const SELECTION_ORDER: &'static [&'static str] = &[";

/// The `CauseId` opening this reader looks for. Anchored exactly, for the same
/// reason.
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
///
/// A rendering the reader cannot anchor in returns [`RenderVerdict::Unreadable`]
/// rather than a verdict about content it never read. Callers assert on the
/// exact verdict they expect; treating `Unreadable` as an acceptable stand-in
/// for `Conforms` disarms every assertion downstream of it.
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
