//! Lane A — the byte-profile scan: one declared textual form, found in the
//! rendered text and reported as found.
//!
//! # The claim, and its exact edge
//!
//! The readers below find one declared textual form in the rendered text and
//! report what they found. The claim they support is exactly this and no more:
//!
//! > *the rendered text contains this exact declared textual form.*
//!
//! That is a claim about BYTES. It is not a claim about structure, and it never
//! becomes one however many anchors are added: a scan that finds
//! `const SELECTION_ORDER: … = &["A", "B"]` has established that those bytes are
//! present somewhere in the text — not that the artifact declares an
//! implementation, not that the implementation targets the right type, not that
//! the constant is a member of it, and not that a comment did not put the same
//! bytes there. Every one of those questions is lane B's, in `structural.rs`,
//! and it answers them by parsing rather than by scanning harder.
//!
//! The lane is worth having because it costs a string search and catches a
//! renderer emitting wrong bytes before those bytes reach a compiler. It is
//! worth having *honestly* only if its claim stays this narrow.
//!
//! # The anchors are exact, and a lost anchor is an alarm
//!
//! Both anchors below are exact spellings on purpose. Where a lawful artifact
//! stops matching one, the reader reports [`RenderVerdict::Unreadable`] and the
//! anchor is re-stated here, deliberately — never loosened to match whatever
//! arrived. The verdict type carries the full reasoning, and
//! `tests/planted_defect.rs` rehearses the alarm rather than trusting it to
//! sound.

use super::types::RenderVerdict;

/// The `SELECTION_ORDER` opening this reader looks for.
///
/// The anchor is an exact spelling on purpose. Where a lawful artifact stops
/// matching it, the reader reports [`RenderVerdict::Unreadable`] and the anchor
/// is re-stated here, deliberately — never loosened to match whatever arrived.
const SELECTION_ORDER_OPENING: &str = "const SELECTION_ORDER : & 'static [ & 'static str ] = &";

/// The family seat's opening this reader looks for. Anchored exactly, for the
/// same reason.
///
/// A cause identity is a PAIR in the artifact, so the scan reads a pair. It
/// joins nothing: a reader that composed `<family>.<local>` and compared the
/// join against one declared string would be re-deriving the artifact's own
/// grammar, and a judge that re-derives the producer's grammar has started
/// agreeing with it.
const CAUSE_FAMILY_OPENING: &str = "RefusalFamilyId :: declared ( \"";

/// The local seat's opening, read after each family seat.
const CAUSE_LOCAL_OPENING: &str = "LocalCauseKey :: declared ( \"";

/// The textual selection order one rendering states, or `None` where the
/// rendering states none.
#[must_use]
pub fn selection_order_in(rendered: &str) -> Option<Vec<String>> {
    let start = rendered
        .find(SELECTION_ORDER_OPENING)?
        .checked_add(SELECTION_ORDER_OPENING.len())?;
    let tail = rendered.get(start..)?;
    let open = tail.find('[')?;
    let end = tail.find(']')?;
    let inner = tail.get(open.checked_add(1)?..end)?;
    Some(inner.split(',').filter_map(unquoted).collect())
}

/// The stable cause identities one rendering states, as the `(family, local)`
/// pairs the artifact spells, in the order it states them.
///
/// Each family seat is read, then the local seat that follows it. A rendering
/// that emitted a family with no local seat after it yields no pair, and the
/// magnitude check downstream reports the shortfall rather than a reader
/// inventing a half-read row.
#[must_use]
pub fn cause_identities_in(rendered: &str) -> Vec<(String, String)> {
    let mut found: Vec<(String, String)> = Vec::new();
    let mut rest = rendered;
    while let Some(at) = rest.find(CAUSE_FAMILY_OPENING) {
        let Some(after_family) = at
            .checked_add(CAUSE_FAMILY_OPENING.len())
            .and_then(|from| rest.get(from..))
        else {
            break;
        };
        let Some((family, after_family_literal)) = quoted_prefix(after_family) else {
            break;
        };
        let Some(local_at) = after_family_literal.find(CAUSE_LOCAL_OPENING) else {
            break;
        };
        let Some(after_local) = local_at
            .checked_add(CAUSE_LOCAL_OPENING.len())
            .and_then(|from| after_family_literal.get(from..))
        else {
            break;
        };
        let Some((local, tail)) = quoted_prefix(after_local) else {
            break;
        };
        found.push((family, local));
        rest = tail;
    }
    found
}

/// The literal a reader is standing inside, and the text after its closing
/// quote.
fn quoted_prefix(after_opening: &str) -> Option<(String, &str)> {
    let end = after_opening.find('"')?;
    let literal = after_opening.get(..end)?.to_owned();
    Some((literal, after_opening.get(end..)?))
}

/// Judge one rendering against an independently declared order.
///
/// The caller states the spellings and the identities it expects, and it states
/// each identity as the `(family, local)` pair the artifact spells rather than
/// as a joined name — so the caller and the reader agree about the shape without
/// either of them composing it. All of it must agree, in the same positions, for
/// the rendering to conform: a rendering that keeps the spellings and recycles
/// an identity is as wrong as one that permutes the spellings, and this judge
/// catches either.
///
/// A rendering the reader cannot anchor in returns [`RenderVerdict::Unreadable`]
/// rather than a verdict about content it never read. Callers assert on the
/// exact verdict they expect; treating `Unreadable` as an acceptable stand-in
/// for `Conforms` disarms every assertion downstream of it.
///
/// **The claim this function supports** is lane A's and only lane A's: the
/// rendered text contains these exact declared textual forms. It says nothing
/// about what the artifact declares.
pub fn judge_declared_order(
    rendered: &str,
    declared_spellings: &[&str],
    declared_identities: &[(&str, &str)],
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
    let same_identities = identities.iter().zip(declared_identities.iter()).all(
        |((read_family, read_local), (declared_family, declared_local))| {
            read_family == declared_family && read_local == declared_local
        },
    );
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
