//! One neutral benchmark declaration used by the behavior lane and the ordinary bench target.

use macroonz_harness::bench::{
    BenchAttachment, BenchBinding, BenchCall, BenchInvocation, BenchMeasurement, BenchReferences,
    BenchRow, BenchStampRefusal, ComplexityClaimRef, ContentionPosture, DeclaredBudgets,
    InputSizeAxis, PlantedWorseRef, PreflightRef, PreflightTrial, WorkConclusion, WorkCurve,
    WorkFormula, WorkGapStanding, WorkJudge, WorkJudgeBinding, WorkJudgment, WorkJudgmentInput,
    WorkObservationRef, WorkRecorder, WorkRecordingRefusal, WorkloadRef,
};
use macroonz_harness::clock::HarnessClock;
use macroonz_harness::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, ExecutableAttachment, ExecutionSuite, NameRefusal,
    Origin, PopulationRef, Provenance, RevisionBinding, Role, Row, SubjectRoute, Tag,
    TrialTableRefusal,
};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, FindingCause, InvocationProfile, TargetBinding, TargetTriple,
    TimeBudget, ToolchainIdentity, TrialConclusion, TrialSite,
};
use macroonz_harness::runner::{Invocation, TrialBinding};
use std::sync::atomic::{AtomicU64, Ordering};

const OWNER: &str = "harness.bench.consumer";
const LINEAR_FORMULA: &[u8] = b"work=samples*n";
const MEASURED_REFUSED: FindingCause = FindingCause::named(OWNER, "measured-work-refused");
const WORSE_REFUSED: FindingCause = FindingCause::named(OWNER, "planted-worse-refused");
const GAP_REFUSED: FindingCause = FindingCause::named(OWNER, "declared-gap-not-observed");
const REVISION_TAG: DomainTag = DomainTag::declared(
    "bench-consumer-revision",
    IdentityProfileVersion::declared(1),
);

static BENCH_CLOCK: AtomicU64 = AtomicU64::new(1u64);

fn benchmark_clock() -> u64 {
    BENCH_CLOCK.fetch_add(10u64, Ordering::SeqCst)
}

pub(super) fn target() -> TargetBinding {
    TargetBinding::bound(
        TargetTriple::declared("neutral-bench-target"),
        ToolchainIdentity::declared("1.98.0"),
    )
}

pub(super) fn invocation() -> BenchInvocation {
    invocation_with(HarnessClock::reading(benchmark_clock))
}

pub(super) fn invocation_with(clock: HarnessClock) -> BenchInvocation {
    BenchInvocation::declared(target(), clock, ContentionPosture::NoDeclaredContention)
}

fn workload() -> Result<WorkloadRef, NameRefusal> {
    WorkloadRef::named(OWNER, "linear-workload")
}

fn preflight_ref() -> Result<PreflightRef, NameRefusal> {
    PreflightRef::named(OWNER, "correctness-preflight")
}

fn worse_ref() -> Result<PlantedWorseRef, NameRefusal> {
    PlantedWorseRef::named(OWNER, "quadratic-control")
}

fn complexity() -> Result<ComplexityClaimRef, NameRefusal> {
    ComplexityClaimRef::named(OWNER, "linear-growth")
}

fn observation() -> Result<WorkObservationRef, NameRefusal> {
    WorkObservationRef::named(OWNER, "unit-work")
}

pub(super) fn measured(
    input_size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    let observation = observation().map_err(WorkRecordingRefusal::ObservationName)?;
    recorder.record(observation, input_size)
}

pub(super) fn planted_worse(
    input_size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    let observation = observation().map_err(WorkRecordingRefusal::ObservationName)?;
    let Some(units) = input_size.checked_mul(input_size) else {
        return Err(WorkRecordingRefusal::AmountOverflow {
            observation,
            input_size,
        });
    };
    recorder.record(observation, units)
}

fn linear_count(input_size: u64, samples: u32) -> Option<u64> {
    input_size.checked_mul(u64::from(samples))
}

fn quadratic_count(input_size: u64, samples: u32) -> Option<u64> {
    input_size
        .checked_mul(input_size)?
        .checked_mul(u64::from(samples))
}

fn has_shape(curve: &WorkCurve, samples: u32, expected: fn(u64, u32) -> Option<u64>) -> bool {
    curve.points().iter().all(|point| {
        let expected = expected(point.input_size(), samples);
        let [found] = point.counts() else {
            return false;
        };
        Some(found.count()) == expected
    })
}

fn has_declared_gap(input: &WorkJudgmentInput<'_>) -> bool {
    if input.measured().points().len() != input.planted_worse().points().len() {
        return false;
    }
    input
        .measured()
        .points()
        .iter()
        .zip(input.planted_worse().points())
        .all(|(measured_point, worse_point)| {
            let [measured] = measured_point.counts() else {
                return false;
            };
            let [worse] = worse_point.counts() else {
                return false;
            };
            let ratio = input.budgets().ratio();
            let left = worse.count().checked_mul(ratio.denominator());
            let right = measured.count().checked_mul(ratio.numerator());
            matches!((left, right), (Some(left), Some(right)) if left >= right)
        })
}

pub(super) fn lawful_judge(input: &WorkJudgmentInput<'_>) -> WorkJudgment {
    let owner_matches = complexity().is_ok_and(|expected| input.complexity() == expected)
        && input
            .formula()
            .is_some_and(|formula| formula.bytes() == LINEAR_FORMULA);
    let measured_holds =
        owner_matches && has_shape(input.measured(), input.budgets().samples(), linear_count);
    let worse_is_hostile = owner_matches
        && has_shape(
            input.planted_worse(),
            input.budgets().samples(),
            quadratic_count,
        );
    let measured = if measured_holds {
        WorkConclusion::Satisfied
    } else {
        WorkConclusion::Refused(MEASURED_REFUSED)
    };
    let planted_worse = if worse_is_hostile {
        WorkConclusion::Refused(WORSE_REFUSED)
    } else {
        WorkConclusion::Satisfied
    };
    let gap = if owner_matches && worse_is_hostile && has_declared_gap(input) {
        WorkGapStanding::Distinguished
    } else {
        WorkGapStanding::NotDistinguished(GAP_REFUSED)
    };
    WorkJudgment::stated(measured, planted_worse, gap)
}

pub(super) fn preflight_passes(_invocation: &Invocation) -> TrialConclusion {
    TrialConclusion::Passed
}

fn trial_binding(
    call: fn(&Invocation) -> TrialConclusion,
) -> Result<TrialBinding, TrialTableRefusal> {
    let subject = SubjectRoute::named(OWNER, "neutral-workload")?;
    let check = CheckRef::named(OWNER, "bench-correctness")?;
    let row = Row::declared(
        ClaimRef::named(OWNER, "workload-is-correct")?,
        ExecutionSuite::named(OWNER, "bench-preflight")?,
        Classification::authored(
            vec![Role::named(OWNER, "benchmark")?],
            vec![Tag::named(OWNER, "neutral")?],
        )?,
        subject,
        check,
        PopulationRef::named(OWNER, "one-neutral-input")?,
        Origin::HandWritten,
    )?;
    let revision = RevisionBinding::declared(ContentAddress::derived(
        REVISION_TAG,
        b"neutral-bench-workload/v1",
    ));
    Ok(Binding::bound(
        row,
        ExecutableAttachment::attached(subject, check, revision, revision, call),
        Provenance::Unproduced,
    )?)
}

fn preflight_invocation(target: TargetBinding) -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1u32),
            ByteBudget::declared(8u64),
            TimeBudget::declared(1u64),
        ),
        target,
        TrialSite::located(module_path!(), file!(), line!(), "bench-preflight"),
        HarnessClock::unavailable(),
    )
}

pub(super) fn row_with_axis(sizes: Vec<u64>) -> Result<BenchRow, BenchStampRefusal> {
    let references =
        BenchReferences::declared(workload()?, preflight_ref()?, worse_ref()?, complexity()?);
    let measurement = BenchMeasurement::declared(
        InputSizeAxis::declared(sizes)?,
        DeclaredBudgets::declared(2u32, 1u32, 2u64, 1u64)?,
        ContentionPosture::NoDeclaredContention,
        Some(WorkFormula::encoded(LINEAR_FORMULA.to_vec())?),
    );
    Ok(BenchRow::declared(references, measurement)?)
}

pub(super) fn lawful_row() -> Result<BenchRow, BenchStampRefusal> {
    row_with_axis(vec![2u64, 4u64, 8u64])
}

pub(super) fn attachment_with_refs(
    workload_ref: WorkloadRef,
    planted_ref: PlantedWorseRef,
    complexity_ref: ComplexityClaimRef,
    measured_call: BenchCall,
    worse_call: BenchCall,
    judge: WorkJudge,
    observations: Vec<WorkObservationRef>,
) -> Result<BenchAttachment, BenchStampRefusal> {
    Ok(BenchAttachment::attached(
        workload_ref,
        measured_call,
        planted_ref,
        worse_call,
        WorkJudgeBinding::bound(complexity_ref, judge),
        observations,
    )?)
}

pub(super) fn lawful_attachment(
    measured_call: BenchCall,
    worse_call: BenchCall,
    judge: WorkJudge,
) -> Result<BenchAttachment, BenchStampRefusal> {
    attachment_with_refs(
        workload()?,
        worse_ref()?,
        complexity()?,
        measured_call,
        worse_call,
        judge,
        vec![observation()?],
    )
}

pub(super) fn preflight_with(
    reference: PreflightRef,
    call: fn(&Invocation) -> TrialConclusion,
    preflight_target: TargetBinding,
) -> Result<PreflightTrial, BenchStampRefusal> {
    Ok(PreflightTrial::bound(
        reference,
        trial_binding(call)?,
        preflight_invocation(preflight_target),
    ))
}

pub(super) fn lawful_preflight(
    call: fn(&Invocation) -> TrialConclusion,
) -> Result<PreflightTrial, BenchStampRefusal> {
    preflight_with(preflight_ref()?, call, target())
}

pub(super) fn binding(
    measured_call: BenchCall,
    worse_call: BenchCall,
    judge: WorkJudge,
    preflight_call: fn(&Invocation) -> TrialConclusion,
) -> Result<BenchBinding, BenchStampRefusal> {
    let row = lawful_row()?;
    let attachment = lawful_attachment(measured_call, worse_call, judge)?;
    let preflight = lawful_preflight(preflight_call)?;
    Ok(BenchBinding::bound(row, attachment, preflight)?)
}

pub(super) fn lawful_binding() -> Result<BenchBinding, BenchStampRefusal> {
    binding(measured, planted_worse, lawful_judge, preflight_passes)
}

macroonz_harness::generated_support! {
    expected: [
        185, 251, 251, 45, 168, 146, 85, 42, 248, 177, 196, 48, 117, 229, 207, 5,
        84, 120, 104, 25, 150, 41, 202, 2, 243, 73, 31, 148, 241, 22, 122, 34,
    ],
    harness: macroonz_harness,
    benches: {
        pub(super) fn lawful_table named("harness.bench.consumer", "neutral-benchmark-table") {
            provenance: Provenance::Unproduced,
            bindings: [lawful_binding()],
        }
    },
    reporter: {
        pub(super) fn render(report: &macroonz_harness::bench::BenchReport) {
            std::hint::black_box((report.table(), report.provenance(), report.denominator()));
            for reading in report.readings() {
                std::hint::black_box((reading.row().key(), reading.outcome().stage()));
            }
        }
    },
}
