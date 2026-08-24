//! A refusal body cannot be written as a literal from outside, so a body one pass established cannot be reseated under another pass's refusal.
//!
//! A refusal whose one seat is public is a one-field record any holder can rewrite: a body established by one pass would read exactly like a body established by another, and the record carrying it would be minted by a caller rather than by the pass whose findings it reports.
//!
//! The body below is WELL FORMED, and that is the point — the bounded collection is public, so anybody can build one over the compiler's own issue type, and this is exactly the body a forger would arrive holding.
//! Nothing about it is defective; the record it is being written into simply has no seat anybody out here can name.

use macroonz::{BoundAxis, Capped, NonEmpty, PLAN_ISSUE_LIMIT, PlanError, PlanIssue};

fn main() {
    let body = Capped::all(NonEmpty::<PlanIssue, PLAN_ISSUE_LIMIT>::one(
        PlanIssue::BoundExceeded {
            axis: BoundAxis::Declarations,
            bound: 1,
            observed: 2,
        },
    ));
    let _reseated = PlanError { body };
}
