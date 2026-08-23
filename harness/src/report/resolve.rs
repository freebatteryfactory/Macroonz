//! The posture readings over one attachment: what the meet of its two revision
//! bindings buys the cache, and what it lets a reproduction claim.
//!
//! An attachment carries two posture-bearing revision bindings — one for the
//! subject, one for the check — and every per-posture sentence in the harness
//! reads over their MEET. Reading either binding alone would let a derived
//! subject revision carry an author's-word check revision into a claim neither
//! of them supports.
//!
//! The meet itself is the descriptor home's operation
//! ([`RevisionPosture::meet`]); what it MEANS is this home's, and the two are
//! composed here rather than either being restated.

use super::{CacheEligibility, ReplayPosture};
use crate::descriptor::RevisionPosture;

/// Whether a rerun cache may stand in for executing this attachment again.
///
/// The reading is the meet's image under the one eligibility statement
/// [`CacheEligibility`] owns, so a mixed attachment gets the weaker answer,
/// always.
///
/// A caller holding the attachment itself passes its own
/// [`ExecutableAttachment::posture`](crate::descriptor::ExecutableAttachment::posture),
/// which is that meet.
#[must_use]
pub fn attachment_cache_eligibility(
    subject: RevisionPosture,
    check: RevisionPosture,
) -> CacheEligibility {
    CacheEligibility::from(subject.meet(check))
}

/// What a reproduction of this attachment's execution can claim.
///
/// The posture is the meet's image, which is why a mixed attachment can never
/// mint an exact-replay claim over an author's-word check revision.
#[must_use]
pub fn attachment_replay_posture(
    subject: RevisionPosture,
    check: RevisionPosture,
) -> ReplayPosture {
    ReplayPosture::from(subject.meet(check))
}
