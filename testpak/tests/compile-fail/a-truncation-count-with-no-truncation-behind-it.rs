//! The reversal for the truncation posture: the posture is taken off an act.
//!
//! `CompletionPosture::examined_completely` is the only mint for
//! `ReportTruncation`, and what selects the posture is the remainder the body's
//! own construction handed back. A road taking a bare count would let a body that
//! carried every issue it established write down that seven were dropped: the
//! posture would still be a typed value, and it would be describing a truncation
//! that never happened.
//!
//! The parameter is therefore the witness, and the witness has no public mint.
//! The call below is the shape that would reopen the gap, and it does not
//! compile.

use threadpak::refusal::{CompletionPosture, StopBound};

fn main() {
    // Nothing was truncated, and nothing here holds a witness saying otherwise.
    let _ = CompletionPosture::examined_completely(7, StopBound::DeclaredIssueBound);
}
