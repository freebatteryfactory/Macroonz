//! The metamorphic laws: permutation insensitivity, run-twice determinism, and
//! ambient-pathway invariance.
//!
//! A metamorphic law needs no oracle for the answer at all. It relates two runs
//! of the subject to each other — the same input rearranged, the same input
//! twice, the same input down two declared pathways — so it holds without
//! anybody knowing what the right answer is. That is what makes this family the
//! escalation for subjects whose outputs nothing can predict.

use super::conclude::agreement;
use super::types::{
    AMBIENT_PATHWAY_DISAGREEMENT, DETERMINISM_DISAGREEMENT, Equivalence,
    PERMUTATION_DISAGREEMENT, Road,
};
use crate::report::TrialConclusion;

/// The permutation-insensitivity law: rearranging the input does not move the
/// answer.
///
/// # Bounds
///
/// The rearrangement is the OWNER's declared permutation, because what counts as
/// a rearrangement is a fact about the input's meaning: reordering a set's
/// members rearranges it, and reordering a sequence's members is a different
/// input. A subject that must be sensitive to order therefore never reaches this
/// law by accident.
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
/// # Authority
///
/// The cheapest ambient-freedom law there is. A subject that reads a clock, an
/// environment, an address, or an unseeded iteration order is a subject whose
/// two runs can differ, and this law is what turns that difference into a
/// finding rather than into a flake somebody reruns until it is green.
///
/// # Nonclaims
///
/// Two runs agreeing is not proof of ambient freedom. A subject reading an
/// ambient fact that did not change between the two runs agrees with itself, and
/// only the population's spread across runs, machines, and targets pushes on
/// that.
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

/// The ambient-pathway-invariance law: one meaning reached down two declared
/// pathways is one answer.
///
/// # Authority
///
/// The claim is about the PATHWAYS, not about the meaning: whichever road an
/// owner's caller takes to the same subject — through a cache and around it,
/// warm and cold, in one process and across a boundary — the answer is the
/// subject's rather than the road's.
///
/// # Nonclaims
///
/// The subject the two pathways reach is what they share, and this law is silent
/// about it. That is why it is not a parity suite: a parity suite's two roads
/// are two separately maintained roads to one meaning, so what they share is
/// stated in full ([`SharedSubstrate`](crate::properties::SharedSubstrate));
/// here the shared thing is the subject itself, which is what the law is about
/// rather than a silence hiding inside it.
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
