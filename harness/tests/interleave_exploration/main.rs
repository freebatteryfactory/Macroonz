//! The exploration road, exercised from outside: orders that break are caught at their interleaving, orders that commute survive the whole space, and both walks say exactly what they established.
//!
//! One toy transfer system carries every claim: two parties over one account, where a withdrawal before its deposit overdraws.
//! The exhaustive lanes pin the counterexample's exact interleaving and site, the sampled lanes pin census honesty and replay, the fault lane composes per-party adversity through the ordinary fault home, and the refusal lanes reverse one clause each of what a strand, a set, a bound, and an encoding promise.

use core::cmp::Ordering;
use macroonz_harness::descriptor::{NameRefusal, NamespacedName, PopulationRef};
use macroonz_harness::fault::{
    FaultAdapter, FaultCampaign, FaultCampaignRefusal, FaultInjectionRefusal, FaultSchedule,
    FaultSelectionRefusal, InjectedCommand, ScheduledFault, SequencePosition, inject,
};
use macroonz_harness::generate::{GenerationDisposition, GenerationHalt, RootSeed};
use macroonz_harness::interleave::{
    ADDRESSABLE_STRANDS, Counterexample, EncodingRefusal, ExplorationBound,
    ExplorationBoundRefusal, ExplorationMode, ExplorationReading, ExplorationRefusal,
    ExplorationSite, ExplorationStanding, Interleaving, InterleavingSpace, Strand, StrandRefusal,
    StrandSet, StrandSetRefusal, concluded, encoded, explored, interpreted,
};
use macroonz_harness::properties::{
    ContractRefusal, Holding, TemporalClaim, TemporalDemand, TransitionContract, holds_over_history,
};
use macroonz_harness::report::{FindingCause, TrialConclusion};

/// The cause a history that overdraws is cited under.
const NEVER_OVERDRAWN: FindingCause = FindingCause::named("lane", "never-overdrawn");

/// The cause a history whose balance shrank is cited under.
const BALANCE_GREW: FindingCause = FindingCause::named("lane", "balance-never-decreases");

/// One step a party takes against the account.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    /// Add this much.
    Deposit(u64),
    /// Take this much, overdrawing if it is not there.
    Withdraw(u64),
}

/// Whether the account has ever gone below zero; once it has, it stays marked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Solvency {
    /// The balance has never gone below zero.
    Standing,
    /// The balance went below zero at some point.
    Overdrawn,
}

/// The account two parties race over.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Account {
    balance: i128,
    solvency: Solvency,
}

/// Every history opens on an empty, solvent account.
fn opening() -> Account {
    Account {
        balance: 0i128,
        solvency: Solvency::Standing,
    }
}

/// One move against the account; an overdraft latches.
fn applied(state: &Account, command: &Move) -> Account {
    let balance = match *command {
        Move::Deposit(amount) => state.balance.saturating_add(i128::from(amount)),
        Move::Withdraw(amount) => state.balance.saturating_sub(i128::from(amount)),
    };
    let solvency = if balance < 0i128 {
        Solvency::Overdrawn
    } else {
        state.solvency
    };
    Account { balance, solvency }
}

/// Whether the account stands overdrawn.
fn overdrawn(state: &Account) -> Holding {
    match state.solvency {
        Solvency::Overdrawn => Holding::Holds,
        Solvency::Standing => Holding::Fails,
    }
}

/// Accounts ranked by balance.
fn by_balance(earlier: &Account, later: &Account) -> Ordering {
    earlier.balance.cmp(&later.balance)
}

/// Everything a lane road can refuse, carried as itself.
enum LaneFailure {
    Name(NameRefusal),
    Strand(StrandRefusal),
    Set(StrandSetRefusal),
    Bound(ExplorationBoundRefusal),
    Exploration(ExplorationRefusal),
    Contract(ContractRefusal),
    Encoding(EncodingRefusal),
    Campaign(FaultCampaignRefusal),
    Selection(FaultSelectionRefusal),
    Injection(FaultInjectionRefusal),
    /// A reading did not carry the shape the claim demanded.
    Standing,
}

impl core::fmt::Debug for LaneFailure {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Name(refusal) => formatter.debug_tuple("Name").field(refusal).finish(),
            Self::Strand(refusal) => formatter.debug_tuple("Strand").field(refusal).finish(),
            Self::Set(refusal) => formatter.debug_tuple("Set").field(refusal).finish(),
            Self::Bound(refusal) => formatter.debug_tuple("Bound").field(refusal).finish(),
            Self::Exploration(refusal) => {
                formatter.debug_tuple("Exploration").field(refusal).finish()
            }
            Self::Contract(refusal) => formatter.debug_tuple("Contract").field(refusal).finish(),
            Self::Encoding(refusal) => formatter.debug_tuple("Encoding").field(refusal).finish(),
            Self::Campaign(refusal) => formatter.debug_tuple("Campaign").field(refusal).finish(),
            Self::Selection(refusal) => formatter.debug_tuple("Selection").field(refusal).finish(),
            Self::Injection(refusal) => formatter.debug_tuple("Injection").field(refusal).finish(),
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

impl From<ContractRefusal> for LaneFailure {
    fn from(refusal: ContractRefusal) -> Self {
        Self::Contract(refusal)
    }
}

impl From<EncodingRefusal> for LaneFailure {
    fn from(refusal: EncodingRefusal) -> Self {
        Self::Encoding(refusal)
    }
}

impl From<FaultCampaignRefusal> for LaneFailure {
    fn from(refusal: FaultCampaignRefusal) -> Self {
        Self::Campaign(refusal)
    }
}

impl From<FaultSelectionRefusal> for LaneFailure {
    fn from(refusal: FaultSelectionRefusal) -> Self {
        Self::Selection(refusal)
    }
}

impl From<FaultInjectionRefusal> for LaneFailure {
    fn from(refusal: FaultInjectionRefusal) -> Self {
        Self::Injection(refusal)
    }
}

/// One lane-owned name.
fn name(stem: &'static str) -> Result<NamespacedName, NameRefusal> {
    NamespacedName::named("lane", stem)
}

/// The population every sampled draw in this lane is declared under.
fn population() -> Result<PopulationRef, NameRefusal> {
    PopulationRef::named("lane", "interleave-choices")
}

/// One depositor and one withdrawer over the same five units.
fn transfer_set() -> Result<StrandSet<Move>, LaneFailure> {
    let depositor = Strand::declared(name("depositor")?, vec![Move::Deposit(5u64)])?;
    let withdrawer = Strand::declared(name("withdrawer")?, vec![Move::Withdraw(5u64)])?;
    Ok(StrandSet::declared(vec![depositor, withdrawer])?)
}

/// The contract that no history may overdraw.
fn solvency_contract() -> Result<TransitionContract<Account, Move>, ContractRefusal> {
    TransitionContract::declared(
        opening,
        applied,
        vec![TemporalClaim::declared(
            NEVER_OVERDRAWN,
            TemporalDemand::Never(overdrawn),
        )],
    )
}

/// The counterexample a reading carries, or the lane's own refusal.
fn found(reading: &ExplorationReading) -> Result<&Counterexample, LaneFailure> {
    match reading.standing() {
        ExplorationStanding::CounterexampleFound(counterexample) => Ok(counterexample),
        ExplorationStanding::SpaceExhaustedAllHold | ExplorationStanding::SampledAllHold => {
            Err(LaneFailure::Standing)
        }
    }
}

/// A withdrawal ordered before its deposit is caught at exactly that interleaving, in an exhausted space of two.
#[test]
fn a_noncommutative_order_is_caught_at_its_interleaving() -> Result<(), LaneFailure> {
    let set = transfer_set()?;
    let contract = solvency_contract()?;
    let reading = explored(
        &set,
        &contract,
        ExplorationBound::declared(16u32, 8u32)?,
        population()?,
        RootSeed::declared(11u64),
    )?;
    assert_eq!(reading.space(), InterleavingSpace::Counted(2u128));
    assert_eq!(reading.mode(), ExplorationMode::Exhaustive);
    assert_eq!(reading.explored(), 2u64);
    let counterexample = found(&reading)?;
    assert_eq!(counterexample.interleaving().choices(), [1u8, 0u8]);
    assert_eq!(
        counterexample.site(),
        ExplorationSite::Enumerated { ordinal: 1u64 }
    );
    assert_eq!(counterexample.finding().cause(), NEVER_OVERDRAWN);
    let TrialConclusion::Refused(finding) = concluded(&reading) else {
        return Err(LaneFailure::Standing);
    };
    assert_eq!(finding.cause(), NEVER_OVERDRAWN);
    Ok(())
}

/// Two parties that only deposit hold a monotonicity claim across every one of their six merge orders.
#[test]
fn a_commutative_control_holds_over_the_exhausted_space() -> Result<(), LaneFailure> {
    let steady = Strand::declared(
        name("steady")?,
        vec![Move::Deposit(1u64), Move::Deposit(2u64)],
    )?;
    let eager = Strand::declared(
        name("eager")?,
        vec![Move::Deposit(3u64), Move::Deposit(4u64)],
    )?;
    let set = StrandSet::declared(vec![steady, eager])?;
    let contract = TransitionContract::declared(
        opening,
        applied,
        vec![TemporalClaim::declared(
            BALANCE_GREW,
            TemporalDemand::NeverDecreases(by_balance),
        )],
    )?;
    let reading = explored(
        &set,
        &contract,
        ExplorationBound::declared(16u32, 8u32)?,
        population()?,
        RootSeed::declared(11u64),
    )?;
    assert_eq!(reading.space(), InterleavingSpace::Counted(6u128));
    assert_eq!(reading.mode(), ExplorationMode::Exhaustive);
    assert_eq!(reading.explored(), 6u64);
    assert_eq!(
        reading.standing(),
        &ExplorationStanding::SpaceExhaustedAllHold
    );
    assert_eq!(concluded(&reading), TrialConclusion::Passed);
    Ok(())
}

/// Beyond the declared ceiling the space is sampled, the census counts every drawn schedule, the standing claims only the sample, and one seed reads the same twice.
#[test]
fn beyond_the_bound_the_space_is_sampled_and_the_census_says_so() -> Result<(), LaneFailure> {
    let steady = Strand::declared(name("steady")?, vec![Move::Deposit(1u64); 6])?;
    let eager = Strand::declared(name("eager")?, vec![Move::Deposit(2u64); 6])?;
    let set = StrandSet::declared(vec![steady, eager])?;
    let contract = TransitionContract::declared(
        opening,
        applied,
        vec![TemporalClaim::declared(
            BALANCE_GREW,
            TemporalDemand::NeverDecreases(by_balance),
        )],
    )?;
    let bound = ExplorationBound::declared(100u32, 32u32)?;
    let reading = explored(
        &set,
        &contract,
        bound,
        population()?,
        RootSeed::declared(3u64),
    )?;
    assert_eq!(reading.space(), InterleavingSpace::Counted(924u128));
    assert_eq!(reading.explored(), 32u64);
    assert_eq!(reading.standing(), &ExplorationStanding::SampledAllHold);
    let ExplorationMode::Sampled { census, halt } = reading.mode() else {
        return Err(LaneFailure::Standing);
    };
    assert_eq!(census.count_of(GenerationDisposition::Generated), 32u32);
    assert_eq!(halt, GenerationHalt::CaseBudgetMet);
    assert_eq!(concluded(&reading), TrialConclusion::Passed);
    let again = explored(
        &set,
        &contract,
        bound,
        population()?,
        RootSeed::declared(3u64),
    )?;
    assert_eq!(reading, again);
    for seed in [0u64, u64::MAX, 0xA5A5_5A5A_F0F0_0F0Fu64] {
        let first = explored(
            &set,
            &contract,
            bound,
            population()?,
            RootSeed::declared(seed),
        )?;
        let repeated = explored(
            &set,
            &contract,
            bound,
            population()?,
            RootSeed::declared(seed),
        )?;
        assert_eq!(first, repeated);
    }
    Ok(())
}

/// A larger four-party space is still walked literally when its exact multinomial count fits the declared exhaustive ceiling.
#[test]
#[ignore = "long deterministic local exhaustive-schedule campaign; run explicitly"]
fn a_long_exhaustive_campaign_walks_every_counted_schedule() -> Result<(), LaneFailure> {
    let north = Strand::declared(name("north")?, vec![Move::Deposit(1u64); 3])?;
    let east = Strand::declared(name("east")?, vec![Move::Deposit(2u64); 3])?;
    let south = Strand::declared(name("south")?, vec![Move::Deposit(3u64); 3])?;
    let west = Strand::declared(name("west")?, vec![Move::Deposit(4u64); 3])?;
    let set = StrandSet::declared(vec![north, east, south, west])?;
    let contract = TransitionContract::declared(
        opening,
        applied,
        vec![TemporalClaim::declared(
            BALANCE_GREW,
            TemporalDemand::NeverDecreases(by_balance),
        )],
    )?;
    let reading = explored(
        &set,
        &contract,
        ExplorationBound::declared(369_600u32, 1u32)?,
        population()?,
        RootSeed::declared(0u64),
    )?;
    assert_eq!(reading.space(), InterleavingSpace::Counted(369_600u128));
    assert_eq!(reading.mode(), ExplorationMode::Exhaustive);
    assert_eq!(reading.explored(), 369_600u64);
    assert_eq!(
        reading.standing(),
        &ExplorationStanding::SpaceExhaustedAllHold
    );
    assert_eq!(concluded(&reading), TrialConclusion::Passed);
    Ok(())
}

/// A larger space remains explicitly sampled, and one declared seed reproduces the same bounded reading without gaining exhaustive standing.
#[test]
#[ignore = "long deterministic local sampled-schedule campaign; run explicitly"]
fn a_long_sampled_campaign_repeats_without_claiming_the_space() -> Result<(), LaneFailure> {
    let steady = Strand::declared(name("steady-long")?, vec![Move::Deposit(1u64); 32])?;
    let eager = Strand::declared(name("eager-long")?, vec![Move::Deposit(2u64); 32])?;
    let set = StrandSet::declared(vec![steady, eager])?;
    let contract = TransitionContract::declared(
        opening,
        applied,
        vec![TemporalClaim::declared(
            BALANCE_GREW,
            TemporalDemand::NeverDecreases(by_balance),
        )],
    )?;
    let bound = ExplorationBound::declared(1_000_000u32, 4_096u32)?;
    let seed = RootSeed::declared(0xA5A5_5A5A_F0F0_0F0Fu64);
    let first = explored(&set, &contract, bound, population()?, seed)?;
    let repeated = explored(&set, &contract, bound, population()?, seed)?;
    assert_eq!(first, repeated);
    assert_eq!(
        first.space(),
        InterleavingSpace::Counted(1_832_624_140_942_590_534u128)
    );
    assert_eq!(first.explored(), 4_096u64);
    assert_eq!(first.standing(), &ExplorationStanding::SampledAllHold);
    let ExplorationMode::Sampled { census, halt } = first.mode() else {
        return Err(LaneFailure::Standing);
    };
    assert_eq!(census.count_of(GenerationDisposition::Generated), 4_096u32);
    assert_eq!(halt, GenerationHalt::CaseBudgetMet);
    assert_eq!(concluded(&first), TrialConclusion::Passed);
    Ok(())
}

/// A sampled counterexample replays through its interleaving alone, and a one-byte material still breaks the same claim.
#[test]
fn a_sampled_counterexample_replays_through_its_interleaving() -> Result<(), LaneFailure> {
    let depositor = Strand::declared(name("depositor")?, vec![Move::Deposit(5u64); 7])?;
    let withdrawer = Strand::declared(name("withdrawer")?, vec![Move::Withdraw(5u64)])?;
    let set = StrandSet::declared(vec![depositor, withdrawer])?;
    let contract = solvency_contract()?;
    let reading = explored(
        &set,
        &contract,
        ExplorationBound::declared(4u32, 16u32)?,
        population()?,
        RootSeed::declared(7u64),
    )?;
    assert_eq!(reading.space(), InterleavingSpace::Counted(8u128));
    let counterexample = found(&reading)?;
    let ExplorationSite::Sampled { case: _ } = counterexample.site() else {
        return Err(LaneFailure::Standing);
    };
    let material = encoded(&set, counterexample.interleaving())?;
    let replayed = interpreted(&set, &material);
    assert_eq!(replayed.interleaving(), counterexample.interleaving());
    let TrialConclusion::Refused(finding) = holds_over_history(&contract, replayed.commands())
    else {
        return Err(LaneFailure::Standing);
    };
    assert_eq!(finding.cause(), counterexample.finding().cause());
    assert_eq!(finding.class(), counterexample.finding().class());
    let shortest = interpreted(&set, &[1u8]);
    let TrialConclusion::Refused(shrunk) = holds_over_history(&contract, shortest.commands())
    else {
        return Err(LaneFailure::Standing);
    };
    assert_eq!(shrunk.cause(), NEVER_OVERDRAWN);
    Ok(())
}

/// A fault schedule injected into one strand rides exploration like any commands: the suppressed withdrawal can no longer overdraw any order.
#[test]
fn faulted_strands_explore_like_any_other_commands() -> Result<(), LaneFailure> {
    /// What the lane's one fault does.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Suppression {
        /// The command at this position is not applied.
        DropTheCommand,
    }
    /// What the lane promises still holds when its fault fires.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Promise {
        /// The account is exactly what it was.
        StateUnchanged,
    }
    /// One move under injected adversity; a faulted move is dropped.
    fn applied_under_faults(
        state: &Account,
        command: &InjectedCommand<Move, Suppression, Promise>,
    ) -> Account {
        if command.faults().is_empty() {
            applied(state, command.command())
        } else {
            *state
        }
    }
    let quiet = FaultSchedule::declared(name("quiet-control")?, Vec::new());
    let suppressed = FaultSchedule::declared(
        name("suppress-the-withdrawal")?,
        vec![ScheduledFault::at(
            SequencePosition::at(0u32),
            FaultAdapter::declared(Suppression::DropTheCommand, Promise::StateUnchanged),
        )],
    );
    let campaign = FaultCampaign::declared(vec![quiet, suppressed])?;
    let calm = inject(
        &campaign.select(name("quiet-control")?)?,
        vec![Move::Deposit(5u64)],
    )?;
    let hostile = inject(
        &campaign.select(name("suppress-the-withdrawal")?)?,
        vec![Move::Withdraw(5u64)],
    )?;
    let depositor = Strand::declared(name("depositor")?, calm.commands().to_vec())?;
    let withdrawer = Strand::declared(name("withdrawer")?, hostile.commands().to_vec())?;
    let set = StrandSet::declared(vec![depositor, withdrawer])?;
    let contract = TransitionContract::declared(
        opening,
        applied_under_faults,
        vec![TemporalClaim::declared(
            NEVER_OVERDRAWN,
            TemporalDemand::Never(overdrawn),
        )],
    )?;
    let reading = explored(
        &set,
        &contract,
        ExplorationBound::declared(16u32, 8u32)?,
        population()?,
        RootSeed::declared(11u64),
    )?;
    assert_eq!(reading.explored(), 2u64);
    assert_eq!(
        reading.standing(),
        &ExplorationStanding::SpaceExhaustedAllHold
    );
    Ok(())
}

/// An authored interleaving encodes to its material, realizes back to itself, and merges in exactly the spelled order.
#[test]
fn a_directed_interleaving_is_authored_encoded_and_realized() -> Result<(), LaneFailure> {
    let set = transfer_set()?;
    let authored = Interleaving::declared(vec![1u8, 0u8]);
    let material = encoded(&set, &authored)?;
    assert_eq!(material, vec![1u8, 0u8]);
    let realized = interpreted(&set, &material);
    assert_eq!(realized.interleaving(), &authored);
    assert_eq!(
        realized.commands(),
        [Move::Withdraw(5u64), Move::Deposit(5u64)]
    );
    Ok(())
}

/// Every schedule in a two-by-two space roundtrips through material, and the roster count agrees with the multinomial reading.
#[test]
fn every_small_space_schedule_roundtrips_and_matches_its_count() -> Result<(), LaneFailure> {
    let steady = Strand::declared(
        name("steady")?,
        vec![Move::Deposit(1u64), Move::Deposit(2u64)],
    )?;
    let eager = Strand::declared(
        name("eager")?,
        vec![Move::Deposit(3u64), Move::Deposit(4u64)],
    )?;
    let set = StrandSet::declared(vec![steady, eager])?;
    let choices = [
        [0u8, 0u8, 1u8, 1u8],
        [0u8, 1u8, 0u8, 1u8],
        [0u8, 1u8, 1u8, 0u8],
        [1u8, 0u8, 0u8, 1u8],
        [1u8, 0u8, 1u8, 0u8],
        [1u8, 1u8, 0u8, 0u8],
    ];
    let mut materials = Vec::new();
    for spelling in choices {
        let authored = Interleaving::declared(spelling.to_vec());
        let material = encoded(&set, &authored)?;
        assert!(!materials.contains(&material));
        let realized = interpreted(&set, &material);
        assert_eq!(realized.interleaving(), &authored);
        materials.push(material);
    }
    assert_eq!(materials.len(), 6usize);
    let reading = explored(
        &set,
        &TransitionContract::declared(
            opening,
            applied,
            vec![TemporalClaim::declared(
                BALANCE_GREW,
                TemporalDemand::NeverDecreases(by_balance),
            )],
        )?,
        ExplorationBound::declared(6u32, 1u32)?,
        population()?,
        RootSeed::declared(1u64),
    )?;
    assert_eq!(reading.space(), InterleavingSpace::Counted(6u128));
    assert_eq!(
        reading.explored(),
        u64::try_from(materials.len()).unwrap_or(u64::MAX)
    );
    Ok(())
}

/// Empty, hostile, and surplus material all realize a complete deterministic schedule, and canonical encoding replays it.
#[test]
fn arbitrary_material_is_total_deterministic_and_replayable() -> Result<(), LaneFailure> {
    let set = transfer_set()?;
    let materials = [Vec::new(), vec![u8::MAX], vec![u8::MAX, 0u8, 91u8]];
    for material in materials {
        let first = interpreted(&set, &material);
        let repeated = interpreted(&set, &material);
        assert_eq!(first, repeated);
        assert_eq!(first.commands().len(), set.steps());
        let canonical = encoded(&set, first.interleaving())?;
        assert_eq!(interpreted(&set, &canonical), first);
    }
    assert_eq!(interpreted(&set, &[]).interleaving().choices(), [0u8, 1u8]);
    assert_eq!(
        interpreted(&set, &[u8::MAX, 0u8, 91u8])
            .interleaving()
            .choices(),
        [1u8, 0u8]
    );
    Ok(())
}

/// A strand with no commands names a party that never acts, and is refused where it is written.
#[test]
fn an_empty_strand_refuses() -> Result<(), LaneFailure> {
    let idle = name("idle")?;
    assert_eq!(
        Strand::<Move>::declared(idle, Vec::new()),
        Err(StrandRefusal::EmptyStrand(idle))
    );
    Ok(())
}

/// A lone strand has nothing to reorder, and the set says so.
#[test]
fn a_lone_strand_refuses() -> Result<(), LaneFailure> {
    let solo = Strand::declared(name("solo")?, vec![Move::Deposit(1u64)])?;
    assert_eq!(
        StrandSet::declared(vec![solo]),
        Err(StrandSetRefusal::FewerThanTwoStrands { strands: 1usize })
    );
    Ok(())
}

/// Two strands under one name would leave a choice with two answers.
#[test]
fn a_repeated_strand_name_refuses() -> Result<(), LaneFailure> {
    let twin = name("twin")?;
    let first = Strand::declared(twin, vec![Move::Deposit(1u64)])?;
    let second = Strand::declared(twin, vec![Move::Deposit(2u64)])?;
    assert_eq!(
        StrandSet::declared(vec![first, second]),
        Err(StrandSetRefusal::DuplicateStrand(twin))
    );
    Ok(())
}

/// One choice byte cannot address a set with more than 256 parties, and the set refuses that bound before exploration.
#[test]
fn more_strands_than_one_choice_byte_addresses_refuses() -> Result<(), LaneFailure> {
    let words = [
        "s00", "s01", "s02", "s03", "s04", "s05", "s06", "s07", "s08", "s09", "s10", "s11", "s12",
        "s13", "s14", "s15", "s16",
    ];
    let mut strands = Vec::new();
    'names: for &namespace in &words {
        for &stem in &words {
            strands.push(Strand::declared(
                NamespacedName::named(namespace, stem)?,
                vec![Move::Deposit(1u64)],
            )?);
            if strands.len() > ADDRESSABLE_STRANDS {
                break 'names;
            }
        }
    }
    assert_eq!(
        StrandSet::declared(strands),
        Err(StrandSetRefusal::MoreStrandsThanAddressable {
            strands: ADDRESSABLE_STRANDS.saturating_add(1usize)
        })
    );
    Ok(())
}

/// Exactly [`ADDRESSABLE_STRANDS`] parties remain lawful; the refusal boundary is strict greater-than.
#[test]
fn the_addressable_strand_ceiling_itself_is_admitted() -> Result<(), LaneFailure> {
    let words = [
        "s00", "s01", "s02", "s03", "s04", "s05", "s06", "s07", "s08", "s09", "s10", "s11", "s12",
        "s13", "s14", "s15", "s16",
    ];
    let mut strands = Vec::new();
    'names: for &namespace in &words {
        for &stem in &words {
            strands.push(Strand::declared(
                NamespacedName::named(namespace, stem)?,
                vec![Move::Deposit(1u64)],
            )?);
            if strands.len() == ADDRESSABLE_STRANDS {
                break 'names;
            }
        }
    }
    assert_eq!(strands.len(), ADDRESSABLE_STRANDS);
    let set = StrandSet::declared(strands)?;
    assert_eq!(set.strands().len(), ADDRESSABLE_STRANDS);
    Ok(())
}

/// A bound with an empty seat on either side is a budget that could never spend.
#[test]
fn a_zero_seated_bound_refuses() {
    assert_eq!(
        ExplorationBound::declared(0u32, 8u32),
        Err(ExplorationBoundRefusal::ZeroInterleavings)
    );
    assert_eq!(
        ExplorationBound::declared(8u32, 0u32),
        Err(ExplorationBoundRefusal::ZeroSamples)
    );
}

/// An interleaving foreign to the set refuses at the exact clause it breaks.
#[test]
fn a_foreign_interleaving_refuses_at_its_step() -> Result<(), LaneFailure> {
    let set = transfer_set()?;
    assert_eq!(
        encoded(&set, &Interleaving::declared(vec![0u8])),
        Err(EncodingRefusal::StepsMismatch {
            declared: 1usize,
            steps: 2usize
        })
    );
    assert_eq!(
        encoded(&set, &Interleaving::declared(vec![9u8, 0u8])),
        Err(EncodingRefusal::ChoiceOutsideStrands {
            at: 0usize,
            choice: 9u8
        })
    );
    assert_eq!(
        encoded(&set, &Interleaving::declared(vec![0u8, 0u8])),
        Err(EncodingRefusal::StrandExhausted {
            at: 1usize,
            choice: 0u8
        })
    );
    Ok(())
}
