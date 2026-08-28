pub(super) use macroonz_harness::descriptor::{NameRefusal, NamespacedName, PopulationRef};
pub(super) use macroonz_harness::generate::RootSeed;
pub(super) use macroonz_harness::interleave::{
    ExplorationBound, ExplorationBoundRefusal, ExplorationRefusal, ExplorationStanding, Strand,
    StrandRefusal, StrandSet, StrandSetRefusal, explored,
};
pub(super) use macroonz_harness::network::{
    Delivery, Link, LinkDiscipline, LinkFault, NetworkCampaign, NetworkCampaignRefusal,
    NetworkSchedule, NetworkScheduleRefusal, NetworkSelection, NetworkSelectionRefusal, NodeRef,
    SendOrdinal, SendRefusal, SimNet, SimNetRefusal, Tick, TickSpan, TickSpanRefusal, Topology,
    TopologyRefusal,
};
pub(super) use macroonz_harness::properties::{
    ContractRefusal, Holding, TemporalClaim, TemporalDemand, TransitionContract, holds_over_history,
};
pub(super) use macroonz_harness::report::{FindingCause, TrialConclusion};
pub(super) use std::collections::BTreeSet;

/// The cause a server that applied one request twice is cited under.
pub(super) const AT_MOST_ONCE: FindingCause = FindingCause::named("lane", "applied-at-most-once");

/// The cause a client that never saw its reply is cited under.
pub(super) const REPLY_ARRIVES: FindingCause = FindingCause::named("lane", "reply-arrives");

/// The cause a balance that went below zero is cited under.
pub(super) const NEVER_NEGATIVE: FindingCause = FindingCause::named("lane", "never-negative");

/// How many ticks every protocol run is driven for.
pub(super) const HORIZON: u64 = 8;

/// One message of the lane's toy protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Message {
    /// The client asks.
    Request {
        /// The request's own identity.
        id: u64,
    },
    /// The server answers.
    Reply {
        /// The identity of the request this answers.
        id: u64,
    },
}

/// One application of a request against the server's state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Application {
    id: u64,
    at: u64,
}

/// How many applications the server has performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Tally {
    applied: u128,
}

/// One event the client's side of a run records.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ClientEvent {
    /// The client placed a request.
    SentRequest {
        /// The request's identity.
        id: u64,
    },
    /// The client saw its reply.
    ReceivedReply {
        /// The identity the reply carries.
        id: u64,
    },
}

/// How many replies the client has seen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Progress {
    replies: u128,
}

/// Whether the server applies every delivery or remembers what it has seen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ServerKind {
    /// Applies every delivered request, duplicates included.
    Naive,
    /// Applies each request identity at most once.
    Deduplicating,
}

/// Everything a lane road can refuse, carried as itself.
pub(super) enum LaneFailure {
    Name(NameRefusal),
    Topology(TopologyRefusal),
    Span(TickSpanRefusal),
    Schedule(NetworkScheduleRefusal),
    Campaign(NetworkCampaignRefusal),
    Selection(NetworkSelectionRefusal),
    Sim(SimNetRefusal),
    Send(SendRefusal),
    Contract(ContractRefusal),
    Strand(StrandRefusal),
    Set(StrandSetRefusal),
    Bound(ExplorationBoundRefusal),
    Exploration(ExplorationRefusal),
    /// A reading did not carry the shape the claim demanded.
    Standing,
}

impl core::fmt::Debug for LaneFailure {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Name(refusal) => formatter.debug_tuple("Name").field(refusal).finish(),
            Self::Topology(refusal) => formatter.debug_tuple("Topology").field(refusal).finish(),
            Self::Span(refusal) => formatter.debug_tuple("Span").field(refusal).finish(),
            Self::Schedule(refusal) => formatter.debug_tuple("Schedule").field(refusal).finish(),
            Self::Campaign(refusal) => formatter.debug_tuple("Campaign").field(refusal).finish(),
            Self::Selection(refusal) => formatter.debug_tuple("Selection").field(refusal).finish(),
            Self::Sim(refusal) => formatter.debug_tuple("Sim").field(refusal).finish(),
            Self::Send(refusal) => formatter.debug_tuple("Send").field(refusal).finish(),
            Self::Contract(refusal) => formatter.debug_tuple("Contract").field(refusal).finish(),
            Self::Strand(refusal) => formatter.debug_tuple("Strand").field(refusal).finish(),
            Self::Set(refusal) => formatter.debug_tuple("Set").field(refusal).finish(),
            Self::Bound(refusal) => formatter.debug_tuple("Bound").field(refusal).finish(),
            Self::Exploration(refusal) => {
                formatter.debug_tuple("Exploration").field(refusal).finish()
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

impl From<TickSpanRefusal> for LaneFailure {
    fn from(refusal: TickSpanRefusal) -> Self {
        Self::Span(refusal)
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

impl From<ContractRefusal> for LaneFailure {
    fn from(refusal: ContractRefusal) -> Self {
        Self::Contract(refusal)
    }
}

impl From<StrandRefusal> for LaneFailure {
    fn from(refusal: StrandRefusal) -> Self {
        Self::Strand(refusal)
    }
}

impl From<StrandSetRefusal> for LaneFailure {
    fn from(refusal: StrandSetRefusal) -> Self {
        Self::Set(refusal)
    }
}

impl From<ExplorationBoundRefusal> for LaneFailure {
    fn from(refusal: ExplorationBoundRefusal) -> Self {
        Self::Bound(refusal)
    }
}

impl From<ExplorationRefusal> for LaneFailure {
    fn from(refusal: ExplorationRefusal) -> Self {
        Self::Exploration(refusal)
    }
}

/// One lane-owned name.
pub(super) fn name(stem: &'static str) -> Result<NamespacedName, NameRefusal> {
    NamespacedName::named("lane", stem)
}

/// The client's node.
pub(super) fn client() -> Result<NodeRef, NameRefusal> {
    Ok(NodeRef::declared(name("client")?))
}

/// The server's node.
pub(super) fn server() -> Result<NodeRef, NameRefusal> {
    Ok(NodeRef::declared(name("server")?))
}

/// The client-to-server link.
pub(super) fn forward() -> Result<Link, NameRefusal> {
    Ok(Link::between(client()?, server()?))
}

/// The server-to-client link.
pub(super) fn back() -> Result<Link, NameRefusal> {
    Ok(Link::between(server()?, client()?))
}

/// Two nodes, one link each way.
pub(super) fn pair_topology() -> Result<Topology, LaneFailure> {
    Ok(Topology::declared(
        vec![client()?, server()?],
        vec![forward()?, back()?],
    )?)
}

/// The quiet control every campaign here carries.
pub(super) fn quiet_control() -> Result<NetworkSchedule, LaneFailure> {
    Ok(NetworkSchedule::declared(
        name("quiet-control")?,
        Vec::new(),
    )?)
}

/// The contract that one request is applied at most once.
pub(super) fn once_contract() -> Result<TransitionContract<Tally, Application>, ContractRefusal> {
    TransitionContract::declared(
        opening_tally,
        applied_tally,
        vec![TemporalClaim::declared(
            AT_MOST_ONCE,
            TemporalDemand::Never(applied_twice),
        )],
    )
}

/// The server opens having applied nothing.
pub(super) fn opening_tally() -> Tally {
    Tally { applied: 0u128 }
}

/// One more application.
pub(super) fn applied_tally(state: &Tally, _application: &Application) -> Tally {
    Tally {
        applied: state.applied.saturating_add(1u128),
    }
}

/// Whether the server has applied more than once.
pub(super) fn applied_twice(state: &Tally) -> Holding {
    if state.applied >= 2u128 {
        Holding::Holds
    } else {
        Holding::Fails
    }
}

/// The contract that the client eventually sees a reply.
pub(super) fn reply_contract() -> Result<TransitionContract<Progress, ClientEvent>, ContractRefusal>
{
    TransitionContract::declared(
        opening_progress,
        applied_progress,
        vec![TemporalClaim::declared(
            REPLY_ARRIVES,
            TemporalDemand::Eventually(replied_once),
        )],
    )
}

/// The client opens with no reply seen.
pub(super) fn opening_progress() -> Progress {
    Progress { replies: 0u128 }
}

/// One event moves the client's progress.
pub(super) fn applied_progress(state: &Progress, event: &ClientEvent) -> Progress {
    match *event {
        ClientEvent::ReceivedReply { id: _ } => Progress {
            replies: state.replies.saturating_add(1u128),
        },
        ClientEvent::SentRequest { id: _ } => *state,
    }
}

/// Whether at least one reply arrived.
pub(super) fn replied_once(state: &Progress) -> Holding {
    if state.replies >= 1u128 {
        Holding::Holds
    } else {
        Holding::Fails
    }
}

/// Run the request path alone: one request in, the server applying what arrives.
pub(super) fn served(
    selection: NetworkSelection<'_>,
    kind: ServerKind,
) -> Result<Vec<Application>, LaneFailure> {
    let mut sim = SimNet::declared(pair_topology()?, selection)?;
    sim.send(forward()?, Message::Request { id: 7u64 })?;
    let mut seen: BTreeSet<u64> = BTreeSet::new();
    let mut applications = Vec::new();
    while sim.pending() > 0usize {
        for delivery in sim.advance() {
            let Message::Request { id } = *delivery.payload() else {
                continue;
            };
            let fresh = seen.insert(id);
            let applies = match kind {
                ServerKind::Naive => true,
                ServerKind::Deduplicating => fresh,
            };
            if applies {
                applications.push(Application {
                    id,
                    at: delivery.delivered_at().ordinal(),
                });
            }
        }
    }
    Ok(applications)
}

/// Run the whole protocol: a client that may retry, a server that always answers.
pub(super) fn driven(
    selection: NetworkSelection<'_>,
    retries: u32,
    timeout: u64,
) -> Result<(Vec<ClientEvent>, SimNet<Message>), LaneFailure> {
    let mut sim = SimNet::declared(pair_topology()?, selection)?;
    let request = Message::Request { id: 1u64 };
    sim.send(forward()?, request)?;
    let mut events = vec![ClientEvent::SentRequest { id: 1u64 }];
    let mut sent_at = 0u64;
    let mut spent = 0u32;
    let mut replied = false;
    for _ in 0u64..HORIZON {
        for delivery in sim.advance() {
            match *delivery.payload() {
                Message::Request { id } => {
                    sim.send(back()?, Message::Reply { id })?;
                }
                Message::Reply { id } => {
                    replied = true;
                    events.push(ClientEvent::ReceivedReply { id });
                }
            }
        }
        let now = sim.tick().ordinal();
        if !replied && spent < retries && now.saturating_sub(sent_at) >= timeout {
            sim.send(forward()?, request)?;
            events.push(ClientEvent::SentRequest { id: 1u64 });
            sent_at = now;
            spent = spent.saturating_add(1u32);
        }
    }
    Ok((events, sim))
}
