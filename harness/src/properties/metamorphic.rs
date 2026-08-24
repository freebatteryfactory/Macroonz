//! The metamorphic laws: permutation insensitivity, run-twice determinism, and ambient-pathway invariance.
//!
//! A metamorphic law needs no oracle for the answer at all.
//! It relates two runs of the subject to each other — the same input rearranged, the same input twice, the same input down two declared pathways — so it holds without anybody knowing what the right answer is.
//! That is what makes this family the escalation for subjects whose outputs nothing can predict.

use super::conclude::agreement;
use super::types::{
    AMBIENT_PATHWAY_DISAGREEMENT, DETERMINISM_DISAGREEMENT, Equivalence, PERMUTATION_DISAGREEMENT,
    Road,
};
use crate::report::TrialConclusion;

/// The permutation-insensitivity law: rearranging the input does not move the answer.
///
/// The rearrangement is the owner's declared permutation, because what counts as one is a fact about the input's meaning: reordering a set's members rearranges it, and reordering a sequence's members is a different input.
#[must_use]
#[track_caller]
pub fn permutation_insensitivity<Domain, Image>(
    subject: Road<Domain, Image>,
    permute: Road<Domain, Domain>,
    same: Equivalence<Image>,
    value: &Domain,
) -> TrialConclusion {
    let straight = subject(value);
    let rearranged = subject(&permute(value));
    agreement(same, &straight, &rearranged, PERMUTATION_DISAGREEMENT)
}

/// The determinism law: one input, run twice, gives one answer.
///
/// The cheapest ambient-freedom law there is: a subject that reads a clock, an environment, an address, or an unseeded iteration order is one whose two runs can differ, and this turns that difference into a finding rather than a flake somebody reruns until it is green.
/// Two runs agreeing is not proof of ambient freedom, because a subject reading an ambient fact that did not change between them agrees with itself.
#[must_use]
#[track_caller]
pub fn determinism_run_twice<Domain, Image>(
    subject: Road<Domain, Image>,
    same: Equivalence<Image>,
    value: &Domain,
) -> TrialConclusion {
    let first = subject(value);
    let second = subject(value);
    agreement(same, &first, &second, DETERMINISM_DISAGREEMENT)
}

/// The ambient-pathway-invariance law: one meaning reached down two declared pathways is one answer.
///
/// The claim is about the pathways, not about the meaning: whichever road a caller takes to the same subject — through a cache and around it, warm and cold, in one process and across a boundary — the answer is the subject's rather than the road's.
/// It is not a parity suite, because what these two roads share is the subject the law is about rather than a silence hiding inside it.
#[must_use]
#[track_caller]
pub fn ambient_pathway_invariance<Domain, Image>(
    one_pathway: Road<Domain, Image>,
    another_pathway: Road<Domain, Image>,
    same: Equivalence<Image>,
    value: &Domain,
) -> TrialConclusion {
    let one = one_pathway(value);
    let another = another_pathway(value);
    agreement(same, &one, &another, AMBIENT_PATHWAY_DISAGREEMENT)
}
