//! Every public type of the network home, declared and nothing else.
//!
//! Construction and reading live in this module's own child `type_guard.rs`, and the sim's two moves — placing a send and advancing the tick — live in its child `sim.rs`, which are the two places the private fields are reachable.
//!
//! The names are [`crate::descriptor`]'s; everything else is this home's own: nodes and links, logical time, the per-link fault roster, the campaign shape the fault home established, and the sim value that joins them.

use crate::descriptor::NamespacedName;
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use std::collections::BTreeMap;

#[path = "type_guard.rs"]
mod guard;

#[path = "pack.rs"]
mod pack;

#[path = "sim.rs"]
mod sim;

pub use pack::{read, recorded};

// The topology.

/// One adopter-named node.
///
/// A node is its name and nothing more: what runs there is the adopter's, and this home never learns it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeRef(NamespacedName);

/// One directed link between two declared nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Link {
    from: NodeRef,
    to: NodeRef,
}

/// The declared nodes and the directed links between them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Topology {
    nodes: Vec<NodeRef>,
    links: Vec<Link>,
}

/// Why one topology was refused.
#[must_use = "a refusal is the reason a topology was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyRefusal {
    /// The topology declares no node.
    NoNode,
    /// Two node rows declare the same name.
    DuplicateNode(NodeRef),
    /// The topology declares no link, so no message could ever travel.
    NoLink,
    /// Two link rows declare the same ends.
    DuplicateLink(Link),
    /// A link names a node the topology never declared.
    LinkForeignNode {
        /// The undeclared node the link names.
        node: NodeRef,
    },
}

// Logical time.

/// One point of the sim's logical time, counted from zero.
///
/// The sim owns every tick; no wall clock participates anywhere in this home.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tick(u64);

/// A positive stretch of logical time, as a delay declares it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TickSpan(u32);

/// Why one tick span was refused.
#[must_use = "a refusal is the reason a tick span was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TickSpanRefusal {
    /// The span covers no tick, so a delay built on it would declare pressure and apply none.
    ZeroTicks,
}

/// One zero-based send on a link, in the order the sends were placed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendOrdinal(u32);

// The discipline.

/// One thing a link does to its traffic, from this home's own closed roster.
///
/// Closed rather than open, because these are the sim's own realizations — a fault nothing implements could otherwise be declared and never fire.
/// Reordering is deliberately absent: a reorder is a delay that crosses, which is how real networks reorder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkFault {
    /// The send at this position is lost.
    DropAt {
        /// The send the fault fires on.
        position: SendOrdinal,
    },
    /// The send at this position comes due later, by this many ticks.
    DelayAt {
        /// The send the fault fires on.
        position: SendOrdinal,
        /// How much later its delivery comes due.
        ticks: TickSpan,
    },
    /// The send at this position is delivered twice.
    DuplicateAt {
        /// The send the fault fires on.
        position: SendOrdinal,
    },
    /// Every send placed while the interval is open is lost.
    Partition {
        /// The first tick the interval covers.
        opens: Tick,
        /// The first tick past the interval; a send placed here travels again.
        heals: Tick,
    },
}

/// One link and the faults declared over its traffic, in authored order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkDiscipline {
    link: Link,
    faults: Vec<LinkFault>,
}

/// One named course of network adversity, from an empty control to a discipline per link.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkSchedule {
    name: NamespacedName,
    disciplines: Vec<LinkDiscipline>,
}

/// Why one network schedule was refused.
#[must_use = "a refusal is the reason a network schedule was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkScheduleRefusal {
    /// Two discipline rows name one link, which would leave a send with two answers.
    DuplicateDiscipline(Link),
    /// A discipline row declares no fault; a quiet link is spelled by omitting the row.
    EmptyDiscipline(Link),
    /// A partition interval closes at or before it opens, covering nothing.
    EmptyPartition {
        /// The link whose discipline declares the empty interval.
        link: Link,
    },
}

/// The uniquely named schedules one run chooses among.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkCampaign {
    schedules: Vec<NetworkSchedule>,
}

/// Why one network campaign was refused.
#[must_use = "a refusal is the reason a network campaign was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkCampaignRefusal {
    /// The campaign declares no schedule, so no selection could be satisfied.
    NoSchedule,
    /// Two schedules declare the same name.
    DuplicateSchedule(NamespacedName),
    /// Every schedule is an empty control, so the campaign declares no pressure at all.
    NoFaultDeclared,
}

/// One schedule, handed back by the campaign that declares it.
///
/// The selection borrows its campaign member, so a sim can never be opened over one campaign's schedule beside another campaign's name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetworkSelection<'campaign> {
    schedule: &'campaign NetworkSchedule,
}

/// Why one selection was refused.
#[must_use = "a refusal is the reason a network schedule was not selected"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkSelectionRefusal {
    /// The campaign declares no schedule under this name.
    ScheduleAbsent(NamespacedName),
}

// The sim.

/// The deterministic message-passing sim: one topology, one selected schedule, and the logical time they share.
///
/// A value, not a socket — nothing here binds a port or touches an operating system.
/// Every delivery, drop, and delay follows from the declared inputs alone, so two identically driven sims produce identical histories.
#[derive(Debug, Clone)]
pub struct SimNet<Payload> {
    topology: Topology,
    schedule: NetworkSchedule,
    tick: Tick,
    sequence: u64,
    placed: BTreeMap<Link, u32>,
    in_flight: Vec<sim::InFlight<Payload>>,
    census: NetworkCensus,
}

/// Why one sim was refused.
#[must_use = "a refusal is the reason a sim was not opened"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimNetRefusal {
    /// The schedule disciplines a link outside the topology.
    DisciplineForeignLink {
        /// The link the discipline names.
        link: Link,
    },
}

/// Why one send was refused.
#[must_use = "a refusal is the reason a send was not placed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendRefusal {
    /// The link is not in the sim's topology.
    LinkUndeclared(Link),
}

/// What became of one placed send.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendFate {
    /// This many deliveries were scheduled, all due at this tick.
    Scheduled {
        /// One for the original, plus one per duplicate fault that fired.
        copies: u32,
        /// The tick the deliveries come due.
        due: Tick,
    },
    /// A drop fault fired on this send.
    DroppedByDiscipline,
    /// The send was placed while a partition interval stood open.
    DroppedByPartition,
}

/// The experimenter's record of one send: which link, which ordinal, and its fate.
///
/// The fate is the sim's truth, not the subject's — a real sender never learns its packet died, and what the subject under test may see is the adopter's port's decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SendReceipt {
    link: Link,
    ordinal: SendOrdinal,
    fate: SendFate,
}

/// Whether one delivery is the send's original or a duplicate the discipline added.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeliveryCopy {
    /// The send's own delivery.
    Original,
    /// A copy a duplicate fault added.
    Duplicate,
}

/// One delivery: a command-shaped event carrying its whole lineage.
///
/// This shape is the keystone — deliveries feed a [`crate::properties`] transition contract directly, and per-link delivery sequences stand as [`crate::interleave`] strands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delivery<Payload> {
    link: Link,
    ordinal: SendOrdinal,
    payload: Payload,
    sent_at: Tick,
    delivered_at: Tick,
    copy: DeliveryCopy,
}

/// The accounting over every send a sim was asked to place.
///
/// Every seat is counted where it happens, so a schedule that quietly dropped half the traffic cannot read as a calm run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetworkCensus {
    sends: u64,
    scheduled_deliveries: u64,
    delivered: u64,
    dropped_by_discipline: u64,
    dropped_by_partition: u64,
}

// The transcript.

/// The body format the transcript reader understands.
pub const TRANSCRIPT_FORMAT_VERSION: u32 = 1;

/// The content-address family every transcript body is derived under.
pub const TRANSCRIPT_TAG: DomainTag =
    DomainTag::declared("network-transcript", IdentityProfileVersion::declared(1));

/// Where a transcript's deliveries came from.
///
/// The provenance is part of the addressed body, because a claim graded over recorded-live material takes the honest ceiling posture and nothing may quietly promote it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TranscriptProvenance {
    /// The deliveries came out of a [`SimNet`] run, fully declared and re-derivable.
    Simulated,
    /// The deliveries were witnessed on a live network by an adopter's adapter, outside this crate.
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

/// One admitted transcript: its provenance, its deliveries in delivery order, and the envelope carrying them.
///
/// The envelope is retained exactly as it was derived, so persisting a pack never needs a second writer that could disagree with the first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranscriptPack {
    provenance: TranscriptProvenance,
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
    /// The body declares a provenance slot this reader does not know.
    UnknownProvenance {
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
    entries: Vec<TranscriptEntry>,
    at: usize,
    tick: Tick,
}
