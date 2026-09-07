//! One ordered reading of the facts required for no-mutation parity.

use super::ParityQualificationRefusal;

pub(super) fn first_refusal(
    production: Result<(), ()>,
    evaluation: Result<(), ()>,
    firings: u32,
    equivalence: Result<(), ()>,
) -> Option<ParityQualificationRefusal> {
    if production.is_err() {
        Some(ParityQualificationRefusal::ProductionDidNotQualify)
    } else if evaluation.is_err() {
        Some(ParityQualificationRefusal::EvaluationDidNotQualify)
    } else if firings != 0 {
        Some(ParityQualificationRefusal::NoMutationActivated { firings })
    } else if equivalence.is_err() {
        Some(ParityQualificationRefusal::MeaningsDisagreed)
    } else {
        None
    }
}
