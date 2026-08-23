//! The human rendering of one typed answer.
//!
//! There is no seat for a caller's sentence: the rendering is a function of the
//! answer, so "the rendering agrees with the answer" is not a check that
//! passed, it is a disagreement that cannot be written down.
//! Nothing is stored either — the line is composed every time it is asked,
//! because a stored projection is a second value that can disagree with the one
//! it was projected from.
//!
//! Every line below is a static literal proven to fit its limit family at
//! compile time, so this module carries no refusal road and no truncation.
//! The typed content stays in the answer, where a reader that needs the exact
//! identities reads it; the line says which question was answered and in what
//! terms.

use super::ExplanationAnswer;
use crate::plane::{HumanProjection, HumanTextLimit, human_projection};

/// The rendering one typed answer projects for a person.
///
/// Total and exhaustive: every answer variant renders, so an answer that could
/// not be shown is not a value that exists.
pub(super) fn human_line(answer: &ExplanationAnswer) -> HumanProjection<HumanTextLimit> {
    match answer {
        ExplanationAnswer::Kind { .. } => {
            human_projection!(HumanTextLimit, "the projection kind this output is")
        }
        ExplanationAnswer::Owner { .. } => {
            human_projection!(HumanTextLimit, "the owner fact that required this output")
        }
        ExplanationAnswer::CausingDeclarations { .. } => human_projection!(
            HumanTextLimit,
            "the declarations this output was derived from"
        ),
        ExplanationAnswer::PatternInstance { .. } => human_projection!(
            HumanTextLimit,
            "the authored pattern and the instantiation of it that produced this output"
        ),
        ExplanationAnswer::Profile { .. } => {
            human_projection!(HumanTextLimit, "the projection profile at its version")
        }
        ExplanationAnswer::AssumptionsAndSpecializations { .. } => human_projection!(
            HumanTextLimit,
            "the owner facts this output rests on as assumptions"
        ),
        ExplanationAnswer::OutputAndDigest { .. } => human_projection!(
            HumanTextLimit,
            "the planned member, and the digest the closure proved over its rendered bytes"
        ),
        ExplanationAnswer::ChallengingTests { .. } => {
            human_projection!(HumanTextLimit, "the tests that challenge this output")
        }
        ExplanationAnswer::MeasuringBenchmarks { .. } => {
            human_projection!(HumanTextLimit, "the benchmarks that measure this output")
        }
        ExplanationAnswer::Invalidators { .. } => human_projection!(
            HumanTextLimit,
            "the watched identities whose change makes this output stale"
        ),
        ExplanationAnswer::RelatedProjectionDisposition { .. } => human_projection!(
            HumanTextLimit,
            "what happened to a related projection, and why"
        ),
        ExplanationAnswer::Repairs { .. } => {
            human_projection!(HumanTextLimit, "the owner-declared repairs that apply")
        }
    }
}
