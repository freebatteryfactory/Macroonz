//! Every public type of transcript custody, reproduction, and replay, declared and nothing else.
//!
//! Construction lives in this module's type guard; admission, encoding, reading, reproduction, and replay each keep one operation role.

use super::super::simulation::{
    DeliveryCopy, Link, NetworkSchedule, SendOrdinal, SendRefusal, SimNetRefusal, Tick, Topology,
};
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};

#[path = "type_guard.rs"]
mod guard;

#[path = "admit.rs"]
mod admit;
#[path = "encode.rs"]
mod encode;
#[path = "project.rs"]
mod project;
#[path = "read.rs"]
mod read;
#[path = "replay.rs"]
mod replay;
#[path = "reproduce.rs"]
mod reproduce_run;
#[path = "write.rs"]
mod write;

pub use read::{read_recorded_live, read_simulated};
pub use reproduce_run::reproduce;
pub use write::{recorded_live, simulated};

// The transcript.

/// The body format the transcript reader understands.
pub const TRANSCRIPT_FORMAT_VERSION: u32 = 2;

/// The content-address family every transcript body is derived under.
pub const TRANSCRIPT_TAG: DomainTag =
    DomainTag::declared("network-transcript", IdentityProfileVersion::declared(2));

/// What one transcript body claims about where its deliveries came from.
///
/// A source claim is addressed material, not standing that the claim was reproduced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TranscriptSourceClaim {
    /// The body carries a complete simulation manifest.
    Simulated,
    /// The body carries deliveries witnessed by an adopter's live adapter.
    RecordedLive,
}

/// One byte-valued input action retained by a simulated transcript.
///
/// The action is declaration material until [`crate::network::reproduce`] executes its whole manifest through [`crate::network::SimNet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationAction {
    /// Place these payload bytes on this link at the current logical tick.
    Send {
        /// The link the action sends on.
        link: Link,
        /// The payload bytes handed to the reproduced sim.
        payload: Vec<u8>,
    },
    /// Advance the reproduced sim by one logical tick.
    Advance,
}

/// The selected schedule and complete ordered action trace one simulated body declares.
///
/// The schedule remains an owner-built value rather than decoded names minted from foreign bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationManifest {
    schedule: NetworkSchedule,
    actions: Vec<SimulationAction>,
}

/// The source-specific material retained by an admitted pack.
#[derive(Debug, Clone, PartialEq, Eq)]
enum TranscriptMaterial {
    /// A simulation claim with the inputs needed to execute it again.
    Simulated(SimulationManifest),
    /// A live-recorded claim carrying no reproducible input manifest.
    RecordedLive,
}

/// One witnessed delivery, as a transcript retains it: the whole lineage, with the payload in bytes.
///
/// Openly mintable, because a live adapter must be able to write down what it observed; what a pack of entries can claim is bounded by its provenance, not by who spelled the rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranscriptEntry {
    link: Link,
    ordinal: SendOrdinal,
    payload: Vec<u8>,
    sent_at: Tick,
    delivered_at: Tick,
    copy: DeliveryCopy,
}

/// The content address of one complete transcript body.
///
/// Only the writer and the reader mint one, and both derive it under [`TRANSCRIPT_TAG`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TranscriptAddress(ContentAddress);

/// One admitted transcript: its topology, source material, deliveries in delivery order, and the envelope carrying them.
///
/// The envelope is retained exactly as it was derived, so persisting a pack never needs a second writer that could disagree with the first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranscriptPack {
    topology: Topology,
    material: TranscriptMaterial,
    address: TranscriptAddress,
    entries: Vec<TranscriptEntry>,
    encoded: Vec<u8>,
}

/// Why one transcript was not written, or not read.
#[must_use = "a refusal is the reason a transcript was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranscriptRefusal {
    /// The transcript holds no delivery, and an empty transcript replays nothing.
    NoDelivery,
    /// An entry travels a link the topology never declared.
    ForeignLink {
        /// The entry's position in delivery order.
        at: usize,
    },
    /// An entry is stamped earlier than the entry before it.
    DeliveryOrderBroken {
        /// The position where the order breaks.
        at: usize,
    },
    /// An entry claims a delivery stamped earlier than its own send.
    ///
    /// A delivery before its send is temporally impossible under any provenance — simulated or live-recorded — so the row refuses at admission rather than entering a pack whose claims a replay would then faithfully repeat.
    DeliveryBeforeSend {
        /// The impossible entry's position in delivery order.
        at: usize,
    },
    /// The envelope ends inside a member it had already declared.
    Truncated,
    /// The leading claim is not the address the body derives.
    AddressMismatch {
        /// What the body actually derives.
        derived: TranscriptAddress,
    },
    /// The body declares a format this reader does not understand.
    UnsupportedFormat {
        /// The format position found in the body.
        found: u32,
    },
    /// The body declares a source-claim slot this reader does not know.
    UnknownSourceClaim {
        /// The slot found in the body.
        found: u32,
    },
    /// The body declares a different source posture than the reading road accepts.
    SourceClaimMismatch {
        /// The source posture the reading road accepts.
        expected: TranscriptSourceClaim,
        /// The source posture the body declares.
        found: TranscriptSourceClaim,
    },
    /// The encoded simulation schedule is not the owner-built schedule the caller supplied.
    ScheduleMismatch,
    /// The body declares a link-fault slot this reader does not know.
    UnknownFault {
        /// The slot found in the body.
        found: u32,
    },
    /// The body declares a simulation-action slot this reader does not know.
    UnknownAction {
        /// The slot found in the body.
        found: u32,
    },
    /// The body declares a delivery-copy slot this reader does not know.
    UnknownCopy {
        /// The slot found in the body.
        found: u32,
    },
    /// The encoded topology is not the one the caller opened the pack for.
    TopologyMismatch,
    /// One decoded simulation action sends on a link outside the expected topology.
    SimulationActionForeignLink {
        /// The action's position in manifest order.
        at: usize,
    },
    /// The retained schedule could not open over the retained topology.
    SimulationNotOpened(SimNetRefusal),
    /// One retained action was refused by the reproduced sim.
    SimulationSendRefused {
        /// The action's position in manifest order.
        at: usize,
        /// Why the send was refused.
        refusal: SendRefusal,
    },
    /// The reproduced delivery roster first differs from the addressed roster here.
    SimulationRowsDiverge {
        /// The first different or absent row.
        at: usize,
    },
    /// A recorded-live transcript has no simulation manifest to reproduce.
    RecordedLiveCannotReproduce,
    /// A declared length is wider than this platform can index.
    LengthOutsidePlatform {
        /// The unrepresentable length.
        declared: u64,
    },
    /// Bytes remain after the last entry the declared count admitted.
    TrailingBytes {
        /// How many are left over.
        count: usize,
    },
}

/// A pack played back: exactly the recorded deliveries, at exactly their recorded ticks.
///
/// A replay takes no sends and consults no discipline — it is the record, walked forward, which is what turns live traffic into a deterministic regression input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Replay {
    address: TranscriptAddress,
    entries: Vec<TranscriptEntry>,
    total: usize,
    at: usize,
    tick: Tick,
}

/// Evidence that one exact simulation manifest reproduced its addressed delivery roster.
///
/// It proves reproduction of the byte-valued manifest and nothing about how a caller later handles those deliveries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationReproduction {
    address: TranscriptAddress,
    actions: usize,
    rows: usize,
    final_tick: Tick,
}

/// Evidence that one replay handed out every row in one exact addressed transcript.
///
/// Handed out is the ceiling: this value does not claim the caller processed, accepted, or applied any delivery.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplayExhaustion {
    address: TranscriptAddress,
    total: usize,
    final_tick: Tick,
}

/// Why one replay could not mint [`ReplayExhaustion`].
#[must_use = "a refusal is the reason replay exhaustion was not minted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplayIncomplete {
    address: TranscriptAddress,
    remaining: usize,
}

/// The exact join of reproduced simulation material and an exhausted replay over the same transcript.
///
/// It proves that the addressed input manifest reproduced the addressed rows and that playback handed every addressed row out.
/// It does not prove that a caller processed those rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReproducedReplay {
    reproduction: SimulationReproduction,
    exhaustion: ReplayExhaustion,
}

/// Why simulation reproduction and replay exhaustion did not join.
#[must_use = "a refusal is the reason reproduced replay standing was not minted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReproducedReplayRefusal {
    /// The two values describe different addressed transcripts.
    AddressMismatch {
        /// The transcript the simulation reproduction names.
        reproduction: TranscriptAddress,
        /// The transcript the replay exhaustion names.
        replay: TranscriptAddress,
    },
}
