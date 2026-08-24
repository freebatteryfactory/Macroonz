//! The declared-algebra laws: roundtrip, idempotence, conservation, and monotonicity.
//!
//! The oracle for each is the declared algebra itself, so no second implementation is needed for the law to be checkable — which is what separates this family from parity, where two roads are the whole point.
//!
//! A law here proves the subject honors the algebra its owner declared, and can never falsify the declaration: a subject whose owner declared the wrong algebra and implemented it faithfully passes everything below.

use super::conclude::{agreement, ranking};
use super::types::{
    CONSERVATION_DISAGREEMENT, Equivalence, IDEMPOTENCE_DISAGREEMENT, MONOTONICITY_DISAGREEMENT,
    Measure, Order, ROUNDTRIP_DISAGREEMENT, Road,
};
use crate::report::TrialConclusion;
use core::cmp::Ordering;

/// The roundtrip law: decoding what was encoded yields the value that was encoded.
///
/// Both roads are total, so a decoder that can refuse is a road whose image is the owner's own outcome type and the pair is judged as a roundtrip over that outcome.
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
/// The comparison is between the first image and the second, never between the input and the image, because a subject that normalizes its input is idempotent without being an identity.
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

/// The conservation law: the quantity read entering the subject is the quantity read leaving it.
///
/// Two readings rather than one, because a domain and an image are two types in the general case; a subject that maps a type to itself passes one reading in both seats.
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
/// The pair is ordered by the declared domain order before the images are read, so every pair a population supplies is exercised rather than only the pairs that happened to arrive in order.
/// Non-strict, and the two orders are separate because a domain and an image are two types in the general case.
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
