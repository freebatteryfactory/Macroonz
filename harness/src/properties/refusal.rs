//! Fail-closed behavior and its lawful twin.
//!
//! These are check adapters rather than algebraic laws: each wraps an owner's subject, reads what it did through the owner's own declared reading, and concludes on whether the answer was refusal-shaped where a refusal was owed.
//!
//! [`fail_closed`] and [`admits_lawful`] are a pair, and a hostile battery that ships without the second one passes for the wrong reason: a subject that silently stopped existing refuses everything, hostile and lawful alike, and every refusal row in it goes green.

use super::conclude::concluded;
use super::types::{
    FAIL_CLOSED_ANSWERED, Holding, LAWFUL_TWIN_REFUSED, PoisonResponse, ResponseReading, Road,
};
use crate::report::{FailureClass, TrialConclusion};

/// The fail-closed law: material the subject was supposed to refuse comes back as a refusal.
///
/// Poison in, refusal out — never a default, never a fallback, never a value somebody chose to stand in for the answer nobody could compute.
#[must_use]
#[track_caller]
pub fn fail_closed<Poison, Response>(
    subject: Road<Poison, Response>,
    reading: ResponseReading<Response>,
    poison: &Poison,
) -> TrialConclusion {
    let holding = match reading(&subject(poison)) {
        PoisonResponse::Refused => Holding::Holds,
        PoisonResponse::Answered => Holding::Fails,
    };
    concluded(holding, FailureClass::RefusedByCheck, FAIL_CLOSED_ANSWERED)
}

/// The lawful twin of [`fail_closed`]: material the subject was supposed to admit comes back as an answer.
///
/// The acceptance half of a hostile battery, and the reason a refusal row in it means anything.
#[must_use]
#[track_caller]
pub fn admits_lawful<Lawful, Response>(
    subject: Road<Lawful, Response>,
    reading: ResponseReading<Response>,
    lawful: &Lawful,
) -> TrialConclusion {
    let holding = match reading(&subject(lawful)) {
        PoisonResponse::Answered => Holding::Holds,
        PoisonResponse::Refused => Holding::Fails,
    };
    concluded(holding, FailureClass::RefusedByCheck, LAWFUL_TWIN_REFUSED)
}
