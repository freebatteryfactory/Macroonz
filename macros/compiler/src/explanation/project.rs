//! The line one typed answer shows a person.
//!
//! There is no seat for a caller's sentence: the line is a function of the answer, so "the rendering agrees with the answer" is not a check that passed — it is a disagreement that cannot be written down.
//! Nothing is stored either, because a stored projection is a second value that can drift from the one it was projected from.
//! The typed content stays in the answer, where a reader that needs the exact identities reads it; the line says which question was answered and in what terms.

use super::UniversalAnswer;

/// The rendering one typed answer projects.
///
/// Total: every arm renders, so an answer that could not be shown is not a value that exists.
pub(super) fn human_line(answer: &UniversalAnswer) -> String {
    match answer {
        UniversalAnswer::Kind { .. } => "the kind this output is",
        UniversalAnswer::Owner { .. } => "the owner fact that required this output",
        UniversalAnswer::CausingDeclarations { .. } => {
            "the declarations this output was derived from"
        }
        UniversalAnswer::Profile { .. } => "the profile this output was decided under",
        UniversalAnswer::OutputAndDigest { .. } => {
            "the planned member, and the digest proved over its rendered bytes"
        }
        UniversalAnswer::Assumptions { .. } => "the owner facts this output rests on",
        UniversalAnswer::Invalidators { .. } => {
            "the watched things whose change makes this output stale"
        }
        UniversalAnswer::RelatedDispositions { .. } => {
            "what happened to every related kind, and why"
        }
        UniversalAnswer::Repairs { .. } => "the owner-declared repairs that apply",
    }
    .to_owned()
}
