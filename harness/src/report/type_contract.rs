//! The two total images of a revision posture: what it buys the cache, and what it lets a reproduction claim.
//!
//! Written as conversions rather than as branches inside a caller, so every reader of eligibility and every reader of replay reaches the same answer from the same posture.

use super::{CacheEligibility, ReplayPosture};
use crate::descriptor::RevisionPosture;

impl From<RevisionPosture> for CacheEligibility {
    /// What one posture buys the rerun cache.
    ///
    /// A caller holding an attachment reads the meet of its two bindings first; this map answers for one posture, not for a pair.
    fn from(posture: RevisionPosture) -> Self {
        Self::from(ReplayPosture::from(posture))
    }
}

impl From<ReplayPosture> for CacheEligibility {
    /// Whether the complete replay ceiling permits a rerun cache.
    fn from(posture: ReplayPosture) -> Self {
        match posture {
            ReplayPosture::ExactDerived => Self::Eligible,
            ReplayPosture::DeclaredByAuthor | ReplayPosture::UnavailableBecauseUntracked => {
                Self::NeverEligible
            }
        }
    }
}

impl From<RevisionPosture> for ReplayPosture {
    /// What one posture lets a reproduction claim.
    ///
    /// A caller holding an attachment reads the meet of its two bindings first; this map answers for one posture, not for a pair.
    fn from(posture: RevisionPosture) -> Self {
        match posture {
            RevisionPosture::Derived => Self::ExactDerived,
            RevisionPosture::Declared => Self::DeclaredByAuthor,
            RevisionPosture::Untracked => Self::UnavailableBecauseUntracked,
        }
    }
}
