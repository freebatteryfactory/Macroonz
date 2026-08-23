//! The record vocabulary's declarative trait implementations: the two images of
//! a revision posture.
//!
//! Both are total maps declared once. Writing them as conversions rather than
//! as branches inside a caller is what keeps every reader of eligibility and
//! every reader of replay reaching the same answer from the same posture.

use super::{CacheEligibility, ReplayPosture};
use crate::descriptor::RevisionPosture;

impl From<RevisionPosture> for CacheEligibility {
    /// What one posture buys the rerun cache.
    ///
    /// Derived is fully eligible; declared skips only while the author's
    /// declared revisions are unchanged, at the author's-word ceiling the
    /// report states; untracked always reruns and is never cached.
    ///
    /// The caller with an ATTACHMENT reads the meet of its two bindings first —
    /// this map answers for one posture, not for a pair.
    fn from(posture: RevisionPosture) -> Self {
        match posture {
            RevisionPosture::Derived => Self::Eligible,
            RevisionPosture::Declared => Self::EligibleWhileDeclaredRevisionsUnchanged,
            RevisionPosture::Untracked => Self::NeverEligible,
        }
    }
}

impl From<RevisionPosture> for ReplayPosture {
    /// What one posture lets a reproduction claim.
    ///
    /// Derived earns the phrase "replay exactly"; declared inherits the
    /// author's-word ceiling; untracked leaves the historical run and its input
    /// as real evidence while every rendering states that reproduction is
    /// non-exact.
    ///
    /// The caller with an ATTACHMENT reads the meet of its two bindings first —
    /// this map answers for one posture, not for a pair.
    fn from(posture: RevisionPosture) -> Self {
        match posture {
            RevisionPosture::Derived => Self::ExactDerived,
            RevisionPosture::Declared => Self::DeclaredByAuthor,
            RevisionPosture::Untracked => Self::UnavailableBecauseUntracked,
        }
    }
}
