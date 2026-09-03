//! Benchmark choreography shared by the two independent recipe work families.

use macroonz_harness::bench::{
    BenchAttachment, BenchBinding, BenchCall, BenchMeasurement, BenchOutcome, BenchReferences,
    BenchReport, BenchRow, BenchStage, BenchTable, BenchTableName, ComplexityClaimRef,
    ContentionPosture, DeclaredBudgets, InputSizeAxis, PlantedWorseRef, PreflightRef,
    PreflightTrial, WorkFormula, WorkJudgeBinding, WorkObservationRef, WorkRecorder,
    WorkRecordingRefusal, WorkloadRef, bench_verdict,
};
use macroonz_harness::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, ExecutableAttachment, ExecutionSuite, Origin,
    PopulationRef, Provenance, RevisionBinding, Role, Row, SubjectRoute, Tag,
};
use macroonz_harness::identity::{ContentAddress, DomainTag};
use macroonz_harness::report::{FindingCause, TrialConclusion};
use macroonz_harness::runner::{Invocation, TrialBinding};

#[derive(Clone, Copy)]
pub(super) enum Control {
    Repeated,
    Identical,
}

pub(super) trait WorkFamily: Copy {
    const CHECK: &'static str;
    const EXECUTION_SUITE: &'static str;
    const POPULATION: &'static str;
    const PREFLIGHT_REFUSED: FindingCause;
    const REVISION_TAG: DomainTag;
    const TAG: &'static str;

    fn stem(self) -> &'static str;

    fn axes(self) -> &'static [u64];

    fn preflight_stem(self) -> &'static str;

    fn repeated_stem(self) -> &'static str;

    fn complexity_stem(self) -> &'static str;

    fn claim_stem(self) -> &'static str;

    fn counts(self, axis: u64) -> Vec<u64>;

    fn observation_names() -> &'static [&'static str];

    fn preflight() -> Result<(), String>;
}

pub(super) fn table<F: WorkFamily>(
    control: Control,
    repeated_name: &'static str,
    identical_name: &'static str,
    first: (F, BenchCall, BenchCall),
    remaining: &[(F, BenchCall, BenchCall)],
) -> Result<BenchTable, String> {
    let bindings = match control {
        Control::Repeated => core::iter::once(first)
            .chain(remaining.iter().copied())
            .map(|(family, measured, worse)| binding(family, measured, worse))
            .collect::<Result<Vec<_>, _>>()?,
        Control::Identical => vec![binding(first.0, first.1, first.1)?],
    };
    BenchTable::authored(
        BenchTableName::named(
            super::OWNER,
            match control {
                Control::Repeated => repeated_name,
                Control::Identical => identical_name,
            },
        )
        .map_err(super::debug)?,
        Provenance::Unproduced,
        bindings,
    )
    .map_err(super::debug)
}

pub(super) fn record<F: WorkFamily>(
    family: F,
    axis: u64,
    repetitions: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    let counts = family.counts(axis);
    for _ in 0..repetitions {
        for (name, count) in F::observation_names()
            .iter()
            .copied()
            .zip(counts.iter().copied())
        {
            let observation = WorkObservationRef::named(super::OWNER, name)
                .map_err(WorkRecordingRefusal::ObservationName)?;
            recorder.record(observation, count)?;
        }
    }
    Ok(())
}

pub(super) fn assert_repeated(
    report: &BenchReport,
    expected: usize,
    unexpected_stage: &'static str,
) -> Result<(), String> {
    bench_verdict(report).map_err(super::debug)?;
    assert_eq!(report.readings().len(), expected);
    for reading in report.readings() {
        let BenchOutcome::Qualified {
            measured,
            planted_worse,
            judgment,
            ..
        } = reading.outcome()
        else {
            return Err(format!(
                "{unexpected_stage}: {:?}",
                reading.outcome().stage()
            ));
        };
        assert!(judgment.qualifies());
        assert_eq!(measured.points().len(), planted_worse.points().len());
        for (measured_point, planted_point) in measured.points().iter().zip(planted_worse.points())
        {
            assert_eq!(measured_point.input_size(), planted_point.input_size());
            assert_eq!(measured_point.counts().len(), planted_point.counts().len());
            for (measured_count, planted_count) in
                measured_point.counts().iter().zip(planted_point.counts())
            {
                assert_eq!(measured_count.observation(), planted_count.observation());
                assert_eq!(
                    measured_count.count().checked_mul(2),
                    Some(planted_count.count())
                );
            }
        }
    }
    Ok(())
}

pub(super) fn assert_identical(
    report: &BenchReport,
    missing_reading: &'static str,
) -> Result<(), String> {
    let [reading] = report.readings() else {
        return Err(String::from(missing_reading));
    };
    assert_eq!(
        reading.outcome().stage(),
        BenchStage::PlantedWorseNotDistinguished
    );
    Ok(())
}

fn binding<F: WorkFamily>(
    family: F,
    measured: BenchCall,
    worse: BenchCall,
) -> Result<BenchBinding, String> {
    let stem = family.stem();
    let workload = WorkloadRef::named(super::OWNER, stem).map_err(super::debug)?;
    let preflight =
        PreflightRef::named(super::OWNER, family.preflight_stem()).map_err(super::debug)?;
    let planted =
        PlantedWorseRef::named(super::OWNER, family.repeated_stem()).map_err(super::debug)?;
    let complexity =
        ComplexityClaimRef::named(super::OWNER, family.complexity_stem()).map_err(super::debug)?;
    let row = BenchRow::declared(
        BenchReferences::declared(workload, preflight, planted, complexity),
        BenchMeasurement::declared(
            InputSizeAxis::declared(family.axes().to_vec()).map_err(super::debug)?,
            DeclaredBudgets::declared(1, 0, 2, 1).map_err(super::debug)?,
            ContentionPosture::NoDeclaredContention,
            Some(WorkFormula::encoded(super::FORMULA.to_vec()).map_err(super::debug)?),
        ),
    )
    .map_err(super::debug)?;
    let attachment = BenchAttachment::attached(
        workload,
        measured,
        planted,
        worse,
        WorkJudgeBinding::bound(complexity, super::judge),
        observations::<F>()?,
    )
    .map_err(super::debug)?;
    let preflight = PreflightTrial::bound(
        preflight,
        trial_binding(family, preflight_call::<F>)?,
        super::preflight_invocation(),
    );
    BenchBinding::bound(row, attachment, preflight).map_err(super::debug)
}

fn observations<F: WorkFamily>() -> Result<Vec<WorkObservationRef>, String> {
    F::observation_names()
        .iter()
        .copied()
        .map(|name| WorkObservationRef::named(super::OWNER, name).map_err(super::debug))
        .collect()
}

fn preflight_call<F: WorkFamily>(_: &Invocation) -> TrialConclusion {
    if F::preflight().is_ok() {
        TrialConclusion::Passed
    } else {
        TrialConclusion::Refused(macroonz_harness::report::TrialFinding::established(
            macroonz_harness::report::FailureClass::RefusedByCheck,
            F::PREFLIGHT_REFUSED,
            macroonz_harness::report::FindingLocation::at(file!(), line!()),
            None,
        ))
    }
}

fn trial_binding<F: WorkFamily>(
    family: F,
    call: fn(&Invocation) -> TrialConclusion,
) -> Result<TrialBinding, String> {
    let stem = family.stem();
    let subject = SubjectRoute::named(super::OWNER, stem).map_err(super::debug)?;
    let check = CheckRef::named(super::OWNER, F::CHECK).map_err(super::debug)?;
    let row = Row::declared(
        ClaimRef::named(super::OWNER, family.claim_stem()).map_err(super::debug)?,
        ExecutionSuite::named(super::OWNER, F::EXECUTION_SUITE).map_err(super::debug)?,
        Classification::authored(
            vec![Role::named(super::OWNER, "benchmark").map_err(super::debug)?],
            vec![Tag::named(super::OWNER, F::TAG).map_err(super::debug)?],
        )
        .map_err(super::debug)?,
        subject,
        check,
        PopulationRef::named(super::OWNER, F::POPULATION).map_err(super::debug)?,
        Origin::HandWritten,
    )
    .map_err(super::debug)?;
    let revision =
        RevisionBinding::declared(ContentAddress::derived(F::REVISION_TAG, stem.as_bytes()));
    Binding::bound(
        row,
        ExecutableAttachment::attached(subject, check, revision, revision, call),
        Provenance::Unproduced,
    )
    .map_err(super::debug)
}
