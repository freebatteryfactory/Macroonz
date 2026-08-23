//! The composed-roads suite: the same algebraic shapes, read over a wiring
//! rather than over a step.
//!
//! Composition owes its own laws. Two operations that are each correct, wired in
//! the wrong order or over the wrong intermediate, are still a defect — and
//! neither step's own suite can see it, because each step is doing exactly what
//! it promised. So a wiring carries a small named suite of its own.
//!
//! # The returning wiring
//!
//! A wiring whose exit type is its entry type owes two more laws than the
//! general case: its composition must return the value it was handed
//! ([`composed_return`]) and must settle after one application
//! ([`composed_idempotence`]). Those two are the roundtrip and idempotence laws
//! with the wiring as the subject, and they are expressible here precisely
//! because the entry and the exit are one type.

use super::conclude::agreement;
use super::types::{
    COMPOSED_CONSERVATION_DISAGREEMENT, COMPOSED_DETERMINISM_DISAGREEMENT,
    COMPOSED_IDEMPOTENCE_DISAGREEMENT, COMPOSED_RETURN_DISAGREEMENT, ComposedRoads, Equivalence,
    Measure,
};
use crate::report::TrialConclusion;

/// The composed road: the second step over the first step's image.
///
/// The one place the wiring is applied, so every law below reads the same
/// composition and no two of them can drift into composing it differently.
#[must_use]
pub fn composed<Entry, Middle, Exit>(
    suite: &ComposedRoads<Entry, Middle, Exit>,
    entry: &Entry,
) -> Exit {
    (suite.second())(&(suite.first())(entry))
}

/// The determinism law over a wiring: one entry, run twice through both steps,
/// gives one exit.
///
/// # Authority
///
/// Read over the wiring rather than over either step, because an intermediate
/// value is where an ambient fact hides most comfortably: a first step that
/// answers differently each time is invisible to a second step that faithfully
/// transforms whatever it was handed.
#[must_use]
#[track_caller]
pub fn composed_determinism<Entry, Middle, Exit>(
    suite: &ComposedRoads<Entry, Middle, Exit>,
    entry: &Entry,
) -> TrialConclusion {
    let first = composed(suite, entry);
    let second = composed(suite, entry);
    agreement(
        suite.same(),
        &first,
        &second,
        COMPOSED_DETERMINISM_DISAGREEMENT,
    )
}

/// The conservation law over a wiring: the quantity entering the first step is
/// the quantity leaving the second.
///
/// # Nonclaims
///
/// It says nothing about the intermediate. A wiring that loses a quantity in the
/// first step and invents it back in the second conserves it end to end, and
/// naming the two steps' own conservation laws is what pushes on that.
#[must_use]
#[track_caller]
pub fn composed_conservation<Entry, Middle, Exit, Quantity>(
    suite: &ComposedRoads<Entry, Middle, Exit>,
    entering: Measure<Entry, Quantity>,
    leaving: Measure<Exit, Quantity>,
    same: Equivalence<Quantity>,
    entry: &Entry,
) -> TrialConclusion {
    let before = entering(entry);
    let after = leaving(&composed(suite, entry));
    agreement(same, &before, &after, COMPOSED_CONSERVATION_DISAGREEMENT)
}

/// The roundtrip law over a returning wiring: what goes in comes back.
///
/// The exit type is the entry type, so the second step is the first step's
/// inverse if this law holds — and the wiring, rather than either step, is what
/// stated that it would.
#[must_use]
#[track_caller]
pub fn composed_return<Value, Middle>(
    suite: &ComposedRoads<Value, Middle, Value>,
    value: &Value,
) -> TrialConclusion {
    let returned = composed(suite, value);
    agreement(suite.same(), value, &returned, COMPOSED_RETURN_DISAGREEMENT)
}

/// The idempotence law over a returning wiring: driving the composition over its
/// own exit changes nothing.
///
/// A wiring that normalizes is idempotent without returning what it was handed,
/// which is why this law and [`composed_return`] are two rows rather than one.
#[must_use]
#[track_caller]
pub fn composed_idempotence<Value, Middle>(
    suite: &ComposedRoads<Value, Middle, Value>,
    value: &Value,
) -> TrialConclusion {
    let once = composed(suite, value);
    let twice = composed(suite, &once);
    agreement(
        suite.same(),
        &once,
        &twice,
        COMPOSED_IDEMPOTENCE_DISAGREEMENT,
    )
}
