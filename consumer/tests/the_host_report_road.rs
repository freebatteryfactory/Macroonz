//! An outside host's report road: typed host observations become evidence only after the public runner joins them to one table, selection, binding, and invocation.

use harness::clock::{HarnessClock, MeasurementReading, RecordedDuration};
use harness::descriptor::{
    AuthoredTableName, AuthoredTableRefusal, Binding, CheckRef, ClaimRef, Classification,
    ExecutableAttachment, ExecutionSuite, Origin, PopulationRef, Provenance, RevisionBinding, Role,
    Row, SubjectRoute, Tag, TrialTableRefusal,
};
use harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use harness::report::{
    ByteBudget, CaseBudget, EmptySelectionReason, HostTrialRecord, InfrastructureFault,
    InvocationProfile, NotSelectedReason, OutcomeClass, RunAttempt, SelectionOutcome, SkipReason,
    TargetBinding, TargetTriple, TimeBudget, ToolchainIdentity, TrialConclusion, TrialId,
    TrialSite,
};
use harness::runner::{
    Invocation, ReportRecordingRefusal, SeatRefusal, Selection, SelectionPlan, TrialBinding,
    TrialTable,
};
use std::collections::BTreeSet;

const HOST: &str = "consumer-host";
const REVISION_TAG: DomainTag = DomainTag::declared(
    "consumer-host-revision",
    IdentityProfileVersion::declared(1),
);

enum HostRoadFailure {
    Table(TrialTableRefusal),
    TableAssembly(AuthoredTableRefusal),
    Recording(ReportRecordingRefusal),
    MissingFixtureTrial,
    MissingOtherFixtureTrial(TrialId),
    MissingCensusSeat(TrialId),
    MissingSelectedReport(TrialId),
}

impl core::fmt::Debug for HostRoadFailure {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Table(refusal) => formatter.debug_tuple("Table").field(refusal).finish(),
            Self::TableAssembly(refusal) => formatter
                .debug_tuple("TableAssembly")
                .field(refusal)
                .finish(),
            Self::Recording(refusal) => formatter.debug_tuple("Recording").field(refusal).finish(),
            Self::MissingFixtureTrial => formatter.write_str("MissingFixtureTrial"),
            Self::MissingOtherFixtureTrial(trial) => formatter
                .debug_tuple("MissingOtherFixtureTrial")
                .field(trial)
                .finish(),
            Self::MissingCensusSeat(trial) => formatter
                .debug_tuple("MissingCensusSeat")
                .field(trial)
                .finish(),
            Self::MissingSelectedReport(trial) => formatter
                .debug_tuple("MissingSelectedReport")
                .field(trial)
                .finish(),
        }
    }
}

impl From<TrialTableRefusal> for HostRoadFailure {
    fn from(refusal: TrialTableRefusal) -> Self {
        Self::Table(refusal)
    }
}

impl From<ReportRecordingRefusal> for HostRoadFailure {
    fn from(refusal: ReportRecordingRefusal) -> Self {
        Self::Recording(refusal)
    }
}

impl From<AuthoredTableRefusal> for HostRoadFailure {
    fn from(refusal: AuthoredTableRefusal) -> Self {
        Self::TableAssembly(refusal)
    }
}

fn concludes(_invocation: &Invocation) -> TrialConclusion {
    TrialConclusion::Passed
}

fn binding(stem: &'static str) -> Result<TrialBinding, TrialTableRefusal> {
    let subject = SubjectRoute::named(HOST, "subject")?;
    let check = CheckRef::named(HOST, stem)?;
    let row = Row::declared(
        ClaimRef::named(HOST, stem)?,
        ExecutionSuite::named(HOST, "host-recording")?,
        Classification::authored(
            vec![Role::named(HOST, "host")?],
            vec![Tag::named(HOST, "outside")?],
        )?,
        subject,
        check,
        PopulationRef::named(HOST, stem)?,
        Origin::HandWritten,
    )?;
    let revision =
        RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, stem.as_bytes()));
    Binding::bound(
        row,
        ExecutableAttachment::attached(subject, check, revision, revision, concludes),
        Provenance::Unproduced,
    )
    .map_err(TrialTableRefusal::from)
}

fn world() -> Result<TrialTable, HostRoadFailure> {
    let name = AuthoredTableName::named(HOST, "world").map_err(TrialTableRefusal::from)?;
    Ok(TrialTable::authored(
        name,
        Provenance::Unproduced,
        vec![binding("first")?, binding("second")?],
    )?)
}

fn invocation() -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1u32),
            ByteBudget::declared(64u64),
            TimeBudget::declared(1_000_000_000u64),
        ),
        TargetBinding::bound(
            TargetTriple::declared("x86_64-pc-windows-msvc"),
            ToolchainIdentity::declared("1.98.0"),
        ),
        TrialSite::located(module_path!(), file!(), line!(), "the-host-report-road"),
        HarnessClock::unavailable(),
    )
}

fn trials(world: &TrialTable) -> Vec<TrialId> {
    world
        .bindings()
        .iter()
        .map(|binding| harness::runner::trial_identity(binding.row()))
        .collect()
}

fn two_trials(world: &TrialTable) -> Result<(TrialId, TrialId), HostRoadFailure> {
    let mut trials = trials(world).into_iter();
    let first = trials.next().ok_or(HostRoadFailure::MissingFixtureTrial)?;
    let second = trials
        .next()
        .ok_or(HostRoadFailure::MissingOtherFixtureTrial(first))?;
    Ok((first, second))
}

fn first_binding(world: &TrialTable) -> Result<&TrialBinding, HostRoadFailure> {
    world
        .bindings()
        .first()
        .ok_or(HostRoadFailure::MissingFixtureTrial)
}

fn selecting(trial: TrialId) -> SelectionPlan {
    SelectionPlan::of(Selection::ByTrialIds(BTreeSet::from([trial])))
}

fn host_record(trial: TrialId, attempt: RunAttempt) -> HostTrialRecord {
    HostTrialRecord::recorded(
        trial,
        attempt,
        MeasurementReading::Observed(RecordedDuration::recorded(17u64)),
    )
}

/// A lawful host record reaches a complete report without supplying any report-owned fact.
#[test]
fn a_host_record_is_admitted_over_the_complete_world() -> Result<(), HostRoadFailure> {
    let world = world()?;
    let invocation = invocation();
    let (selected_trial, unselected_trial) = two_trials(&world)?;
    let expected_trials = trials(&world);
    let report = harness::runner::record_all(
        &world.view(),
        &selecting(selected_trial),
        &invocation,
        vec![host_record(
            selected_trial,
            RunAttempt::SkippedWithReason(SkipReason::PrerequisiteAbsent),
        )],
    )?;

    assert_eq!(
        report
            .census()
            .iter()
            .map(harness::report::TrialAccounting::trial)
            .collect::<Vec<_>>(),
        expected_trials
    );
    assert_eq!(report.denominator(), world.view().bindings().count());
    assert_eq!(report.posture(), world.view().posture());
    assert_eq!(report.invocation(), invocation.profile());
    assert_eq!(report.selection(), SelectionOutcome::Satisfied);
    let selected_accounting = report
        .census()
        .iter()
        .find(|accounting| accounting.trial() == selected_trial)
        .ok_or(HostRoadFailure::MissingCensusSeat(selected_trial))?;
    let unselected_accounting = report
        .census()
        .iter()
        .find(|accounting| accounting.trial() == unselected_trial)
        .ok_or(HostRoadFailure::MissingCensusSeat(unselected_trial))?;
    let selected = selected_accounting
        .disposition()
        .report()
        .ok_or(HostRoadFailure::MissingSelectedReport(selected_trial))?;
    assert_eq!(selected_accounting.trial(), selected.trial());
    assert_eq!(
        selected.measurement(),
        MeasurementReading::Observed(RecordedDuration::recorded(17u64))
    );
    assert_eq!(
        unselected_accounting.disposition().outcome(),
        OutcomeClass::NotSelected(NotSelectedReason::OutsideSelection)
    );
    assert!(matches!(
        harness::runner::seat_verdict(&report),
        Err(SeatRefusal::RunFailed { .. })
    ));

    let empty = harness::runner::record_all(
        &world.view(),
        &SelectionPlan::allowing_empty(
            Selection::ByTrialIds(BTreeSet::new()),
            EmptySelectionReason::AskingWhatTheWorldHolds,
        ),
        &invocation,
        Vec::new(),
    )?;
    assert_eq!(empty.denominator(), world.view().bindings().count());
    assert_eq!(
        empty.selection(),
        SelectionOutcome::EmptyAsStated(EmptySelectionReason::AskingWhatTheWorldHolds)
    );
    assert!(
        empty
            .census()
            .iter()
            .all(|entry| entry.disposition().report().is_none())
    );
    Ok(())
}

/// Two host inputs cannot occupy one selected trial seat.
#[test]
fn a_duplicate_host_record_refuses() -> Result<(), HostRoadFailure> {
    let world = world()?;
    let invocation = invocation();
    let (trial, _) = two_trials(&world)?;
    let record = host_record(trial, RunAttempt::Executed(TrialConclusion::Passed));
    assert_eq!(
        harness::runner::record_all(
            &world.view(),
            &selecting(trial),
            &invocation,
            vec![record.clone(), record],
        ),
        Err(ReportRecordingRefusal::DuplicateHostRecord(trial))
    );
    Ok(())
}

/// Every selected trial must receive one host observation.
#[test]
fn a_missing_selected_host_record_refuses() -> Result<(), HostRoadFailure> {
    let world = world()?;
    let invocation = invocation();
    let (trial, _) = two_trials(&world)?;
    assert_eq!(
        harness::runner::record_all(&world.view(), &selecting(trial), &invocation, Vec::new()),
        Err(ReportRecordingRefusal::MissingSelectedRecord(trial))
    );
    Ok(())
}

/// A record for another trial cannot fill the selected seat or enter from outside the table.
#[test]
fn another_trials_record_cannot_fill_the_selected_seat() -> Result<(), HostRoadFailure> {
    let world = world()?;
    let invocation = invocation();
    let (selected, other) = two_trials(&world)?;
    let other_record = host_record(
        other,
        RunAttempt::InfrastructureFailed(InfrastructureFault::CaptureFailed),
    );

    assert_eq!(
        harness::runner::record_one(first_binding(&world)?, &invocation, other_record.clone()),
        Err(ReportRecordingRefusal::TrialMismatch {
            expected: selected,
            recorded: other,
        })
    );
    assert_eq!(
        harness::runner::record_all(
            &world.view(),
            &selecting(selected),
            &invocation,
            vec![other_record],
        ),
        Err(ReportRecordingRefusal::RecordForUnselectedTrial(other))
    );

    let outsider = binding("outside")?;
    let outsider_trial = harness::runner::trial_identity(outsider.row());
    assert_eq!(
        harness::runner::record_all(
            &world.view(),
            &selecting(selected),
            &invocation,
            vec![host_record(
                outsider_trial,
                RunAttempt::TimedOut(TimeBudget::declared(1u64)),
            )],
        ),
        Err(ReportRecordingRefusal::TrialOutsideTable(outsider_trial))
    );
    Ok(())
}
