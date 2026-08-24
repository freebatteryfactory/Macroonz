//! The entry a replay-bearing admission stores, and the caller-owned boundary it stores across.

use crate::descriptor::{ProposalId, ReplayRef};
use crate::report::ReplayCapsule;

#[path = "type_guard.rs"]
mod guard;

/// One replay capsule entry, assembled by an admission act.
///
/// The reference is derived from the capsule's own content at the private mint, so the three members can only agree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayCapsuleEntry {
    proposal: ProposalId,
    replay: ReplayRef,
    capsule: ReplayCapsule,
}

/// The caller's storage location for one admitted entry.
///
/// The replay reference rides with the location, so an admission can refuse a response about some other entry instead of trusting a neighboring value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredReplayEntryRef {
    replay: ReplayRef,
    token: String,
}

/// Why the caller's replay depot did not store an admitted entry.
///
/// Durability is the sink's own statement: the harness reaches no filesystem and cannot establish where anything was persisted.
#[must_use = "a refusal is the reason a replay entry was not stored"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayDepotRefusal {
    /// The depot is not accepting an admission.
    Unavailable,
    /// The depot already carries this content-derived replay entry.
    AlreadyStored(ReplayRef),
    /// The offered storage token is empty and names nowhere.
    EmptyLocation,
    /// The caller states that the destination is not review-durable.
    DestinationNotDurable,
}

/// The caller-owned storage effect an explicit human admission invokes.
///
/// The sink receives an entry that is already assembled and already immutable: it stores or refuses, and returning success is its own statement that storage occurred.
pub trait ReplayDepotSink {
    /// Store the exact admitted entry and return its caller-owned location.
    ///
    /// # Errors
    ///
    /// The sink's own [`ReplayDepotRefusal`].
    fn store(
        &mut self,
        entry: &ReplayCapsuleEntry,
    ) -> Result<StoredReplayEntryRef, ReplayDepotRefusal>;
}
