//! The judge home's public types: what one reading of one rendered artifact
//! concluded.

/// What a judge found when it read one rendered projection against an
/// independently declared order.
///
/// Three answers, and none of them is silence: a reading that found nothing to
/// compare says so rather than passing.
///
/// # `Unreadable` is a failure class with its own alarm
///
/// [`RenderVerdict::Unreadable`] is not noise, not a skip, and not a softer
/// [`RenderVerdict::Deviates`]. It is its own failure class and it means one
/// specific thing: **the judge could not find the construct it anchors on.**
/// Either the artifact stopped stating that construct, or the artifact still
/// states it and the anchor no longer matches the text — a renamed constant, a
/// reformatted literal, a moved attribute.
///
/// Both of those are real findings, so a test asserting a lawful rendering
/// conforms MUST fail on `Unreadable`, and must never be written to accept it
/// alongside `Conforms`. A silent `Unreadable` is worse than a deviation: a
/// deviation says the renderer is wrong, while an ignored `Unreadable` says
/// nothing at all while every downstream assertion quietly stops testing
/// anything.
///
/// **The response to a false alarm is to fix the anchor deliberately, never to
/// loosen the reader.** When the artifact legitimately changes shape, the
/// anchor is re-stated to match the new shape, in one place, on purpose, and
/// the change is visible in the diff. Widening the reader until it matches
/// again — trimming whitespace, matching a prefix, falling back to a looser
/// pattern — buys a green run by making the judge cleverer, and a clever judge
/// starts agreeing with the renderer about what the text means, which is the
/// failure mode this whole package exists to avoid.
///
/// The reader is rehearsed against that alarm rather than trusted to raise it:
/// `testpak/tests/planted_defect.rs` shifts whitespace inside a lawful
/// rendering and requires `Unreadable`, so the alarm is known to sound before
/// anyone has to interpret one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderVerdict {
    /// The rendering states exactly the declared order, spelling for spelling
    /// and identity for identity.
    Conforms,
    /// The rendering and the declared order disagree.
    Deviates,
    /// No projection could be read out of the rendering at all. A failure class
    /// of its own — see the type's documentation.
    Unreadable,
}
