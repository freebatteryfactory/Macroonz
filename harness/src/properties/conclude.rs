//! The one road from a demand to a typed conclusion, and the caller-tracking that gives every refusal its file and line.
//!
//! Every law in this home reaches its verdict through [`concluded`], so a disagreement becomes a finding in exactly one place.
//!
//! Every road below is `#[track_caller]`, and so is every law that calls one, and the attribute is transparent up the chain — so a refusal is born knowing the site of the owner's check, with no panic machinery anywhere on the road.
//!
//! Nothing foreign enters a conclusion built here: the material is the owner's own typed values, the verdict is a typed demand verdict, and the cause is a declared identity pair.

use super::types::{Agreement, Equivalence, Holding, Order};
use crate::report::{FailureClass, FindingCause, FindingLocation, TrialConclusion, TrialFinding};
use core::cmp::Ordering;
use core::panic::Location;

/// Where the check that reached this road was written.
///
/// A tracked caller's location lives for the program, which is what lets it stand in the record vocabulary's `'static` seat; a location observed at run time does not, and never arrives through here.
#[must_use]
#[track_caller]
pub fn raised_here() -> FindingLocation {
    let raised = Location::caller();
    FindingLocation::at(raised.file(), raised.line())
}

/// The conclusion one demand reaches.
///
/// The class and the cause are both the caller's declarations: the class says what kind of disagreement this is, and the cause names it in the caller's own spelling.
#[must_use]
#[track_caller]
pub fn concluded(holding: Holding, class: FailureClass, cause: FindingCause) -> TrialConclusion {
    match holding {
        Holding::Holds => TrialConclusion::Passed,
        Holding::Fails => {
            TrialConclusion::Refused(TrialFinding::established(class, cause, raised_here(), None))
        }
    }
}

/// The conclusion one demanded agreement reaches.
///
/// The class is [`FailureClass::PropertyDisagreement`] always, because a demanded agreement is a declared law and a law that disagreed with its subject is what that class names.
#[must_use]
#[track_caller]
pub fn agreement<Value>(
    same: Equivalence<Value>,
    left: &Value,
    right: &Value,
    cause: FindingCause,
) -> TrialConclusion {
    let holding = match same(left, right) {
        Agreement::Agrees => Holding::Holds,
        Agreement::Differs => Holding::Fails,
    };
    concluded(holding, FailureClass::PropertyDisagreement, cause)
}

/// The conclusion one demanded ranking reaches: the lower value does not rank above the upper one.
///
/// Non-strict, so equal ranks satisfy the demand and every law built on this one demands non-decrease rather than strict increase.
#[must_use]
#[track_caller]
pub fn ranking<Value>(
    order: Order<Value>,
    lower: &Value,
    upper: &Value,
    cause: FindingCause,
) -> TrialConclusion {
    let holding = match order(lower, upper) {
        Ordering::Less | Ordering::Equal => Holding::Holds,
        Ordering::Greater => Holding::Fails,
    };
    concluded(holding, FailureClass::PropertyDisagreement, cause)
}

/// The conclusion one outcome that was owed an answer reaches.
///
/// The class is [`FailureClass::RefusedByCheck`], because this is a check's own contract about its subject rather than an algebraic law.
#[must_use]
#[track_caller]
pub fn admitted<Answer, Refusal>(
    outcome: &Result<Answer, Refusal>,
    cause: FindingCause,
) -> TrialConclusion {
    let holding = match *outcome {
        Ok(_) => Holding::Holds,
        Err(_) => Holding::Fails,
    };
    concluded(holding, FailureClass::RefusedByCheck, cause)
}

/// The conclusion one outcome that was owed a refusal reaches.
///
/// It reads that the subject refused and nothing about which refusal it answered with; a check that owes the exact refusal composes the value itself through [`ensure_refused_with!`](crate::ensure_refused_with).
#[must_use]
#[track_caller]
pub fn refused<Answer, Refusal>(
    outcome: &Result<Answer, Refusal>,
    cause: FindingCause,
) -> TrialConclusion {
    let holding = match *outcome {
        Ok(_) => Holding::Fails,
        Err(_) => Holding::Holds,
    };
    concluded(holding, FailureClass::RefusedByCheck, cause)
}
