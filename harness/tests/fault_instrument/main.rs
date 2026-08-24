//! An outside consumer schedules typed port adapters, observes their postconditions through a temporal contract, and records the earned conclusion through the ordinary runner.

use macroonz_harness::clock::HarnessClock;
use macroonz_harness::descriptor::{
    AuthoredTableName, Binding, CheckRef, ClaimRef, Classification, ExecutableAttachment,
    ExecutionSuite, NameRefusal, NamespacedName, Origin, PopulationRef, Provenance,
    RevisionBinding, Role, Row, SubjectRoute, Tag, TrialTableRefusal,
};
use macroonz_harness::fault::{
    FaultAdapter, FaultCampaign, FaultCampaignRefusal, FaultInjectionRefusal, FaultSchedule,
    FaultSelectionRefusal, InjectedCommand, ScheduledFault, SequencePosition, inject,
};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::properties::{
    ContractRefusal, Holding, TemporalClaim, TemporalDemand, TransitionContract, concluded,
    holds_over_history,
};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, InvocationProfile, RunAttempt,
    SelectionOutcome, TargetBinding, TargetTriple, TimeBudget, ToolchainIdentity, TrialConclusion,
    TrialSite,
};
use macroonz_harness::runner::{Invocation, Selection, SelectionPlan, TrialTable, run_all};
use std::{cell::Cell, rc::Rc};

const CONSUMER: &str = "harness.fault.consumer";
const TRACE_CAUSE: FindingCause = FindingCause::named(CONSUMER, "write-trace");
const FIXTURE_CAUSE: FindingCause = FindingCause::named(CONSUMER, "fixture-refused");
const REVISION_TAG: DomainTag = DomainTag::declared(
    "fault-consumer-revision",
    IdentityProfileVersion::declared(1),
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WriteFault {
    Capacity,
    Poison,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WritePostcondition {
    StateUnchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WriteRefusal {
    Capacity,
    Poison,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WriteCommand(u8);

#[derive(Debug, PartialEq, Eq)]
struct CloneCounter {
    calls: Rc<Cell<u32>>,
}

impl CloneCounter {
    fn tracking(calls: Rc<Cell<u32>>) -> Self {
        Self { calls }
    }
}

impl Clone for CloneCounter {
    fn clone(&self) -> Self {
        self.calls.set(self.calls.get().saturating_add(1u32));
        Self::tracking(Rc::clone(&self.calls))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WriteState {
    bytes: Vec<u8>,
    commands: u32,
    refusals: u32,
    validity: StateValidity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StateValidity {
    Valid,
    Violated,
}

trait WritePort {
    fn write(&self, bytes: &mut Vec<u8>, command: WriteCommand) -> Result<(), WriteRefusal>;
}

impl WritePort for WriteFault {
    fn write(&self, _bytes: &mut Vec<u8>, _command: WriteCommand) -> Result<(), WriteRefusal> {
        match self {
            Self::Capacity => Err(WriteRefusal::Capacity),
            Self::Poison => Err(WriteRefusal::Poison),
        }
    }
}

enum FaultRoadFailure {
    Name(NameRefusal),
    Campaign(FaultCampaignRefusal),
    Selection(FaultSelectionRefusal),
    Injection(FaultInjectionRefusal),
    Contract(ContractRefusal),
    Table(TrialTableRefusal),
    MissingReport,
}

impl core::fmt::Debug for FaultRoadFailure {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Name(refusal) => formatter.debug_tuple("Name").field(refusal).finish(),
            Self::Campaign(refusal) => formatter.debug_tuple("Campaign").field(refusal).finish(),
            Self::Selection(refusal) => formatter.debug_tuple("Selection").field(refusal).finish(),
            Self::Injection(refusal) => formatter.debug_tuple("Injection").field(refusal).finish(),
            Self::Contract(refusal) => formatter.debug_tuple("Contract").field(refusal).finish(),
            Self::Table(refusal) => formatter.debug_tuple("Table").field(refusal).finish(),
            Self::MissingReport => formatter.write_str("MissingReport"),
        }
    }
}

impl From<NameRefusal> for FaultRoadFailure {
    fn from(refusal: NameRefusal) -> Self {
        Self::Name(refusal)
    }
}

impl From<FaultCampaignRefusal> for FaultRoadFailure {
    fn from(refusal: FaultCampaignRefusal) -> Self {
        Self::Campaign(refusal)
    }
}

impl From<FaultSelectionRefusal> for FaultRoadFailure {
    fn from(refusal: FaultSelectionRefusal) -> Self {
        Self::Selection(refusal)
    }
}

impl From<FaultInjectionRefusal> for FaultRoadFailure {
    fn from(refusal: FaultInjectionRefusal) -> Self {
        Self::Injection(refusal)
    }
}

impl From<ContractRefusal> for FaultRoadFailure {
    fn from(refusal: ContractRefusal) -> Self {
        Self::Contract(refusal)
    }
}

impl From<TrialTableRefusal> for FaultRoadFailure {
    fn from(refusal: TrialTableRefusal) -> Self {
        Self::Table(refusal)
    }
}

fn schedule_name(stem: &'static str) -> Result<NamespacedName, NameRefusal> {
    NamespacedName::named(CONSUMER, stem)
}

fn campaign() -> Result<FaultCampaign<WriteFault, WritePostcondition>, FaultRoadFailure> {
    let control = FaultSchedule::declared(schedule_name("lawful-control")?, Vec::new());
    let hostile = FaultSchedule::declared(
        schedule_name("capacity-at-second-write")?,
        vec![ScheduledFault::at(
            SequencePosition::at(1u32),
            FaultAdapter::declared(WriteFault::Capacity, WritePostcondition::StateUnchanged),
        )],
    );
    Ok(FaultCampaign::declared(vec![control, hostile])?)
}

fn opening_state() -> WriteState {
    WriteState {
        bytes: Vec::new(),
        commands: 0u32,
        refusals: 0u32,
        validity: StateValidity::Valid,
    }
}

fn apply_command(
    state: &WriteState,
    injected: &InjectedCommand<WriteCommand, WriteFault, WritePostcondition>,
) -> WriteState {
    let mut next = state.clone();
    next.commands = next.commands.saturating_add(1u32);
    if injected.faults().is_empty() {
        next.bytes.push(injected.command().0);
        return next;
    }
    for adapter in injected.faults() {
        let before = next.bytes.clone();
        match adapter
            .behavior()
            .write(&mut next.bytes, *injected.command())
        {
            Err(WriteRefusal::Capacity | WriteRefusal::Poison) => {
                next.refusals = next.refusals.saturating_add(1u32);
                next.validity = match (next.validity, adapter.postcondition(), next.bytes == before)
                {
                    (StateValidity::Valid, WritePostcondition::StateUnchanged, true) => {
                        StateValidity::Valid
                    }
                    _ => StateValidity::Violated,
                };
            }
            Ok(()) => next.validity = StateValidity::Violated,
        }
    }
    next
}

fn lawful_trace(state: &WriteState) -> Holding {
    let expected = match state.commands {
        0 => Some(&[][..]),
        1 => Some(&[1u8][..]),
        2 => Some(&[1u8, 2u8][..]),
        _ => None,
    };
    if state.validity == StateValidity::Valid
        && state.refusals == 0u32
        && expected.is_some_and(|bytes| state.bytes == bytes)
    {
        Holding::Holds
    } else {
        Holding::Fails
    }
}

fn hostile_trace(state: &WriteState) -> Holding {
    let expected = match state.commands {
        0 => Some((&[][..], 0u32)),
        1 => Some((&[1u8][..], 0u32)),
        2 => Some((&[1u8][..], 1u32)),
        _ => None,
    };
    if state.validity == StateValidity::Valid
        && expected
            .is_some_and(|(bytes, refusals)| state.bytes == bytes && state.refusals == refusals)
    {
        Holding::Holds
    } else {
        Holding::Fails
    }
}

fn contract(
    predicate: fn(&WriteState) -> Holding,
) -> Result<
    TransitionContract<WriteState, InjectedCommand<WriteCommand, WriteFault, WritePostcondition>>,
    ContractRefusal,
> {
    TransitionContract::declared(
        opening_state,
        apply_command,
        vec![TemporalClaim::declared(
            TRACE_CAUSE,
            TemporalDemand::Always(predicate),
        )],
    )
}

fn commands() -> Vec<WriteCommand> {
    vec![WriteCommand(1u8), WriteCommand(2u8)]
}

fn exercised_campaign() -> Result<TrialConclusion, FaultRoadFailure> {
    let campaign = campaign()?;
    let lawful = campaign.select(schedule_name("lawful-control")?)?;
    let hostile = campaign.select(schedule_name("capacity-at-second-write")?)?;
    let lawful = inject(&lawful, commands())?;
    let hostile = inject(&hostile, commands())?;
    let lawful_conclusion = holds_over_history(&contract(lawful_trace)?, lawful.commands());
    if lawful_conclusion != TrialConclusion::Passed {
        return Ok(lawful_conclusion);
    }
    Ok(holds_over_history(
        &contract(hostile_trace)?,
        hostile.commands(),
    ))
}

fn fixture_refusal() -> TrialConclusion {
    concluded(Holding::Fails, FailureClass::RefusedByCheck, FIXTURE_CAUSE)
}

fn fault_trial(_invocation: &Invocation) -> TrialConclusion {
    exercised_campaign().unwrap_or_else(|_refusal| fixture_refusal())
}

fn world() -> Result<TrialTable, TrialTableRefusal> {
    let subject = SubjectRoute::named(CONSUMER, "bounded-write-port")?;
    let check = CheckRef::named(CONSUMER, "selected-fault-schedule")?;
    let row = Row::declared(
        ClaimRef::named(CONSUMER, "refusal-preserves-state")?,
        ExecutionSuite::named(CONSUMER, "fault-campaign")?,
        Classification::authored(
            vec![Role::named(CONSUMER, "fault")?],
            vec![Tag::named(CONSUMER, "neutral")?],
        )?,
        subject,
        check,
        PopulationRef::named(CONSUMER, "two-writes")?,
        Origin::HandWritten,
    )?;
    let revision = RevisionBinding::declared(ContentAddress::derived(
        REVISION_TAG,
        b"bounded-write-port/v1",
    ));
    let binding = Binding::bound(
        row,
        ExecutableAttachment::attached(subject, check, revision, revision, fault_trial),
        Provenance::Unproduced,
    )?;
    TrialTable::authored(
        AuthoredTableName::named(CONSUMER, "fault-world")?,
        Provenance::Unproduced,
        vec![binding],
    )
    .map_err(TrialTableRefusal::TableNotAuthored)
}

fn invocation() -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1u32),
            ByteBudget::declared(2u64),
            TimeBudget::declared(1u64),
        ),
        TargetBinding::bound(
            TargetTriple::declared("neutral-test-target"),
            ToolchainIdentity::declared("1.98.0"),
        ),
        TrialSite::located(module_path!(), file!(), line!(), "fault-instrument"),
        HarnessClock::unavailable(),
    )
}

fn invalid_positions_refuse_before_adapters_are_cloned() -> Result<(), FaultRoadFailure> {
    let clone_calls = Rc::new(Cell::new(0u32));
    let validation_order = FaultCampaign::declared(vec![FaultSchedule::declared(
        schedule_name("validation-before-cloning")?,
        vec![
            ScheduledFault::at(
                SequencePosition::at(0u32),
                FaultAdapter::declared(CloneCounter::tracking(Rc::clone(&clone_calls)), ()),
            ),
            ScheduledFault::at(
                SequencePosition::at(2u32),
                FaultAdapter::declared(CloneCounter::tracking(Rc::clone(&clone_calls)), ()),
            ),
        ],
    )])?;
    let validation_order = validation_order.select(schedule_name("validation-before-cloning")?)?;
    assert_eq!(
        inject(&validation_order, commands()),
        Err(FaultInjectionRefusal::PositionOutsideSequence {
            position: SequencePosition::at(2u32),
            commands: commands().len(),
        })
    );
    assert_eq!(clone_calls.get(), 0u32);
    Ok(())
}

#[test]
fn selected_faults_reach_temporal_evidence_and_the_ordinary_report() -> Result<(), FaultRoadFailure>
{
    let world = world()?;
    let report = run_all(
        &world.view(),
        &SelectionPlan::of(Selection::All),
        &invocation(),
    );
    assert_eq!(report.selection(), SelectionOutcome::Satisfied);
    assert_eq!(report.denominator(), world.view().bindings().count());
    let trial = report
        .census()
        .first()
        .and_then(|accounting| accounting.disposition().report())
        .ok_or(FaultRoadFailure::MissingReport)?;
    assert_eq!(
        trial.attempt(),
        &RunAttempt::Executed(TrialConclusion::Passed)
    );
    Ok(())
}

#[test]
fn campaign_and_injection_refuse_vacuity_ambiguity_and_dropped_positions()
-> Result<(), FaultRoadFailure> {
    assert_eq!(
        FaultCampaign::<WriteFault, WritePostcondition>::declared(Vec::new()),
        Err(FaultCampaignRefusal::NoSchedule)
    );
    let empty_name = schedule_name("empty-control")?;
    assert_eq!(
        FaultCampaign::declared(vec![
            FaultSchedule::<WriteFault, WritePostcondition>::declared(empty_name, Vec::new(),)
        ]),
        Err(FaultCampaignRefusal::NoFaultDeclared)
    );
    let adapter = FaultAdapter::declared(WriteFault::Capacity, WritePostcondition::StateUnchanged);
    let first_same = FaultSchedule::declared(
        schedule_name("same")?,
        vec![ScheduledFault::at(
            SequencePosition::at(0u32),
            adapter.clone(),
        )],
    );
    let second_same = FaultSchedule::declared(
        schedule_name("same")?,
        vec![ScheduledFault::at(
            SequencePosition::at(1u32),
            adapter.clone(),
        )],
    );
    assert_eq!(
        FaultCampaign::declared(vec![first_same, second_same]),
        Err(FaultCampaignRefusal::DuplicateSchedule(schedule_name(
            "same"
        )?))
    );

    let campaign = campaign()?;
    assert_eq!(
        campaign.select(schedule_name("absent")?),
        Err(FaultSelectionRefusal::ScheduleAbsent(schedule_name(
            "absent"
        )?))
    );
    let outside = FaultCampaign::declared(vec![FaultSchedule::declared(
        schedule_name("outside")?,
        vec![ScheduledFault::at(SequencePosition::at(2u32), adapter)],
    )])?;
    let outside = outside.select(schedule_name("outside")?)?;
    assert_eq!(
        inject(&outside, commands()),
        Err(FaultInjectionRefusal::PositionOutsideSequence {
            position: SequencePosition::at(2u32),
            commands: commands().len(),
        })
    );

    invalid_positions_refuse_before_adapters_are_cloned()?;

    let stacked = FaultCampaign::declared(vec![FaultSchedule::declared(
        schedule_name("stacked")?,
        vec![
            ScheduledFault::at(
                SequencePosition::at(0u32),
                FaultAdapter::declared(WriteFault::Capacity, WritePostcondition::StateUnchanged),
            ),
            ScheduledFault::at(
                SequencePosition::at(0u32),
                FaultAdapter::declared(WriteFault::Poison, WritePostcondition::StateUnchanged),
            ),
        ],
    )])?;
    let selected = stacked.select(schedule_name("stacked")?)?;
    let injected = inject(&selected, vec![WriteCommand(1u8)])?;
    let first_command = injected
        .commands()
        .first()
        .ok_or(FaultRoadFailure::MissingReport)?;
    assert_eq!(
        first_command
            .faults()
            .iter()
            .map(|fault| *fault.behavior())
            .collect::<Vec<_>>(),
        vec![WriteFault::Capacity, WriteFault::Poison]
    );
    assert_eq!(injected.fault_count(), first_command.faults().len());
    Ok(())
}
