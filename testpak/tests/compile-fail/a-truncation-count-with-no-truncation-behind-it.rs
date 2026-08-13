//! The reversal for the truncation posture: the posture is taken off an act.
//!
//! `AdmittedPrefix::examined_completely` is the only mint for
//! `ReportTruncation`, and what selects the posture is the truncation that road
//! itself performs. A seat a caller could write would let a body that carried
//! every issue it established state that seven were dropped: the posture would
//! still be a typed value, and it would be describing a truncation that never
//! happened.
//!
//! So the bound and the count are private and the variant has no assembly road
//! of its own. The posture written by hand below is the shape that would reopen
//! the gap, and it does not compile.

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
