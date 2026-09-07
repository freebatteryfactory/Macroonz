//! Actual typed execution and reduction provide the earned writer specimen.

use arbitrary::Unstructured;
use macroonz_harness::clock::HarnessClock;
use macroonz_harness::descriptor::{
    Binding, DerivedRevision, ExecutableAttachment, GeneratedSupportSchemaId, NamespacedName,
    Provenance, RevisionBinding,
};
use macroonz_harness::generate::{
    ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget, ReductionPlan,
    ReductionProbeBinding, capture_replay, reduce,
};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::input::{BoundInput, InputBinding, InputLimits, InputProfile, pack};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingLocation, Fingerprint, GenerationProfile,
    InvocationProfile, MinimizationProfile, ReplayCapsule, RunAttempt, TargetBinding, TargetTriple,
    TimeBudget, ToolchainIdentity, TrialConclusion, TrialFinding, TrialReport, TrialSite,
};
use macroonz_harness::runner::{Invocation, run_one};

fn revision() -> RevisionBinding {
    RevisionBinding::derived(DerivedRevision::from_material(include_bytes!("fixture.rs")))
}

fn schema() -> ContentAddress {
    ContentAddress::derived(
        DomainTag::declared("archive-fixture", IdentityProfileVersion::declared(1)),
        b"byte sequence",
    )
}

fn bytes(source: &mut Unstructured<'_>) -> arbitrary::Result<Vec<u8>> {
    source.bytes(source.len()).map(<[u8]>::to_vec)
}

fn subject(invocation: &Invocation<BoundInput<Vec<u8>>>) -> TrialConclusion {
    if invocation.input().value().contains(&1) {
        return refusal();
    }
    TrialConclusion::Passed
}

fn refusal() -> TrialConclusion {
    TrialConclusion::Refused(TrialFinding::established(
        FailureClass::PropertyDisagreement,
        super::super::CAUSE,
        FindingLocation::at(file!(), line!()),
        None,
    ))
}

fn execute<Input>(
    call: fn(&Invocation<Input>) -> TrialConclusion,
    invocation: &Invocation<Input>,
) -> Result<TrialReport, ()> {
    let row = super::super::row()?;
    let attachment =
        ExecutableAttachment::attached(row.subject(), row.check(), revision(), revision(), call);
    let binding = Binding::bound(row, attachment, Provenance::Unproduced).map_err(|_| ())?;
    Ok(run_one(&binding, invocation))
}

pub(super) fn report(payload: &[u8]) -> Result<TrialReport, ()> {
    let profile = InputProfile::declared(
        NamespacedName::named("archive", "bytes").map_err(|_| ())?,
        1,
        schema(),
    );
    let envelope = pack(profile, payload, InputLimits::declared(4096, 64)).map_err(|_| ())?;
    let input = InputBinding::declared(profile, revision(), bytes)
        .decode(envelope)
        .map_err(|_| ())?;
    execute(subject, &invocation().with_input(input))
}

fn invocation() -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1),
            ByteBudget::declared(64),
            TimeBudget::declared(1000),
        ),
        TargetBinding::bound(
            TargetTriple::declared("declared-archive-fixture"),
            ToolchainIdentity::declared("declared-fixture-toolchain"),
        ),
        TrialSite::located(module_path!(), file!(), line!(), "archive subject"),
        HarnessClock::unavailable(),
    )
}

fn probe(payload: &[u8]) -> ProbeOutcome {
    let Ok(report) = report(payload) else {
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

pub(super) fn capsule() -> Result<ReplayCapsule, ()> {
    let report = report(&[1, 2, 3])?;
    reduce_report(&report)
}

pub(super) fn unit_capsule() -> Result<ReplayCapsule, ()> {
    let report = execute(|_| refusal(), &invocation())?;
    reduce_report(&report)
}

fn reduce_report(report: &TrialReport) -> Result<ReplayCapsule, ()> {
    let binding = ReductionProbeBinding::bound(
        report,
        GenerationProfile::declared("archive-byte-sequence", 1),
        GeneratedSupportSchemaId::over(schema()),
        revision(),
        probe,
    )
    .map_err(|_| ())?;
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("archive-reduction", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(32),
    )
    .map_err(|_| ())?;
    let evidence = reduce(&plan, &[1, 2, 3], &binding).map_err(|_| ())?;
    Ok(capture_replay(&evidence))
}
