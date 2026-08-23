//! The replay depot's public entry and caller-owned storage boundary.

use crate::descriptor::{ProposalId, ReplayRef};
use crate::report::ReplayCapsule;

#[path = "type_guard.rs"]
mod guard;

/// One replay capsule entry assembled by a human admission act.
///
/// # Authority
///
/// The replay reference derives from the capsule's content identity at the
/// private mint. A caller cannot pair a proposal with one capsule and a
/// reference to another.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayCapsuleEntry {
    proposal: ProposalId,
    replay: ReplayRef,
    capsule: ReplayCapsule,
}

/// The caller's storage location for one admitted replay entry.
///
/// The replay reference rides with the location, so human admission can refuse
/// a sink response for another entry instead of trusting neighboring values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredReplayEntryRef {
    replay: ReplayRef,
    token: String,
}

/// Why the caller's replay depot did not store an admitted entry.
///
/// Durability is the sink's statement: `TestPak` reaches no filesystem and cannot
/// independently establish where the caller persisted the entry.
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
/// # Authority
///
/// The sink receives an immutable, already-assembled entry. It can store or
/// refuse; it cannot choose the proposal, replay identity, capsule bytes, or
/// posture. Returning success is the caller's statement that storage occurred.
pub trait ReplayDepotSink {
    /// Store the exact admitted entry and return its caller-owned location.
    ///
    /// # Errors
    ///
    /// The sink's own refusal: unavailable, already stored under this replay
    /// reference, an empty location, or a destination it cannot call durable.
    fn store(
        &mut self,
        entry: &ReplayCapsuleEntry,
    ) -> Result<StoredReplayEntryRef, ReplayDepotRefusal>;
}
