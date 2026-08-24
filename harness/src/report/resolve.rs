//! What one attachment's pair of revision bindings buys, read over the weaker of the two.
//!
//! An attachment binds a subject revision and a check revision, and every per-posture sentence in the harness reads over their meet.
//! Reading either binding alone would let a derived subject revision carry an author's-word check revision into a claim neither of them supports.
//! The meet itself is the descriptor home's operation ([`RevisionPosture::meet`]); what it means is this home's, and the two are composed here rather than either being restated.

use super::{CacheEligibility, ReplayPosture};
use crate::descriptor::RevisionPosture;

/// Whether a rerun cache may stand in for executing this attachment again.
///
/// A caller holding the attachment itself passes its own [`ExecutableAttachment::posture`](crate::descriptor::ExecutableAttachment::posture), which is that meet.
#[must_use]
pub fn attachment_cache_eligibility(
    subject: RevisionPosture,
    check: RevisionPosture,
) -> CacheEligibility {
    CacheEligibility::from(subject.meet(check))
}

/// What a reproduction of this attachment's execution can claim.
///
/// Because the reading is the meet's image, a mixed attachment can never mint an exact-replay claim over an author's-word check revision.
#[must_use]
pub fn attachment_replay_posture(
    subject: RevisionPosture,
    check: RevisionPosture,
) -> ReplayPosture {
    ReplayPosture::from(subject.meet(check))
}
