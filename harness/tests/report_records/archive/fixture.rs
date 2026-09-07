//! Actual typed execution and reduction provide the earned writer specimen.

use arbitrary::Unstructured;
use macroonz_harness::clock::{HarnessClock, MeasurementReading};
use macroonz_harness::descriptor::{
    Binding, DerivedRevision, ExecutableAttachment, GeneratedSupportSchemaId, NamespacedName,
    Provenance, RevisionBinding, Row,
};
use macroonz_harness::generate::{
    ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget, ReductionPlan,
    ReductionProbeBinding, capture_replay, reduce,
};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::input::{BoundInput, InputBinding, InputLimits, InputProfile, pack};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingLocation, Fingerprint, GenerationProfile,
    HostTrialRecord, InvocationProfile, MinimizationProfile, ReplayCapsule, RunAttempt,
    TargetBinding, TargetTriple, TimeBudget, ToolchainIdentity, TrialConclusion, TrialFinding,
    TrialReport, TrialSite,
};
use macroonz_harness::runner::{Invocation, TrialBinding, record_one, run_one, trial_identity};
use std::cell::Cell;

std::thread_local! {
    static DECODES: Cell<u8> = const { Cell::new(0) };
}

pub(super) fn reset_decodes() {
    DECODES.set(0);
}

pub(super) fn decode_count() -> u8 {
    DECODES.get()
}

pub(super) fn record_decode() {
    DECODES.set(DECODES.get().saturating_add(1));
}

pub(super) fn revision() -> RevisionBinding {
    RevisionBinding::derived(DerivedRevision::from_material(include_bytes!("fixture.rs")))
}

fn schema() -> ContentAddress {
    ContentAddress::derived(
        DomainTag::declared("archive-fixture", IdentityProfileVersion::declared(1)),
        b"byte sequence",
    )
}

pub(super) fn bytes(source: &mut Unstructured<'_>) -> arbitrary::Result<Vec<u8>> {
    record_decode();
    source.bytes(source.len()).map(<[u8]>::to_vec)
}

pub(super) fn subject(invocation: &Invocation<BoundInput<Vec<u8>>>) -> TrialConclusion {
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
    Ok(run_one(&binding(call)?, invocation))
}

pub(super) fn binding<Input>(
    call: fn(&Invocation<Input>) -> TrialConclusion,
) -> Result<TrialBinding<Input>, ()> {
    binding_under(super::super::row()?, revision(), revision(), call)
}

pub(super) fn binding_under<Input>(
    row: Row,
    subject: RevisionBinding,
    check: RevisionBinding,
    call: fn(&Invocation<Input>) -> TrialConclusion,
) -> Result<TrialBinding<Input>, ()> {
    let attachment =
        ExecutableAttachment::attached(row.subject(), row.check(), subject, check, call);
    Binding::bound(row, attachment, Provenance::Unproduced).map_err(|_| ())
}

pub(super) fn host_report(
    attempt: RunAttempt,
    measurement: MeasurementReading,
) -> Result<TrialReport, ()> {
    let binding = binding(|_| TrialConclusion::Passed)?;
    let record = HostTrialRecord::recorded(trial_identity(binding.row()), attempt, measurement);
    record_one(&binding, &invocation(), record).map_err(|_| ())
}

pub(super) fn report(payload: &[u8]) -> Result<TrialReport, ()> {
    execute(subject, &typed_invocation(payload)?)
}

pub(super) fn typed_invocation(payload: &[u8]) -> Result<Invocation<BoundInput<Vec<u8>>>, ()> {
    let decoder = decoder()?;
    let envelope =
        pack(decoder.profile(), payload, InputLimits::declared(4096, 64)).map_err(|_| ())?;
    let input = decoder.decode(envelope).map_err(|_| ())?;
    Ok(invocation().with_input(input))
}

pub(super) fn decoder() -> Result<InputBinding<Vec<u8>>, ()> {
    let profile = InputProfile::declared(
        NamespacedName::named("archive", "bytes").map_err(|_| ())?,
        1,
        schema(),
    );
    Ok(InputBinding::declared(profile, revision(), bytes))
}

pub(super) fn invocation() -> Invocation {
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
    capsule_for(&[1, 2, 3])
}

pub(super) fn capsule_for(payload: &[u8]) -> Result<ReplayCapsule, ()> {
    let report = report(payload)?;
    reduce_report(&report, payload)
}

pub(super) fn unit_capsule() -> Result<ReplayCapsule, ()> {
    let report = execute(|_| refusal(), &invocation())?;
    reduce_report(&report, &[1, 2, 3])
}

fn reduce_report(report: &TrialReport, payload: &[u8]) -> Result<ReplayCapsule, ()> {
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
    let evidence = reduce(&plan, payload, &binding).map_err(|_| ())?;
    Ok(capture_replay(&evidence))
}
