//! The transcript roads, exercised from outside: a run becomes an addressed pack, the pack reads back whole, and a replay hands back exactly the recorded deliveries.
//!
//! One simulated run and one hand-witnessed live-shaped record walk the same envelope; the refusal lanes reverse one clause each of what the write road and the reader promise, and a tampered envelope dies at its address before a single row is believed.

use macroonz_harness::descriptor::{NameRefusal, NamespacedName};
use macroonz_harness::network::{
    Delivery, DeliveryCopy, Link, LinkDiscipline, LinkFault, NetworkCampaign,
    NetworkCampaignRefusal, NetworkSchedule, NetworkScheduleRefusal, NetworkSelectionRefusal,
    NodeRef, Replay, SendOrdinal, SendRefusal, SimNet, SimNetRefusal, Tick, Topology,
    TopologyRefusal, TranscriptEntry, TranscriptPack, TranscriptProvenance, TranscriptRefusal,
    read, recorded,
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

/// One simulated run under a duplicating schedule, and the deliveries it produced.
fn duplicated_run() -> Result<Vec<Delivery<Vec<u8>>>, LaneFailure> {
    let duplicate = NetworkSchedule::declared(
        name("duplicate-the-request")?,
        vec![LinkDiscipline::declared(
            forward()?,
            vec![LinkFault::DuplicateAt {
                position: SendOrdinal::at(0u32),
            }],
        )],
    )?;
    let campaign = NetworkCampaign::declared(vec![duplicate])?;
    let mut sim = SimNet::declared(
        pair_topology()?,
        campaign.select(name("duplicate-the-request")?)?,
    )?;
    sim.send(forward()?, b"pay".to_vec())?;
    let mut deliveries = Vec::new();
    while sim.pending() > 0usize {
        deliveries.extend(sim.advance());
    }
    Ok(deliveries)
}

/// The transcript rows one run's deliveries spell.
fn witnessed(deliveries: &[Delivery<Vec<u8>>]) -> Vec<TranscriptEntry> {
    deliveries
        .iter()
        .map(|delivery| {
            TranscriptEntry::witnessed(
                delivery.link(),
                delivery.ordinal(),
                delivery.payload().clone(),
                delivery.sent_at(),
                delivery.delivered_at(),
                delivery.copy(),
            )
        })
        .collect()
}

/// One run's deliveries beside the pack they were written into.
type RunAndPack = (Vec<Delivery<Vec<u8>>>, TranscriptPack);

/// One packed simulated run, for the lanes that read and replay it.
fn packed_run() -> Result<RunAndPack, LaneFailure> {
    let deliveries = duplicated_run()?;
    let pack = recorded(
        TranscriptProvenance::Simulated,
        &pair_topology()?,
        witnessed(&deliveries),
    )?;
    Ok((deliveries, pack))
}

/// A simulated run packs, reads back byte for byte, and replays into exactly the deliveries the sim produced.
#[test]
fn a_simulated_run_packs_reads_back_and_replays_identically() -> Result<(), LaneFailure> {
    let (deliveries, pack) = packed_run()?;
    assert_eq!(deliveries.len(), 2usize);
    assert_eq!(pack.provenance(), TranscriptProvenance::Simulated);
    let reread = read(&pair_topology()?, pack.encoded())?;
    assert_eq!(reread, pack);
    let mut replay = Replay::opened(&pack);
    assert_eq!(replay.remaining(), 2usize);
    let mut played = Vec::new();
    while replay.remaining() > 0usize {
        played.extend(replay.advance());
    }
    assert_eq!(played, deliveries);
    let mut again = Replay::opened(&pack);
    let mut second = Vec::new();
    while again.remaining() > 0usize {
        second.extend(again.advance());
    }
    assert_eq!(second, played);
    Ok(())
}

/// Two identical runs derive one address, which is what makes a pack a claim rather than a file.
#[test]
fn one_run_derives_one_address() -> Result<(), LaneFailure> {
    let (_first_deliveries, first) = packed_run()?;
    let (_second_deliveries, second) = packed_run()?;
    assert_eq!(first.address(), second.address());
    assert_eq!(first.encoded(), second.encoded());
    Ok(())
}

/// A hand-witnessed record carries its live provenance through the envelope and replays at its recorded ticks.
#[test]
fn a_live_witnessed_pack_keeps_its_provenance_and_its_ticks() -> Result<(), LaneFailure> {
    let wire = forward()?;
    let entries = vec![
        TranscriptEntry::witnessed(
            wire,
            SendOrdinal::at(0u32),
            b"first".to_vec(),
            Tick::at(1u64),
            Tick::at(2u64),
            DeliveryCopy::Original,
        ),
        TranscriptEntry::witnessed(
            wire,
            SendOrdinal::at(1u32),
            b"second".to_vec(),
            Tick::at(3u64),
            Tick::at(5u64),
            DeliveryCopy::Original,
        ),
    ];
    let pack = recorded(
        TranscriptProvenance::RecordedLive,
        &pair_topology()?,
        entries,
    )?;
    let reread = read(&pair_topology()?, pack.encoded())?;
    assert_eq!(reread.provenance(), TranscriptProvenance::RecordedLive);
    let mut replay = Replay::opened(&pack);
    assert!(replay.advance().is_empty());
    let early = replay.advance();
    assert_eq!(early.len(), 1usize);
    let opening = early.first().ok_or(LaneFailure::Standing)?;
    assert_eq!(opening.delivered_at(), Tick::at(2u64));
    assert_eq!(opening.payload(), b"first");
    assert!(replay.advance().is_empty());
    assert!(replay.advance().is_empty());
    let late = replay.advance();
    assert_eq!(late.len(), 1usize);
    assert_eq!(replay.remaining(), 0usize);
    Ok(())
}

/// The write road refuses an empty record, a foreign row, a delivery stamped before its own send, and a stamp that steps backward.
#[test]
fn the_write_road_refuses_incoherent_records() -> Result<(), LaneFailure> {
    let topology = pair_topology()?;
    assert_eq!(
        recorded(TranscriptProvenance::Simulated, &topology, Vec::new()).err(),
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
        recorded(TranscriptProvenance::Simulated, &topology, vec![backward]).err(),
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
        recorded(TranscriptProvenance::Simulated, &topology, vec![foreign]).err(),
        Some(TranscriptRefusal::ForeignLink { at: 0usize })
    );
    let wire = forward()?;
    let late = TranscriptEntry::witnessed(
        wire,
        SendOrdinal::at(0u32),
        b"late".to_vec(),
        Tick::at(4u64),
        Tick::at(5u64),
        DeliveryCopy::Original,
    );
    let early = TranscriptEntry::witnessed(
        wire,
        SendOrdinal::at(1u32),
        b"early".to_vec(),
        Tick::at(1u64),
        Tick::at(2u64),
        DeliveryCopy::Original,
    );
    assert_eq!(
        recorded(
            TranscriptProvenance::Simulated,
            &topology,
            vec![late, early]
        )
        .err(),
        Some(TranscriptRefusal::DeliveryOrderBroken { at: 1usize })
    );
    Ok(())
}

/// The reader settles the address before believing a row, refuses a truncated envelope, and refuses a foreign topology.
#[test]
fn the_reader_refuses_tampered_and_foreign_envelopes() -> Result<(), LaneFailure> {
    let (_deliveries, pack) = packed_run()?;
    let mut tampered = pack.encoded().to_vec();
    if let Some(last) = tampered.last_mut() {
        *last = last.wrapping_add(1u8);
    }
    let tampered_verdict = read(&pair_topology()?, &tampered).err();
    assert!(matches!(
        tampered_verdict,
        Some(TranscriptRefusal::AddressMismatch { derived: _ })
    ));
    let short = pack
        .encoded()
        .get(0usize..10usize)
        .ok_or(LaneFailure::Standing)?;
    assert_eq!(
        read(&pair_topology()?, short).err(),
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
        read(&elsewhere, pack.encoded()).err(),
        Some(TranscriptRefusal::TopologyMismatch)
    );
    Ok(())
}
