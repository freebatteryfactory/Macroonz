//! The reversal for the truncation posture: the posture is taken off an act.
//!
//! `AdmittedPrefix::examined_completely` is the only mint for
//! `ReportTruncation`, and what selects the posture is the truncation that road
//! itself performs. A seat a caller could write would let a body that carried
//! every issue it established state that seven were dropped: the posture would
//! still be a typed value, and it would be describing a truncation that never
//! happened.
//!
//! So the bound and the count are private. The posture written by hand below is
//! the shape that would reopen the gap, and it does not compile.
//!
//! # What this file establishes, exactly
//!
//! REPRESENTATION PRIVACY: the bound and the count are not a caller's to write.
//! It does NOT establish that the variant has no assembly road of its own. A
//! public `ReportTruncation::assembled(bound, count)` would leave this error
//! untouched — the seats stay private, and the struct literal below stays
//! unwritable — while a caller minted the posture without performing anything.
//!
//! The absence of a second mint is not derived, for the reason the marriage
//! reversal beside this one states: a road IN is not distinguishable from the
//! lawful mint beside it without a declaration of which mint is the one, and the
//! tree carries no such declaration for this package.

use core::num::NonZeroUsize;
use threadpak::refusal::{CompletionPosture, ReportTruncation, StopBound};

fn main() {
    // Nothing was truncated. The count is chosen, the bound is chosen, and
    // neither seat is the caller's to write.
    let fabricated = ReportTruncation {
        stopped_at: StopBound::DeclaredIssueBound,
        omitted: NonZeroUsize::MIN,
    };
    let _ = CompletionPosture::ReportTruncated(fabricated);
}
