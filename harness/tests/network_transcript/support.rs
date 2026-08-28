pub(super) use macroonz_harness::descriptor::{NameRefusal, NamespacedName};
pub(super) use macroonz_harness::identity::ContentAddress;
pub(super) use macroonz_harness::network::{
    Delivery, DeliveryCopy, Link, LinkDiscipline, LinkFault, NetworkCampaign,
    NetworkCampaignRefusal, NetworkSchedule, NetworkScheduleRefusal, NetworkSelectionRefusal,
    NodeRef, Replay, ReplayIncomplete, ReproducedReplay, ReproducedReplayRefusal, SendOrdinal,
    SendRefusal, SimNet, SimNetRefusal, SimulationReproduction, TRANSCRIPT_FORMAT_VERSION,
    TRANSCRIPT_TAG, Tick, Topology, TopologyRefusal, TranscriptEntry, TranscriptPack,
    TranscriptRefusal, TranscriptSourceClaim, read_recorded_live, read_simulated, recorded_live,
    reproduce, simulated,
};

/// Everything a lane road can refuse, carried as itself.
pub(super) enum LaneFailure {
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
pub(super) fn name(stem: &'static str) -> Result<NamespacedName, NameRefusal> {
    NamespacedName::named("lane", stem)
}

/// The client-to-server link over the pair topology.
pub(super) fn forward() -> Result<Link, NameRefusal> {
    Ok(Link::between(
        NodeRef::declared(name("client")?),
        NodeRef::declared(name("server")?),
    ))
}

/// Two nodes, one link each way.
pub(super) fn pair_topology() -> Result<Topology, LaneFailure> {
    let client = NodeRef::declared(name("client")?);
    let server = NodeRef::declared(name("server")?);
    Ok(Topology::declared(
        vec![client, server],
        vec![Link::between(client, server), Link::between(server, client)],
    )?)
}

/// The schedule that duplicates the first client request.
pub(super) fn duplicate_schedule() -> Result<NetworkSchedule, LaneFailure> {
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
pub(super) type DrivenRun = (Vec<Delivery<Vec<u8>>>, SimNet<Vec<u8>>, NetworkSchedule);

/// Drive the duplicating schedule, optionally recording extra empty advances after delivery.
pub(super) fn duplicated_run(extra_advances: usize) -> Result<DrivenRun, LaneFailure> {
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
pub(super) type PackedRun = (
    Vec<Delivery<Vec<u8>>>,
    NetworkSchedule,
    TranscriptPack,
    SimulationReproduction,
);

/// Drive and pack one simulated run.
pub(super) fn packed_run(extra_advances: usize) -> Result<PackedRun, LaneFailure> {
    let (deliveries, sim, schedule) = duplicated_run(extra_advances)?;
    let (pack, reproduction) = simulated(&sim, Vec::clone)?;
    Ok((deliveries, schedule, pack, reproduction))
}

/// One lawful live-recorded row.
pub(super) fn live_entry(
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
pub(super) fn readdress(encoded: &mut [u8]) -> Result<(), LaneFailure> {
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
