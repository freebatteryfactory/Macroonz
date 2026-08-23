//! The reversal for the SEAT half of the coupled seat: a services refusal
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
//! WELL-FORMED — band 00's report package is public, so anybody can build one
//! over the plane's own issue type, and this is exactly the body a forger would
//! arrive holding. Nothing about it is defective; the record it is being written
//! into simply has no seat anybody out here can name.
//!
//! The MINT half is a different claim and has a fixture of its own:
//! `a-services-refusal-minted-outside-its-plane.rs`. Closing one of the two
//! halves closes neither, which is why the two stand together.

use macroonz::AdmittedPrefix;
use threadpak_macroc::refusal::PlanningIssueLimit;
use threadpak_macroc::{BoundAxis, ProjectionPlanning, ProjectionPlanningIssue};

fn main() {
    let body: AdmittedPrefix<ProjectionPlanningIssue, PlanningIssueLimit> =
        AdmittedPrefix::carrying_one(ProjectionPlanningIssue::BoundExceeded {
            axis: BoundAxis::Declarations,
            bound: 1,
            observed: 2,
        });

    let _reseated = ProjectionPlanning { body };
}
