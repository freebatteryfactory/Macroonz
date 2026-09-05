//! Disposable caller-side declarations and rendering over the existing benchmark receiver.
//! The harness owns execution, preflight, work judgment, samples, and refusal; this host supplies a clock.

use macroonz_harness::bench::{
    BenchAttachment, BenchBinding, BenchCall, BenchInvocation, BenchMeasurement, BenchOutcome,
    BenchReferences, BenchRow, BenchTable, BenchTableName, ComplexityClaimRef, ContentionPosture,
    DeclaredBudgets, InputSizeAxis, PlantedWorseRef, PreflightRef, PreflightTrial, WorkFormula,
    WorkJudgeBinding, WorkJudgment, WorkJudgmentInput, WorkObservationRef, WorkloadRef,
    bench_verdict, run_all,
};
use macroonz_harness::clock::{ClockReadRefusal, HarnessClock};
use macroonz_harness::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, DerivedRevision, ExecutableAttachment,
    ExecutionSuite, Origin, PopulationRef, Provenance, RevisionBinding, Role, Row, SubjectRoute,
    Tag,
};
use macroonz_harness::identity::encode_bytes;
use macroonz_harness::report::{
    ByteBudget, CaseBudget, InvocationProfile, TargetBinding, TargetTriple, TimeBudget,
    ToolchainIdentity, TrialConclusion, TrialSite,
};
use macroonz_harness::runner::Invocation;
use std::sync::OnceLock;
use std::time::Instant;

pub(super) const SAMPLES: u32 = 5;
pub(super) const SINGLE: &[u8] = b"one-operation-batch-per-call";
pub(super) const DOUBLE: &[u8] = b"two-operation-batches-per-call";
static START: OnceLock<Instant> = OnceLock::new();

pub(super) struct Workload {
    pub owner: &'static str,
    pub interval: &'static str,
    pub sizes: &'static [u64],
    pub observations: &'static [&'static str],
    pub sources: &'static [&'static [u8]],
    pub preflight: fn(&Invocation) -> TrialConclusion,
    pub judge: fn(&WorkJudgmentInput<'_>) -> WorkJudgment,
    pub once: BenchCall,
    pub twice: BenchCall,
    pub four_times: BenchCall,
}

pub(super) fn debug(value: impl core::fmt::Debug) -> String {
    format!("{value:?}")
}

fn clock() -> Result<u64, ClockReadRefusal> {
    u64::try_from(START.get_or_init(Instant::now).elapsed().as_nanos())
        .map_err(|_| ClockReadRefusal::Refused)
}

fn target() -> TargetBinding {
    TargetBinding::bound(
        TargetTriple::declared(env!("PILOT_TARGET")),
        ToolchainIdentity::declared(env!("PILOT_TOOLCHAIN")),
    )
}

fn table(
    subject: &Workload,
    name: &'static str,
    formula: &[u8],
    measured: BenchCall,
    worse: BenchCall,
) -> Result<BenchTable, String> {
    let owner = subject.owner;
    let workload = WorkloadRef::named(owner, name).map_err(debug)?;
    let check = CheckRef::named(owner, "independent-preflight").map_err(debug)?;
    let route = SubjectRoute::named(owner, "public-generated-rust").map_err(debug)?;
    let preflight = PreflightRef::named(owner, "correctness").map_err(debug)?;
    let planted = PlantedWorseRef::named(owner, "repeated-actual-execution").map_err(debug)?;
    let complexity =
        ComplexityClaimRef::named(owner, "completed-work-not-asymptotics").map_err(debug)?;
    let row = Row::declared(
        ClaimRef::named(owner, "independent-behavior").map_err(debug)?,
        ExecutionSuite::named(owner, "preflight").map_err(debug)?,
        Classification::authored(
            vec![Role::named(owner, "benchmark").map_err(debug)?],
            vec![Tag::named(owner, "pilot").map_err(debug)?],
        )
        .map_err(debug)?,
        route,
        check,
        PopulationRef::named(owner, "declared-sizes").map_err(debug)?,
        Origin::HandWritten,
    )
    .map_err(debug)?;
    let mut material = Vec::new();
    for bytes in [
        env!("PILOT_SOURCE").as_bytes(),
        include_bytes!("measurement.rs"),
    ]
    .into_iter()
    .chain(subject.sources.iter().copied())
    {
        encode_bytes(bytes, &mut material);
    }
    let revision = RevisionBinding::derived(DerivedRevision::from_material(&material));
    let trial = Binding::bound(
        row,
        ExecutableAttachment::attached(route, check, revision, revision, subject.preflight),
        Provenance::Unproduced,
    )
    .map_err(debug)?;
    let invocation = Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1),
            ByteBudget::declared(65_536),
            TimeBudget::declared(60_000_000_000),
        ),
        target(),
        TrialSite::located(module_path!(), file!(), line!(), "runtime-preflight"),
        HarnessClock::unavailable(),
    );
    let row = BenchRow::declared(
        BenchReferences::declared(workload, preflight, planted, complexity),
        BenchMeasurement::declared(
            InputSizeAxis::declared(subject.sizes.to_vec()).map_err(debug)?,
            DeclaredBudgets::declared(SAMPLES, 2, 2, 1).map_err(debug)?,
            ContentionPosture::NoDeclaredContention,
            Some(WorkFormula::encoded(formula.to_vec()).map_err(debug)?),
        ),
    )
    .map_err(debug)?;
    let attachment = BenchAttachment::attached(
        workload,
        measured,
        planted,
        worse,
        WorkJudgeBinding::bound(complexity, subject.judge),
        subject
            .observations
            .iter()
            .map(|name| WorkObservationRef::named(owner, name).map_err(debug))
            .collect::<Result<Vec<_>, _>>()?,
    )
    .map_err(debug)?;
    let binding = BenchBinding::bound(
        row,
        attachment,
        PreflightTrial::bound(preflight, trial, invocation),
    )
    .map_err(debug)?;
    BenchTable::authored(
        BenchTableName::named(owner, name).map_err(debug)?,
        Provenance::Unproduced,
        vec![binding],
    )
    .map_err(debug)
}

pub(super) fn measure(subject: &Workload) -> Result<(), String> {
    if subject.sources.is_empty() || subject.sources.iter().any(|source| source.is_empty()) {
        return Err("missing declared workload source component".to_owned());
    }
    println!(
        "pilot,{},source={},target={},toolchain={},profile=release,interval={}",
        subject.owner,
        env!("PILOT_SOURCE"),
        env!("PILOT_TARGET"),
        env!("PILOT_TOOLCHAIN"),
        subject.interval,
    );
    let invocation = BenchInvocation::declared(
        target(),
        HarnessClock::fallible(clock),
        ContentionPosture::NoDeclaredContention,
    );
    let identical = run_all(
        &table(
            subject,
            "identical-control",
            SINGLE,
            subject.once,
            subject.once,
        )?,
        &invocation,
    )
    .map_err(debug)?;
    let [reading] = identical.readings() else {
        return Err("missing identical control".to_owned());
    };
    if !matches!(
        reading.outcome(),
        BenchOutcome::PlantedWorseNotDistinguished { .. }
    ) {
        return Err("identical control was not refused".to_owned());
    }
    for round in 0..4_u32 {
        let single_a = ("single-a", SINGLE, subject.once, subject.twice);
        let single_b = ("single-b", SINGLE, subject.once, subject.twice);
        let double = (
            "double-execution",
            DOUBLE,
            subject.twice,
            subject.four_times,
        );
        let order = if round % 2 == 0 {
            [single_a, single_b, double]
        } else {
            [double, single_b, single_a]
        };
        for (name, formula, measured, worse) in order {
            let report = run_all(
                &table(subject, name, formula, measured, worse)?,
                &invocation,
            )
            .map_err(debug)?;
            bench_verdict(&report).map_err(debug)?;
            let [reading] = report.readings() else {
                return Err("missing measurement row".to_owned());
            };
            let BenchOutcome::Qualified { secondary, .. } = reading.outcome() else {
                return Err("row did not qualify".to_owned());
            };
            let measurements = secondary.measurements();
            let count = usize::try_from(SAMPLES).map_err(debug)?;
            if Some(measurements.len()) != subject.sizes.len().checked_mul(count) {
                return Err("sample denominator changed".to_owned());
            }
            for (size, samples) in subject.sizes.iter().zip(measurements.chunks_exact(count)) {
                for (sample, reading) in samples.iter().enumerate() {
                    let duration = reading.duration().ok_or("missing actual duration")?;
                    println!(
                        "sample,{},{round},{name},{size},{sample},{}",
                        subject.owner,
                        duration.nanoseconds()
                    );
                }
            }
        }
    }
    Ok(())
}
