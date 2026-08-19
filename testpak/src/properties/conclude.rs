//! The conclusion nucleus: the one road from a demand to a typed conclusion, and
//! the caller-tracking that gives every refusal its file and line.
//!
//! Every law in this home reaches its verdict through [`concluded`], so a
//! disagreement becomes a finding in exactly one place. A law that built a
//! finding of its own would be a second authority over what a refusal from this
//! home looks like.
//!
//! # Ambient provenance
//!
//! Every road here is `#[track_caller]`, and so is every law that calls one. The
//! attribute is transparent up the chain, so [`raised_here`] reports the site of
//! the OWNER's check rather than the line of whichever kernel happened to build
//! the value — a refusal is born knowing where it came from, with no panic
//! machinery anywhere on the road.
//!
//! # No foreign text
//!
//! Nothing foreign enters a conclusion built here. The material a law reads is
//! the owner's own typed values, the verdict is a typed demand verdict, and the
//! cause is a declared identity pair — so the finding's foreign-text seat is
//! empty rather than filled with a rendering of something the harness already
//! knows typed.

use super::types::{Agreement, Equivalence, Holding, Order};
use crate::report::{FailureClass, FindingCause, FindingLocation, TrialConclusion, TrialFinding};
use core::cmp::Ordering;
use core::panic::Location;

/// Where the check that reached this road was written.
///
/// # Authority
///
/// The caller's location, not this file's. A property suite refuses inside
/// itself while the trial it serves lives in a table somewhere else, and this is
/// the first of those two facts — the one a reader jumps to.
///
/// # Bounds
///
/// The location a caller is tracked to lives for the program, which is what lets
/// it stand in the record vocabulary's `'static` location seat. A location
/// observed at run time — a panic hook's, an external tool's — does not, and
/// never arrives through here.
#[must_use]
#[track_caller]
pub fn raised_here() -> FindingLocation {
    let raised = Location::caller();
    FindingLocation::at(raised.file(), raised.line())
}

/// The conclusion one demand reaches.
///
/// # Authority
///
/// The one road from a verdict to a [`TrialConclusion`] in this home. The class
/// and the cause are the caller's declarations: the class says what KIND of
/// disagreement this is, and the cause names it the way the machine spells one.
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
/// # Authority
///
/// The class is [`FailureClass::PropertyDisagreement`], always: a demanded
/// agreement is a declared law, and a law that disagreed with its subject is
/// what that class names.
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

/// The conclusion one demanded ranking reaches: the lower value does not rank
/// above the upper one.
///
/// # Bounds
///
/// Non-strict. Equal ranks satisfy the demand, so a law built on this one
/// demands non-decrease and never strict increase.
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
/// # Authority
///
/// The class is [`FailureClass::RefusedByCheck`]: this is a check's own contract
/// about its subject rather than an algebraic law, and the two are kept apart so
/// that a normalized failure class stays worth reading.
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
/// # Nonclaims
///
/// It reads that the subject refused and nothing about WHICH refusal it
/// answered with. A check that owes the exact refusal composes the value itself
/// — [`ensure_refused_with!`](crate::ensure_refused_with) is that road — because
/// a refusal family is the owner's vocabulary and this home may not read one.
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
