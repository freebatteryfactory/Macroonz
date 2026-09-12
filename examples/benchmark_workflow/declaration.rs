//! Caller-owned benchmark names, revisions and execution bindings.

use super::specimen::{self, OWNER, mapped};
use macroonz::harness::bench::{
    BenchAttachment, BenchBinding, BenchCall, BenchInvocation, BenchMeasurement, BenchReferences,
    BenchRow, BenchTable, BenchTableName, ComplexityClaimRef, ContentionPosture, DeclaredBudgets,
    InputSizeAxis, PlantedWorseRef, PreflightRef, PreflightTrial, WorkJudge, WorkJudgeBinding,
    WorkloadRef,
};
use macroonz::harness::clock::HarnessClock;
use macroonz::harness::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, DerivedRevision, ExecutableAttachment,
    ExecutionSuite, Origin, PopulationRef, Provenance, RevisionBinding, Role, Row, SubjectRoute,
    Tag,
};
use macroonz::harness::report::{
    ByteBudget, CaseBudget, InvocationProfile, TargetBinding, TargetTriple, TimeBudget,
    ToolchainIdentity, TrialConclusion, TrialSite,
};
use macroonz::harness::runner::{Invocation, TrialBinding};

pub(crate) fn target() -> TargetBinding {
    TargetBinding::bound(
        TargetTriple::declared("neutral-counting-host"),
        ToolchainIdentity::declared("1.98.1"),
    )
}

pub(crate) fn invocation(clock: HarnessClock) -> BenchInvocation {
    BenchInvocation::declared(target(), clock, ContentionPosture::NoDeclaredContention)
}

fn preflight_binding(call: fn(&Invocation) -> TrialConclusion) -> Result<TrialBinding, String> {
    let subject = mapped(SubjectRoute::named(OWNER, "count-nonzero"))?;
    let check = mapped(CheckRef::named(OWNER, "literal-cases"))?;
    let row = mapped(Row::declared(
        mapped(ClaimRef::named(OWNER, "correct-count"))?,
        mapped(ExecutionSuite::named(OWNER, "preflight"))?,
        mapped(Classification::authored(
            vec![mapped(Role::named(OWNER, "correctness"))?],
            vec![mapped(Tag::named(OWNER, "counting"))?],
        ))?,
        subject,
        check,
        mapped(PopulationRef::named(OWNER, "literal-cases"))?,
        Origin::HandWritten,
    ))?;
    mapped(Binding::bound(
        row,
        ExecutableAttachment::attached(
            subject,
            check,
            RevisionBinding::derived(DerivedRevision::from_material(b"nonzero loop/v1")),
            RevisionBinding::derived(DerivedRevision::from_material(b"literal nonzero cases/v1")),
            call,
        ),
        Provenance::Unproduced,
    ))
}

pub(crate) fn binding(
    name: &'static str,
    measured: BenchCall,
    worse: BenchCall,
    judge: WorkJudge,
    preflight: fn(&Invocation) -> TrialConclusion,
) -> Result<BenchBinding, String> {
    let workload = mapped(WorkloadRef::named(OWNER, name))?;
    let correctness = mapped(PreflightRef::named(OWNER, "literal-cases"))?;
    let control = mapped(PlantedWorseRef::named(OWNER, "repeated-scan"))?;
    let complexity = mapped(ComplexityClaimRef::named(OWNER, "one-visit-per-element"))?;
    let row = mapped(BenchRow::declared(
        BenchReferences::declared(workload, correctness, control, complexity),
        BenchMeasurement::declared(
            mapped(InputSizeAxis::declared(vec![4, 8, 16]))?,
            mapped(DeclaredBudgets::declared(2, 1, 2, 1))?,
            ContentionPosture::NoDeclaredContention,
            None,
        ),
    ))?;
    let attachment = mapped(BenchAttachment::attached(
        workload,
        measured,
        control,
        worse,
        WorkJudgeBinding::bound(complexity, judge),
        vec![mapped(specimen::observation())?],
    ))?;
    let invocation = Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(4),
            ByteBudget::declared(80),
            TimeBudget::declared(1000),
        ),
        target(),
        TrialSite::located(module_path!(), file!(), line!(), "nonzero count"),
        HarnessClock::unavailable(),
    );
    mapped(BenchBinding::bound(
        row,
        attachment,
        PreflightTrial::bound(correctness, preflight_binding(preflight)?, invocation),
    ))
}

pub(crate) fn table(bindings: Vec<BenchBinding>) -> Result<BenchTable, String> {
    mapped(BenchTable::authored(
        mapped(BenchTableName::named(OWNER, "counting"))?,
        Provenance::Unproduced,
        bindings,
    ))
}
