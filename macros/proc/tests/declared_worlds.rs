//! The two declaration macros applied for real: the generated builders build a working world, and the generated exploration explores it.
//!
//! Compiling this file is half the claim — every generated item stands under the workspace's own lint wall — and the tests are the other half: the declared topology drives a real sim whose partition takes the first send, and the declared exploration finds the same withdraw-before-deposit counterexample the hand-rolled road finds, concluded into the ordinary trial vocabulary.

use macroonz_harness::descriptor::NameRefusal;
use macroonz_harness::interleave::{
    ExplorationStanding, InterleavingSpace, Strand, StrandRefusal, StrandSet, StrandSetRefusal,
};
use macroonz_harness::network::{
    LinkFault, NetworkCampaign, NetworkCampaignRefusal, NetworkSelectionRefusal, SendFate,
    SendRefusal, SimNet, SimNetRefusal,
};
use macroonz_harness::properties::{
    ContractRefusal, Holding, TemporalClaim, TemporalDemand, TransitionContract,
};
use macroonz_harness::report::{FindingCause, TrialConclusion};

macroonz_macros::network! {
    module = net,
    namespace = "proc",
    nodes = [client, server],
    link forward = client to server,
    link back = server to client,
    schedule quiet = [],
    schedule outage = [
        drop forward at 4,
        delay forward at 1 by 2,
        duplicate back at 0,
        partition forward from 0 until 3,
    ],
}

macroonz_macros::concurrency! {
    module = explorations,
    namespace = "proc",
    transfers_never_overdraw {
        population = "transfer-orders",
        interleavings = 16,
        samples = 32,
        seed = 11,
    },
}

/// The cause a history that overdraws is cited under.
const NEVER_OVERDRAWN: FindingCause = FindingCause::named("proc", "never-overdrawn");

/// One step a party takes against the account.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    /// Add this much.
    Deposit(u64),
    /// Take this much, overdrawing if it is not there.
    Withdraw(u64),
}

/// The account two parties race over.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Account {
    balance: i128,
}

/// Every history opens on an empty account.
fn opening() -> Account {
    Account { balance: 0i128 }
}

/// One move against the account.
fn applied(state: &Account, command: &Move) -> Account {
    match *command {
        Move::Deposit(amount) => Account {
            balance: state.balance.saturating_add(i128::from(amount)),
        },
        Move::Withdraw(amount) => Account {
            balance: state.balance.saturating_sub(i128::from(amount)),
        },
    }
}

/// Whether the account stands below zero.
fn negative(state: &Account) -> Holding {
    if state.balance < 0i128 {
        Holding::Holds
    } else {
        Holding::Fails
    }
}

/// Everything a lane road can refuse, carried as itself.
enum LaneFailure {
    Name(NameRefusal),
    Strand(StrandRefusal),
    Set(StrandSetRefusal),
    Contract(ContractRefusal),
    Campaign(NetworkCampaignRefusal),
    Selection(NetworkSelectionRefusal),
    Sim(SimNetRefusal),
    Send(SendRefusal),
    Net(net::Fault),
    Exploration(explorations::Fault),
    /// A value did not carry the shape the claim demanded.
    Standing,
}

impl core::fmt::Debug for LaneFailure {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Name(refusal) => formatter.debug_tuple("Name").field(refusal).finish(),
            Self::Strand(refusal) => formatter.debug_tuple("Strand").field(refusal).finish(),
            Self::Set(refusal) => formatter.debug_tuple("Set").field(refusal).finish(),
            Self::Contract(refusal) => formatter.debug_tuple("Contract").field(refusal).finish(),
            Self::Campaign(refusal) => formatter.debug_tuple("Campaign").field(refusal).finish(),
            Self::Selection(refusal) => formatter.debug_tuple("Selection").field(refusal).finish(),
            Self::Sim(refusal) => formatter.debug_tuple("Sim").field(refusal).finish(),
            Self::Send(refusal) => formatter.debug_tuple("Send").field(refusal).finish(),
            Self::Net(refusal) => formatter.debug_tuple("Net").field(refusal).finish(),
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

impl From<ContractRefusal> for LaneFailure {
    fn from(refusal: ContractRefusal) -> Self {
        Self::Contract(refusal)
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

impl From<net::Fault> for LaneFailure {
    fn from(refusal: net::Fault) -> Self {
        Self::Net(refusal)
    }
}

impl From<explorations::Fault> for LaneFailure {
    fn from(refusal: explorations::Fault) -> Self {
        Self::Exploration(refusal)
    }
}

/// One depositor and one withdrawer over the same five units.
fn transfer_set() -> Result<StrandSet<Move>, LaneFailure> {
    let depositor = Strand::declared(
        macroonz_harness::descriptor::NamespacedName::named("proc", "depositor")?,
        vec![Move::Deposit(5u64)],
    )?;
    let withdrawer = Strand::declared(
        macroonz_harness::descriptor::NamespacedName::named("proc", "withdrawer")?,
        vec![Move::Withdraw(5u64)],
    )?;
    Ok(StrandSet::declared(vec![depositor, withdrawer])?)
}

/// The contract that no history may go below zero.
fn solvency_contract() -> Result<TransitionContract<Account, Move>, ContractRefusal> {
    TransitionContract::declared(
        opening,
        applied,
        vec![TemporalClaim::declared(
            NEVER_OVERDRAWN,
            TemporalDemand::Never(negative),
        )],
    )
}

/// The declared topology and schedules build, compose into a campaign in one line, and drive a real sim.
#[test]
fn the_declared_world_builds_and_the_partition_bites() -> Result<(), LaneFailure> {
    let topology = net::topology()?;
    assert_eq!(topology.nodes().len(), 2usize);
    assert_eq!(topology.links().len(), 2usize);
    let quiet = net::quiet()?;
    assert!(quiet.disciplines().is_empty());
    let outage = net::outage()?;
    assert_eq!(outage.disciplines().len(), 2usize);
    let forward_discipline = outage.disciplines().first().ok_or(LaneFailure::Standing)?;
    assert_eq!(forward_discipline.faults().len(), 3usize);
    assert!(
        forward_discipline
            .faults()
            .iter()
            .any(|fault| matches!(fault, LinkFault::Partition { .. }))
    );
    let campaign = NetworkCampaign::declared(vec![net::quiet()?, net::outage()?])?;
    let selection = campaign.select(outage.name())?;
    let mut sim = SimNet::declared(net::topology()?, selection)?;
    let forward = *topology.links().first().ok_or(LaneFailure::Standing)?;
    let receipt = sim.send(forward, 7u8)?;
    assert_eq!(receipt.fate(), SendFate::DroppedByPartition);
    Ok(())
}

/// The declared exploration finds the withdraw-first counterexample and concludes it into the trial vocabulary.
#[test]
fn the_declared_exploration_finds_and_concludes() -> Result<(), LaneFailure> {
    let set = transfer_set()?;
    let contract = solvency_contract()?;
    let (reading, conclusion) = explorations::transfers_never_overdraw(&set, &contract)?;
    assert_eq!(reading.space(), InterleavingSpace::Counted(2u128));
    let ExplorationStanding::CounterexampleFound(counterexample) = reading.standing() else {
        return Err(LaneFailure::Standing);
    };
    assert_eq!(counterexample.interleaving().choices(), [1u8, 0u8]);
    let TrialConclusion::Refused(finding) = conclusion else {
        return Err(LaneFailure::Standing);
    };
    assert_eq!(finding.cause(), NEVER_OVERDRAWN);
    let (again, _conclusion) = explorations::transfers_never_overdraw(&set, &contract)?;
    assert_eq!(again, reading);
    Ok(())
}
