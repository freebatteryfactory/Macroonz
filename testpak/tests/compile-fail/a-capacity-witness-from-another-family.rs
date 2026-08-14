//! The reversal for the capacity witness's family seat: one family's runtime
//! capacity is not another's, whatever the two magnitudes are.
//!
//! Both families below declare their magnitude evidence-selected, so the
//! declaration gate is satisfied on both sides and cannot be what stops this.
//! Both would carry the same number. What refuses is the seat: the family rides
//! on the witness's own type parameter, so a capacity admitted for one family
//! does not typecheck where the other's is required.
//!
//! No mint appears anywhere below. That is deliberate rather than a shortcut:
//! the claim under judgement is about the TYPES, and driving it through a
//! constructor would risk the refusal coming from the construction instead. It
//! is also the only road available from outside the crate — `LimitWitness` has
//! no public mint until the schema home carries the real declaration path — and
//! this fixture is written to need none.

use threadpak::types::{
    EvidenceSelectedLimit, EvidenceSelectedMagnitude, Limit, PositiveLimitWitness,
};

/// One family whose magnitude the owner's evidence selects.
struct FirstFamily;

impl Limit for FirstFamily {
    type Authority = EvidenceSelectedMagnitude;
}

impl EvidenceSelectedLimit for FirstFamily {}

/// A second family on the same ladder, so the declaration is not the difference.
struct SecondFamily;

impl Limit for SecondFamily {
    type Authority = EvidenceSelectedMagnitude;
}

impl EvidenceSelectedLimit for SecondFamily {}

/// The consumer, naming exactly which family's capacity it will act on.
fn admits_the_first(_capacity: &PositiveLimitWitness<FirstFamily>) {}

/// The lawful half, and it must stay lawful: the consumer takes its own
/// family's capacity.
fn lawful(capacity: &PositiveLimitWitness<FirstFamily>) {
    admits_the_first(capacity);
}

/// The unlawful half: the other family's capacity handed to the same consumer.
fn crossed(capacity: &PositiveLimitWitness<SecondFamily>) {
    admits_the_first(capacity);
}

fn main() {
    let _ = lawful;
    let _ = crossed;
}
