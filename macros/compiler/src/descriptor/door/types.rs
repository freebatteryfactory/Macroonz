//! The door home's declarations: the facts an absent axis cites, stated once beside the roads that cite them.

use crate::identity::OwnerFact;

/// The fact a road cites where an axis is empty because this attribute carries one reading and no other.
pub const SOLE_READING_FACT: OwnerFact = OwnerFact {
    home: "descriptor",
    name: "a-generic-attribute-carries-its-own-reading-alone",
};

/// The fact a road cites where the bench axis is empty because a trials-form carrier has no bench seat.
pub const TRIALS_FORM_FACT: OwnerFact = OwnerFact {
    home: "descriptor",
    name: "a-trials-form-carrier-has-no-bench-seat",
};

/// The fact a road cites where the deferred axis is empty because a bench-form carrier has no deferred seat.
pub const BENCH_FORM_FACT: OwnerFact = OwnerFact {
    home: "descriptor",
    name: "a-bench-form-carrier-has-no-deferred-seat",
};
