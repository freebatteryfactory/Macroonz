//! The public handwritten benchmark receiver: structural admission, host ordering, non-vacuous primary work, and secondary wall observation.

#[path = "bench_receiver/fixture.rs"]
mod fixture;

use std::sync::atomic::{AtomicU64, Ordering};
use std::{fmt, num::TryFromIntError};
use threadpak_testpak::bench::{
    BenchAttachment, BenchAttachmentRefusal, BenchBinding, BenchBindingRefusal, BenchOutcome,
    BenchReport, BenchRowKey, BenchRunRefusal, BenchStage, BenchStampRefusal, BenchTable,
    BenchTableName, BenchTableRefusal, BenchTargetMismatch, BenchVerdictRefusal,
    ComplexityClaimRef, DeclaredBudgets, DeclaredBudgetsRefusal, InputSizeAxis,
    InputSizeAxisRefusal, PlantedWorseRef, PreflightRef, PrimaryWorkPhase,
    SecondaryObservationRefusal, WorkFormula, WorkFormulaRefusal, WorkJudgment, WorkJudgmentInput,
    WorkObservationRef, WorkRecorder, WorkRecordingRefusal, WorkloadRef, bench_verdict, run_all,
};
use threadpak_testpak::clock::{HarnessClock, MeasurementReading};
use threadpak_testpak::descriptor::{NameRefusal, Provenance};
use threadpak_testpak::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use threadpak_testpak::report::{
    FailureClass, FindingCause, TargetBinding, TargetTriple, ToolchainIdentity, TrialConclusion,
};
use threadpak_testpak::runner::Invocation;

const OWNER: &str = "testpak.bench.receiver";
static PREFLIGHT_MEASURED_CALLS: AtomicU64 = AtomicU64::new(0u64);
static PREFLIGHT_WORSE_CALLS: AtomicU64 = AtomicU64::new(0u64);
static PREFLIGHT_JUDGE_CALLS: AtomicU64 = AtomicU64::new(0u64);
static PREFLIGHT_CLOCK_CALLS: AtomicU64 = AtomicU64::new(0u64);
static CONTROL_CLOCK_CALLS: AtomicU64 = AtomicU64::new(0u64);
static PRIMARY_CLOCK_CALLS: AtomicU64 = AtomicU64::new(0u64);
static TARGET_PREFLIGHT_CALLS: AtomicU64 = AtomicU64::new(0u64);
static TARGET_MEASURED_CALLS: AtomicU64 = AtomicU64::new(0u64);
static TARGET_WORSE_CALLS: AtomicU64 = AtomicU64::new(0u64);
static TARGET_JUDGE_CALLS: AtomicU64 = AtomicU64::new(0u64);
static TARGET_CLOCK_CALLS: AtomicU64 = AtomicU64::new(0u64);
static FAST_CLOCK: AtomicU64 = AtomicU64::new(1u64);
static SLOW_CLOCK: AtomicU64 = AtomicU64::new(1u64);
static DRIFT_CALLS: AtomicU64 = AtomicU64::new(0u64);
static DRIFT_PRIMARY_CALLS: AtomicU64 = AtomicU64::new(0u64);
const MEASURED_REFUSED: FindingCause = FindingCause::named(OWNER, "measured-work-refused");
const WORSE_REFUSED: FindingCause = FindingCause::named(OWNER, "planted-worse-refused");
const GAP_REFUSED: FindingCause = FindingCause::named(OWNER, "declared-gap-not-observed");

enum BenchRoadFailure {
    Stamp(BenchStampRefusal),
    Run(BenchRunRefusal),
    Name(NameRefusal),
    Verdict(BenchVerdictRefusal),
    MissingReading,
    MissingVerdictRefusal,
    CountOutsideU64(TryFromIntError),
}

impl fmt::Debug for BenchRoadFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stamp(refusal) => formatter.debug_tuple("Stamp").field(refusal).finish(),
            Self::Run(refusal) => formatter.debug_tuple("Run").field(refusal).finish(),
            Self::Name(refusal) => formatter.debug_tuple("Name").field(refusal).finish(),
            Self::Verdict(refusal) => formatter.debug_tuple("Verdict").field(refusal).finish(),
            Self::MissingReading => formatter.write_str("MissingReading"),
            Self::MissingVerdictRefusal => formatter.write_str("MissingVerdictRefusal"),
            Self::CountOutsideU64(refusal) => formatter
                .debug_tuple("CountOutsideU64")
                .field(refusal)
                .finish(),
        }
    }
}

impl From<BenchStampRefusal> for BenchRoadFailure {
    fn from(refusal: BenchStampRefusal) -> Self {
        Self::Stamp(refusal)
    }
}

impl From<BenchRunRefusal> for BenchRoadFailure {
    fn from(refusal: BenchRunRefusal) -> Self {
        Self::Run(refusal)
    }
}

impl From<NameRefusal> for BenchRoadFailure {
    fn from(refusal: NameRefusal) -> Self {
        Self::Name(refusal)
    }
}

impl From<BenchVerdictRefusal> for BenchRoadFailure {
    fn from(refusal: BenchVerdictRefusal) -> Self {
        Self::Verdict(refusal)
    }
}

impl From<TryFromIntError> for BenchRoadFailure {
    fn from(refusal: TryFromIntError) -> Self {
        Self::CountOutsideU64(refusal)
    }
}

fn table_with(binding: BenchBinding) -> Result<BenchTable, BenchRoadFailure> {
    Ok(BenchTable::authored(
        BenchTableName::named(OWNER, "hostile-table")?,
        Provenance::Unproduced,
        vec![binding],
    )
    .map_err(BenchStampRefusal::from)?)
}

fn first_reading(
    report: &BenchReport,
) -> Result<&threadpak_testpak::bench::BenchReading, BenchRoadFailure> {
    report
        .readings()
        .first()
        .ok_or(BenchRoadFailure::MissingReading)
}

fn reset_target_counters() {
    TARGET_MEASURED_CALLS.store(0u64, Ordering::SeqCst);
    TARGET_WORSE_CALLS.store(0u64, Ordering::SeqCst);
    TARGET_JUDGE_CALLS.store(0u64, Ordering::SeqCst);
    TARGET_CLOCK_CALLS.store(0u64, Ordering::SeqCst);
    TARGET_PREFLIGHT_CALLS.store(0u64, Ordering::SeqCst);
}

fn assert_target_callables_were_withheld() {
    assert_eq!(TARGET_MEASURED_CALLS.load(Ordering::SeqCst), 0u64);
    assert_eq!(TARGET_WORSE_CALLS.load(Ordering::SeqCst), 0u64);
    assert_eq!(TARGET_JUDGE_CALLS.load(Ordering::SeqCst), 0u64);
    assert_eq!(TARGET_CLOCK_CALLS.load(Ordering::SeqCst), 0u64);
    assert_eq!(TARGET_PREFLIGHT_CALLS.load(Ordering::SeqCst), 0u64);
}

fn table_with_foreign_preflight(
    name: &'static str,
    target: TargetBinding,
) -> Result<(BenchTable, BenchRowKey), BenchRoadFailure> {
    let first = fixture::binding(
        target_counted_measured,
        target_counted_worse,
        target_counted_judge,
        target_counted_preflight,
    )?;
    let second_preflight = fixture::preflight_with(
        PreflightRef::named("testpak.bench.consumer", "correctness-preflight")?,
        fixture::preflight_passes,
        target,
    )?;
    let second = BenchBinding::bound(
        fixture::row_with_axis(vec![2u64, 8u64, 32u64])?,
        fixture::lawful_attachment(
            fixture::measured,
            fixture::planted_worse,
            fixture::lawful_judge,
        )?,
        second_preflight,
    )
    .map_err(BenchStampRefusal::from)?;
    let expected_refusal_row = second.row().key();
    let table = BenchTable::authored(
        BenchTableName::named(OWNER, name)?,
        Provenance::Unproduced,
        vec![first, second],
    )
    .map_err(BenchStampRefusal::from)?;
    Ok((table, expected_refusal_row))
}

fn write_identity_bytes(output: &mut Vec<u8>, bytes: &[u8]) -> Result<(), BenchRoadFailure> {
    output.extend_from_slice(&u64::try_from(bytes.len())?.to_be_bytes());
    output.extend_from_slice(bytes);
    Ok(())
}

fn write_identity_name(
    output: &mut Vec<u8>,
    namespace: &[u8],
    stem: &[u8],
) -> Result<(), BenchRoadFailure> {
    write_identity_bytes(output, namespace)?;
    write_identity_bytes(output, stem)
}

fn independent_lawful_row_address() -> Result<ContentAddress, BenchRoadFailure> {
    let mut preimage = Vec::new();
    let sizes = [2u64, 4u64, 8u64];
    write_identity_name(&mut preimage, b"testpak.bench.consumer", b"linear-workload")?;
    preimage.extend_from_slice(&u64::try_from(sizes.len())?.to_be_bytes());
    for size in sizes {
        preimage.extend_from_slice(&size.to_be_bytes());
    }
    write_identity_name(
        &mut preimage,
        b"testpak.bench.consumer",
        b"correctness-preflight",
    )?;
    write_identity_name(
        &mut preimage,
        b"testpak.bench.consumer",
        b"quadratic-control",
    )?;
    preimage.extend_from_slice(&2u32.to_be_bytes());
    preimage.extend_from_slice(&1u32.to_be_bytes());
    preimage.extend_from_slice(&2u64.to_be_bytes());
    preimage.extend_from_slice(&1u64.to_be_bytes());
    preimage.push(0u8);
    preimage.push(1u8);
    write_identity_bytes(&mut preimage, b"work=samples*n")?;
    write_identity_name(&mut preimage, b"testpak.bench.consumer", b"linear-growth")?;
    Ok(ContentAddress::derived(
        DomainTag::declared("bench-row-key", IdentityProfileVersion::declared(1u32)),
        &preimage,
    ))
}

fn preflight_counted_measured(
    input_size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    PREFLIGHT_MEASURED_CALLS.fetch_add(1u64, Ordering::SeqCst);
    fixture::measured(input_size, recorder)
}

fn preflight_counted_worse(
    input_size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    PREFLIGHT_WORSE_CALLS.fetch_add(1u64, Ordering::SeqCst);
    fixture::planted_worse(input_size, recorder)
}

fn preflight_counted_judge(input: &WorkJudgmentInput<'_>) -> WorkJudgment {
    PREFLIGHT_JUDGE_CALLS.fetch_add(1u64, Ordering::SeqCst);
    fixture::lawful_judge(input)
}

fn preflight_counted_clock() -> u64 {
    PREFLIGHT_CLOCK_CALLS.fetch_add(1u64, Ordering::SeqCst)
}

fn control_counted_clock() -> u64 {
    CONTROL_CLOCK_CALLS.fetch_add(1u64, Ordering::SeqCst)
}

fn primary_counted_clock() -> u64 {
    PRIMARY_CLOCK_CALLS.fetch_add(1u64, Ordering::SeqCst)
}

fn target_counted_preflight(invocation: &Invocation) -> TrialConclusion {
    TARGET_PREFLIGHT_CALLS.fetch_add(1u64, Ordering::SeqCst);
    fixture::preflight_passes(invocation)
}

fn target_counted_measured(
    input_size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    TARGET_MEASURED_CALLS.fetch_add(1u64, Ordering::SeqCst);
    fixture::measured(input_size, recorder)
}

fn target_counted_worse(
    input_size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    TARGET_WORSE_CALLS.fetch_add(1u64, Ordering::SeqCst);
    fixture::planted_worse(input_size, recorder)
}

fn target_counted_judge(input: &WorkJudgmentInput<'_>) -> WorkJudgment {
    TARGET_JUDGE_CALLS.fetch_add(1u64, Ordering::SeqCst);
    fixture::lawful_judge(input)
}

fn target_counted_clock() -> u64 {
    TARGET_CLOCK_CALLS.fetch_add(1u64, Ordering::SeqCst)
}

fn fast_clock() -> u64 {
    FAST_CLOCK.fetch_add(5u64, Ordering::SeqCst)
}

fn slow_clock() -> u64 {
    SLOW_CLOCK.fetch_add(50u64, Ordering::SeqCst)
}

fn zero_clock() -> u64 {
    0u64
}

fn drifting_measured(
    input_size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    let at = DRIFT_CALLS.fetch_add(1u64, Ordering::SeqCst);
    if at < DRIFT_PRIMARY_CALLS.load(Ordering::SeqCst) {
        fixture::measured(input_size, recorder)
    } else {
        fixture::planted_worse(input_size, recorder)
    }
}

fn unknown_observation(
    input_size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    let unknown = WorkObservationRef::named(OWNER, "unknown-work")
        .map_err(WorkRecordingRefusal::ObservationName)?;
    recorder.record(unknown, input_size)
}

fn overflowing_count(
    _input_size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    let observation = WorkObservationRef::named("testpak.bench.consumer", "unit-work")
        .map_err(WorkRecordingRefusal::ObservationName)?;
    recorder.record(observation, u64::MAX)?;
    recorder.record(observation, 1u64)
}

fn zeroed_measured_work(
    _input_size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    let observation = WorkObservationRef::named("testpak.bench.consumer", "unit-work")
        .map_err(WorkRecordingRefusal::ObservationName)?;
    recorder.record(observation, 0u64)
}

fn always_satisfy(_input: &WorkJudgmentInput<'_>) -> WorkJudgment {
    WorkJudgment::stated(
        threadpak_testpak::bench::WorkConclusion::Satisfied,
        threadpak_testpak::bench::WorkConclusion::Satisfied,
        threadpak_testpak::bench::WorkGapStanding::NotDistinguished(GAP_REFUSED),
    )
}

fn always_refuse(_input: &WorkJudgmentInput<'_>) -> WorkJudgment {
    WorkJudgment::stated(
        threadpak_testpak::bench::WorkConclusion::Refused(MEASURED_REFUSED),
        threadpak_testpak::bench::WorkConclusion::Refused(WORSE_REFUSED),
        threadpak_testpak::bench::WorkGapStanding::Distinguished,
    )
}

fn refuse_worse_without_gap(_input: &WorkJudgmentInput<'_>) -> WorkJudgment {
    WorkJudgment::stated(
        threadpak_testpak::bench::WorkConclusion::Satisfied,
        threadpak_testpak::bench::WorkConclusion::Refused(WORSE_REFUSED),
        threadpak_testpak::bench::WorkGapStanding::NotDistinguished(GAP_REFUSED),
    )
}

fn preflight_refuses(_invocation: &Invocation) -> TrialConclusion {
    threadpak_testpak::properties::concluded(
        threadpak_testpak::properties::Holding::Fails,
        FailureClass::RefusedByCheck,
        FindingCause::named(OWNER, "preflight-refused"),
    )
}

#[test]
fn lawful_receiver_retains_complete_primary_and_secondary_readings() -> Result<(), BenchRoadFailure>
{
    let table = fixture::lawful_table()?;
    let report = run_all(&table, &fixture::invocation())?;
    assert_eq!(report.denominator(), table.bindings().len());
    bench_verdict(&report)?;
    fixture::render(&report);
    let reading = first_reading(&report)?;
    let binding = table
        .bindings()
        .first()
        .ok_or(BenchRoadFailure::MissingReading)?;
    assert_eq!(reading.row(), binding.row());
    assert_eq!(reading.row().formula(), binding.row().formula());
    assert_eq!(reading.row().complexity(), binding.row().complexity());
    let BenchOutcome::Qualified {
        measured,
        planted_worse,
        judgment,
        secondary,
        ..
    } = reading.outcome()
    else {
        return Err(BenchRoadFailure::MissingReading);
    };
    assert_eq!(
        measured.points().len(),
        binding.row().input_sizes().sizes().len()
    );
    assert_eq!(planted_worse.points().len(), measured.points().len());
    assert!(judgment.qualifies());
    let expected_measurements = binding
        .row()
        .input_sizes()
        .sizes()
        .iter()
        .flat_map(|_| 0..binding.row().budgets().samples())
        .count();
    assert_eq!(secondary.measurements().len(), expected_measurements);
    assert!(secondary.judgment().qualifies());
    assert!(
        secondary
            .measurements()
            .iter()
            .all(|measurement| matches!(measurement, MeasurementReading::Observed(duration) if duration.nanoseconds() > 0u64))
    );
    Ok(())
}

#[test]
fn row_key_matches_an_independent_eight_fact_transcript() -> Result<(), BenchRoadFailure> {
    assert_eq!(
        fixture::lawful_row()?.key().address(),
        independent_lawful_row_address()?
    );
    Ok(())
}

#[test]
fn refused_preflight_withholds_every_benchmark_callable_and_clock() -> Result<(), BenchRoadFailure>
{
    PREFLIGHT_MEASURED_CALLS.store(0u64, Ordering::SeqCst);
    PREFLIGHT_WORSE_CALLS.store(0u64, Ordering::SeqCst);
    PREFLIGHT_JUDGE_CALLS.store(0u64, Ordering::SeqCst);
    PREFLIGHT_CLOCK_CALLS.store(0u64, Ordering::SeqCst);
    let binding = fixture::binding(
        preflight_counted_measured,
        preflight_counted_worse,
        preflight_counted_judge,
        preflight_refuses,
    )?;
    let report = run_all(
        &table_with(binding)?,
        &fixture::invocation_with(HarnessClock::reading(preflight_counted_clock)),
    )?;
    assert_eq!(
        first_reading(&report)?.outcome().stage(),
        BenchStage::PreflightRefused
    );
    assert_eq!(PREFLIGHT_MEASURED_CALLS.load(Ordering::SeqCst), 0u64);
    assert_eq!(PREFLIGHT_WORSE_CALLS.load(Ordering::SeqCst), 0u64);
    assert_eq!(PREFLIGHT_JUDGE_CALLS.load(Ordering::SeqCst), 0u64);
    assert_eq!(PREFLIGHT_CLOCK_CALLS.load(Ordering::SeqCst), 0u64);
    Ok(())
}

#[test]
fn planted_worse_and_judge_controls_are_non_vacuous() -> Result<(), BenchRoadFailure> {
    CONTROL_CLOCK_CALLS.store(0u64, Ordering::SeqCst);
    let same_callable = fixture::binding(
        fixture::measured,
        fixture::measured,
        fixture::lawful_judge,
        fixture::preflight_passes,
    )?;
    let same_callable = run_all(
        &table_with(same_callable)?,
        &fixture::invocation_with(HarnessClock::reading(control_counted_clock)),
    )?;
    assert_eq!(
        first_reading(&same_callable)?.outcome().stage(),
        BenchStage::PlantedWorseNotDistinguished
    );
    assert_eq!(CONTROL_CLOCK_CALLS.load(Ordering::SeqCst), 0u64);
    let Err(verdict) = bench_verdict(&same_callable) else {
        return Err(BenchRoadFailure::MissingVerdictRefusal);
    };
    assert_eq!(verdict.row(), first_reading(&same_callable)?.row().key());
    assert_eq!(verdict.stage(), BenchStage::PlantedWorseNotDistinguished);

    CONTROL_CLOCK_CALLS.store(0u64, Ordering::SeqCst);
    let always_satisfy = fixture::binding(
        fixture::measured,
        fixture::planted_worse,
        always_satisfy,
        fixture::preflight_passes,
    )?;
    let always_satisfy = run_all(
        &table_with(always_satisfy)?,
        &fixture::invocation_with(HarnessClock::reading(control_counted_clock)),
    )?;
    assert_eq!(
        first_reading(&always_satisfy)?.outcome().stage(),
        BenchStage::PlantedWorseNotDistinguished
    );
    assert_eq!(CONTROL_CLOCK_CALLS.load(Ordering::SeqCst), 0u64);

    CONTROL_CLOCK_CALLS.store(0u64, Ordering::SeqCst);
    let gap_not_distinguished = fixture::binding(
        fixture::measured,
        fixture::planted_worse,
        refuse_worse_without_gap,
        fixture::preflight_passes,
    )?;
    let gap_not_distinguished = run_all(
        &table_with(gap_not_distinguished)?,
        &fixture::invocation_with(HarnessClock::reading(control_counted_clock)),
    )?;
    assert_eq!(
        first_reading(&gap_not_distinguished)?.outcome().stage(),
        BenchStage::PlantedWorseNotDistinguished
    );
    assert_eq!(CONTROL_CLOCK_CALLS.load(Ordering::SeqCst), 0u64);

    let always_refuse = fixture::binding(
        fixture::measured,
        fixture::planted_worse,
        always_refuse,
        fixture::preflight_passes,
    )?;
    CONTROL_CLOCK_CALLS.store(0u64, Ordering::SeqCst);
    let always_refuse = run_all(
        &table_with(always_refuse)?,
        &fixture::invocation_with(HarnessClock::reading(control_counted_clock)),
    )?;
    assert_eq!(
        first_reading(&always_refuse)?.outcome().stage(),
        BenchStage::PrimaryWorkRefused
    );
    assert_eq!(CONTROL_CLOCK_CALLS.load(Ordering::SeqCst), 0u64);
    Ok(())
}

#[test]
fn damaged_measured_work_cannot_be_rescued_by_favorable_time() -> Result<(), BenchRoadFailure> {
    PRIMARY_CLOCK_CALLS.store(0u64, Ordering::SeqCst);
    let binding = fixture::binding(
        zeroed_measured_work,
        fixture::planted_worse,
        fixture::lawful_judge,
        fixture::preflight_passes,
    )?;
    let report = run_all(
        &table_with(binding)?,
        &fixture::invocation_with(HarnessClock::reading(primary_counted_clock)),
    )?;
    let reading = first_reading(&report)?;
    assert_eq!(reading.outcome().stage(), BenchStage::PrimaryWorkRefused);
    let BenchOutcome::PrimaryWorkRefused { judgment, .. } = reading.outcome() else {
        return Err(BenchRoadFailure::MissingReading);
    };
    assert!(matches!(
        judgment.measured(),
        threadpak_testpak::bench::WorkConclusion::Refused(_)
    ));
    assert!(matches!(
        judgment.planted_worse(),
        threadpak_testpak::bench::WorkConclusion::Refused(_)
    ));
    assert_eq!(
        judgment.gap(),
        threadpak_testpak::bench::WorkGapStanding::Distinguished
    );
    assert_eq!(PRIMARY_CLOCK_CALLS.load(Ordering::SeqCst), 0u64);
    Ok(())
}

#[test]
fn declaration_boundaries_refuse_vacuity_and_duplicates() -> Result<(), BenchRoadFailure> {
    assert!(matches!(
        InputSizeAxis::declared(Vec::new()),
        Err(InputSizeAxisRefusal::TooShort { found: 0 })
    ));
    assert!(matches!(
        InputSizeAxis::declared(vec![4u64, 4u64]),
        Err(InputSizeAxisRefusal::DuplicateSize {
            size: 4,
            first: 0,
            duplicate: 1,
        })
    ));
    assert!(matches!(
        DeclaredBudgets::declared(0u32, 0u32, 1u64, 1u64),
        Err(DeclaredBudgetsRefusal::NoSamples)
    ));
    assert!(matches!(
        DeclaredBudgets::declared(1u32, 0u32, 0u64, 1u64),
        Err(DeclaredBudgetsRefusal::ZeroRatioNumerator)
    ));
    assert!(matches!(
        DeclaredBudgets::declared(1u32, 0u32, 1u64, 0u64),
        Err(DeclaredBudgetsRefusal::ZeroRatioDenominator)
    ));
    assert!(matches!(
        WorkFormula::encoded(Vec::new()),
        Err(WorkFormulaRefusal::Empty)
    ));

    let observation = WorkObservationRef::named(OWNER, "one-observation")?;
    let workload = WorkloadRef::named(OWNER, "workload")?;
    let worse = PlantedWorseRef::named(OWNER, "worse")?;
    let complexity = ComplexityClaimRef::named(OWNER, "complexity")?;
    assert!(matches!(
        BenchAttachment::attached(
            workload,
            fixture::measured,
            worse,
            fixture::planted_worse,
            threadpak_testpak::bench::WorkJudgeBinding::bound(complexity, fixture::lawful_judge,),
            Vec::new(),
        ),
        Err(BenchAttachmentRefusal::NoObservation)
    ));
    assert!(matches!(
        BenchAttachment::attached(
            workload,
            fixture::measured,
            worse,
            fixture::planted_worse,
            threadpak_testpak::bench::WorkJudgeBinding::bound(
                complexity,
                fixture::lawful_judge,
            ),
            vec![observation, observation],
        ),
        Err(BenchAttachmentRefusal::DuplicateObservation {
            observation: repeated,
            first: 0,
            duplicate: 1,
        }) if repeated == observation
    ));
    Ok(())
}

#[test]
fn wall_readings_change_without_changing_primary_qualification() -> Result<(), BenchRoadFailure> {
    let table = fixture::lawful_table()?;
    let fast = run_all(
        &table,
        &fixture::invocation_with(HarnessClock::reading(fast_clock)),
    )?;
    let slow = run_all(
        &table,
        &fixture::invocation_with(HarnessClock::reading(slow_clock)),
    )?;
    bench_verdict(&fast)?;
    bench_verdict(&slow)?;
    let BenchOutcome::Qualified {
        measured: fast_work,
        planted_worse: fast_worse,
        judgment: fast_judgment,
        secondary: fast_secondary,
        ..
    } = first_reading(&fast)?.outcome()
    else {
        return Err(BenchRoadFailure::MissingReading);
    };
    let BenchOutcome::Qualified {
        measured: slow_work,
        planted_worse: slow_worse,
        judgment: slow_judgment,
        secondary: slow_secondary,
        ..
    } = first_reading(&slow)?.outcome()
    else {
        return Err(BenchRoadFailure::MissingReading);
    };
    assert_eq!(fast_work, slow_work);
    assert_eq!(fast_worse, slow_worse);
    assert_eq!(fast_judgment, slow_judgment);
    assert!(fast_judgment.qualifies());
    assert_eq!(fast_secondary.work(), slow_secondary.work());
    assert_eq!(fast_secondary.judgment(), slow_secondary.judgment());
    assert_ne!(fast_secondary.measurements(), slow_secondary.measurements());
    Ok(())
}

#[test]
fn unavailable_wall_readings_are_not_observed_zero() -> Result<(), BenchRoadFailure> {
    let table = fixture::lawful_table()?;
    let unavailable = run_all(
        &table,
        &fixture::invocation_with(HarnessClock::unavailable()),
    )?;
    let zero = run_all(
        &table,
        &fixture::invocation_with(HarnessClock::reading(zero_clock)),
    )?;
    bench_verdict(&unavailable)?;
    bench_verdict(&zero)?;
    let BenchOutcome::Qualified {
        secondary: unavailable_secondary,
        ..
    } = first_reading(&unavailable)?.outcome()
    else {
        return Err(BenchRoadFailure::MissingReading);
    };
    let BenchOutcome::Qualified {
        secondary: zero_secondary,
        ..
    } = first_reading(&zero)?.outcome()
    else {
        return Err(BenchRoadFailure::MissingReading);
    };
    assert!(
        unavailable_secondary
            .measurements()
            .iter()
            .all(|reading| *reading == MeasurementReading::Unavailable)
    );
    assert!(zero_secondary.measurements().iter().all(|reading| matches!(
        reading,
        MeasurementReading::Observed(duration) if duration.nanoseconds() == 0u64
    )));
    Ok(())
}

#[test]
fn binding_refusals_name_each_mismatched_relationship() -> Result<(), BenchRoadFailure> {
    let row = fixture::lawful_row()?;
    let preflight = fixture::lawful_preflight(fixture::preflight_passes)?;
    let observation = WorkObservationRef::named(OWNER, "unit-work")?;
    let foreign_workload = WorkloadRef::named(OWNER, "foreign-workload")?;
    let foreign_worse = PlantedWorseRef::named(OWNER, "foreign-control")?;
    let foreign_complexity = ComplexityClaimRef::named(OWNER, "foreign-complexity")?;
    let foreign_preflight = PreflightRef::named(OWNER, "foreign-preflight")?;

    let workload_attachment = fixture::attachment_with_refs(
        foreign_workload,
        row.planted_worse(),
        row.complexity(),
        fixture::measured,
        fixture::planted_worse,
        fixture::lawful_judge,
        vec![observation],
    )?;
    assert!(matches!(
        BenchBinding::bound(row.clone(), workload_attachment, preflight.clone()),
        Err(BenchBindingRefusal::Workload { .. })
    ));

    let planted_worse_attachment = fixture::attachment_with_refs(
        row.workload(),
        foreign_worse,
        row.complexity(),
        fixture::measured,
        fixture::planted_worse,
        fixture::lawful_judge,
        vec![observation],
    )?;
    assert!(matches!(
        BenchBinding::bound(row.clone(), planted_worse_attachment, preflight.clone()),
        Err(BenchBindingRefusal::PlantedWorse { .. })
    ));

    let complexity_attachment = fixture::attachment_with_refs(
        row.workload(),
        row.planted_worse(),
        foreign_complexity,
        fixture::measured,
        fixture::planted_worse,
        fixture::lawful_judge,
        vec![observation],
    )?;
    assert!(matches!(
        BenchBinding::bound(row.clone(), complexity_attachment, preflight.clone()),
        Err(BenchBindingRefusal::Complexity { .. })
    ));

    let foreign_preflight_trial = fixture::preflight_with(
        foreign_preflight,
        fixture::preflight_passes,
        fixture::target(),
    )?;
    assert!(matches!(
        BenchBinding::bound(
            row,
            fixture::lawful_attachment(
                fixture::measured,
                fixture::planted_worse,
                fixture::lawful_judge
            )?,
            foreign_preflight_trial
        ),
        Err(BenchBindingRefusal::Preflight { .. })
    ));
    Ok(())
}

#[test]
fn table_refuses_vacuity_and_exact_duplicate_identity() -> Result<(), BenchRoadFailure> {
    let name = BenchTableName::named(OWNER, "table-refusals")?;
    assert!(matches!(
        BenchTable::authored(name, Provenance::Unproduced, Vec::new()),
        Err(BenchTableRefusal::Empty)
    ));
    let binding = fixture::lawful_binding()?;
    let expected_row = binding.row().key();
    assert!(matches!(
        BenchTable::authored(name, Provenance::Unproduced, vec![binding.clone(), binding]),
        Err(BenchTableRefusal::DuplicateRow {
            row,
            first: 0,
            duplicate: 1,
        }) if row == expected_row
    ));
    Ok(())
}

#[test]
fn target_mismatch_refuses_before_any_benchmark_caller_code() -> Result<(), BenchRoadFailure> {
    reset_target_counters();
    let (table, expected_refusal_row) = table_with_foreign_preflight(
        "complete-target-prevalidation",
        TargetBinding::bound(
            TargetTriple::declared("foreign-target"),
            ToolchainIdentity::declared("1.98.0"),
        ),
    )?;
    let expected_benchmark_target = fixture::target().target().clone();
    let target_result = run_all(
        &table,
        &fixture::invocation_with(HarnessClock::reading(target_counted_clock)),
    );
    assert!(matches!(
        target_result,
        Err(BenchRunRefusal::PreflightTargetMismatch {
            row,
            mismatch: BenchTargetMismatch::Target {
                benchmark,
                preflight,
            },
        }) if row == expected_refusal_row
            && benchmark == expected_benchmark_target
            && preflight == TargetTriple::declared("foreign-target")
    ));
    assert_target_callables_were_withheld();
    Ok(())
}

#[test]
fn toolchain_mismatch_refuses_before_any_benchmark_caller_code() -> Result<(), BenchRoadFailure> {
    reset_target_counters();
    let (table, expected_refusal_row) = table_with_foreign_preflight(
        "complete-toolchain-prevalidation",
        TargetBinding::bound(
            fixture::target().target().clone(),
            ToolchainIdentity::declared("foreign-toolchain"),
        ),
    )?;
    let expected_benchmark_toolchain = fixture::target().toolchain().clone();
    let toolchain_result = run_all(
        &table,
        &fixture::invocation_with(HarnessClock::reading(target_counted_clock)),
    );
    assert!(matches!(
        toolchain_result,
        Err(BenchRunRefusal::PreflightTargetMismatch {
            row,
            mismatch: BenchTargetMismatch::Toolchain {
                benchmark,
                preflight,
            },
        }) if row == expected_refusal_row
            && benchmark == expected_benchmark_toolchain
            && preflight == ToolchainIdentity::declared("foreign-toolchain")
    ));
    assert_target_callables_were_withheld();
    Ok(())
}

#[test]
fn recorder_and_secondary_failures_never_become_qualified_reports() -> Result<(), BenchRoadFailure>
{
    let unknown = fixture::binding(
        unknown_observation,
        fixture::planted_worse,
        fixture::lawful_judge,
        fixture::preflight_passes,
    )?;
    assert!(matches!(
        run_all(&table_with(unknown)?, &fixture::invocation()),
        Err(BenchRunRefusal::WorkNotRecorded {
            phase: PrimaryWorkPhase::Measured,
            refusal: WorkRecordingRefusal::UnknownObservation(_),
            ..
        })
    ));

    let overflow = fixture::binding(
        overflowing_count,
        fixture::planted_worse,
        fixture::lawful_judge,
        fixture::preflight_passes,
    )?;
    assert!(matches!(
        run_all(&table_with(overflow)?, &fixture::invocation()),
        Err(BenchRunRefusal::WorkNotRecorded {
            phase: PrimaryWorkPhase::Measured,
            refusal: WorkRecordingRefusal::CountOverflow { .. },
            ..
        })
    ));

    DRIFT_CALLS.store(0u64, Ordering::SeqCst);
    let drift = fixture::binding(
        drifting_measured,
        fixture::planted_worse,
        fixture::lawful_judge,
        fixture::preflight_passes,
    )?;
    let primary_calls = drift
        .row()
        .input_sizes()
        .sizes()
        .iter()
        .flat_map(|_| 0..drift.row().budgets().samples())
        .count();
    DRIFT_PRIMARY_CALLS.store(u64::try_from(primary_calls)?, Ordering::SeqCst);
    assert!(matches!(
        run_all(&table_with(drift)?, &fixture::invocation()),
        Err(BenchRunRefusal::SecondaryWorkRefused {
            refusal: SecondaryObservationRefusal::Judgment(_),
            ..
        })
    ));
    Ok(())
}

#[test]
fn complete_report_retains_two_distinct_rows_in_table_order() -> Result<(), BenchRoadFailure> {
    let positive_first = fixture::lawful_binding()?;
    let second = BenchBinding::bound(
        fixture::row_with_axis(vec![2u64, 8u64, 32u64])?,
        fixture::lawful_attachment(
            fixture::measured,
            fixture::planted_worse,
            fixture::lawful_judge,
        )?,
        fixture::lawful_preflight(fixture::preflight_passes)?,
    )
    .map_err(BenchStampRefusal::from)?;
    let bindings = vec![positive_first, second];
    let expected_rows = bindings
        .iter()
        .map(|binding| binding.row().key())
        .collect::<Vec<_>>();
    let table = BenchTable::authored(
        BenchTableName::named(OWNER, "same-workload-distinct-rows")?,
        Provenance::Unproduced,
        bindings,
    )
    .map_err(BenchStampRefusal::from)?;
    let report = run_all(&table, &fixture::invocation())?;
    let found_rows = report
        .readings()
        .iter()
        .map(|reading| reading.row().key())
        .collect::<Vec<_>>();
    assert_eq!(found_rows, expected_rows);
    assert_eq!(report.denominator(), table.len());
    bench_verdict(&report)?;

    let verdict_first = fixture::lawful_binding()?;
    let hostile = BenchBinding::bound(
        fixture::row_with_axis(vec![2u64, 8u64, 32u64])?,
        fixture::lawful_attachment(fixture::measured, fixture::measured, fixture::lawful_judge)?,
        fixture::lawful_preflight(fixture::preflight_passes)?,
    )
    .map_err(BenchStampRefusal::from)?;
    let expected_refusal_row = hostile.row().key();
    let hostile_table = BenchTable::authored(
        BenchTableName::named(OWNER, "first-red-in-authored-order")?,
        Provenance::Unproduced,
        vec![verdict_first, hostile],
    )
    .map_err(BenchStampRefusal::from)?;
    let hostile_report = run_all(&hostile_table, &fixture::invocation())?;
    let Err(refusal) = bench_verdict(&hostile_report) else {
        return Err(BenchRoadFailure::MissingVerdictRefusal);
    };
    assert_eq!(refusal.row(), expected_refusal_row);
    assert_eq!(refusal.stage(), BenchStage::PlantedWorseNotDistinguished);
    Ok(())
}
