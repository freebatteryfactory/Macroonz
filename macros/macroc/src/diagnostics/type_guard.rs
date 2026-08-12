//! The diagnostics home's invariant nucleus: the one road that reaches the one
//! seat a caller may not write.
//!
//! Declared inside `types.rs` as its own child, so the truncation's bound and
//! count are reachable here and nowhere else. Everything else in this home is a
//! plain readable seat, and this file exists for the single fact that is about an
//! ACT rather than about a value: how many per-issue identities a set-building
//! road left behind.
//!
//! The road below takes the dropped material rather than a number. That is the
//! whole discipline — the count is read off what was actually dropped, so the
//! posture and the set are two readings of one act, and a set that dropped
//! nothing has nothing to read a count off.

use super::{RelatedSetCompletion, RelatedSetTruncation};
use core::num::NonZeroUsize;
use threadpak::refusal::StopBound;

impl RelatedSetTruncation {
    /// The declared bound the set was truncated at.
    #[must_use]
    pub const fn stopped_at(self) -> StopBound {
        self.stopped_at
    }

    /// How many per-issue identities the set does not carry; at least one, by
    /// shape — a truncation that dropped nothing is
    /// [`RelatedSetCompletion::Complete`] and is unrepresentable here.
    #[must_use]
    pub const fn omitted(self) -> NonZeroUsize {
        self.omitted
    }
}

impl RelatedSetCompletion {
    /// The posture a related set amounts to, given the per-issue identities it
    /// could not carry.
    ///
    /// The dropped material selects the posture rather than the caller: nothing
    /// dropped is `Complete`, and anything dropped is a truncation naming the
    /// bound and the exact count. Taking the slice rather than its length is what
    /// keeps the count attached to the act — a road that took a number would be
    /// recording an assertion, and the seat it wrote into would be a fact nobody
    /// established.
    ///
    /// It is generic over what was dropped because the count is the only thing
    /// read: this home names no element type, and binding one here would be the
    /// diagnostics home taking a position on whose identities a set carries.
    pub(crate) fn carrying_all_but<T>(dropped: &[T], stopped_at: StopBound) -> Self {
        match NonZeroUsize::new(dropped.len()) {
            None => Self::Complete,
            Some(omitted) => Self::ReportTruncated(RelatedSetTruncation {
                stopped_at,
                omitted,
            }),
        }
    }
}
