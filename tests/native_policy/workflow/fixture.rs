//! A neutral byte-count subject and its separately authored length oracle.

use macroonz::harness::clock::HarnessClock;
use macroonz::harness::descriptor::{
    AuthoredTableName, Binding, CheckRef, ClaimRef, Classification, DerivedRevision,
    ExecutableAttachment, ExecutionSuite, GeneratedSupportSchemaId, NamespacedName, Origin,
    PopulationRef, Provenance, RevisionBinding, Role, Row, SubjectRoute, Tag,
};
use macroonz::harness::generate::{
    ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget, ReductionPlan,
    ReductionProbeBinding, capture_replay, reduce,
};
use macroonz::harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz::harness::input::{BoundInput, InputBinding, InputLimits, InputProfile};
use macroonz::harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, FindingLocation, Fingerprint,
    GenerationProfile, InvocationProfile, MinimizationProfile, ReplayCapsule, RunAttempt,
    TargetBinding, TargetTriple, TimeBudget, ToolchainIdentity, TrialConclusion, TrialFinding,
    TrialReport, TrialSite,
};
use macroonz::harness::runner::{
    Invocation, Selection, SelectionPlan, TrialBinding, TrialTable, trial_identity,
};
use macroonz::workflow::{self, InputRun};
use std::cell::Cell;

pub(super) const INPUT_LIMITS: InputLimits = InputLimits::declared(4096, 64);
std::thread_local! {
    static DECODES: Cell<u32> = const { Cell::new(0) };
    static CALLS: Cell<u32> = const { Cell::new(0) };
    static PROBES: Cell<u32> = const { Cell::new(0) };
}

pub(super) fn mapped<T>(result: Result<T, impl std::fmt::Debug>) -> Result<T, String> {
    result.map_err(|error| format!("{error:?}"))
}
pub(super) fn reset() {
    DECODES.set(0);
    CALLS.set(0);
    PROBES.set(0);
}
pub(super) fn observations() -> (u32, u32, u32) {
    (DECODES.get(), CALLS.get(), PROBES.get())
}

pub(super) fn revision() -> RevisionBinding {
    RevisionBinding::derived(DerivedRevision::from_material(
        b"workflow byte-count control",
    ))
}
#[cfg(feature = "native-tooling")]
pub(super) fn fixed_revision() -> RevisionBinding {
    RevisionBinding::derived(DerivedRevision::from_material(
        b"workflow byte-count correction",
    ))
}

pub(super) fn decoder() -> Result<InputBinding<Vec<u8>>, String> {
    let schema = ContentAddress::derived(
        DomainTag::declared("workflow-count", IdentityProfileVersion::declared(1)),
        b"each byte denotes one item",
    );
    let profile = InputProfile::declared(
        mapped(NamespacedName::named("workflow-count", "bytes"))?,
        1,
        schema,
    );
    Ok(InputBinding::declared(profile, revision(), |source| {
        DECODES.set(DECODES.get().saturating_add(1));
        source.bytes(source.len()).map(<[u8]>::to_vec)
    }))
}

fn row(population: &'static str) -> Result<Row, String> {
    mapped(Row::declared(
        mapped(ClaimRef::named("workflow-count", "every-byte-counts"))?,
        mapped(ExecutionSuite::named("workflow-count", "model"))?,
        mapped(Classification::authored(
            vec![mapped(Role::named("workflow-count", "property"))?],
            vec![mapped(Tag::named("workflow-count", "bytes"))?],
        ))?,
        mapped(SubjectRoute::named("workflow-count", "count"))?,
        mapped(CheckRef::named("workflow-count", "length-oracle"))?,
        mapped(PopulationRef::named("workflow-count", population))?,
        Origin::HandWritten,
    ))
}

pub(super) fn binding(
    population: &'static str,
    revision: RevisionBinding,
    call: fn(&Invocation<BoundInput<Vec<u8>>>) -> TrialConclusion,
) -> Result<TrialBinding<BoundInput<Vec<u8>>>, String> {
    let row = row(population)?;
    let attachment = ExecutableAttachment::attached(
        row.subject(),
        row.check(),
        revision,
        self::revision(),
        call,
    );
    mapped(Binding::bound(row, attachment, Provenance::Unproduced))
}

pub(super) fn table() -> Result<TrialTable<BoundInput<Vec<u8>>>, String> {
    mapped(TrialTable::authored(
        mapped(AuthoredTableName::named("workflow-count", "world"))?,
        Provenance::Unproduced,
        vec![
            binding("selected", revision(), defective)?,
            binding("unselected", revision(), unselected)?,
        ],
    ))
}

pub(super) fn selection() -> Result<SelectionPlan, String> {
    Ok(SelectionPlan::of(Selection::ByTrialIds(
        [trial_identity(&row("selected")?)].into_iter().collect(),
    )))
}

pub(super) fn invocation(cases: u32) -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(cases),
            ByteBudget::declared(64),
            TimeBudget::declared(1000),
        ),
        TargetBinding::bound(
            TargetTriple::declared("declared-workflow-host"),
            ToolchainIdentity::declared("declared-workflow-toolchain"),
        ),
        TrialSite::located(module_path!(), file!(), line!(), "byte count"),
        HarnessClock::unavailable(),
    )
}

fn judge(actual: usize, expected: usize) -> TrialConclusion {
    CALLS.set(CALLS.get().saturating_add(1));
    if actual == expected {
        TrialConclusion::Passed
    } else {
        TrialConclusion::Refused(TrialFinding::established(
            FailureClass::PropertyDisagreement,
            FindingCause::named("workflow-count", "length-disagreement"),
            FindingLocation::at(file!(), line!()),
            None,
        ))
    }
}

pub(super) fn defective(invocation: &Invocation<BoundInput<Vec<u8>>>) -> TrialConclusion {
    let values = invocation.input().value();
    let actual = values.iter().filter(|value| **value != 1u8).count();
    judge(actual, values.len())
}

#[cfg(feature = "native-tooling")]
pub(super) fn fixed(invocation: &Invocation<BoundInput<Vec<u8>>>) -> TrialConclusion {
    let values = invocation.input().value();
    let actual = values
        .iter()
        .enumerate()
        .next_back()
        .map_or(0, |(index, _value)| index.saturating_add(1));
    judge(actual, values.len())
}

fn unselected(_invocation: &Invocation<BoundInput<Vec<u8>>>) -> TrialConclusion {
    std::panic::resume_unwind(Box::new("an unselected row executed"))
}

pub(super) fn run(payload: &[u8]) -> Result<InputRun, String> {
    mapped(workflow::run(
        &table()?.view(),
        &selection()?,
        &decoder()?,
        payload,
        invocation(1),
        INPUT_LIMITS,
    ))
}

pub(super) fn selected(run: &InputRun) -> Result<&TrialReport, String> {
    run.report()
        .census()
        .first()
        .and_then(|row| row.disposition().report())
        .ok_or_else(|| "selected report missing".to_owned())
}

fn probe(payload: &[u8]) -> ProbeOutcome {
    PROBES.set(PROBES.get().saturating_add(1));
    let Ok(run) = run(payload) else {
        return ProbeOutcome::NoFailure;
    };
    let Ok(report) = selected(&run) else {
        return ProbeOutcome::NoFailure;
    };
    match report.attempt() {
        RunAttempt::Executed(TrialConclusion::Refused(finding)) => {
            ProbeOutcome::Reproduced(Fingerprint::of(report.trial(), finding))
        }
        RunAttempt::Executed(TrialConclusion::Passed)
        | RunAttempt::SkippedWithReason(_)
        | RunAttempt::TimedOut
        | RunAttempt::InfrastructureFailed(_) => ProbeOutcome::NoFailure,
    }
}

pub(super) fn capsule(run: &InputRun) -> Result<ReplayCapsule, String> {
    let binding = mapped(ReductionProbeBinding::bound(
        selected(run)?,
        GenerationProfile::declared("workflow-input", 1),
        GeneratedSupportSchemaId::over(decoder()?.profile().schema()),
        revision(),
        probe,
    ))?;
    let plan = mapped(ReductionPlan::declared(
        MinimizationProfile::declared("workflow-shrink", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(32),
    ))?;
    Ok(capture_replay(&mapped(reduce(
        &plan,
        run.input().payload(),
        &binding,
    ))?))
}
