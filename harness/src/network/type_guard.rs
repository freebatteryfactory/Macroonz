//! Constructors, readers, and the checks that decide whether a topology, a schedule, a campaign, and a sim exist at all.

use super::{
    Delivery, DeliveryCopy, Link, LinkDiscipline, LinkFault, NetworkCampaign,
    NetworkCampaignRefusal, NetworkCensus, NetworkSchedule, NetworkScheduleRefusal,
    NetworkSelection, NetworkSelectionRefusal, NodeRef, ReplayExhaustion, ReplayIncomplete,
    ReproducedReplay, ReproducedReplayRefusal, SendFate, SendOrdinal, SendReceipt, SimNet,
    SimNetRefusal, SimulationAction, SimulationManifest, SimulationReproduction, Tick, TickSpan,
    TickSpanRefusal, Topology, TopologyRefusal, TranscriptAddress, TranscriptEntry,
    TranscriptMaterial, TranscriptPack, TranscriptSourceClaim,
};
use crate::descriptor::NamespacedName;
use crate::identity::ContentAddress;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

impl NodeRef {
    /// The node its adopter named.
    #[must_use]
    pub const fn declared(name: NamespacedName) -> Self {
        Self(name)
    }

    /// The name this node is told apart by.
    #[must_use]
    pub const fn name(self) -> NamespacedName {
        self.0
    }
}

impl Link {
    /// One directed link, from one named node to another.
    #[must_use]
    pub const fn between(from: NodeRef, to: NodeRef) -> Self {
        Self { from, to }
    }

    /// The sending end.
    #[must_use]
    pub const fn from(self) -> NodeRef {
        self.from
    }

    /// The receiving end.
    #[must_use]
    pub const fn to(self) -> NodeRef {
        self.to
    }
}

impl Topology {
    /// The declared nodes and links, in authored order.
    ///
    /// # Errors
    ///
    /// Refuses an empty node roster, then a repeated node, then an empty link roster, then a repeated link, then the first link naming a node never declared.
    pub fn declared(nodes: Vec<NodeRef>, links: Vec<Link>) -> Result<Self, TopologyRefusal> {
        if nodes.is_empty() {
            return Err(TopologyRefusal::NoNode);
        }
        let mut seen_nodes = BTreeSet::new();
        for node in &nodes {
            if !seen_nodes.insert(*node) {
                return Err(TopologyRefusal::DuplicateNode(*node));
            }
        }
        if links.is_empty() {
            return Err(TopologyRefusal::NoLink);
        }
        let mut seen_links = BTreeSet::new();
        for link in &links {
            if !seen_links.insert(*link) {
                return Err(TopologyRefusal::DuplicateLink(*link));
            }
            let foreign = [link.from(), link.to()]
                .into_iter()
                .find(|end| !seen_nodes.contains(end));
            if let Some(node) = foreign {
                return Err(TopologyRefusal::LinkForeignNode { node });
            }
        }
        Ok(Self { nodes, links })
    }

    /// The nodes, in authored order.
    #[must_use]
    pub fn nodes(&self) -> &[NodeRef] {
        &self.nodes
    }

    /// The links, in authored order.
    #[must_use]
    pub fn links(&self) -> &[Link] {
        &self.links
    }
}

impl Tick {
    /// One point of logical time, counted from zero.
    #[must_use]
    pub const fn at(ordinal: u64) -> Self {
        Self(ordinal)
    }

    /// The ordinal this tick carries.
    #[must_use]
    pub const fn ordinal(self) -> u64 {
        self.0
    }

    /// The tick after this one.
    #[must_use]
    pub(super) const fn next(self) -> Self {
        Self(self.0.saturating_add(1u64))
    }

    /// The tick this many ticks after this one.
    #[must_use]
    pub(super) const fn later_by(self, ticks: u64) -> Self {
        Self(self.0.saturating_add(ticks))
    }
}

impl TickSpan {
    /// The stretch its author declared.
    ///
    /// # Errors
    ///
    /// Refuses a span of zero ticks, because a delay built on it would declare pressure and apply none.
    pub const fn declared(ticks: u32) -> Result<Self, TickSpanRefusal> {
        if ticks == 0u32 {
            return Err(TickSpanRefusal::ZeroTicks);
        }
        Ok(Self(ticks))
    }

    /// How many ticks the span covers.
    #[must_use]
    pub const fn ticks(self) -> u32 {
        self.0
    }
}

impl SendOrdinal {
    /// The send a fault fires on, counted from zero in placement order.
    #[must_use]
    pub const fn at(ordinal: u32) -> Self {
        Self(ordinal)
    }

    /// The ordinal this send sits at.
    #[must_use]
    pub const fn ordinal(self) -> u32 {
        self.0
    }
}

impl LinkDiscipline {
    /// One link and the faults declared over its traffic, in authored order.
    #[must_use]
    pub const fn declared(link: Link, faults: Vec<LinkFault>) -> Self {
        Self { link, faults }
    }

    /// The link this discipline governs.
    #[must_use]
    pub const fn link(&self) -> Link {
        self.link
    }

    /// The faults, in authored order.
    #[must_use]
    pub fn faults(&self) -> &[LinkFault] {
        &self.faults
    }
}

impl NetworkSchedule {
    /// A named schedule over per-link disciplines, in authored order.
    ///
    /// # Errors
    ///
    /// Refuses two disciplines on one link, then a discipline declaring no fault, then the first partition interval that closes at or before it opens.
    pub fn declared(
        name: NamespacedName,
        disciplines: Vec<LinkDiscipline>,
    ) -> Result<Self, NetworkScheduleRefusal> {
        let mut seen = BTreeSet::new();
        for discipline in &disciplines {
            if !seen.insert(discipline.link()) {
                return Err(NetworkScheduleRefusal::DuplicateDiscipline(
                    discipline.link(),
                ));
            }
            if discipline.faults().is_empty() {
                return Err(NetworkScheduleRefusal::EmptyDiscipline(discipline.link()));
            }
            lawful_partitions(discipline)?;
        }
        Ok(Self { name, disciplines })
    }

    /// The name this schedule is selected by.
    #[must_use]
    pub const fn name(&self) -> NamespacedName {
        self.name
    }

    /// The per-link disciplines, in authored order.
    #[must_use]
    pub fn disciplines(&self) -> &[LinkDiscipline] {
        &self.disciplines
    }

    /// The discipline governing one link, where the schedule declares one.
    #[must_use]
    pub(super) fn discipline_of(&self, link: Link) -> Option<&LinkDiscipline> {
        self.disciplines
            .iter()
            .find(|discipline| discipline.link() == link)
    }
}

impl NetworkCampaign {
    /// A campaign over uniquely named schedules, in authored order.
    ///
    /// # Errors
    ///
    /// Refuses an empty campaign, then the first repeated name, then a campaign whose schedules are all empty controls.
    pub fn declared(schedules: Vec<NetworkSchedule>) -> Result<Self, NetworkCampaignRefusal> {
        if schedules.is_empty() {
            return Err(NetworkCampaignRefusal::NoSchedule);
        }
        let mut seen = BTreeSet::new();
        for schedule in &schedules {
            if !seen.insert(schedule.name()) {
                return Err(NetworkCampaignRefusal::DuplicateSchedule(schedule.name()));
            }
        }
        if schedules
            .iter()
            .all(|schedule| schedule.disciplines().is_empty())
        {
            return Err(NetworkCampaignRefusal::NoFaultDeclared);
        }
        Ok(Self { schedules })
    }

    /// The campaign's schedules, in authored order.
    #[must_use]
    pub fn schedules(&self) -> &[NetworkSchedule] {
        &self.schedules
    }

    /// The schedule this campaign declares under `name`.
    ///
    /// # Errors
    ///
    /// Refuses a name no schedule here declares.
    pub fn select(
        &self,
        name: NamespacedName,
    ) -> Result<NetworkSelection<'_>, NetworkSelectionRefusal> {
        self.schedules
            .iter()
            .find(|schedule| schedule.name() == name)
            .map(|schedule| NetworkSelection { schedule })
            .ok_or(NetworkSelectionRefusal::ScheduleAbsent(name))
    }
}

impl<'campaign> NetworkSelection<'campaign> {
    /// The schedule the campaign handed back.
    #[must_use]
    pub const fn schedule(self) -> &'campaign NetworkSchedule {
        self.schedule
    }
}

/// Whether every partition interval one discipline declares covers at least one tick.
///
/// # Errors
///
/// Refuses the first interval that closes at or before it opens.
fn lawful_partitions(discipline: &LinkDiscipline) -> Result<(), NetworkScheduleRefusal> {
    for fault in discipline.faults() {
        if let LinkFault::Partition { opens, heals } = *fault
            && heals <= opens
        {
            return Err(NetworkScheduleRefusal::EmptyPartition {
                link: discipline.link(),
            });
        }
    }
    Ok(())
}

impl<Payload> SimNet<Payload> {
    /// Open one sim over a topology and a selected schedule, at tick zero.
    ///
    /// The selection's schedule is cloned in, so the sim owns its whole declared world.
    ///
    /// # Errors
    ///
    /// Refuses a schedule that disciplines a link outside the topology.
    pub fn declared(
        topology: Topology,
        selection: NetworkSelection<'_>,
    ) -> Result<Self, SimNetRefusal> {
        let schedule = selection.schedule().clone();
        for discipline in schedule.disciplines() {
            if !topology.links().contains(&discipline.link()) {
                return Err(SimNetRefusal::DisciplineForeignLink {
                    link: discipline.link(),
                });
            }
        }
        Ok(Self {
            topology,
            schedule,
            tick: Tick::at(0u64),
            sequence: 0u64,
            placed: BTreeMap::new(),
            in_flight: Vec::new(),
            actions: Vec::new(),
            history: Vec::new(),
            census: NetworkCensus {
                sends: 0u64,
                scheduled_deliveries: 0u64,
                delivered: 0u64,
                dropped_by_discipline: 0u64,
                dropped_by_partition: 0u64,
            },
        })
    }

    /// The current logical tick.
    #[must_use]
    pub const fn tick(&self) -> Tick {
        self.tick
    }

    /// How many scheduled deliveries have not yet come due.
    #[must_use]
    pub fn pending(&self) -> usize {
        self.in_flight.len()
    }

    /// The accounting over every send so far.
    #[must_use]
    pub const fn census(&self) -> NetworkCensus {
        self.census
    }
}

impl SendReceipt {
    /// The link the send was placed on.
    #[must_use]
    pub const fn link(self) -> Link {
        self.link
    }

    /// The send's zero-based ordinal on that link.
    #[must_use]
    pub const fn ordinal(self) -> SendOrdinal {
        self.ordinal
    }

    /// What became of the send.
    #[must_use]
    pub const fn fate(self) -> SendFate {
        self.fate
    }
}

impl<Payload> Delivery<Payload> {
    /// One delivery, minted only by the sim's advance.
    #[must_use]
    pub(super) const fn delivered(
        link: Link,
        ordinal: SendOrdinal,
        payload: Payload,
        sent_at: Tick,
        delivered_at: Tick,
        copy: DeliveryCopy,
    ) -> Self {
        Self {
            link,
            ordinal,
            payload,
            sent_at,
            delivered_at,
            copy,
        }
    }

    /// The link the payload traveled.
    #[must_use]
    pub const fn link(&self) -> Link {
        self.link
    }

    /// The send's zero-based ordinal on that link.
    #[must_use]
    pub const fn ordinal(&self) -> SendOrdinal {
        self.ordinal
    }

    /// The payload, exactly as sent.
    #[must_use]
    pub const fn payload(&self) -> &Payload {
        &self.payload
    }

    /// The tick the send was placed at.
    #[must_use]
    pub const fn sent_at(&self) -> Tick {
        self.sent_at
    }

    /// The tick the delivery came due.
    #[must_use]
    pub const fn delivered_at(&self) -> Tick {
        self.delivered_at
    }

    /// Whether this is the send's original or a duplicate the discipline added.
    #[must_use]
    pub const fn copy(&self) -> DeliveryCopy {
        self.copy
    }
}

impl TranscriptEntry {
    /// One delivery as somebody witnessed it — the sim's own, or a live adapter's observation.
    #[must_use]
    pub const fn witnessed(
        link: Link,
        ordinal: SendOrdinal,
        payload: Vec<u8>,
        sent_at: Tick,
        delivered_at: Tick,
        copy: DeliveryCopy,
    ) -> Self {
        Self {
            link,
            ordinal,
            payload,
            sent_at,
            delivered_at,
            copy,
        }
    }

    /// The link the payload traveled.
    #[must_use]
    pub const fn link(&self) -> Link {
        self.link
    }

    /// The send's zero-based ordinal on that link.
    #[must_use]
    pub const fn ordinal(&self) -> SendOrdinal {
        self.ordinal
    }

    /// The payload, in bytes.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// The tick the send was placed at.
    #[must_use]
    pub const fn sent_at(&self) -> Tick {
        self.sent_at
    }

    /// The tick the delivery came due.
    #[must_use]
    pub const fn delivered_at(&self) -> Tick {
        self.delivered_at
    }

    /// Whether this is the send's original or a duplicate.
    #[must_use]
    pub const fn copy(&self) -> DeliveryCopy {
        self.copy
    }
}

impl TranscriptAddress {
    /// The address the writer or the reader derived, minted nowhere else.
    #[must_use]
    pub(super) const fn derived(address: ContentAddress) -> Self {
        Self(address)
    }

    /// The derivation this address carries.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
}

impl TranscriptPack {
    /// One admitted pack, minted only by the write and read roads.
    #[must_use]
    pub(super) const fn assembled(
        topology: Topology,
        material: TranscriptMaterial,
        address: TranscriptAddress,
        entries: Vec<TranscriptEntry>,
        encoded: Vec<u8>,
    ) -> Self {
        Self {
            topology,
            material,
            address,
            entries,
            encoded,
        }
    }

    /// The topology retained in this pack's addressed body.
    #[must_use]
    pub const fn topology(&self) -> &Topology {
        &self.topology
    }

    /// What this pack's addressed body claims about its source.
    ///
    /// The reading carries no reproduction standing.
    #[must_use]
    pub const fn source_claim(&self) -> TranscriptSourceClaim {
        match self.material {
            TranscriptMaterial::Simulated(_) => TranscriptSourceClaim::Simulated,
            TranscriptMaterial::RecordedLive => TranscriptSourceClaim::RecordedLive,
        }
    }

    /// The simulation inputs this pack retains, where its body claims a simulated source.
    #[must_use]
    pub const fn simulation_manifest(&self) -> Option<&SimulationManifest> {
        match &self.material {
            TranscriptMaterial::Simulated(manifest) => Some(manifest),
            TranscriptMaterial::RecordedLive => None,
        }
    }

    /// The address the whole body derives.
    #[must_use]
    pub const fn address(&self) -> TranscriptAddress {
        self.address
    }

    /// The deliveries, in delivery order.
    #[must_use]
    pub fn entries(&self) -> &[TranscriptEntry] {
        &self.entries
    }

    /// The complete envelope, exactly as derived: the address, then the body it covers.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }
}

impl SimulationManifest {
    /// One selected schedule beside the complete ordered actions driven through it.
    #[must_use]
    pub(super) const fn captured(
        schedule: NetworkSchedule,
        actions: Vec<SimulationAction>,
    ) -> Self {
        Self { schedule, actions }
    }

    /// The selected schedule the manifest declares.
    #[must_use]
    pub const fn schedule(&self) -> &NetworkSchedule {
        &self.schedule
    }

    /// Every send and advance, in exact drive order.
    #[must_use]
    pub fn actions(&self) -> &[SimulationAction] {
        &self.actions
    }
}

impl SimulationReproduction {
    /// One exact transcript address whose manifest reproduced all addressed rows.
    #[must_use]
    pub(super) const fn witnessed(
        address: TranscriptAddress,
        actions: usize,
        rows: usize,
        final_tick: Tick,
    ) -> Self {
        Self {
            address,
            actions,
            rows,
            final_tick,
        }
    }

    /// The transcript whose exact manifest and rows were reproduced.
    #[must_use]
    pub const fn address(self) -> TranscriptAddress {
        self.address
    }

    /// How many manifest actions were executed.
    #[must_use]
    pub const fn actions(self) -> usize {
        self.actions
    }

    /// How many delivery rows the execution reproduced.
    #[must_use]
    pub const fn rows(self) -> usize {
        self.rows
    }

    /// The reproduced sim's logical tick after the final manifest action.
    #[must_use]
    pub const fn final_tick(self) -> Tick {
        self.final_tick
    }
}

impl ReplayExhaustion {
    /// One exhausted replay, minted only by consuming that replay.
    #[must_use]
    pub(super) const fn witnessed(
        address: TranscriptAddress,
        total: usize,
        final_tick: Tick,
    ) -> Self {
        Self {
            address,
            total,
            final_tick,
        }
    }

    /// The transcript whose rows were all handed out.
    #[must_use]
    pub const fn address(self) -> TranscriptAddress {
        self.address
    }

    /// How many addressed rows were handed out.
    #[must_use]
    pub const fn total(self) -> usize {
        self.total
    }

    /// The replay tick at exhaustion.
    #[must_use]
    pub const fn final_tick(self) -> Tick {
        self.final_tick
    }
}

impl ReplayIncomplete {
    /// One replay that still retained rows when exhaustion was requested.
    pub(super) const fn witnessed(address: TranscriptAddress, remaining: usize) -> Self {
        Self { address, remaining }
    }

    /// The transcript whose replay remains incomplete.
    #[must_use]
    pub const fn address(self) -> TranscriptAddress {
        self.address
    }

    /// How many recorded rows were never handed out.
    #[must_use]
    pub const fn remaining(self) -> usize {
        self.remaining
    }
}

impl ReproducedReplay {
    /// Join exact simulation reproduction with exhausted playback over the same address.
    ///
    /// # Errors
    ///
    /// Refuses values that name different transcript addresses.
    pub fn joined(
        reproduction: SimulationReproduction,
        exhaustion: ReplayExhaustion,
    ) -> Result<Self, ReproducedReplayRefusal> {
        if reproduction.address() != exhaustion.address() {
            return Err(ReproducedReplayRefusal::AddressMismatch {
                reproduction: reproduction.address(),
                replay: exhaustion.address(),
            });
        }
        Ok(Self {
            reproduction,
            exhaustion,
        })
    }

    /// The simulation reproduction this join retains.
    #[must_use]
    pub const fn reproduction(self) -> SimulationReproduction {
        self.reproduction
    }

    /// The replay exhaustion this join retains.
    #[must_use]
    pub const fn exhaustion(self) -> ReplayExhaustion {
        self.exhaustion
    }
}

impl NetworkCensus {
    /// How many sends were placed.
    #[must_use]
    pub const fn sends(self) -> u64 {
        self.sends
    }

    /// How many deliveries were scheduled, duplicates included.
    #[must_use]
    pub const fn scheduled_deliveries(self) -> u64 {
        self.scheduled_deliveries
    }

    /// How many deliveries have come due.
    #[must_use]
    pub const fn delivered(self) -> u64 {
        self.delivered
    }

    /// How many sends a drop fault took.
    #[must_use]
    pub const fn dropped_by_discipline(self) -> u64 {
        self.dropped_by_discipline
    }

    /// How many sends an open partition took.
    #[must_use]
    pub const fn dropped_by_partition(self) -> u64 {
        self.dropped_by_partition
    }
}
