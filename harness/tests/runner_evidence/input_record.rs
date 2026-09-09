//! Host input coordinates are independently joined before the common report assembly.

use super::support::{binding, invocation, refused, world};
use arbitrary::Unstructured;
use macroonz_harness::clock::{ClockAttribution, ClockFailure, MeasurementReading};
use macroonz_harness::descriptor::{DerivedRevision, NamespacedName, RevisionBinding};
use macroonz_harness::input::{BoundInput, InputBinding, InputLimits, InputProfile, pack};
use macroonz_harness::report::{
    Baseline, ByteBudget, CacheEligibility, CaseBudget, EmptySelectionReason, ExecutionInput,
    HostTrialRecord, InfrastructureFailure, InfrastructureFault, InvocationProfile, ReplayPosture,
    ReportComparison, RunAttempt, SelectionOutcome, SkipReason, TimeBudget, TrialConclusion,
    TrialId, compare,
};
use macroonz_harness::runner::{
    Invocation, ReportRecordingRefusal, Selection, SelectionPlan, lens_verdict, record_input_all,
    record_input_one, run_all, trial_identity,
};
use std::collections::BTreeSet;

fn revision() -> RevisionBinding {
    RevisionBinding::derived(DerivedRevision::from_material(b"host-input-byte-decoder"))
}

fn decode(source: &mut Unstructured<'_>) -> arbitrary::Result<u8> {
    source.arbitrary()
}

fn input(value: u8, version: u32, revision: RevisionBinding) -> Result<BoundInput<u8>, ()> {
    let profile = InputProfile::declared(
        NamespacedName::named("host-input", "byte").map_err(|_| ())?,
        version,
        DerivedRevision::from_material(b"one byte").revision(),
    );
    InputBinding::declared(profile, revision, decode)
        .decode(pack(profile, &[value], InputLimits::declared(256, 1)).map_err(|_| ())?)
        .map_err(|_| ())
}

fn below_five(invocation: &Invocation<BoundInput<u8>>) -> TrialConclusion {
    if *invocation.input().value() < 5 {
        TrialConclusion::Passed
    } else {
        refused()
    }
}

fn record(
    trial: TrialId,
    input: &BoundInput<u8>,
    attempt: RunAttempt,
) -> HostTrialRecord<ExecutionInput> {
    HostTrialRecord::recorded(trial, attempt, MeasurementReading::Unavailable).with_input(input)
}

#[test]
fn distinct_admitted_witnesses_keep_their_actual_host_conclusions() -> Result<(), ()> {
    let selected = binding("below-five", below_five).map_err(|_| ())?;
    let trial = trial_identity(selected.row());
    let table = world(vec![
        selected,
        binding("unselected", below_five).map_err(|_| ())?,
    ])
    .map_err(|_| ())?;
    let selection = SelectionPlan::of(Selection::ByTrialIds(BTreeSet::from([trial])));
    let mut keys = BTreeSet::new();
    for (value, expected) in [(2u8, TrialConclusion::Passed), (7u8, refused())] {
        let invocation = invocation().with_input(input(value, 1, revision())?);
        let observed = below_five(&invocation);
        assert_eq!(observed, expected);
        let report = record_input_all(
            &table.view(),
            &selection,
            &invocation,
            vec![record(
                trial,
                invocation.input(),
                RunAttempt::Executed(observed),
            )],
        )
        .map_err(|_| ())?;
        assert_eq!(report.denominator(), 2);
        let mut census = report.census().iter();
        let selected_report = census.next().ok_or(())?.disposition().report().ok_or(())?;
        assert_eq!(selected_report.attempt(), &RunAttempt::Executed(expected));
        assert_eq!(
            selected_report.standing().key().input(),
            invocation.input_standing()
        );
        assert!(keys.insert(selected_report.standing().key().address()));
        assert!(census.next().ok_or(())?.disposition().report().is_none());
        assert_eq!(report, run_all(&table.view(), &selection, &invocation));
    }
    Ok(())
}

#[test]
fn input_movement_refuses_instead_of_relabeling_a_host_observation() -> Result<(), ()> {
    let binding = binding("below-five", below_five).map_err(|_| ())?;
    let trial = trial_identity(binding.row());
    let invocation = invocation().with_input(input(7, 1, revision())?);
    for foreign in [
        input(2, 1, revision())?,
        input(7, 2, revision())?,
        input(
            7,
            1,
            RevisionBinding::derived(DerivedRevision::from_material(b"other decoder")),
        )?,
        input(7, 1, RevisionBinding::declared(revision().revision()))?,
        input(7, 1, RevisionBinding::untracked(revision().revision()))?,
    ] {
        let offered = record(
            trial,
            &foreign,
            RunAttempt::Executed(TrialConclusion::Passed),
        );
        assert_eq!(
            record_input_one(&binding, &invocation, offered),
            Err(ReportRecordingRefusal::InputMismatch(trial))
        );
    }
    let independently_decoded = input(7, 1, revision())?;
    assert!(
        record_input_one(
            &binding,
            &invocation,
            record(
                trial,
                &independently_decoded,
                RunAttempt::Executed(refused())
            ),
        )
        .is_ok()
    );
    Ok(())
}

#[test]
fn typed_host_roster_refusals_keep_the_existing_precedence() -> Result<(), ()> {
    let first = binding("first", below_five).map_err(|_| ())?;
    let second = binding("second", below_five).map_err(|_| ())?;
    let outside = binding("outside", below_five).map_err(|_| ())?;
    let first_id = trial_identity(first.row());
    let second_id = trial_identity(second.row());
    let outside_id = trial_identity(outside.row());
    let invocation = invocation().with_input(input(7, 1, revision())?);
    let foreign = input(2, 1, revision())?;
    let wrong_input = record(first_id, &foreign, RunAttempt::TimedOut);
    assert_eq!(
        record_input_one(&second, &invocation, wrong_input.clone()),
        Err(ReportRecordingRefusal::TrialMismatch {
            expected: second_id,
            recorded: first_id
        })
    );
    let table = world(vec![first, second]).map_err(|_| ())?;
    let all = SelectionPlan::of(Selection::All);
    let only_first = SelectionPlan::of(Selection::ByTrialIds(BTreeSet::from([first_id])));
    for (selection, records, refusal) in [
        (
            &all,
            vec![wrong_input.clone(), wrong_input.clone()],
            ReportRecordingRefusal::DuplicateHostRecord(first_id),
        ),
        (
            &all,
            vec![
                wrong_input.clone(),
                record(outside_id, &foreign, RunAttempt::TimedOut),
            ],
            ReportRecordingRefusal::TrialOutsideTable(outside_id),
        ),
        (
            &only_first,
            vec![
                wrong_input.clone(),
                record(second_id, &foreign, RunAttempt::TimedOut),
            ],
            ReportRecordingRefusal::RecordForUnselectedTrial(second_id),
        ),
        (
            &all,
            vec![record(second_id, &foreign, RunAttempt::TimedOut)],
            ReportRecordingRefusal::MissingSelectedRecord(first_id),
        ),
        (
            &all,
            vec![wrong_input],
            ReportRecordingRefusal::InputMismatch(first_id),
        ),
        (
            &all,
            vec![record(first_id, invocation.input(), RunAttempt::TimedOut)],
            ReportRecordingRefusal::MissingSelectedRecord(second_id),
        ),
    ] {
        assert_eq!(
            record_input_all(&table.view(), selection, &invocation, records),
            Err(refusal)
        );
    }
    Ok(())
}

#[test]
fn budget_refusal_accepts_only_an_unmeasured_budget_skip() -> Result<(), ()> {
    let binding = binding("below-five", below_five).map_err(|_| ())?;
    let trial = trial_identity(binding.row());
    let skip = RunAttempt::SkippedWithReason(SkipReason::BudgetExhausted);
    for (cases, bytes) in [(0u32, 1u64), (1u32, 0u64)] {
        let facts = invocation();
        let invocation = Invocation::declared(
            InvocationProfile::declared(
                CaseBudget::declared(cases),
                ByteBudget::declared(bytes),
                TimeBudget::declared(0),
            ),
            facts.target().clone(),
            facts.site(),
            facts.clock(),
        )
        .with_input(input(2, 1, revision())?);
        assert_eq!(
            invocation.input_budget_refusal(),
            Some(SkipReason::BudgetExhausted)
        );
        let skipped = record_input_one(
            &binding,
            &invocation,
            record(trial, invocation.input(), skip.clone()),
        )
        .map_err(|_| ())?;
        assert_eq!(skipped.attempt(), &skip);
        assert!(lens_verdict(&skipped).is_err());
        for (attempt, measurement, attribution) in [
            (
                RunAttempt::Executed(TrialConclusion::Passed),
                MeasurementReading::Unavailable,
                ClockAttribution::Unspecified,
            ),
            (
                RunAttempt::TimedOut,
                MeasurementReading::Unavailable,
                ClockAttribution::Unspecified,
            ),
            (
                RunAttempt::SkippedWithReason(SkipReason::PrerequisiteAbsent),
                MeasurementReading::Unavailable,
                ClockAttribution::Unspecified,
            ),
            (
                RunAttempt::InfrastructureFailed(InfrastructureFailure::recorded(
                    InfrastructureFault::CaptureFailed,
                    None,
                )),
                MeasurementReading::Unavailable,
                ClockAttribution::Unspecified,
            ),
            (
                skip.clone(),
                MeasurementReading::Failed(ClockFailure::OpeningRefused),
                ClockAttribution::Unspecified,
            ),
            (
                skip.clone(),
                MeasurementReading::Unavailable,
                ClockAttribution::Synthetic,
            ),
        ] {
            let offered = HostTrialRecord::recorded_with_attribution(
                trial,
                attempt,
                measurement,
                attribution,
            )
            .with_input(invocation.input());
            assert_eq!(
                record_input_one(&binding, &invocation, offered),
                Err(ReportRecordingRefusal::InputBudgetMismatch(trial))
            );
        }
    }
    Ok(())
}

#[test]
fn admitted_host_outcomes_and_measurement_keep_the_decoder_ceiling() -> Result<(), ()> {
    let binding = binding("below-five", below_five).map_err(|_| ())?;
    let trial = trial_identity(binding.row());
    for (revision, replay, cache) in [
        (
            revision(),
            ReplayPosture::ExactDerived,
            CacheEligibility::Eligible,
        ),
        (
            RevisionBinding::declared(revision().revision()),
            ReplayPosture::DeclaredByAuthor,
            CacheEligibility::NeverEligible,
        ),
        (
            RevisionBinding::untracked(revision().revision()),
            ReplayPosture::UnavailableBecauseUntracked,
            CacheEligibility::NeverEligible,
        ),
    ] {
        let invocation = invocation().with_input(input(7, 1, revision)?);
        assert_eq!(invocation.input_budget_refusal(), None);
        for attempt in [
            RunAttempt::Executed(refused()),
            RunAttempt::TimedOut,
            RunAttempt::SkippedWithReason(SkipReason::TargetUnsupported),
            RunAttempt::InfrastructureFailed(InfrastructureFailure::recorded(
                InfrastructureFault::CaptureFailed,
                None,
            )),
        ] {
            let measurement = MeasurementReading::Failed(ClockFailure::ClosingRefused);
            let offered = HostTrialRecord::recorded_with_attribution(
                trial,
                attempt.clone(),
                measurement,
                ClockAttribution::Monotonic,
            )
            .with_input(invocation.input());
            let report = record_input_one(&binding, &invocation, offered).map_err(|_| ())?;
            assert_eq!(report.attempt(), &attempt);
            assert_eq!(report.measurement(), measurement);
            assert_eq!(report.clock_attribution(), ClockAttribution::Monotonic);
            assert_eq!(report.standing().replay(), replay);
            assert_eq!(report.standing().cache_eligibility(), cache);
            assert!(lens_verdict(&report).is_err());
        }
    }
    Ok(())
}

#[test]
fn empty_host_selection_still_retains_input_movement_and_the_full_census() -> Result<(), ()> {
    let table = world(vec![binding("below-five", below_five).map_err(|_| ())?]).map_err(|_| ())?;
    let none = SelectionPlan::allowing_empty(
        Selection::ByTrialIds(BTreeSet::new()),
        EmptySelectionReason::AskingWhatTheWorldHolds,
    );
    let before_invocation = invocation().with_input(input(2, 1, revision())?);
    let before =
        record_input_all(&table.view(), &none, &before_invocation, Vec::new()).map_err(|_| ())?;
    for changed in [
        input(7, 1, revision())?,
        input(2, 2, revision())?,
        input(2, 1, RevisionBinding::untracked(revision().revision()))?,
    ] {
        let after_invocation = invocation().with_input(changed);
        let after = record_input_all(&table.view(), &none, &after_invocation, Vec::new())
            .map_err(|_| ())?;
        assert_eq!(after.denominator(), 1);
        assert_eq!(after.input(), after_invocation.input_standing());
        assert_eq!(
            after.selection(),
            SelectionOutcome::EmptyAsStated(EmptySelectionReason::AskingWhatTheWorldHolds)
        );
        let ReportComparison::Compared(diff) = compare(Baseline::Previous(&before), &after) else {
            return Err(());
        };
        assert!(diff.execution().input().is_some());
        assert!(diff.execution().flips().is_empty());
    }
    Ok(())
}
