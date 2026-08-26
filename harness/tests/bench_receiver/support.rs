//! Shared construction and hostile callables for the benchmark claim modules.

pub(super) use macroonz_harness::bench::{
    BenchAttachment, BenchAttachmentRefusal, BenchBinding, BenchBindingRefusal, BenchOutcome,
    BenchReading, BenchReport, BenchRowKey, BenchRunRefusal, BenchStage, BenchStampRefusal,
    BenchTable, BenchTableName, BenchTableRefusal, BenchTargetMismatch, BenchVerdictRefusal,
    ComplexityClaimRef, DeclaredBudgets, DeclaredBudgetsRefusal, InputSizeAxis,
    InputSizeAxisRefusal, PlantedWorseRef, PreflightRef, PrimaryWorkPhase,
    SecondaryObservationRefusal, WorkConclusion, WorkFormula, WorkFormulaRefusal, WorkGapStanding,
    WorkJudgeBinding, WorkJudgment, WorkJudgmentInput, WorkObservationRef, WorkRecorder,
    WorkRecordingRefusal, WorkloadRef, bench_verdict, run_all,
};
pub(super) use macroonz_harness::clock::{HarnessClock, MeasurementReading};
pub(super) use macroonz_harness::descriptor::{NameRefusal, Provenance};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::properties::{Holding, concluded};
use macroonz_harness::report::{FailureClass, FindingCause, TrialConclusion};
pub(super) use macroonz_harness::report::{TargetBinding, TargetTriple, ToolchainIdentity};
pub(super) use macroonz_harness::runner::Invocation;
use std::sync::atomic::{AtomicU64, Ordering};
use std::{fmt, num::TryFromIntError};

pub(super) const OWNER: &str = "harness.bench.receiver";
pub(super) static PREFLIGHT_MEASURED_CALLS: AtomicU64 = AtomicU64::new(0u64);
pub(super) static PREFLIGHT_WORSE_CALLS: AtomicU64 = AtomicU64::new(0u64);
pub(super) static PREFLIGHT_JUDGE_CALLS: AtomicU64 = AtomicU64::new(0u64);
pub(super) static PREFLIGHT_CLOCK_CALLS: AtomicU64 = AtomicU64::new(0u64);
pub(super) static CONTROL_CLOCK_CALLS: AtomicU64 = AtomicU64::new(0u64);
pub(super) static PRIMARY_CLOCK_CALLS: AtomicU64 = AtomicU64::new(0u64);
static TARGET_PREFLIGHT_CALLS: AtomicU64 = AtomicU64::new(0u64);
static TARGET_MEASURED_CALLS: AtomicU64 = AtomicU64::new(0u64);
static TARGET_WORSE_CALLS: AtomicU64 = AtomicU64::new(0u64);
static TARGET_JUDGE_CALLS: AtomicU64 = AtomicU64::new(0u64);
static TARGET_CLOCK_CALLS: AtomicU64 = AtomicU64::new(0u64);
static FAST_CLOCK: AtomicU64 = AtomicU64::new(1u64);
static SLOW_CLOCK: AtomicU64 = AtomicU64::new(1u64);
pub(super) static DRIFT_CALLS: AtomicU64 = AtomicU64::new(0u64);
pub(super) static DRIFT_PRIMARY_CALLS: AtomicU64 = AtomicU64::new(0u64);
const MEASURED_REFUSED: FindingCause = FindingCause::named(OWNER, "measured-work-refused");
const WORSE_REFUSED: FindingCause = FindingCause::named(OWNER, "planted-worse-refused");
const GAP_REFUSED: FindingCause = FindingCause::named(OWNER, "declared-gap-not-observed");

pub(super) enum BenchRoadFailure {
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

pub(super) fn table_with(binding: BenchBinding) -> Result<BenchTable, BenchRoadFailure> {
    Ok(BenchTable::authored(
        BenchTableName::named(OWNER, "hostile-table")?,
        Provenance::Unproduced,
        vec![binding],
    )
    .map_err(BenchStampRefusal::from)?)
}

pub(super) fn first_reading(report: &BenchReport) -> Result<&BenchReading, BenchRoadFailure> {
    report
        .readings()
        .first()
        .ok_or(BenchRoadFailure::MissingReading)
}

pub(super) fn reset_target_counters() {
    TARGET_MEASURED_CALLS.store(0u64, Ordering::SeqCst);
    TARGET_WORSE_CALLS.store(0u64, Ordering::SeqCst);
    TARGET_JUDGE_CALLS.store(0u64, Ordering::SeqCst);
    TARGET_CLOCK_CALLS.store(0u64, Ordering::SeqCst);
    TARGET_PREFLIGHT_CALLS.store(0u64, Ordering::SeqCst);
}

pub(super) fn assert_target_callables_were_withheld() {
    assert_eq!(TARGET_MEASURED_CALLS.load(Ordering::SeqCst), 0u64);
    assert_eq!(TARGET_WORSE_CALLS.load(Ordering::SeqCst), 0u64);
    assert_eq!(TARGET_JUDGE_CALLS.load(Ordering::SeqCst), 0u64);
    assert_eq!(TARGET_CLOCK_CALLS.load(Ordering::SeqCst), 0u64);
    assert_eq!(TARGET_PREFLIGHT_CALLS.load(Ordering::SeqCst), 0u64);
}

pub(super) fn table_with_foreign_preflight(
    name: &'static str,
    target: TargetBinding,
) -> Result<(BenchTable, BenchRowKey), BenchRoadFailure> {
    let first = super::fixture::binding(
        target_counted_measured,
        target_counted_worse,
        target_counted_judge,
        target_counted_preflight,
    )?;
    let second_preflight = super::fixture::preflight_with(
        PreflightRef::named("harness.bench.consumer", "correctness-preflight")?,
        super::fixture::preflight_passes,
        target,
    )?;
    let second = BenchBinding::bound(
        super::fixture::row_with_axis(vec![2u64, 8u64, 32u64])?,
        super::fixture::lawful_attachment(
            super::fixture::measured,
            super::fixture::planted_worse,
            super::fixture::lawful_judge,
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

pub(super) fn independent_lawful_row_address() -> Result<ContentAddress, BenchRoadFailure> {
    let mut preimage = Vec::new();
    let sizes = [2u64, 4u64, 8u64];
    write_identity_name(&mut preimage, b"harness.bench.consumer", b"linear-workload")?;
    preimage.extend_from_slice(&u64::try_from(sizes.len())?.to_be_bytes());
    for size in sizes {
        preimage.extend_from_slice(&size.to_be_bytes());
    }
    write_identity_name(
        &mut preimage,
        b"harness.bench.consumer",
        b"correctness-preflight",
    )?;
    write_identity_name(
        &mut preimage,
        b"harness.bench.consumer",
        b"quadratic-control",
    )?;
    preimage.extend_from_slice(&2u32.to_be_bytes());
    preimage.extend_from_slice(&1u32.to_be_bytes());
    preimage.extend_from_slice(&2u64.to_be_bytes());
    preimage.extend_from_slice(&1u64.to_be_bytes());
    preimage.push(0u8);
    preimage.push(1u8);
    write_identity_bytes(&mut preimage, b"work=samples*n")?;
    write_identity_name(&mut preimage, b"harness.bench.consumer", b"linear-growth")?;
    Ok(ContentAddress::derived(
        DomainTag::declared("bench-row-key", IdentityProfileVersion::declared(1u32)),
        &preimage,
    ))
}

pub(super) fn preflight_counted_measured(
    input_size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    PREFLIGHT_MEASURED_CALLS.fetch_add(1u64, Ordering::SeqCst);
    super::fixture::measured(input_size, recorder)
}

pub(super) fn preflight_counted_worse(
    input_size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    PREFLIGHT_WORSE_CALLS.fetch_add(1u64, Ordering::SeqCst);
    super::fixture::planted_worse(input_size, recorder)
}

pub(super) fn preflight_counted_judge(input: &WorkJudgmentInput<'_>) -> WorkJudgment {
    PREFLIGHT_JUDGE_CALLS.fetch_add(1u64, Ordering::SeqCst);
    super::fixture::lawful_judge(input)
}

pub(super) fn preflight_counted_clock() -> u64 {
    PREFLIGHT_CLOCK_CALLS.fetch_add(1u64, Ordering::SeqCst)
}

pub(super) fn control_counted_clock() -> u64 {
    CONTROL_CLOCK_CALLS.fetch_add(1u64, Ordering::SeqCst)
}

pub(super) fn primary_counted_clock() -> u64 {
    PRIMARY_CLOCK_CALLS.fetch_add(1u64, Ordering::SeqCst)
}

fn target_counted_preflight(invocation: &Invocation) -> TrialConclusion {
    TARGET_PREFLIGHT_CALLS.fetch_add(1u64, Ordering::SeqCst);
    super::fixture::preflight_passes(invocation)
}

fn target_counted_measured(
    input_size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    TARGET_MEASURED_CALLS.fetch_add(1u64, Ordering::SeqCst);
    super::fixture::measured(input_size, recorder)
}

fn target_counted_worse(
    input_size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    TARGET_WORSE_CALLS.fetch_add(1u64, Ordering::SeqCst);
    super::fixture::planted_worse(input_size, recorder)
}

fn target_counted_judge(input: &WorkJudgmentInput<'_>) -> WorkJudgment {
    TARGET_JUDGE_CALLS.fetch_add(1u64, Ordering::SeqCst);
    super::fixture::lawful_judge(input)
}

pub(super) fn target_counted_clock() -> u64 {
    TARGET_CLOCK_CALLS.fetch_add(1u64, Ordering::SeqCst)
}

pub(super) fn fast_clock() -> u64 {
    FAST_CLOCK.fetch_add(5u64, Ordering::SeqCst)
}

pub(super) fn slow_clock() -> u64 {
    SLOW_CLOCK.fetch_add(50u64, Ordering::SeqCst)
}

pub(super) fn zero_clock() -> u64 {
    0u64
}

pub(super) fn drifting_measured(
    input_size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    let at = DRIFT_CALLS.fetch_add(1u64, Ordering::SeqCst);
    if at < DRIFT_PRIMARY_CALLS.load(Ordering::SeqCst) {
        super::fixture::measured(input_size, recorder)
    } else {
        super::fixture::planted_worse(input_size, recorder)
    }
}

pub(super) fn unknown_observation(
    input_size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    let unknown = WorkObservationRef::named(OWNER, "unknown-work")
        .map_err(WorkRecordingRefusal::ObservationName)?;
    recorder.record(unknown, input_size)
}

pub(super) fn overflowing_count(
    _input_size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    let observation = WorkObservationRef::named("harness.bench.consumer", "unit-work")
        .map_err(WorkRecordingRefusal::ObservationName)?;
    recorder.record(observation, u64::MAX)?;
    recorder.record(observation, 1u64)
}

pub(super) fn zeroed_measured_work(
    _input_size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    let observation = WorkObservationRef::named("harness.bench.consumer", "unit-work")
        .map_err(WorkRecordingRefusal::ObservationName)?;
    recorder.record(observation, 0u64)
}

pub(super) fn always_satisfy(_input: &WorkJudgmentInput<'_>) -> WorkJudgment {
    WorkJudgment::stated(
        WorkConclusion::Satisfied,
        WorkConclusion::Satisfied,
        WorkGapStanding::NotDistinguished(GAP_REFUSED),
    )
}

pub(super) fn always_refuse(_input: &WorkJudgmentInput<'_>) -> WorkJudgment {
    WorkJudgment::stated(
        WorkConclusion::Refused(MEASURED_REFUSED),
        WorkConclusion::Refused(WORSE_REFUSED),
        WorkGapStanding::Distinguished,
    )
}

pub(super) fn refuse_worse_without_gap(_input: &WorkJudgmentInput<'_>) -> WorkJudgment {
    WorkJudgment::stated(
        WorkConclusion::Satisfied,
        WorkConclusion::Refused(WORSE_REFUSED),
        WorkGapStanding::NotDistinguished(GAP_REFUSED),
    )
}

pub(super) fn preflight_refuses(_invocation: &Invocation) -> TrialConclusion {
    concluded(
        Holding::Fails,
        FailureClass::RefusedByCheck,
        FindingCause::named(OWNER, "preflight-refused"),
    )
}
