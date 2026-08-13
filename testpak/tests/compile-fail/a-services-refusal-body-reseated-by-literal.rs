//! The reversal for the visibility half of the coupled seat: a services refusal
//! family body cannot be written as a literal from outside the crate, so a body
//! one seam produced cannot be reseated under another seam's refusal.
//!
//! The coupled seat already keeps a carry and its posture together. It does not,
//! by itself, keep a BODY and the seam that established it together: a refusal
//! family whose one seat is public is a one-field record any holder can rewrite,
//! so a body established by one pass reads exactly like a body established by
//! another, and the record carrying it is minted by a caller rather than by the
//! pass whose findings it reports.
//!
//! `ProjectionPlanning` stands for all six services families. The body below is
//! REAL — it comes off the public guarded road, and it is the body that road
//! produced — and the refusal it is being written into would be a refusal no
//! seam in the plane ever raised. Nothing about the body is defective; the
//! record it is being written into simply has no seat anybody out here can name.

use threadpak_macroc::{PlanSeat, ProjectionPlanning, ProjectionPlanningIssue};

fn main() {
    let established = ProjectionPlanning::established(ProjectionPlanningIssue::MissingOwnerFact {
        seat: PlanSeat::TargetBinding,
    });
    let body = established.body().clone();

    let _reseated = ProjectionPlanning { body };
}
