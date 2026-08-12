//! Seat 03 — judge: the readers that state a verdict over one rendered
//! artifact, and the mutations they are rehearsed against.
//!
//! # What a judge here may read, and what it may not
//!
//! A judge reads an ARTIFACT — the rendered text a service produced — and
//! compares it against a declared order the caller states independently. It
//! never asks the service under judgement what the answer was, because a
//! comparison between a value and itself proves that the value equals itself.
//!
//! # Three lanes, and a verdict belongs to exactly one of them
//!
//! **Lane A — the byte-profile scan.** The readers below find one declared
//! textual form in the rendered text and report what they found. The claim they
//! support is exactly this and no more:
//!
//! > *the rendered text contains this exact declared textual form.*
//!
//! That is a claim about BYTES. It is not a claim about structure, and it never
//! becomes one however many anchors are added: a scan that finds
//! `const SELECTION_ORDER: … = &["A", "B"]` has established that those bytes are
//! present somewhere in the text — not that the artifact declares an
//! implementation, not that the implementation targets the right type, not that
//! the constant is a member of it, and not that a comment did not put the same
//! bytes there. The lane is worth having because it costs a string search and
//! catches a renderer emitting wrong bytes before those bytes reach a compiler.
//! It is worth having *honestly* only if its claim stays this narrow.
//!
//! **Lane B — the structural read** ([`structural`]). The claim lane A cannot
//! support is structural: what item is this, what does the implementation
//! target, which trait does it realize, what are its members, and are the cause
//! rows the declared ones. Answering that means parsing Rust, which the byte
//! scan deliberately does not do — so the lane hands the text to a parser
//! nobody here wrote and reads the tree back. Its dependency is admitted in this
//! package's README, which states what the lane reads, what it refuses to claim,
//! and which producer components it shares nothing with.
//!
//! **Lane C — the compiled behaviour.** `rustc` compiles the artifact and the
//! test reads its trait constants AS VALUES. There the compiler is the
//! independent decoder: it parses by its own rules, with no anchor of ours in
//! the path, and hands back typed values rather than substrings.
//!
//! The LAWFUL artifact's seat is the consumer-fixture parity tests at
//! `xtask/fixtures/macro-consumer` and `xtask/fixtures/renamed-consumer`, which
//! apply the shell's derive in crates owning neither participant and compare the
//! derived `SHAPE`, `SELECTION_ORDER`, and `DECLARED_ORDER` against hand-written
//! twins. The MUTANTS' seats are this package's `tests/compiled_behaviour.rs`: a
//! mutant is this plane's own damage, so no participant is grading itself when
//! the judge hands its own damaged text to a compiler and reads back a refusal
//! to compile and a disagreeing value.
//!
//! No lane subsumes another and none is a weaker version of another. **A verdict
//! is method-specific**, exactly as the machine's evidence law requires:
//! "the permuted rendering was rejected by the byte scan over these two declared
//! orders" and "the derived implementation equals its hand-written twin under
//! compilation" are two claims, each true of its own method and neither standing
//! in for the other. Reporting one as though it came from another is the
//! collapse the whole plane exists to refuse.
//!
//! # The readers are dumb on purpose, and the reason is not "simplicity"
//!
//! A cleverer reader would have to decide what the text MEANS, and the only way
//! to decide that is to implement the same understanding the renderer already
//! has. Two implementations of one understanding, written by the same hands
//! against the same document, agree because they SHARE THE CHALLENGED
//! IMPLEMENTATION — not because either of them understands Rust. Their agreement
//! is therefore correlated evidence, and correlated evidence about a renderer is
//! not independent of that renderer. Lane C escapes this because `rustc` is a
//! decoder nobody here wrote.

pub mod mutation;
pub mod structural;
pub mod types;

pub use mutation::{ARTIFACT_MUTATIONS, ArtifactMutation, LaneOwnership, mutated};
pub use structural::{
    ArtifactStructure, CauseRow, DeclaredStructure, ImplPosture, ImplementationStructure,
    StructuralDisagreement, StructuralVerdict, judge_structure, structure_of,
};
pub use types::RenderVerdict;

/// The `SELECTION_ORDER` opening this reader looks for.
///
/// The anchor is an exact spelling on purpose. Where a lawful artifact stops
/// matching it, the reader reports [`RenderVerdict::Unreadable`] and the anchor
/// is re-stated here, deliberately — never loosened to match whatever arrived.
const SELECTION_ORDER_OPENING: &str = "const SELECTION_ORDER : & 'static [ & 'static str ] = &";

/// The `CauseId` opening this reader looks for. Anchored exactly, for the same
/// reason.
const CAUSE_IDENTITY_OPENING: &str = "CauseId :: declared ( \"";

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
///
/// **The claim this function supports** is lane A's and only lane A's: the
/// rendered text contains these exact declared textual forms. It says nothing
/// about what the artifact declares.
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
