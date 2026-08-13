//! The reversal for the migration: a collection-shaped family body is one seat,
//! and a caller cannot put one together out of a carry and a posture.
//!
//! Every family in the machine that declares `FamilyShape::IssueCollection`
//! carries its issues and its coverage claim in a single `AdmittedPrefix`, read
//! back through `issues()` and `posture()`. The two-seat records those families
//! used to be let any holder of a `NonEmptyBounded` and any holder of a
//! `CompletionPosture` write the two down together, which is the pairing the
//! coupling exists to end: both halves stay individually honest and the pair is
//! a lie no runtime check can catch, because there is nothing wrong to detect at
//! either end.
//!
//! `RefinementConstruction` stands for all of them. The carry below is real —
//! it is built through a public total road and would have fitted the old
//! `issues` seat exactly — and the posture beside it is the one a completed
//! examination writes. Neither value is defective; the record they are being
//! written into simply has no seat for either.

use threadpak::refusal::CompletionPosture;
use threadpak::schema::{RefinementConstruction, RefinementConstructionIssue, RefinementIssueLimit};
use threadpak::types::NonEmptyBounded;

fn main() {
    let issues: NonEmptyBounded<RefinementConstructionIssue, RefinementIssueLimit> =
        NonEmptyBounded::singleton(RefinementConstructionIssue::NotTotal);

    let _assembled = RefinementConstruction {
        issues,
        posture: CompletionPosture::Complete,
    };
}
