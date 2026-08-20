//! The declared-algebra laws: roundtrip, idempotence, conservation, and
//! monotonicity.
//!
//! The oracle for each of these is the declared algebra itself, so no second
//! implementation is needed for the law to be checkable — which is what
//! separates this family from the parity suites, where two roads are the whole
//! point.
//!
//! # What a declared-algebra law proves
//!
//! That the subject HONORS the law its owner declared. It can never falsify the
//! declaration: a subject whose owner declared the wrong algebra and implemented
//! it faithfully passes every law here, and passing says exactly that much.
//!
//! # The comparison
//!
//! Every law takes the owner's own equivalence or order. Nothing here demands a
//! trait of a subject type, so a product type is judged without ever growing a
//! derive to be judged by.

use super::conclude::{agreement, ranking};
use super::types::{
    CONSERVATION_DISAGREEMENT, Equivalence, IDEMPOTENCE_DISAGREEMENT, MONOTONICITY_DISAGREEMENT,
    Measure, Order, ROUNDTRIP_DISAGREEMENT, Road,
};
use crate::report::TrialConclusion;
use core::cmp::Ordering;

/// The roundtrip law: decoding what was encoded yields the value that was
/// encoded.
///
/// # Bounds
///
/// Both roads are total. A decoder that can refuse is a road whose image is the
/// owner's own outcome type, and the pair is judged as a roundtrip over that
/// outcome — never by this law quietly reading a refusal as a value.
#[must_use]
#[track_caller]
pub fn roundtrip<Value, Encoded>(
    encode: Road<Value, Encoded>,
    decode: Road<Encoded, Value>,
    same: Equivalence<Value>,
    value: &Value,
) -> TrialConclusion {
    let restored = decode(&encode(value));
    agreement(same, value, &restored, ROUNDTRIP_DISAGREEMENT)
}

/// The idempotence law: applying the subject to its own image changes nothing.
///
/// The comparison is between the first image and the second, never between the
/// input and the image: a subject that normalizes its input is idempotent
/// without being an identity, and demanding otherwise would refuse every
/// normalizer.
#[must_use]
#[track_caller]
pub fn idempotence<Value>(
    subject: Road<Value, Value>,
    same: Equivalence<Value>,
    value: &Value,
) -> TrialConclusion {
    let once = subject(value);
    let twice = subject(&once);
    agreement(same, &once, &twice, IDEMPOTENCE_DISAGREEMENT)
}

/// The conservation law: the quantity read entering the subject is the quantity
/// read leaving it.
///
/// # Bounds
///
/// Two readings rather than one, because a transformation's domain and image are
/// two types in the general case. A subject that maps a type to itself passes
/// one reading in both seats, and the law is then the familiar one.
#[must_use]
#[track_caller]
pub fn conservation<Domain, Image, Quantity>(
    subject: Road<Domain, Image>,
    entering: Measure<Domain, Quantity>,
    leaving: Measure<Image, Quantity>,
    same: Equivalence<Quantity>,
    value: &Domain,
) -> TrialConclusion {
    let before = entering(value);
    let after = leaving(&subject(value));
    agreement(same, &before, &after, CONSERVATION_DISAGREEMENT)
}

/// The monotonicity law: ordering the inputs orders the images the same way.
///
/// # Authority
///
/// The pair is ORDERED by the declared domain order before the images are read,
/// so every pair a population supplies is exercised. A law that only judged
/// pairs that happened to arrive in order would pass more often the less its
/// population knew, which is a coverage hole wearing a green light.
///
/// # Bounds
///
/// Non-strict, and the two orders are separate because a subject's domain and
/// image are two types in the general case: the law demands that the lower
/// input's image does not rank above the upper input's, never that it ranks
/// strictly below.
#[must_use]
#[track_caller]
pub fn monotonicity<Domain, Image>(
    subject: Road<Domain, Image>,
    domain_order: Order<Domain>,
    image_order: Order<Image>,
    left: &Domain,
    right: &Domain,
) -> TrialConclusion {
    let (lower, upper) = match domain_order(left, right) {
        Ordering::Greater => (right, left),
        Ordering::Less | Ordering::Equal => (left, right),
    };
    ranking(
        image_order,
        &subject(lower),
        &subject(upper),
        MONOTONICITY_DISAGREEMENT,
    )
}
