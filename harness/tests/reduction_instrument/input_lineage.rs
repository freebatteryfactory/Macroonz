//! An input-bearing report must retain the actual baseline and the reduced witness as different coordinates.

use super::{PRESERVED_CAUSE, SCHEMA_TAG, revision_derived_from, trial_fixture};
use arbitrary::Unstructured;
use macroonz_harness::descriptor::{GeneratedSupportSchemaId, NamespacedName};
use macroonz_harness::generate::{
    ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget, ReductionPlan,
    ReductionProbeBinding, ReductionRefusal, capture_replay, reduce,
};
use macroonz_harness::identity::ContentAddress;
use macroonz_harness::input::{
    BoundInput, InputBinding, InputLimits, InputProfile, InputRefusal, pack,
};
use macroonz_harness::report::{
    FailureClass, FindingLocation, Fingerprint, GenerationProfile, MinimizationProfile, RunAttempt,
    TrialConclusion, TrialFinding, TrialReport,
};
use macroonz_harness::runner::Invocation;
use std::sync::atomic::{AtomicUsize, Ordering};

static REFUSAL_PROBES: AtomicUsize = AtomicUsize::new(0);

fn counted_no_failure(_input: &[u8]) -> ProbeOutcome {
    REFUSAL_PROBES.fetch_add(1, Ordering::SeqCst);
    ProbeOutcome::NoFailure
}

fn bytes(source: &mut Unstructured<'_>) -> arbitrary::Result<Vec<u8>> {
    source.bytes(source.len()).map(<[u8]>::to_vec)
}

fn subject(invocation: &Invocation<BoundInput<Vec<u8>>>) -> TrialConclusion {
    if invocation.input().value().contains(&1) {
        TrialConclusion::Refused(TrialFinding::established(
            FailureClass::PropertyDisagreement,
            PRESERVED_CAUSE,
            FindingLocation::at(file!(), line!()),
            None,
        ))
    } else {
        TrialConclusion::Passed
    }
}

fn report(input: &[u8]) -> Option<TrialReport> {
    let profile = InputProfile::declared(
        NamespacedName::named("reduction", "bytes").ok()?,
        1,
        ContentAddress::derived(SCHEMA_TAG, b"byte sequence"),
    );
    let revision = revision_derived_from(include_bytes!("input_lineage.rs"));
    let envelope = pack(profile, input, InputLimits::declared(4096, 64)).ok()?;
    let admitted = InputBinding::declared(profile, revision, bytes)
        .decode(envelope)
        .ok()?;
    let fixture = trial_fixture()?;
    fixture.report_under(
        subject,
        revision,
        &fixture.invocation().with_input(admitted),
    )
}

fn fingerprint(report: &TrialReport) -> Option<Fingerprint> {
    match report.attempt() {
        RunAttempt::Executed(TrialConclusion::Refused(finding)) => {
            Some(Fingerprint::of(report.trial(), finding))
        }
        RunAttempt::Executed(TrialConclusion::Passed)
        | RunAttempt::SkippedWithReason(_)
        | RunAttempt::TimedOut
        | RunAttempt::InfrastructureFailed(_) => None,
    }
}

fn probe(input: &[u8]) -> ProbeOutcome {
    report(input)
        .as_ref()
        .and_then(fingerprint)
        .map_or(ProbeOutcome::NoFailure, ProbeOutcome::Reproduced)
}

fn plan() -> Result<ReductionPlan, ()> {
    ReductionPlan::declared(
        MinimizationProfile::declared("input-lineage", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(32),
    )
    .map_err(|_| ())
}

fn binding(report: &TrialReport) -> Result<ReductionProbeBinding, ()> {
    binding_with(report, probe)
}

fn binding_with(
    report: &TrialReport,
    probe: fn(&[u8]) -> ProbeOutcome,
) -> Result<ReductionProbeBinding, ()> {
    ReductionProbeBinding::bound(
        report,
        GenerationProfile::declared("input-lineage", 1),
        GeneratedSupportSchemaId::over(ContentAddress::derived(SCHEMA_TAG, b"byte sequence")),
        revision_derived_from(include_bytes!("input_lineage.rs")),
        probe,
    )
    .map_err(|_| ())
}

#[test]
fn another_starting_case_cannot_borrow_the_original_reports_standing() -> Result<(), ()> {
    let original = report(&[1, 2, 3]).ok_or(())?;
    let other = report(&[1, 9]).ok_or(())?;
    assert_eq!(fingerprint(&original), fingerprint(&other));
    assert!(fingerprint(&original).is_some());
    assert_ne!(
        original.standing().key().input(),
        other.standing().key().input()
    );
    assert_eq!(
        reduce(&plan()?, &[1, 9], &binding(&original)?),
        Err(ReductionRefusal::BaselineCaseDiffers {
            expected: original.standing().key().input().ok_or(())?.case(),
            found: other.standing().key().input().ok_or(())?.case(),
        })
    );
    Ok(())
}

#[test]
fn input_admission_precedes_the_probe_and_does_not_replace_reproduction() -> Result<(), ()> {
    let original = report(&[1, 2, 3]).ok_or(())?;
    let empty = report(&[]).ok_or(())?;
    let binding = binding_with(&original, counted_no_failure)?;
    assert_eq!(REFUSAL_PROBES.load(Ordering::SeqCst), 0);
    assert_eq!(
        reduce(&plan()?, &[1; 65], &binding),
        Err(ReductionRefusal::BaselineInputRefused {
            cause: InputRefusal::PayloadTooLarge
        })
    );
    assert_eq!(
        reduce(&plan()?, &[], &binding),
        Err(ReductionRefusal::BaselineCaseDiffers {
            expected: original.standing().key().input().ok_or(())?.case(),
            found: empty.standing().key().input().ok_or(())?.case(),
        })
    );
    assert_eq!(REFUSAL_PROBES.load(Ordering::SeqCst), 0);
    assert_eq!(
        reduce(&plan()?, &[1, 2, 3], &binding),
        Err(ReductionRefusal::BaselineDidNotFail)
    );
    assert_eq!(REFUSAL_PROBES.load(Ordering::SeqCst), 1);
    Ok(())
}

#[test]
fn the_original_case_and_reached_witness_keep_distinct_execution_keys() -> Result<(), ()> {
    let original = report(&[1, 2, 3]).ok_or(())?;
    let reduced = reduce(&plan()?, &[1, 2, 3], &binding(&original)?).map_err(|_| ())?;
    let capsule = capture_replay(&reduced);
    assert_eq!(capsule.input(), &[1]);
    assert_eq!(capsule.key(), original.standing().key());
    let fresh = report(capsule.input()).ok_or(())?;
    assert_eq!(fingerprint(&fresh), Some(capsule.fingerprint()));
    assert_ne!(fresh.standing().key(), capsule.key());
    Ok(())
}
