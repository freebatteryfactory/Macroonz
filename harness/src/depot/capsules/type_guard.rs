//! Construction and readings for an admitted replay entry and the location it was stored at.

use super::{ReplayCapsuleEntry, ReplayDepotRefusal, StoredReplayEntryRef};
use crate::descriptor::{ProposalId, ReplayRef};
use crate::report::ReplayCapsule;

impl ReplayCapsuleEntry {
    /// Assemble the exact entry one replay-bearing admission stores.
    ///
    /// The reference is minted here from the capsule that was handed in, which is the whole reason the two cannot disagree.
    #[must_use]
    pub(crate) fn admitted(proposal: ProposalId, capsule: ReplayCapsule) -> Self {
        let replay = ReplayRef::over(capsule.identity());
        Self {
            proposal,
            replay,
            capsule,
        }
    }

    /// The proposal whose admission authored this entry.
    #[must_use]
    pub const fn proposal(&self) -> ProposalId {
        self.proposal
    }

    /// The content-derived reference an admitted row carries.
    #[must_use]
    pub const fn replay(&self) -> ReplayRef {
        self.replay
    }

    /// The run-bound reproduction account stored under the reference.
    #[must_use]
    pub const fn capsule(&self) -> &ReplayCapsule {
        &self.capsule
    }
}

impl StoredReplayEntryRef {
    /// Bind one caller-owned storage token to the entry it stores.
    ///
    /// # Errors
    ///
    /// Refuses an empty token, which names nowhere.
    pub fn at(replay: ReplayRef, token: &str) -> Result<Self, ReplayDepotRefusal> {
        if token.is_empty() {
            return Err(ReplayDepotRefusal::EmptyLocation);
        }
        Ok(Self {
            replay,
            token: token.to_owned(),
        })
    }

    /// The replay entry this location stores.
    #[must_use]
    pub const fn replay(&self) -> ReplayRef {
        self.replay
    }

    /// The caller-owned storage token.
    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }
}
