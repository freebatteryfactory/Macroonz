//! Transcript custody, reproduction, and replay evidence exercised through the public surface.
//!
//! A simulated pack can arise only from a driven sim, decoding admits no reproduction standing, exact re-execution mints it, and exhausted playback joins only on the same addressed transcript.

use macroonz_harness::descriptor::{NameRefusal, NamespacedName};
use macroonz_harness::identity::ContentAddress;
use macroonz_harness::network::{
    Delivery, DeliveryCopy, Link, LinkDiscipline, LinkFault, NetworkCampaign,
    NetworkCampaignRefusal, NetworkSchedule, NetworkScheduleRefusal, NetworkSelectionRefusal,
    NodeRef, Replay, ReplayIncomplete, ReproducedReplay, ReproducedReplayRefusal, SendOrdinal,
    SendRefusal, SimNet, SimNetRefusal, SimulationReproduction, TRANSCRIPT_TAG, Tick, Topology,
    TopologyRefusal, TranscriptEntry, TranscriptPack, TranscriptRefusal, TranscriptSourceClaim,
    read_recorded_live, read_simulated, recorded_live, reproduce, simulated,
};

/// Everything a lane road can refuse, carried as itself.
enum LaneFailure {
    Name(NameRefusal),
    Topology(TopologyRefusal),
    Schedule(NetworkScheduleRefusal),
    Campaign(NetworkCampaignRefusal),
    Selection(NetworkSelectionRefusal),
    Sim(SimNetRefusal),
    Send(SendRefusal),
    Transcript(TranscriptRefusal),
    Replay(ReplayIncomplete),
    Join(ReproducedReplayRefusal),
    /// A value did not carry the shape the claim demanded.
    Standing,
}

impl core::fmt::Debug for LaneFailure {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Name(refusal) => formatter.debug_tuple("Name").field(refusal).finish(),
            Self::Topology(refusal) => formatter.debug_tuple("Topology").field(refusal).finish(),
            Self::Schedule(refusal) => formatter.debug_tuple("Schedule").field(refusal).finish(),
            Self::Campaign(refusal) => formatter.debug_tuple("Campaign").field(refusal).finish(),
            Self::Selection(refusal) => formatter.debug_tuple("Selection").field(refusal).finish(),
            Self::Sim(refusal) => formatter.debug_tuple("Sim").field(refusal).finish(),
            Self::Send(refusal) => formatter.debug_tuple("Send").field(refusal).finish(),
            Self::Transcript(refusal) => {
                formatter.debug_tuple("Transcript").field(refusal).finish()
            }
            Self::Replay(refusal) => formatter.debug_tuple("Replay").field(refusal).finish(),
            Self::Join(refusal) => formatter.debug_tuple("Join").field(refusal).finish(),
            Self::Standing => formatter.write_str("Standing"),
        }
    }
}

impl From<NameRefusal> for LaneFailure {
    fn from(refusal: NameRefusal) -> Self {
        Self::Name(refusal)
    }
}

impl From<TopologyRefusal> for LaneFailure {
    fn from(refusal: TopologyRefusal) -> Self {
        Self::Topology(refusal)
    }
}

impl From<NetworkScheduleRefusal> for LaneFailure {
    fn from(refusal: NetworkScheduleRefusal) -> Self {
        Self::Schedule(refusal)
    }
}

impl From<NetworkCampaignRefusal> for LaneFailure {
    fn from(refusal: NetworkCampaignRefusal) -> Self {
        Self::Campaign(refusal)
    }
}

impl From<NetworkSelectionRefusal> for LaneFailure {
    fn from(refusal: NetworkSelectionRefusal) -> Self {
        Self::Selection(refusal)
    }
}

impl From<SimNetRefusal> for LaneFailure {
    fn from(refusal: SimNetRefusal) -> Self {
        Self::Sim(refusal)
    }
}

impl From<SendRefusal> for LaneFailure {
    fn from(refusal: SendRefusal) -> Self {
        Self::Send(refusal)
    }
}

impl From<TranscriptRefusal> for LaneFailure {
    fn from(refusal: TranscriptRefusal) -> Self {
        Self::Transcript(refusal)
    }
}

impl From<ReplayIncomplete> for LaneFailure {
    fn from(refusal: ReplayIncomplete) -> Self {
        Self::Replay(refusal)
    }
}

impl From<ReproducedReplayRefusal> for LaneFailure {
    fn from(refusal: ReproducedReplayRefusal) -> Self {
        Self::Join(refusal)
    }
}

/// One lane-owned name.
fn name(stem: &'static str) -> Result<NamespacedName, NameRefusal> {
    NamespacedName::named("lane", stem)
}

/// The client-to-server link over the pair topology.
fn forward() -> Result<Link, NameRefusal> {
    Ok(Link::between(
        NodeRef::declared(name("client")?),
        NodeRef::declared(name("server")?),
    ))
}

/// Two nodes, one link each way.
fn pair_topology() -> Result<Topology, LaneFailure> {
    let client = NodeRef::declared(name("client")?);
    let server = NodeRef::declared(name("server")?);
    Ok(Topology::declared(
        vec![client, server],
        vec![Link::between(client, server), Link::between(server, client)],
    )?)
}

/// The schedule that duplicates the first client request.
fn duplicate_schedule() -> Result<NetworkSchedule, LaneFailure> {
    Ok(NetworkSchedule::declared(
        name("duplicate-the-request")?,
        vec![LinkDiscipline::declared(
            forward()?,
            vec![LinkFault::DuplicateAt {
                position: SendOrdinal::at(0u32),
            }],
        )],
    )?)
}

/// One driven sim, its selected schedule, and every delivery handed out by that drive.
type DrivenRun = (Vec<Delivery<Vec<u8>>>, SimNet<Vec<u8>>, NetworkSchedule);

/// Drive the duplicating schedule, optionally recording extra empty advances after delivery.
fn duplicated_run(extra_advances: usize) -> Result<DrivenRun, LaneFailure> {
    let schedule = duplicate_schedule()?;
    let campaign = NetworkCampaign::declared(vec![schedule.clone()])?;
    let mut sim = SimNet::declared(
        pair_topology()?,
        campaign.select(name("duplicate-the-request")?)?,
    )?;
    sim.send(forward()?, b"pay".to_vec())?;
    let mut deliveries = Vec::new();
    while sim.pending() > 0usize {
        deliveries.extend(sim.advance());
    }
    for _ in 0..extra_advances {
        deliveries.extend(sim.advance());
    }
    Ok((deliveries, sim, schedule))
}

/// One run's deliveries beside its pack, selected schedule, and minted reproduction.
type PackedRun = (
    Vec<Delivery<Vec<u8>>>,
    NetworkSchedule,
    TranscriptPack,
    SimulationReproduction,
);

/// Drive and pack one simulated run.
fn packed_run(extra_advances: usize) -> Result<PackedRun, LaneFailure> {
    let (deliveries, sim, schedule) = duplicated_run(extra_advances)?;
    let (pack, reproduction) = simulated(&sim, Vec::clone)?;
    Ok((deliveries, schedule, pack, reproduction))
}

/// One lawful live-recorded row.
fn live_entry(
    payload: &[u8],
    ordinal: u32,
    delivered_at: u64,
) -> Result<TranscriptEntry, LaneFailure> {
    Ok(TranscriptEntry::witnessed(
        forward()?,
        SendOrdinal::at(ordinal),
        payload.to_vec(),
        Tick::at(delivered_at.saturating_sub(1u64)),
        Tick::at(delivered_at),
        DeliveryCopy::Original,
    ))
}

/// Replace the envelope's address after one hostile body edit.
fn readdress(encoded: &mut [u8]) -> Result<(), LaneFailure> {
    let address_width = ContentAddress::derived(TRANSCRIPT_TAG, &[])
        .as_bytes()
        .len();
    let body = encoded.get(address_width..).ok_or(LaneFailure::Standing)?;
    let address = ContentAddress::derived(TRANSCRIPT_TAG, body);
    let claim = encoded
        .get_mut(0usize..address_width)
        .ok_or(LaneFailure::Standing)?;
    claim.copy_from_slice(address.as_bytes());
    Ok(())
}

/// A complete simulated manifest packs, reads as declaration material, reproduces, and replays under exact address custody.
#[test]
fn a_simulated_run_reproduces_and_replays_under_one_address() -> Result<(), LaneFailure> {
    let (deliveries, schedule, pack, written_reproduction) = packed_run(0usize)?;
    assert_eq!(deliveries.len(), 2usize);
    assert_eq!(pack.source_claim(), TranscriptSourceClaim::Simulated);
    let manifest = pack.simulation_manifest().ok_or(LaneFailure::Standing)?;
    assert_eq!(manifest.schedule(), &schedule);
    assert_eq!(manifest.actions().len(), 2usize);
    assert_eq!(written_reproduction.address(), pack.address());
    assert_eq!(written_reproduction.rows(), 2usize);

    let reread = read_simulated(&pair_topology()?, &schedule, pack.encoded())?;
    assert_eq!(reread, pack);
    let decoded_reproduction = reproduce(&reread)?;
    assert_eq!(decoded_reproduction, written_reproduction);

    let (mut replay, opening) = Replay::opened(&reread);
    assert!(opening.is_empty());
    let mut played = Vec::new();
    while replay.remaining() > 0usize {
        played.extend(replay.advance());
    }
    assert_eq!(played, deliveries);
    let exhaustion = replay.exhaust()?;
    assert_eq!(exhaustion.address(), pack.address());
    assert_eq!(exhaustion.total(), 2usize);
    let joined = ReproducedReplay::joined(decoded_reproduction, exhaustion)?;
    assert_eq!(joined.reproduction(), decoded_reproduction);
    assert_eq!(joined.exhaustion(), exhaustion);
    Ok(())
}

/// Identical drives derive one address, while an extra empty advance moves the manifest identity without changing the delivery rows.
#[test]
fn the_complete_action_manifest_moves_the_address() -> Result<(), LaneFailure> {
    let (_first_rows, _first_schedule, first, _first_reproduction) = packed_run(0usize)?;
    let (_second_rows, _second_schedule, second, _second_reproduction) = packed_run(0usize)?;
    assert_eq!(first.address(), second.address());
    assert_eq!(first.encoded(), second.encoded());

    let (_extended_rows, _extended_schedule, extended, standing) = packed_run(1usize)?;
    assert_eq!(first.entries(), extended.entries());
    assert_ne!(first.address(), extended.address());
    assert_ne!(first.encoded(), extended.encoded());
    assert_eq!(standing.actions(), 3usize);
    assert_eq!(standing.final_tick(), Tick::at(2u64));
    Ok(())
}

/// A dropped send remains in the complete action denominator even though it produces no delivery row.
#[test]
fn dropped_inputs_remain_in_the_reproduced_manifest() -> Result<(), LaneFailure> {
    let schedule = NetworkSchedule::declared(
        name("drop-first")?,
        vec![LinkDiscipline::declared(
            forward()?,
            vec![LinkFault::DropAt {
                position: SendOrdinal::at(0u32),
            }],
        )],
    )?;
    let campaign = NetworkCampaign::declared(vec![schedule.clone()])?;
    let mut sim = SimNet::declared(pair_topology()?, campaign.select(name("drop-first")?)?)?;
    sim.send(forward()?, b"lost".to_vec())?;
    sim.send(forward()?, b"kept".to_vec())?;
    let delivered = sim.advance();
    assert_eq!(delivered.len(), 1usize);
    let (pack, standing) = simulated(&sim, Vec::clone)?;
    let manifest = pack.simulation_manifest().ok_or(LaneFailure::Standing)?;
    assert_eq!(manifest.actions().len(), 3usize);
    assert_eq!(pack.entries().len(), 1usize);
    assert_eq!(standing.actions(), 3usize);
    assert_eq!(standing.rows(), 1usize);
    Ok(())
}

/// Live-recorded material preserves its honest source ceiling and cannot enter simulation reproduction.
#[test]
fn a_live_record_is_replayable_but_not_reproducible() -> Result<(), LaneFailure> {
    let topology = pair_topology()?;
    let pack = recorded_live(
        &topology,
        vec![
            live_entry(b"first", 0u32, 2u64)?,
            live_entry(b"second", 1u32, 5u64)?,
        ],
    )?;
    assert_eq!(pack.source_claim(), TranscriptSourceClaim::RecordedLive);
    assert!(pack.simulation_manifest().is_none());
    let reread = read_recorded_live(&topology, pack.encoded())?;
    assert_eq!(reread, pack);
    assert_eq!(
        reproduce(&reread).err(),
        Some(TranscriptRefusal::RecordedLiveCannotReproduce)
    );
    let (mut replay, opening) = Replay::opened(&reread);
    assert!(opening.is_empty());
    while replay.remaining() > 0usize {
        let _handed_out = replay.advance();
    }
    assert_eq!(replay.exhaust()?.total(), 2usize);
    Ok(())
}

/// Source-specific readers reject a body from the other road, and simulation reading demands the exact selected schedule.
#[test]
fn readers_do_not_upgrade_or_relabel_source_material() -> Result<(), LaneFailure> {
    let topology = pair_topology()?;
    let (_rows, schedule, simulated_pack, _standing) = packed_run(0usize)?;
    assert_eq!(
        read_recorded_live(&topology, simulated_pack.encoded()).err(),
        Some(TranscriptRefusal::SourceClaimMismatch {
            expected: TranscriptSourceClaim::RecordedLive,
            found: TranscriptSourceClaim::Simulated,
        })
    );
    let live = recorded_live(&topology, vec![live_entry(b"live", 0u32, 1u64)?])?;
    assert_eq!(
        read_simulated(&topology, &schedule, live.encoded()).err(),
        Some(TranscriptRefusal::SourceClaimMismatch {
            expected: TranscriptSourceClaim::Simulated,
            found: TranscriptSourceClaim::RecordedLive,
        })
    );
    let other = NetworkSchedule::declared(name("other")?, Vec::new())?;
    assert_eq!(
        read_simulated(&topology, &other, simulated_pack.encoded()).err(),
        Some(TranscriptRefusal::ScheduleMismatch)
    );
    Ok(())
}

/// A self-consistent address over altered output rows still cannot mint simulation reproduction.
#[test]
fn addressed_bytes_do_not_impersonate_reproduction() -> Result<(), LaneFailure> {
    let topology = pair_topology()?;
    let (_rows, schedule, pack, _standing) = packed_run(0usize)?;
    let mut altered = pack.encoded().to_vec();
    let positions: Vec<_> = altered
        .windows(3usize)
        .enumerate()
        .filter_map(|(at, bytes)| (bytes == b"pay").then_some(at))
        .collect();
    assert_eq!(positions.len(), 3usize);
    let last = positions.last().copied().ok_or(LaneFailure::Standing)?;
    let byte = altered.get_mut(last).ok_or(LaneFailure::Standing)?;
    *byte = b'x';
    readdress(&mut altered)?;
    let decoded = read_simulated(&topology, &schedule, &altered)?;
    assert_eq!(
        reproduce(&decoded).err(),
        Some(TranscriptRefusal::SimulationRowsDiverge { at: 1usize })
    );
    Ok(())
}

/// Tick-zero delivery is handed out by opening and counts toward exact replay exhaustion.
#[test]
fn tick_zero_is_part_of_the_exhaustion_denominator() -> Result<(), LaneFailure> {
    let topology = pair_topology()?;
    let entry = TranscriptEntry::witnessed(
        forward()?,
        SendOrdinal::at(0u32),
        b"epoch".to_vec(),
        Tick::at(0u64),
        Tick::at(0u64),
        DeliveryCopy::Original,
    );
    let pack = recorded_live(&topology, vec![entry])?;
    let reread = read_recorded_live(&topology, pack.encoded())?;
    let (replay, opening) = Replay::opened(&reread);
    assert_eq!(opening.len(), 1usize);
    assert_eq!(replay.remaining(), 0usize);
    let exhaustion = replay.exhaust()?;
    assert_eq!(exhaustion.total(), 1usize);
    assert_eq!(exhaustion.final_tick(), Tick::at(0u64));
    Ok(())
}

/// Exhaustion refuses with the exact remaining-row count and cannot join reproduction from another address.
#[test]
fn incomplete_or_foreign_playback_cannot_open_the_join() -> Result<(), LaneFailure> {
    let (_first_rows, _first_schedule, first, first_reproduction) = packed_run(0usize)?;
    let (incomplete, first_opening) = Replay::opened(&first);
    assert!(first_opening.is_empty());
    let refusal = incomplete.exhaust().err().ok_or(LaneFailure::Standing)?;
    assert_eq!(refusal.address(), first.address());
    assert_eq!(refusal.remaining(), 2usize);

    let (_second_rows, _second_schedule, second, _second_reproduction) = packed_run(1usize)?;
    let (mut replay, second_opening) = Replay::opened(&second);
    assert!(second_opening.is_empty());
    while replay.remaining() > 0usize {
        let _handed_out = replay.advance();
    }
    let exhaustion = replay.exhaust()?;
    assert_eq!(
        ReproducedReplay::joined(first_reproduction, exhaustion),
        Err(ReproducedReplayRefusal::AddressMismatch {
            reproduction: first.address(),
            replay: second.address(),
        })
    );
    Ok(())
}

/// The live writer refuses empty, foreign, impossible, and backward records at their exact clauses.
#[test]
fn the_live_write_road_refuses_incoherent_records() -> Result<(), LaneFailure> {
    let topology = pair_topology()?;
    assert_eq!(
        recorded_live(&topology, Vec::new()).err(),
        Some(TranscriptRefusal::NoDelivery)
    );
    let backward = TranscriptEntry::witnessed(
        forward()?,
        SendOrdinal::at(0u32),
        b"impossible".to_vec(),
        Tick::at(99u64),
        Tick::at(0u64),
        DeliveryCopy::Original,
    );
    assert_eq!(
        recorded_live(&topology, vec![backward]).err(),
        Some(TranscriptRefusal::DeliveryBeforeSend { at: 0usize })
    );
    let stranger = Link::between(
        NodeRef::declared(name("stranger")?),
        NodeRef::declared(name("server")?),
    );
    let foreign = TranscriptEntry::witnessed(
        stranger,
        SendOrdinal::at(0u32),
        b"lost".to_vec(),
        Tick::at(0u64),
        Tick::at(1u64),
        DeliveryCopy::Original,
    );
    assert_eq!(
        recorded_live(&topology, vec![foreign]).err(),
        Some(TranscriptRefusal::ForeignLink { at: 0usize })
    );
    assert_eq!(
        recorded_live(
            &topology,
            vec![
                live_entry(b"late", 0u32, 5u64)?,
                live_entry(b"early", 1u32, 2u64)?
            ]
        )
        .err(),
        Some(TranscriptRefusal::DeliveryOrderBroken { at: 1usize })
    );
    Ok(())
}

/// The reader settles the address first, rejects foreign topology, and refuses the retired wire version explicitly.
#[test]
fn the_reader_refuses_tampered_foreign_and_retired_envelopes() -> Result<(), LaneFailure> {
    let (_rows, schedule, pack, _standing) = packed_run(0usize)?;
    let topology = pair_topology()?;
    let mut tampered = pack.encoded().to_vec();
    if let Some(last) = tampered.last_mut() {
        *last = last.wrapping_add(1u8);
    }
    assert!(matches!(
        read_simulated(&topology, &schedule, &tampered).err(),
        Some(TranscriptRefusal::AddressMismatch { derived: _ })
    ));
    let short = pack
        .encoded()
        .get(0usize..10usize)
        .ok_or(LaneFailure::Standing)?;
    assert_eq!(
        read_simulated(&topology, &schedule, short).err(),
        Some(TranscriptRefusal::Truncated)
    );
    let elsewhere = Topology::declared(
        vec![
            NodeRef::declared(name("alpha")?),
            NodeRef::declared(name("beta")?),
        ],
        vec![Link::between(
            NodeRef::declared(name("alpha")?),
            NodeRef::declared(name("beta")?),
        )],
    )?;
    assert_eq!(
        read_simulated(&elsewhere, &schedule, pack.encoded()).err(),
        Some(TranscriptRefusal::TopologyMismatch)
    );

    let address_width = ContentAddress::derived(TRANSCRIPT_TAG, &[])
        .as_bytes()
        .len();
    let mut retired = pack.encoded().to_vec();
    let version_end = address_width.saturating_add(4usize);
    let version = retired
        .get_mut(address_width..version_end)
        .ok_or(LaneFailure::Standing)?;
    version.copy_from_slice(&1u32.to_be_bytes());
    readdress(&mut retired)?;
    assert_eq!(
        read_simulated(&topology, &schedule, &retired).err(),
        Some(TranscriptRefusal::UnsupportedFormat { found: 1u32 })
    );
    Ok(())
}
