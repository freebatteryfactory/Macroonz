//! Transcript writing, source-specific reading, simulation reproduction, and playback.
//!
//! The addressed body is exactly this, with no separators and no padding — `u32be(n)` is an integer in four big-endian bytes, `u64be(n)` in eight, and `bytes(x)` is `u64be(x.len())` followed by `x`:
//!
//! ```text
//! u32be(TRANSCRIPT_FORMAT_VERSION)
//! u32be(source-claim slot)
//! topology
//! [selected schedule, action roster]                         simulated only
//! entry roster
//! ```
//!
//! A topology is its named nodes and links in authored order.
//! A selected schedule is its name, ordered link disciplines, and ordered fault rosters.
//! An action is one byte-valued send or one logical-tick advance.
//! An entry retains the delivery's complete lineage.
//! Every roster starts with `u64be(count)` and every name is its two length-prefixed parts.
//!
//! The address is that body under [`TRANSCRIPT_TAG`], and the complete envelope is the address followed by the body it addresses.
//! An address never covers itself.

use super::sim::Action;
use super::{
    Delivery, DeliveryCopy, Link, LinkDiscipline, LinkFault, NetworkSchedule, NetworkSelection,
    Replay, ReplayExhaustion, ReplayIncomplete, SendOrdinal, SimNet, SimulationAction,
    SimulationManifest, SimulationReproduction, TRANSCRIPT_FORMAT_VERSION, TRANSCRIPT_TAG, Tick,
    Topology, TranscriptAddress, TranscriptEntry, TranscriptMaterial, TranscriptPack,
    TranscriptRefusal, TranscriptSourceClaim,
};
use crate::descriptor::NamespacedName;
use crate::identity::ContentAddress;
use crate::report::{encode_bytes, encode_length};

/// Write one content-addressed live-recorded transcript in delivery order.
///
/// The address is derived here rather than accepted from anyone, and every row is judged against the topology before a byte is written.
/// This road records an adopter's source claim and mints no simulation standing.
///
/// # Errors
///
/// Refuses a transcript with no delivery, then the first entry on a link the topology never declared, then the first entry delivered before its own send, then the first entry stamped earlier than the entry before it.
pub fn recorded_live(
    topology: &Topology,
    entries: Vec<TranscriptEntry>,
) -> Result<TranscriptPack, TranscriptRefusal> {
    write_pack(topology.clone(), TranscriptMaterial::RecordedLive, entries)
}

/// Write one content-addressed simulated transcript from the sim that retained its whole drive.
///
/// The payload encoder supplies the byte projection used both by the input actions and by the resulting delivery rows.
/// The writer executes the completed byte-valued manifest again before returning its separately minted reproduction standing.
///
/// # Errors
///
/// Refuses everything the ordinary transcript writer refuses, then any failure to re-open, drive, or exactly reproduce the retained manifest.
pub fn simulated<Payload>(
    sim: &SimNet<Payload>,
    encode_payload: fn(&Payload) -> Vec<u8>,
) -> Result<(TranscriptPack, SimulationReproduction), TranscriptRefusal> {
    let actions = encoded_actions(&sim.actions, encode_payload);
    let entries = sim
        .history
        .iter()
        .map(|delivery| encoded_delivery(delivery, encode_payload))
        .collect();
    let material =
        TranscriptMaterial::Simulated(SimulationManifest::captured(sim.schedule.clone(), actions));
    let pack = write_pack(sim.topology.clone(), material, entries)?;
    let reproduction = reproduce(&pack)?;
    Ok((pack, reproduction))
}

/// Read one live-recorded transcript envelope for the topology the caller expects.
///
/// The leading claim is settled before a member of the body is interpreted, and the body must declare the live-recorded source posture.
///
/// # Errors
///
/// Refuses every malformed envelope in reading order and a body whose source claim is not live-recorded.
pub fn read_recorded_live(
    expected: &Topology,
    encoded: &[u8],
) -> Result<TranscriptPack, TranscriptRefusal> {
    read_as(expected, None, TranscriptSourceClaim::RecordedLive, encoded)
}

/// Read one simulated transcript envelope for the topology and selected schedule the caller expects.
///
/// The static names remain owner-built values: foreign bytes are compared with the supplied schedule and never mint names.
/// Reading admits addressed declaration material only; [`reproduce`] is the separate operation that can mint simulation standing.
///
/// # Errors
///
/// Refuses every malformed envelope in reading order, a body whose source claim is not simulated, or a schedule different from the owner-built expected value.
pub fn read_simulated(
    expected: &Topology,
    schedule: &NetworkSchedule,
    encoded: &[u8],
) -> Result<TranscriptPack, TranscriptRefusal> {
    read_as(
        expected,
        Some(schedule),
        TranscriptSourceClaim::Simulated,
        encoded,
    )
}

/// Execute one admitted simulation manifest and compare its complete delivery roster with the addressed rows.
///
/// # Errors
///
/// Refuses recorded-live material, a schedule that cannot open over the retained topology, a send the reproduced sim refuses, or the first reproduced row that differs or is absent.
pub fn reproduce(pack: &TranscriptPack) -> Result<SimulationReproduction, TranscriptRefusal> {
    let TranscriptMaterial::Simulated(manifest) = &pack.material else {
        return Err(TranscriptRefusal::RecordedLiveCannotReproduce);
    };
    let selection = NetworkSelection {
        schedule: manifest.schedule(),
    };
    let mut sim = SimNet::declared(pack.topology.clone(), selection)
        .map_err(TranscriptRefusal::SimulationNotOpened)?;
    let mut deliveries = Vec::new();
    for (at, action) in manifest.actions().iter().enumerate() {
        match action {
            SimulationAction::Send { link, payload } => {
                sim.send(*link, payload.clone())
                    .map_err(|refusal| TranscriptRefusal::SimulationSendRefused { at, refusal })?;
            }
            SimulationAction::Advance => deliveries.extend(sim.advance()),
        }
    }
    let reproduced: Vec<_> = deliveries
        .iter()
        .map(|delivery| encoded_delivery(delivery, Vec::clone))
        .collect();
    if let Some(at) = first_divergence(&reproduced, pack.entries()) {
        return Err(TranscriptRefusal::SimulationRowsDiverge { at });
    }
    Ok(SimulationReproduction::witnessed(
        pack.address(),
        manifest.actions().len(),
        reproduced.len(),
        sim.tick(),
    ))
}

impl Replay {
    /// Open one admitted pack for playback at tick zero, handing back every delivery already due at the opening tick.
    ///
    /// A live recording whose epoch starts at zero lawfully carries a delivery stamped at tick zero, and the opening is where it plays — never shifted onto a later tick.
    /// The simulator's own transcripts never carry one, because its time law places every delivery at least one tick after its send; for them the opening hand is empty and playback is unchanged.
    /// The drain rides the constructor rather than a second call, so there is no road to a replay value that has not already surrendered its opening hand — what a caller does with a hand it was dealt is its own affair, at this tick as at every later one.
    #[must_use]
    pub fn opened(pack: &TranscriptPack) -> (Self, Vec<Delivery<Vec<u8>>>) {
        let mut replay = Self {
            address: pack.address(),
            entries: pack.entries().to_vec(),
            total: pack.entries().len(),
            at: 0usize,
            tick: Tick::at(0u64),
        };
        let opening = replay.due_now();
        (replay, opening)
    }

    /// The current logical tick.
    #[must_use]
    pub const fn tick(&self) -> Tick {
        self.tick
    }

    /// How many recorded deliveries have not yet been handed out.
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.entries.len().saturating_sub(self.at)
    }

    /// Advance one tick and hand back every recorded delivery due by it, stamps included.
    ///
    /// The deliveries are exactly the record's, in the record's order — a replay invents nothing and reorders nothing.
    #[must_use]
    pub fn advance(&mut self) -> Vec<Delivery<Vec<u8>>> {
        self.tick = self.tick.next();
        self.due_now()
    }

    /// Consume playback and mint evidence that every addressed row was handed out.
    ///
    /// # Errors
    ///
    /// Refuses while any recorded row remains, naming exactly how many were never handed out.
    pub fn exhaust(self) -> Result<ReplayExhaustion, ReplayIncomplete> {
        let remaining = self.remaining();
        if remaining != 0usize {
            return Err(ReplayIncomplete::witnessed(self.address, remaining));
        }
        Ok(ReplayExhaustion::witnessed(
            self.address,
            self.total,
            self.tick,
        ))
    }

    /// Every recorded delivery due by the current tick and not yet handed out, stamps included.
    fn due_now(&mut self) -> Vec<Delivery<Vec<u8>>> {
        let now = self.tick;
        let mut played = Vec::new();
        while let Some(entry) = self.entries.get(self.at) {
            if entry.delivered_at() > now {
                break;
            }
            played.push(Delivery::delivered(
                entry.link(),
                entry.ordinal(),
                entry.payload().to_vec(),
                entry.sent_at(),
                entry.delivered_at(),
                entry.copy(),
            ));
            self.at = self.at.saturating_add(1usize);
        }
        played
    }
}

/// Encode every retained sim action through the caller's declared payload projection.
fn encoded_actions<Payload>(
    actions: &[Action<Payload>],
    encode_payload: fn(&Payload) -> Vec<u8>,
) -> Vec<SimulationAction> {
    actions
        .iter()
        .map(|action| match action {
            Action::Send { link, payload } => SimulationAction::Send {
                link: *link,
                payload: encode_payload(payload),
            },
            Action::Advance => SimulationAction::Advance,
        })
        .collect()
}

/// Encode one delivery without changing its lineage.
fn encoded_delivery<Payload>(
    delivery: &Delivery<Payload>,
    encode_payload: fn(&Payload) -> Vec<u8>,
) -> TranscriptEntry {
    TranscriptEntry::witnessed(
        delivery.link(),
        delivery.ordinal(),
        encode_payload(delivery.payload()),
        delivery.sent_at(),
        delivery.delivered_at(),
        delivery.copy(),
    )
}

/// Build and address a pack after its source-specific caller has assembled complete material.
fn write_pack(
    topology: Topology,
    material: TranscriptMaterial,
    entries: Vec<TranscriptEntry>,
) -> Result<TranscriptPack, TranscriptRefusal> {
    lawful_entries(&topology, &entries)?;
    let body = encode_body(&material, &topology, &entries);
    let address = TranscriptAddress::derived(ContentAddress::derived(TRANSCRIPT_TAG, &body));
    let claim = address.address();
    let mut encoded = Vec::with_capacity(claim.as_bytes().len().saturating_add(body.len()));
    encoded.extend_from_slice(claim.as_bytes());
    encoded.extend_from_slice(&body);
    Ok(TranscriptPack::assembled(
        topology, material, address, entries, encoded,
    ))
}

/// Read one envelope under the source-specific public road that called here.
fn read_as(
    expected: &Topology,
    schedule: Option<&NetworkSchedule>,
    source: TranscriptSourceClaim,
    encoded: &[u8],
) -> Result<TranscriptPack, TranscriptRefusal> {
    let (address, body) = addressed_body(encoded)?;
    let (material, entries) = read_body(expected, schedule, source, body)?;
    lawful_entries(expected, &entries)?;
    Ok(TranscriptPack::assembled(
        expected.clone(),
        material,
        address,
        entries,
        encoded.to_vec(),
    ))
}

/// Whether every row belongs to the topology, no delivery precedes its own send, and the stamps never step backward.
fn lawful_entries(
    topology: &Topology,
    entries: &[TranscriptEntry],
) -> Result<(), TranscriptRefusal> {
    if entries.is_empty() {
        return Err(TranscriptRefusal::NoDelivery);
    }
    let mut latest = Tick::at(0u64);
    for (at, entry) in entries.iter().enumerate() {
        if !topology.links().contains(&entry.link()) {
            return Err(TranscriptRefusal::ForeignLink { at });
        }
        if entry.delivered_at() < entry.sent_at() {
            return Err(TranscriptRefusal::DeliveryBeforeSend { at });
        }
        if entry.delivered_at() < latest {
            return Err(TranscriptRefusal::DeliveryOrderBroken { at });
        }
        latest = entry.delivered_at();
    }
    Ok(())
}

/// The complete address preimage of one transcript.
fn encode_body(
    material: &TranscriptMaterial,
    topology: &Topology,
    entries: &[TranscriptEntry],
) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(&TRANSCRIPT_FORMAT_VERSION.to_be_bytes());
    body.extend_from_slice(&source_slot(source_claim(material)).to_be_bytes());
    encode_topology(topology, &mut body);
    if let TranscriptMaterial::Simulated(manifest) = material {
        encode_manifest(manifest, &mut body);
    }
    encode_entries(entries, &mut body);
    body
}

/// Append one topology in authored order.
fn encode_topology(topology: &Topology, into: &mut Vec<u8>) {
    encode_length(topology.nodes().len(), into);
    for node in topology.nodes() {
        encode_node(*node, into);
    }
    encode_length(topology.links().len(), into);
    for link in topology.links() {
        encode_link(*link, into);
    }
}

/// Append one complete selected-schedule and action manifest.
fn encode_manifest(manifest: &SimulationManifest, into: &mut Vec<u8>) {
    let schedule = manifest.schedule();
    schedule.name().encode_into(into);
    encode_length(schedule.disciplines().len(), into);
    for discipline in schedule.disciplines() {
        encode_discipline(discipline, into);
    }
    encode_length(manifest.actions().len(), into);
    for action in manifest.actions() {
        encode_action(action, into);
    }
}

/// Append one link discipline and its ordered fault roster.
fn encode_discipline(discipline: &LinkDiscipline, into: &mut Vec<u8>) {
    encode_link(discipline.link(), into);
    encode_length(discipline.faults().len(), into);
    for fault in discipline.faults() {
        encode_fault(*fault, into);
    }
}

/// Append one fault under its stable slot.
fn encode_fault(fault: LinkFault, into: &mut Vec<u8>) {
    match fault {
        LinkFault::DropAt { position } => {
            into.extend_from_slice(&0u32.to_be_bytes());
            into.extend_from_slice(&position.ordinal().to_be_bytes());
        }
        LinkFault::DelayAt { position, ticks } => {
            into.extend_from_slice(&1u32.to_be_bytes());
            into.extend_from_slice(&position.ordinal().to_be_bytes());
            into.extend_from_slice(&ticks.ticks().to_be_bytes());
        }
        LinkFault::DuplicateAt { position } => {
            into.extend_from_slice(&2u32.to_be_bytes());
            into.extend_from_slice(&position.ordinal().to_be_bytes());
        }
        LinkFault::Partition { opens, heals } => {
            into.extend_from_slice(&3u32.to_be_bytes());
            into.extend_from_slice(&opens.ordinal().to_be_bytes());
            into.extend_from_slice(&heals.ordinal().to_be_bytes());
        }
    }
}

/// Append one simulation action under its stable slot.
fn encode_action(action: &SimulationAction, into: &mut Vec<u8>) {
    match action {
        SimulationAction::Send { link, payload } => {
            into.extend_from_slice(&0u32.to_be_bytes());
            encode_link(*link, into);
            encode_bytes(payload, into);
        }
        SimulationAction::Advance => into.extend_from_slice(&1u32.to_be_bytes()),
    }
}

/// Append the ordered delivery roster.
fn encode_entries(entries: &[TranscriptEntry], into: &mut Vec<u8>) {
    encode_length(entries.len(), into);
    for entry in entries {
        encode_link(entry.link(), into);
        into.extend_from_slice(&entry.ordinal().ordinal().to_be_bytes());
        encode_bytes(entry.payload(), into);
        into.extend_from_slice(&entry.sent_at().ordinal().to_be_bytes());
        into.extend_from_slice(&entry.delivered_at().ordinal().to_be_bytes());
        into.extend_from_slice(&copy_slot(entry.copy()).to_be_bytes());
    }
}

/// Append one node's name through the type's own seated spelling.
fn encode_node(node: super::NodeRef, into: &mut Vec<u8>) {
    node.name().encode_into(into);
}

/// Append one link's four name parts.
fn encode_link(link: Link, into: &mut Vec<u8>) {
    encode_node(link.from(), into);
    encode_node(link.to(), into);
}

/// The source claim retained by one internal material arm.
const fn source_claim(material: &TranscriptMaterial) -> TranscriptSourceClaim {
    match material {
        TranscriptMaterial::Simulated(_) => TranscriptSourceClaim::Simulated,
        TranscriptMaterial::RecordedLive => TranscriptSourceClaim::RecordedLive,
    }
}

/// The stable slot one source claim occupies.
const fn source_slot(source: TranscriptSourceClaim) -> u32 {
    match source {
        TranscriptSourceClaim::Simulated => 0u32,
        TranscriptSourceClaim::RecordedLive => 1u32,
    }
}

/// The source claim one stable slot spells.
const fn source_of(slot: u32) -> Result<TranscriptSourceClaim, TranscriptRefusal> {
    match slot {
        0u32 => Ok(TranscriptSourceClaim::Simulated),
        1u32 => Ok(TranscriptSourceClaim::RecordedLive),
        found => Err(TranscriptRefusal::UnknownSourceClaim { found }),
    }
}

/// The stable slot one delivery-copy posture occupies.
const fn copy_slot(copy: DeliveryCopy) -> u32 {
    match copy {
        DeliveryCopy::Original => 0u32,
        DeliveryCopy::Duplicate => 1u32,
    }
}

/// The delivery-copy posture one stable slot spells.
const fn copy_of(slot: u32) -> Result<DeliveryCopy, TranscriptRefusal> {
    match slot {
        0u32 => Ok(DeliveryCopy::Original),
        1u32 => Ok(DeliveryCopy::Duplicate),
        found => Err(TranscriptRefusal::UnknownCopy { found }),
    }
}

/// Split the envelope at its address claim and keep the body only if the body derives that claim.
fn addressed_body(encoded: &[u8]) -> Result<(TranscriptAddress, &[u8]), TranscriptRefusal> {
    let width = ContentAddress::derived(TRANSCRIPT_TAG, &[])
        .as_bytes()
        .len();
    let Some((claimed, body)) = encoded.split_at_checked(width) else {
        return Err(TranscriptRefusal::Truncated);
    };
    let address = TranscriptAddress::derived(ContentAddress::derived(TRANSCRIPT_TAG, body));
    if claimed != address.address().as_bytes() {
        return Err(TranscriptRefusal::AddressMismatch { derived: address });
    }
    Ok((address, body))
}

/// Read every member the body declares under the source-specific public road.
fn read_body(
    expected: &Topology,
    expected_schedule: Option<&NetworkSchedule>,
    expected_source: TranscriptSourceClaim,
    body: &[u8],
) -> Result<(TranscriptMaterial, Vec<TranscriptEntry>), TranscriptRefusal> {
    let mut reader = BodyReader::over(body);
    let found = reader.u32()?;
    if found != TRANSCRIPT_FORMAT_VERSION {
        return Err(TranscriptRefusal::UnsupportedFormat { found });
    }
    let source = source_of(reader.u32()?)?;
    if source != expected_source {
        return Err(TranscriptRefusal::SourceClaimMismatch {
            expected: expected_source,
            found: source,
        });
    }
    read_topology(expected, &mut reader)?;
    let material = read_material(source, expected, expected_schedule, &mut reader)?;
    let entries = read_entries(expected, &mut reader)?;
    let trailing = reader.remaining();
    if trailing != 0usize {
        return Err(TranscriptRefusal::TrailingBytes { count: trailing });
    }
    Ok((material, entries))
}

/// Read source-specific material after the common header and topology.
fn read_material(
    source: TranscriptSourceClaim,
    expected: &Topology,
    expected_schedule: Option<&NetworkSchedule>,
    reader: &mut BodyReader<'_>,
) -> Result<TranscriptMaterial, TranscriptRefusal> {
    match source {
        TranscriptSourceClaim::RecordedLive => Ok(TranscriptMaterial::RecordedLive),
        TranscriptSourceClaim::Simulated => {
            let Some(schedule) = expected_schedule else {
                return Err(TranscriptRefusal::ScheduleMismatch);
            };
            read_schedule(schedule, reader)?;
            let actions = read_actions(expected, reader)?;
            Ok(TranscriptMaterial::Simulated(SimulationManifest::captured(
                schedule.clone(),
                actions,
            )))
        }
    }
}

/// Compare the encoded topology section with the owner-built expected value.
fn read_topology(
    expected: &Topology,
    reader: &mut BodyReader<'_>,
) -> Result<(), TranscriptRefusal> {
    let nodes = reader.count()?;
    if nodes != expected.nodes().len() {
        return Err(TranscriptRefusal::TopologyMismatch);
    }
    for node in expected.nodes() {
        read_expected_name(node.name(), reader, TranscriptRefusal::TopologyMismatch)?;
    }
    let links = reader.count()?;
    if links != expected.links().len() {
        return Err(TranscriptRefusal::TopologyMismatch);
    }
    for link in expected.links() {
        read_expected_link(*link, reader, TranscriptRefusal::TopologyMismatch)?;
    }
    Ok(())
}

/// Compare the encoded schedule section with the owner-built expected value.
fn read_schedule(
    expected: &NetworkSchedule,
    reader: &mut BodyReader<'_>,
) -> Result<(), TranscriptRefusal> {
    read_expected_name(expected.name(), reader, TranscriptRefusal::ScheduleMismatch)?;
    let count = reader.count()?;
    if count != expected.disciplines().len() {
        return Err(TranscriptRefusal::ScheduleMismatch);
    }
    for discipline in expected.disciplines() {
        read_discipline(discipline, reader)?;
    }
    Ok(())
}

/// Compare one encoded discipline and fault roster with the expected value.
fn read_discipline(
    expected: &LinkDiscipline,
    reader: &mut BodyReader<'_>,
) -> Result<(), TranscriptRefusal> {
    read_expected_link(expected.link(), reader, TranscriptRefusal::ScheduleMismatch)?;
    let count = reader.count()?;
    if count != expected.faults().len() {
        return Err(TranscriptRefusal::ScheduleMismatch);
    }
    for fault in expected.faults() {
        read_fault(*fault, reader)?;
    }
    Ok(())
}

/// Compare one encoded fault with its expected typed value.
fn read_fault(expected: LinkFault, reader: &mut BodyReader<'_>) -> Result<(), TranscriptRefusal> {
    let slot = reader.u32()?;
    let matches = match slot {
        0u32 => {
            expected
                == LinkFault::DropAt {
                    position: SendOrdinal::at(reader.u32()?),
                }
        }
        1u32 => {
            let position = SendOrdinal::at(reader.u32()?);
            let ticks = reader.u32()?;
            matches!(expected, LinkFault::DelayAt { position: expected_position, ticks: expected_ticks } if expected_position == position && expected_ticks.ticks() == ticks)
        }
        2u32 => {
            expected
                == LinkFault::DuplicateAt {
                    position: SendOrdinal::at(reader.u32()?),
                }
        }
        3u32 => {
            expected
                == LinkFault::Partition {
                    opens: Tick::at(reader.u64()?),
                    heals: Tick::at(reader.u64()?),
                }
        }
        found => return Err(TranscriptRefusal::UnknownFault { found }),
    };
    if !matches {
        return Err(TranscriptRefusal::ScheduleMismatch);
    }
    Ok(())
}

/// Read the complete action roster, resolving every send link against the expected topology.
fn read_actions(
    expected: &Topology,
    reader: &mut BodyReader<'_>,
) -> Result<Vec<SimulationAction>, TranscriptRefusal> {
    let count = reader.count()?;
    let mut actions = Vec::new();
    for at in 0..count {
        match reader.u32()? {
            0u32 => {
                let link = read_action_link(expected, at, reader)?;
                let payload = reader.bytes()?.to_vec();
                actions.push(SimulationAction::Send { link, payload });
            }
            1u32 => actions.push(SimulationAction::Advance),
            found => return Err(TranscriptRefusal::UnknownAction { found }),
        }
    }
    Ok(actions)
}

/// Read the entry roster, resolving each row's link against the expected topology.
fn read_entries(
    expected: &Topology,
    reader: &mut BodyReader<'_>,
) -> Result<Vec<TranscriptEntry>, TranscriptRefusal> {
    let count = reader.count()?;
    let mut entries = Vec::new();
    for at in 0..count {
        let link = read_entry_link(expected, at, reader)?;
        let ordinal = SendOrdinal::at(reader.u32()?);
        let payload = reader.bytes()?.to_vec();
        let sent_at = Tick::at(reader.u64()?);
        let delivered_at = Tick::at(reader.u64()?);
        let copy = copy_of(reader.u32()?)?;
        entries.push(TranscriptEntry::witnessed(
            link,
            ordinal,
            payload,
            sent_at,
            delivered_at,
            copy,
        ));
    }
    Ok(entries)
}

/// Read one expected name without minting names from foreign bytes.
fn read_expected_name(
    expected: NamespacedName,
    reader: &mut BodyReader<'_>,
    mismatch: TranscriptRefusal,
) -> Result<(), TranscriptRefusal> {
    let namespace = reader.bytes()?;
    let stem = reader.bytes()?;
    if namespace != expected.namespace().written().as_bytes()
        || stem != expected.stem().written().as_bytes()
    {
        return Err(mismatch);
    }
    Ok(())
}

/// Read one expected link without minting node names from foreign bytes.
fn read_expected_link(
    expected: Link,
    reader: &mut BodyReader<'_>,
    mismatch: TranscriptRefusal,
) -> Result<(), TranscriptRefusal> {
    read_expected_name(expected.from().name(), reader, mismatch)?;
    read_expected_name(expected.to().name(), reader, mismatch)
}

/// Resolve one action link from encoded name bytes.
fn read_action_link(
    expected: &Topology,
    at: usize,
    reader: &mut BodyReader<'_>,
) -> Result<Link, TranscriptRefusal> {
    read_link(expected, reader)?.ok_or(TranscriptRefusal::SimulationActionForeignLink { at })
}

/// Resolve one delivery-entry link from encoded name bytes.
fn read_entry_link(
    expected: &Topology,
    at: usize,
    reader: &mut BodyReader<'_>,
) -> Result<Link, TranscriptRefusal> {
    read_link(expected, reader)?.ok_or(TranscriptRefusal::ForeignLink { at })
}

/// Read four name parts and resolve them against the expected topology's links.
fn read_link(
    expected: &Topology,
    reader: &mut BodyReader<'_>,
) -> Result<Option<Link>, TranscriptRefusal> {
    let from_namespace = reader.bytes()?.to_vec();
    let from_stem = reader.bytes()?.to_vec();
    let to_namespace = reader.bytes()?.to_vec();
    let to_stem = reader.bytes()?.to_vec();
    Ok(expected
        .links()
        .iter()
        .find(|link| {
            spells(link.from(), &from_namespace, &from_stem)
                && spells(link.to(), &to_namespace, &to_stem)
        })
        .copied())
}

/// Whether this node's name is spelled by these two byte strings.
fn spells(node: super::NodeRef, namespace: &[u8], stem: &[u8]) -> bool {
    let name = node.name();
    name.namespace().written().as_bytes() == namespace && name.stem().written().as_bytes() == stem
}

/// The first roster position where two exact delivery accounts differ.
fn first_divergence(left: &[TranscriptEntry], right: &[TranscriptEntry]) -> Option<usize> {
    let common = left.len().min(right.len());
    left.iter()
        .zip(right)
        .position(|(left, right)| left != right)
        .or_else(|| (left.len() != right.len()).then_some(common))
}

/// A cursor over one body, which never indexes and never trusts a declared width.
struct BodyReader<'body> {
    body: &'body [u8],
    at: usize,
}

impl<'body> BodyReader<'body> {
    /// Open at the first body byte.
    const fn over(body: &'body [u8]) -> Self {
        Self { body, at: 0 }
    }

    /// Read one fixed-width 32-bit integer.
    fn u32(&mut self) -> Result<u32, TranscriptRefusal> {
        self.fixed::<4>().map(u32::from_be_bytes)
    }

    /// Read one fixed-width 64-bit integer.
    fn u64(&mut self) -> Result<u64, TranscriptRefusal> {
        self.fixed::<8>().map(u64::from_be_bytes)
    }

    /// Read one declared count, refused where the platform cannot index it.
    fn count(&mut self) -> Result<usize, TranscriptRefusal> {
        let declared = self.u64()?;
        usize::try_from(declared)
            .map_err(|_beyond_platform| TranscriptRefusal::LengthOutsidePlatform { declared })
    }

    /// Read one length-prefixed byte string.
    fn bytes(&mut self) -> Result<&'body [u8], TranscriptRefusal> {
        let length = self.count()?;
        self.take(length)
    }

    /// Read one fixed-width byte array.
    fn fixed<const WIDTH: usize>(&mut self) -> Result<[u8; WIDTH], TranscriptRefusal> {
        let bytes = self.take(WIDTH)?;
        <[u8; WIDTH]>::try_from(bytes).map_err(|_unexpected_width| TranscriptRefusal::Truncated)
    }

    /// Advance over exactly this many bytes, or refuse the envelope as truncated.
    fn take(&mut self, width: usize) -> Result<&'body [u8], TranscriptRefusal> {
        let Some(end) = self.at.checked_add(width) else {
            return Err(TranscriptRefusal::Truncated);
        };
        let Some(bytes) = self.body.get(self.at..end) else {
            return Err(TranscriptRefusal::Truncated);
        };
        self.at = end;
        Ok(bytes)
    }

    /// How many bytes are still unread.
    const fn remaining(&self) -> usize {
        self.body.len().saturating_sub(self.at)
    }
}
