//! The network sim, exercised from outside: a toy request/reply protocol meets declared adversity, and every claim is judged on the harness's ordinary temporal road.
//!
//! A duplicate breaks the naive server's at-most-once claim and the deduplicating server survives it; a drop starves the reply claim until the client retries; a partition heals and retries cross it, with the census counting every send the interval took.
//! A delayed send crosses a later one — reordering as latency, not a sorting demon — and two identically driven sims hand back identical histories.
//! The keystone lane feeds per-link deliveries into [`macroonz_harness::interleave`] strands, so a delivery-order bug is caught by the same exploration that catches any other ordering bug.

use macroonz_harness::descriptor::{NameRefusal, NamespacedName, PopulationRef};
use macroonz_harness::generate::RootSeed;
use macroonz_harness::interleave::{
    ExplorationBound, ExplorationBoundRefusal, ExplorationRefusal, ExplorationStanding, Strand,
    StrandRefusal, StrandSet, StrandSetRefusal, explored,
};
use macroonz_harness::network::{
    Delivery, Link, LinkDiscipline, LinkFault, NetworkCampaign, NetworkCampaignRefusal,
    NetworkSchedule, NetworkScheduleRefusal, NetworkSelection, NetworkSelectionRefusal, NodeRef,
    SendOrdinal, SendRefusal, SimNet, SimNetRefusal, Tick, TickSpan, TickSpanRefusal, Topology,
    TopologyRefusal,
};
use macroonz_harness::properties::{
    ContractRefusal, Holding, TemporalClaim, TemporalDemand, TransitionContract, holds_over_history,
};
use macroonz_harness::report::{FindingCause, TrialConclusion};
use std::collections::BTreeSet;

/// The cause a server that applied one request twice is cited under.
const AT_MOST_ONCE: FindingCause = FindingCause::named("lane", "applied-at-most-once");

/// The cause a client that never saw its reply is cited under.
const REPLY_ARRIVES: FindingCause = FindingCause::named("lane", "reply-arrives");

/// The cause a balance that went below zero is cited under.
const NEVER_NEGATIVE: FindingCause = FindingCause::named("lane", "never-negative");

/// How many ticks every protocol run is driven for.
const HORIZON: u64 = 8;

/// One message of the lane's toy protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Message {
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
struct Application {
    id: u64,
    at: u64,
}

/// How many applications the server has performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tally {
    applied: u128,
}

/// One event the client's side of a run records.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClientEvent {
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
struct Progress {
    replies: u128,
}

/// Whether the server applies every delivery or remembers what it has seen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ServerKind {
    /// Applies every delivered request, duplicates included.
    Naive,
    /// Applies each request identity at most once.
    Deduplicating,
}

/// Everything a lane road can refuse, carried as itself.
enum LaneFailure {
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
fn name(stem: &'static str) -> Result<NamespacedName, NameRefusal> {
    NamespacedName::named("lane", stem)
}

/// The client's node.
fn client() -> Result<NodeRef, NameRefusal> {
    Ok(NodeRef::declared(name("client")?))
}

/// The server's node.
fn server() -> Result<NodeRef, NameRefusal> {
    Ok(NodeRef::declared(name("server")?))
}

/// The client-to-server link.
fn forward() -> Result<Link, NameRefusal> {
    Ok(Link::between(client()?, server()?))
}

/// The server-to-client link.
fn back() -> Result<Link, NameRefusal> {
    Ok(Link::between(server()?, client()?))
}

/// Two nodes, one link each way.
fn pair_topology() -> Result<Topology, LaneFailure> {
    Ok(Topology::declared(
        vec![client()?, server()?],
        vec![forward()?, back()?],
    )?)
}

/// The quiet control every campaign here carries.
fn quiet_control() -> Result<NetworkSchedule, LaneFailure> {
    Ok(NetworkSchedule::declared(
        name("quiet-control")?,
        Vec::new(),
    )?)
}

/// The contract that one request is applied at most once.
fn once_contract() -> Result<TransitionContract<Tally, Application>, ContractRefusal> {
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
fn opening_tally() -> Tally {
    Tally { applied: 0u128 }
}

/// One more application.
fn applied_tally(state: &Tally, _application: &Application) -> Tally {
    Tally {
        applied: state.applied.saturating_add(1u128),
    }
}

/// Whether the server has applied more than once.
fn applied_twice(state: &Tally) -> Holding {
    if state.applied >= 2u128 {
        Holding::Holds
    } else {
        Holding::Fails
    }
}

/// The contract that the client eventually sees a reply.
fn reply_contract() -> Result<TransitionContract<Progress, ClientEvent>, ContractRefusal> {
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
fn opening_progress() -> Progress {
    Progress { replies: 0u128 }
}

/// One event moves the client's progress.
fn applied_progress(state: &Progress, event: &ClientEvent) -> Progress {
    match *event {
        ClientEvent::ReceivedReply { id: _ } => Progress {
            replies: state.replies.saturating_add(1u128),
        },
        ClientEvent::SentRequest { id: _ } => *state,
    }
}

/// Whether at least one reply arrived.
fn replied_once(state: &Progress) -> Holding {
    if state.replies >= 1u128 {
        Holding::Holds
    } else {
        Holding::Fails
    }
}

/// Run the request path alone: one request in, the server applying what arrives.
fn served(
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
fn driven(
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

/// A duplicated request double-applies on a naive server, the control stays calm, and a deduplicating server survives the same schedule.
#[test]
fn a_duplicate_breaks_at_most_once_and_deduplication_restores_it() -> Result<(), LaneFailure> {
    let duplicate = NetworkSchedule::declared(
        name("duplicate-the-request")?,
        vec![LinkDiscipline::declared(
            forward()?,
            vec![LinkFault::DuplicateAt {
                position: SendOrdinal::at(0u32),
            }],
        )],
    )?;
    let campaign = NetworkCampaign::declared(vec![quiet_control()?, duplicate])?;
    let contract = once_contract()?;
    let stressed = served(
        campaign.select(name("duplicate-the-request")?)?,
        ServerKind::Naive,
    )?;
    let TrialConclusion::Refused(finding) = holds_over_history(&contract, &stressed) else {
        return Err(LaneFailure::Standing);
    };
    assert_eq!(finding.cause(), AT_MOST_ONCE);
    let calm = served(campaign.select(name("quiet-control")?)?, ServerKind::Naive)?;
    assert_eq!(calm.len(), 1usize);
    assert_eq!(
        holds_over_history(&contract, &calm),
        TrialConclusion::Passed
    );
    let hardened = served(
        campaign.select(name("duplicate-the-request")?)?,
        ServerKind::Deduplicating,
    )?;
    assert_eq!(
        holds_over_history(&contract, &hardened),
        TrialConclusion::Passed
    );
    Ok(())
}

/// A dropped request starves the reply claim, and one retry recovers it under the same schedule.
#[test]
fn a_drop_starves_the_reply_and_a_retry_recovers_it() -> Result<(), LaneFailure> {
    let drop_first = NetworkSchedule::declared(
        name("drop-the-first-request")?,
        vec![LinkDiscipline::declared(
            forward()?,
            vec![LinkFault::DropAt {
                position: SendOrdinal::at(0u32),
            }],
        )],
    )?;
    let campaign = NetworkCampaign::declared(vec![quiet_control()?, drop_first])?;
    let contract = reply_contract()?;
    let (starved, _starved_sim) = driven(
        campaign.select(name("drop-the-first-request")?)?,
        0u32,
        2u64,
    )?;
    let TrialConclusion::Refused(finding) = holds_over_history(&contract, &starved) else {
        return Err(LaneFailure::Standing);
    };
    assert_eq!(finding.cause(), REPLY_ARRIVES);
    let (recovered, _recovered_sim) = driven(
        campaign.select(name("drop-the-first-request")?)?,
        1u32,
        2u64,
    )?;
    assert_eq!(
        holds_over_history(&contract, &recovered),
        TrialConclusion::Passed
    );
    let (control, _control_sim) = driven(campaign.select(name("quiet-control")?)?, 0u32, 2u64)?;
    assert_eq!(
        holds_over_history(&contract, &control),
        TrialConclusion::Passed
    );
    Ok(())
}

/// Retries cross a healed partition, and the census counts exactly what the open interval took.
#[test]
fn a_partition_heals_and_retries_cross_it() -> Result<(), LaneFailure> {
    let parted = NetworkSchedule::declared(
        name("partition-then-heal")?,
        vec![LinkDiscipline::declared(
            forward()?,
            vec![LinkFault::Partition {
                opens: Tick::at(0u64),
                heals: Tick::at(3u64),
            }],
        )],
    )?;
    let campaign = NetworkCampaign::declared(vec![quiet_control()?, parted])?;
    let contract = reply_contract()?;
    let (outage, _outage_sim) = driven(campaign.select(name("partition-then-heal")?)?, 0u32, 2u64)?;
    let TrialConclusion::Refused(finding) = holds_over_history(&contract, &outage) else {
        return Err(LaneFailure::Standing);
    };
    assert_eq!(finding.cause(), REPLY_ARRIVES);
    let (healed, sim) = driven(campaign.select(name("partition-then-heal")?)?, 2u32, 2u64)?;
    assert_eq!(
        holds_over_history(&contract, &healed),
        TrialConclusion::Passed
    );
    let census = sim.census();
    assert_eq!(census.sends(), 4u64);
    assert_eq!(census.dropped_by_partition(), 2u64);
    assert_eq!(census.dropped_by_discipline(), 0u64);
    assert_eq!(census.scheduled_deliveries(), 2u64);
    assert_eq!(census.delivered(), 2u64);
    Ok(())
}

/// A held send crosses a later one — reordering as latency — and two identically driven sims agree delivery for delivery.
#[test]
fn a_delayed_send_crosses_a_later_one_deterministically() -> Result<(), LaneFailure> {
    fn crossing() -> Result<Vec<Delivery<Message>>, LaneFailure> {
        let hold = NetworkSchedule::declared(
            name("hold-the-first")?,
            vec![LinkDiscipline::declared(
                forward()?,
                vec![LinkFault::DelayAt {
                    position: SendOrdinal::at(0u32),
                    ticks: TickSpan::declared(2u32)?,
                }],
            )],
        )?;
        let campaign = NetworkCampaign::declared(vec![quiet_control()?, hold])?;
        let mut sim =
            SimNet::declared(pair_topology()?, campaign.select(name("hold-the-first")?)?)?;
        sim.send(forward()?, Message::Request { id: 1u64 })?;
        sim.send(forward()?, Message::Request { id: 2u64 })?;
        let mut deliveries = Vec::new();
        while sim.pending() > 0usize {
            deliveries.extend(sim.advance());
        }
        Ok(deliveries)
    }
    let first = crossing()?;
    let second = crossing()?;
    assert_eq!(first, second);
    assert_eq!(first.len(), 2usize);
    let crossed = first.first().ok_or(LaneFailure::Standing)?;
    assert_eq!(crossed.ordinal(), SendOrdinal::at(1u32));
    assert_eq!(crossed.delivered_at(), Tick::at(1u64));
    let held = first.get(1usize).ok_or(LaneFailure::Standing)?;
    assert_eq!(held.ordinal(), SendOrdinal::at(0u32));
    assert_eq!(held.delivered_at(), Tick::at(3u64));
    Ok(())
}

/// Per-link deliveries stand as strands, so a delivery-order bug is caught by the ordinary exploration.
#[test]
fn deliveries_stand_as_strands_for_exploration() -> Result<(), LaneFailure> {
    /// One transfer against the shared balance.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Payment {
        amount: i128,
    }
    /// The balance the two senders race over.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Balance {
        held: i128,
    }
    fn opening_balance() -> Balance {
        Balance { held: 0i128 }
    }
    fn applied_payment(state: &Balance, payment: &Payment) -> Balance {
        Balance {
            held: state.held.saturating_add(payment.amount),
        }
    }
    fn negative(state: &Balance) -> Holding {
        if state.held < 0i128 {
            Holding::Holds
        } else {
            Holding::Fails
        }
    }
    let alpha = NodeRef::declared(name("alpha")?);
    let beta = NodeRef::declared(name("beta")?);
    let hub = NodeRef::declared(name("hub")?);
    let from_alpha = Link::between(alpha, hub);
    let from_beta = Link::between(beta, hub);
    let topology = Topology::declared(vec![alpha, beta, hub], vec![from_alpha, from_beta])?;
    let hostile = NetworkSchedule::declared(
        name("drop-alpha-first")?,
        vec![LinkDiscipline::declared(
            from_alpha,
            vec![LinkFault::DropAt {
                position: SendOrdinal::at(0u32),
            }],
        )],
    )?;
    let campaign = NetworkCampaign::declared(vec![quiet_control()?, hostile])?;
    let mut sim = SimNet::declared(topology, campaign.select(name("quiet-control")?)?)?;
    sim.send(from_alpha, Payment { amount: 5i128 })?;
    sim.send(from_beta, Payment { amount: -5i128 })?;
    let mut alpha_commands = Vec::new();
    let mut beta_commands = Vec::new();
    while sim.pending() > 0usize {
        for delivery in sim.advance() {
            if delivery.link() == from_alpha {
                alpha_commands.push(*delivery.payload());
            } else {
                beta_commands.push(*delivery.payload());
            }
        }
    }
    let set = StrandSet::declared(vec![
        Strand::declared(name("from-alpha")?, alpha_commands)?,
        Strand::declared(name("from-beta")?, beta_commands)?,
    ])?;
    let contract = TransitionContract::declared(
        opening_balance,
        applied_payment,
        vec![TemporalClaim::declared(
            NEVER_NEGATIVE,
            TemporalDemand::Never(negative),
        )],
    )?;
    let reading = explored(
        &set,
        &contract,
        ExplorationBound::declared(8u32, 4u32)?,
        PopulationRef::named("lane", "delivery-orders")?,
        RootSeed::declared(11u64),
    )?;
    let ExplorationStanding::CounterexampleFound(counterexample) = reading.standing() else {
        return Err(LaneFailure::Standing);
    };
    assert_eq!(counterexample.interleaving().choices(), [1u8, 0u8]);
    assert_eq!(counterexample.finding().cause(), NEVER_NEGATIVE);
    Ok(())
}

/// A malformed topology refuses at exactly the clause it breaks.
#[test]
fn a_malformed_topology_refuses_at_its_clause() -> Result<(), LaneFailure> {
    let a = client()?;
    let b = server()?;
    let stranger = NodeRef::declared(name("stranger")?);
    let wire = Link::between(a, b);
    assert_eq!(
        Topology::declared(Vec::new(), vec![wire]),
        Err(TopologyRefusal::NoNode)
    );
    assert_eq!(
        Topology::declared(vec![a, a], vec![wire]),
        Err(TopologyRefusal::DuplicateNode(a))
    );
    assert_eq!(
        Topology::declared(vec![a, b], Vec::new()),
        Err(TopologyRefusal::NoLink)
    );
    assert_eq!(
        Topology::declared(vec![a, b], vec![wire, wire]),
        Err(TopologyRefusal::DuplicateLink(wire))
    );
    assert_eq!(
        Topology::declared(vec![a, b], vec![Link::between(a, stranger)]),
        Err(TopologyRefusal::LinkForeignNode { node: stranger })
    );
    Ok(())
}

/// A schedule or campaign that declares pressure incoherently refuses at its clause.
#[test]
fn an_incoherent_discipline_refuses_at_its_clause() -> Result<(), LaneFailure> {
    assert_eq!(TickSpan::declared(0u32), Err(TickSpanRefusal::ZeroTicks));
    let wire = forward()?;
    let drop_fault = LinkFault::DropAt {
        position: SendOrdinal::at(0u32),
    };
    assert_eq!(
        NetworkSchedule::declared(
            name("twice")?,
            vec![
                LinkDiscipline::declared(wire, vec![drop_fault]),
                LinkDiscipline::declared(wire, vec![drop_fault]),
            ],
        ),
        Err(NetworkScheduleRefusal::DuplicateDiscipline(wire))
    );
    assert_eq!(
        NetworkSchedule::declared(
            name("hollow")?,
            vec![LinkDiscipline::declared(wire, Vec::new())],
        ),
        Err(NetworkScheduleRefusal::EmptyDiscipline(wire))
    );
    assert_eq!(
        NetworkSchedule::declared(
            name("never-open")?,
            vec![LinkDiscipline::declared(
                wire,
                vec![LinkFault::Partition {
                    opens: Tick::at(5u64),
                    heals: Tick::at(5u64),
                }],
            )],
        ),
        Err(NetworkScheduleRefusal::EmptyPartition { link: wire })
    );
    assert_eq!(
        NetworkCampaign::declared(Vec::new()),
        Err(NetworkCampaignRefusal::NoSchedule)
    );
    let twin = name("twin")?;
    assert_eq!(
        NetworkCampaign::declared(vec![
            NetworkSchedule::declared(twin, Vec::new())?,
            NetworkSchedule::declared(twin, Vec::new())?,
        ]),
        Err(NetworkCampaignRefusal::DuplicateSchedule(twin))
    );
    assert_eq!(
        NetworkCampaign::declared(vec![quiet_control()?]),
        Err(NetworkCampaignRefusal::NoFaultDeclared)
    );
    let lawful = NetworkCampaign::declared(vec![
        quiet_control()?,
        NetworkSchedule::declared(
            name("real")?,
            vec![LinkDiscipline::declared(wire, vec![drop_fault])],
        )?,
    ])?;
    assert_eq!(
        lawful.select(name("absent")?).err(),
        Some(NetworkSelectionRefusal::ScheduleAbsent(name("absent")?))
    );
    Ok(())
}

/// A sim refuses a schedule outside its topology, and a send outside its links.
#[test]
fn a_sim_refuses_foreign_disciplines_and_undeclared_links() -> Result<(), LaneFailure> {
    let stranger_link = Link::between(server()?, server()?);
    let foreign = NetworkSchedule::declared(
        name("foreign")?,
        vec![LinkDiscipline::declared(
            stranger_link,
            vec![LinkFault::DropAt {
                position: SendOrdinal::at(0u32),
            }],
        )],
    )?;
    let campaign = NetworkCampaign::declared(vec![quiet_control()?, foreign])?;
    assert_eq!(
        SimNet::<Message>::declared(pair_topology()?, campaign.select(name("foreign")?)?).err(),
        Some(SimNetRefusal::DisciplineForeignLink {
            link: stranger_link
        })
    );
    let mut sim = SimNet::declared(pair_topology()?, campaign.select(name("quiet-control")?)?)?;
    assert_eq!(
        sim.send(stranger_link, Message::Request { id: 1u64 }).err(),
        Some(SendRefusal::LinkUndeclared(stranger_link))
    );
    Ok(())
}
